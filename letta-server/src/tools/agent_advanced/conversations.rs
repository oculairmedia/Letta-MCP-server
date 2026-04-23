use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use letta_types::StandardResponse;
use serde_json::Value;
use turbomcp::McpError;

use super::AgentAdvancedRequest;

pub(crate) async fn handle_list_conversations(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for list_conversations",
    )?;
    let letta_agent_id = require_id(Some(agent_id), "agent_id")?;

    let conversations = client
        .conversations()
        .list(&letta_agent_id, None)
        .await
        .map_err(|e| sdk_err("list conversations", e))?;

    // Client-side pagination (SDK does not support server-side params)
    let total = conversations.len();
    let limit = 50.min(total);
    let paginated: Vec<_> = conversations.into_iter().take(limit).collect();

    Ok(StandardResponse::success(
        "list_conversations",
        serde_json::json!({
            "total": total,
            "returned": paginated.len(),
            "conversations": paginated,
        }),
        format!("Returned {} of {} conversations", paginated.len(), total),
    ))
}

pub(crate) async fn handle_get_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = require_field(
        request.conversation_id,
        "conversation_id is required for get_conversation",
    )?;
    let letta_conversation_id = require_id(Some(conversation_id), "conversation_id")?;

    let conversation = client
        .conversations()
        .get(&letta_conversation_id)
        .await
        .map_err(|e| sdk_err("get conversation", e))?;

    Ok(StandardResponse::success(
        "get_conversation",
        serde_json::to_value(conversation)?,
        "Conversation retrieved successfully",
    ))
}

pub(crate) async fn handle_send_conversation_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = require_field(
        request.conversation_id.clone(),
        "conversation_id is required for send_conversation_message",
    )?;
    let messages = require_field(
        request.messages,
        "messages is required for send_conversation_message",
    )?;
    let letta_conversation_id = require_id(Some(conversation_id.clone()), "conversation_id")?;

    let message_values: Vec<Value> = messages
        .into_iter()
        .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
        .collect();

    // The Letta conversation messages endpoint defaults to SSE streaming.
    // We drain the SSE stream ourselves and assemble a JSON response,
    // which bypasses the server-side non-streaming serialization bug.
    let response_value =
        send_conversation_message_via_sse(client, &letta_conversation_id, message_values)
            .await
            .map_err(|e| {
                tracing::warn!(
                    "send_conversation_message failed for conv {}: {}",
                    conversation_id,
                    e
                );
                sdk_err("send conversation message", e)
            })?;

    Ok(StandardResponse::success(
        "send_conversation_message",
        response_value,
        "Conversation message sent successfully",
    ))
}

/// Send a conversation message by POSTing directly and draining the SSE
/// response.  The conversation messages endpoint defaults to streaming,
/// so we accept the SSE and reassemble it into JSON rather than fighting
/// the content-type mismatch.
async fn send_conversation_message_via_sse(
    client: &LettaClient,
    conversation_id: &letta::types::LettaId,
    message_values: Vec<Value>,
) -> Result<serde_json::Value, String> {
    use reqwest::header::HeaderMap;

    let url = client
        .base_url()
        .join(&format!("v1/conversations/{}/messages", conversation_id))
        .map_err(|e| format!("Bad URL: {}", e))?;

    let body = serde_json::json!({
        "messages": message_values,
        "streaming": true,
    });

    use reqwest::header::{ACCEPT, CONTENT_TYPE};

    let mut headers = HeaderMap::new();
    client
        .auth()
        .apply_to_headers(&mut headers)
        .map_err(|e| format!("Auth error: {}", e))?;
    headers.insert(
        CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        ACCEPT,
        reqwest::header::HeaderValue::from_static("text/event-stream"),
    );

    let response = client
        .http()
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body_text = response
            .text()
            .await
            .unwrap_or_else(|_| "unknown".to_string());
        return Err(format!("API error {}: {}", status, body_text));
    }

    // Read the full SSE body and parse events manually.
    // SSE format: "data: {json}\n\n" per event, with optional "event: type\n".
    let body_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read SSE body: {}", e))?;

    let mut collected_messages: Vec<serde_json::Value> = Vec::new();
    let mut stop_reason: Option<serde_json::Value> = None;

    for line in body_text.lines() {
        let line = line.trim();
        if !line.starts_with("data: ") {
            continue;
        }
        let data = &line[6..]; // strip "data: "
        if data.is_empty() || data == "[DONE]" {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
            let msg_type = val
                .get("message_type")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match msg_type {
                "assistant_message"
                | "tool_call_message"
                | "tool_return_message"
                | "reasoning_message"
                | "user_message"
                | "system_message" => {
                    collected_messages.push(val);
                }
                "stop_reason" => {
                    stop_reason = Some(val);
                }
                "error_message" => {
                    tracing::debug!("Conversation stream cleanup error: {}", data);
                }
                _ => {
                    // pings, unknown types — skip
                }
            }
        }
    }

    if collected_messages.is_empty() {
        return Err("Stream completed without returning any messages".to_string());
    }

    Ok(serde_json::json!({
        "messages": collected_messages,
        "stop_reason": stop_reason.unwrap_or(serde_json::json!({"stop_reason": "end_turn"})),
        "usage": {},
    }))
}

pub(crate) async fn handle_cancel_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = require_field(
        request.conversation_id,
        "conversation_id is required for cancel_conversation",
    )?;
    let letta_conversation_id = require_id(Some(conversation_id), "conversation_id")?;

    let response = client
        .conversations()
        .cancel(&letta_conversation_id)
        .await
        .map_err(|e| sdk_err("cancel conversation", e))?;

    Ok(StandardResponse::success(
        "cancel_conversation",
        response,
        "Conversation cancelled successfully",
    ))
}

pub(crate) async fn handle_compact_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = require_field(
        request.conversation_id,
        "conversation_id is required for compact_conversation",
    )?;
    let letta_conversation_id = require_id(Some(conversation_id), "conversation_id")?;

    let compact_payload = request.update_data;

    let response = client
        .conversations()
        .compact(&letta_conversation_id, compact_payload)
        .await
        .map_err(|e| sdk_err("compact conversation", e))?;

    Ok(StandardResponse::success(
        "compact_conversation",
        response,
        "Conversation compacted successfully",
    ))
}
