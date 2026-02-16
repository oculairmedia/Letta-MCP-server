use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use letta_types::StandardResponse;
use turbomcp::McpError;

use super::{truncate_text, AgentAdvancedRequest};

pub(crate) async fn handle_send_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for send_message operation".to_string())
    })?;

    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages is required for send_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let verbose = request.verbose.unwrap_or(false);

    let response = client
        .messages()
        .create(&letta_id, messages_request)
        .await
        .map_err(|e| sdk_err("send message", e))?;

    let mut response_value = serde_json::to_value(&response)?;

    if !verbose {
        if let Some(messages) = response_value
            .get_mut("messages")
            .and_then(|m| m.as_array_mut())
        {
            for msg in messages.iter_mut() {
                if let Some(content) = msg.get("text").and_then(|t| t.as_str()) {
                    let original_length = content.len();
                    if original_length > 1000 {
                        msg["text"] = serde_json::json!(truncate_text(content, 1000));
                        msg["full_response_length"] = serde_json::json!(original_length);
                    }
                }
            }
        }
        response_value["hint"] =
            serde_json::json!("Full response visible in agent's message history");
    }

    Ok(StandardResponse::success(
        "send_message",
        response_value,
        "Message sent successfully",
    ))
}

pub(crate) async fn handle_stream(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    Err(McpError::internal(
        "Stream operation not supported in MCP tool context - use async_message instead"
            .to_string(),
    ))
}

pub(crate) async fn handle_async_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for async_message operation".to_string())
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages are required for async_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let run_id = client
        .messages()
        .create_async(&letta_id, messages_request)
        .await
        .map_err(|e| sdk_err("create async message", e))?;

    Ok(StandardResponse::success(
        "async_message",
        serde_json::json!({ "run_id": run_id }),
        "Async message created successfully",
    ))
}

pub(crate) async fn handle_cancel_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for cancel_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    client
        .messages()
        .cancel(&letta_id, None)
        .await
        .map_err(|e| sdk_err("cancel message", e))?;

    Ok(StandardResponse::success_no_data(
        "cancel_message",
        "Message cancelled successfully",
    ))
}

pub(crate) async fn handle_search_messages(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let query = request.query.ok_or_else(|| {
        McpError::invalid_request("query is required for search_messages operation".to_string())
    })?;

    let search_request = letta::types::MessageSearchRequest {
        query: Some(query),
        ..Default::default()
    };

    let results = client
        .messages()
        .search(search_request)
        .await
        .map_err(|e| sdk_err("search messages", e))?;

    let default_limit = 10;
    let max_limit = 50;
    let limit = request
        .pagination
        .and_then(|p| p.limit)
        .unwrap_or(default_limit)
        .min(max_limit);

    let message_summaries: Vec<serde_json::Value> = results
        .iter()
        .take(limit)
        .map(|msg| {
            let msg_value = serde_json::to_value(msg).unwrap_or(serde_json::json!({}));

            let content = msg_value
                .get("text")
                .or_else(|| msg_value.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or("");

            let content_length = content.len();
            let content_preview = truncate_text(content, 200);

            let role = msg_value
                .get("role")
                .and_then(|r| r.as_str())
                .unwrap_or("unknown");

            let created_at = msg_value
                .get("created_at")
                .and_then(|c| c.as_str())
                .unwrap_or("");

            let id = msg_value.get("id").and_then(|i| i.as_str()).unwrap_or("");

            serde_json::json!({
                "id": id,
                "role": role,
                "content_preview": content_preview,
                "content_length": content_length,
                "created_at": created_at,
            })
        })
        .collect();

    let total = results.len();
    let returned = message_summaries.len();
    let has_more = total > returned;

    let response_data = serde_json::json!({
        "total": total,
        "returned": returned,
        "has_more": has_more,
        "messages": message_summaries,
        "hint": "Use get_message with message_id for full content",
    });

    Ok(StandardResponse::success(
        "search_messages",
        response_data,
        format!("Found {} of {} messages (preview mode)", returned, total),
    ))
}

pub(crate) async fn handle_get_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let messages = client
        .messages()
        .list(&letta_id, None)
        .await
        .map_err(|e| sdk_err("list messages", e))?;

    Ok(StandardResponse::success(
        "get_message",
        serde_json::to_value(&messages)?,
        format!("Retrieved {} messages (filter client-side)", messages.len()),
    ))
}

pub(crate) async fn handle_count(
    client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let count = client
        .agents()
        .count()
        .await
        .map_err(|e| sdk_err("count agents", e))?;

    Ok(StandardResponse::success(
        "count",
        serde_json::json!({ "count": count }),
        format!("Total agents: {}", count),
    ))
}

pub(crate) async fn handle_preview_payload(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for preview_payload operation".to_string())
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages are required for preview_payload operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let preview = client
        .messages()
        .preview(&letta_id, messages_request)
        .await
        .map_err(|e| sdk_err("preview payload", e))?;

    Ok(StandardResponse::success(
        "preview_payload",
        serde_json::to_value(preview)?,
        "Payload preview generated successfully",
    ))
}
