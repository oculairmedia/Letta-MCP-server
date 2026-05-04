use crate::tools::response_utils::limits::max_value_len;
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use chrono::{DateTime, Utc};
use letta::LettaClient;
use letta::types::{LettaMessageUnion, ListMessagesRequest};
use serde_json::Value;
use turbomcp::McpError;

use super::{
    ArchivalSearchResult, MemoryUnifiedRequest, MessageMatch, MessageSearchResult, SearchSource,
};
use crate::tools::response_utils::ToolResponse;

pub(crate) async fn handle_search_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for search_memory")?;
    let query = require_field(request.query, "query is required for search_memory")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let source = request.source.unwrap_or_default();
    let limit = request.limit.unwrap_or(50) as usize;
    let start_date = request.start_date;
    let end_date = request.end_date;

    let (archival_result, messages_result) = match source {
        SearchSource::Both => {
            let (arch, msgs) = tokio::join!(
                search_archival_memory(client, &letta_id, &query, limit, verbose, start_date, end_date),
                search_messages(client, &letta_id, &query, limit, verbose, start_date, end_date)
            );
            (Some(arch?), Some(msgs?))
        }
        SearchSource::Archival => {
            let arch =
                search_archival_memory(client, &letta_id, &query, limit, verbose, start_date, end_date)
                    .await?;
            (Some(arch), None)
        }
        SearchSource::Messages => {
            let msgs =
                search_messages(client, &letta_id, &query, limit, verbose, start_date, end_date).await?;
            (None, Some(msgs))
        }
    };

    let archival_count = archival_result.as_ref().map(|r| r.count).unwrap_or(0);
    let messages_count = messages_result.as_ref().map(|r| r.count).unwrap_or(0);

    Ok(ToolResponse::success(
        "search_memory",
        format!(
            "Found {} archival passages and {} messages",
            archival_count, messages_count
        ),
    )
    .with_count(archival_count + messages_count)
    .with_extra(serde_json::json!({
        "agent_id": agent_id,
        "archival": archival_result,
        "messages": messages_result,
    })))
}

async fn search_archival_memory(
    client: &LettaClient,
    agent_id: &letta::types::LettaId,
    query: &str,
    limit: usize,
    verbose: bool,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<ArchivalSearchResult, McpError> {
    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: Some(query.to_string()),
        limit: Some(limit as u32),
        before: None,
        after: None,
        ascending: None,
    };

    let passages = client
        .memory()
        .list_archival_memory(agent_id, Some(params))
        .await
        .map_err(|e| sdk_err("search archival memory", e))?;

    let mut filtered_passages: Vec<Value> = Vec::new();
    for passage in passages {
        let passage_value = serde_json::to_value(&passage)?;

        if let Some(created_at) = passage_value.get("created_at").and_then(|v| v.as_str())
            && let Ok(passage_date) = DateTime::parse_from_rfc3339(created_at)
        {
            let passage_date_utc = passage_date.with_timezone(&Utc);
            if let Some(ref start) = start_date
                && passage_date_utc < *start
            {
                continue;
            }
            if let Some(ref end) = end_date
                && passage_date_utc > *end
            {
                continue;
            }
        }

        let mut passage_obj = passage_value
            .as_object()
            .cloned()
            .unwrap_or_else(serde_json::Map::new);
        passage_obj.remove("embedding");

        let mut passage_val = Value::Object(passage_obj);
        if !verbose {
            crate::tools::memory_utils::truncate_passage_text(&mut passage_val, max_value_len());
        }
        filtered_passages.push(passage_val);
    }

    let count = filtered_passages.len();
    Ok(ArchivalSearchResult {
        passages: filtered_passages,
        count,
    })
}

async fn search_messages(
    client: &LettaClient,
    agent_id: &letta::types::LettaId,
    query: &str,
    limit: usize,
    verbose: bool,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<MessageSearchResult, McpError> {
    let query_lower = query.to_lowercase();
    let mut matching_messages: Vec<MessageMatch> = Vec::new();
    let page_size = 100i32;
    let mut cursor: Option<String> = None;
    let mut has_more = true;

    while has_more && matching_messages.len() < limit {
        let params = ListMessagesRequest {
            limit: Some(page_size),
            before: None,
            after: cursor.clone(),
            group_id: None,
            use_assistant_message: None,
            assistant_message_tool_name: None,
            assistant_message_tool_kwargs: None,
        };

        let messages = client
            .messages()
            .list(agent_id, Some(params))
            .await
            .map_err(|e| sdk_err("list messages", e))?;

        if messages.is_empty() {
            break;
        }

        for msg in &messages {
            let (id, date, message_type, content) = extract_message_info(msg);

            if let Ok(msg_date) = DateTime::parse_from_rfc3339(&date) {
                let msg_date_utc = msg_date.with_timezone(&Utc);

                if let Some(ref start) = start_date
                    && msg_date_utc < *start
                {
                    continue;
                }
                if let Some(ref end) = end_date
                    && msg_date_utc > *end
                {
                    has_more = false;
                    break;
                }
            }
            if content.to_lowercase().contains(&query_lower) {
                let message_content = if !verbose && content.len() > max_value_len() {
                    crate::tools::response_utils::truncate_preview(&content, max_value_len())
                } else {
                    content
                };

                matching_messages.push(MessageMatch {
                    id,
                    date,
                    message_type,
                    content: message_content,
                });

                if matching_messages.len() >= limit {
                    break;
                }
            }
        }

        if let Some(last_msg) = messages.last() {
            cursor = Some(extract_message_id(last_msg));
        }

        if (messages.len() as i32) < page_size {
            has_more = false;
        }
    }

    matching_messages.sort_by(|a, b| a.date.cmp(&b.date));

    let count = matching_messages.len();
    Ok(MessageSearchResult {
        messages: matching_messages,
        count,
    })
}

fn extract_message_info(msg: &LettaMessageUnion) -> (String, String, String, String) {
    match msg {
        LettaMessageUnion::SystemMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "system_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::UserMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "user_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::AssistantMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "assistant_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::ReasoningMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "reasoning_message".to_string(),
            m.reasoning.clone(),
        ),
        LettaMessageUnion::HiddenReasoningMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "hidden_reasoning_message".to_string(),
            "[hidden]".to_string(),
        ),
        LettaMessageUnion::ToolCallMessage(m) => {
            let tool_call_str = format!("{}({})", m.tool_call.name, m.tool_call.arguments);
            (
                m.id.to_string(),
                m.date.to_rfc3339(),
                "tool_call_message".to_string(),
                tool_call_str,
            )
        }
        LettaMessageUnion::ToolReturnMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "tool_return_message".to_string(),
            m.tool_return.clone(),
        ),
    }
}

fn extract_message_id(msg: &LettaMessageUnion) -> String {
    match msg {
        LettaMessageUnion::SystemMessage(m) => m.id.to_string(),
        LettaMessageUnion::UserMessage(m) => m.id.to_string(),
        LettaMessageUnion::AssistantMessage(m) => m.id.to_string(),
        LettaMessageUnion::ReasoningMessage(m) => m.id.to_string(),
        LettaMessageUnion::HiddenReasoningMessage(m) => m.id.to_string(),
        LettaMessageUnion::ToolCallMessage(m) => m.id.to_string(),
        LettaMessageUnion::ToolReturnMessage(m) => m.id.to_string(),
    }
}
