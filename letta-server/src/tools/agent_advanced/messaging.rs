use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use futures::StreamExt;
use letta::api::messages::StreamingEvent;
use letta::LettaClient;
use letta_types::StandardResponse;
use turbomcp::McpError;

use super::{AgentAdvancedRequest, truncate_text};

pub(crate) async fn handle_send_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for send_message operation",
    )?;
    let messages = require_field(
        request.messages,
        "messages is required for send_message operation",
    )?;
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let verbose = request.verbose.unwrap_or(false);

    // Use streaming to bypass the Letta server's non-streaming response
    // serialization bug (returns 500 after successful processing).
    // Drain the SSE stream and assemble the response ourselves — same
    // pattern LettaBot's session.stream() uses in production.
    let mut response_value = match send_message_via_stream(client, &letta_id, messages_request)
        .await
    {
        Ok(value) => value,
        Err(e) => {
            tracing::warn!(
                "Streaming send_message failed for agent {}, error: {}",
                agent_id,
                e
            );
            return Err(sdk_err("send message", e));
        }
    };

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

/// Send a message via the streaming endpoint and drain all events into a
/// synthesized LettaResponse-shaped JSON value.  This avoids the server-side
/// non-streaming response serialization bug while keeping the MCP tool's
/// interface unchanged.
async fn send_message_via_stream(
    client: &LettaClient,
    agent_id: &letta::types::LettaId,
    request: letta::types::CreateMessagesRequest,
) -> Result<serde_json::Value, String> {
    let mut stream = client
        .messages()
        .create_stream(agent_id, request, false)
        .await
        .map_err(|e| format!("Failed to open message stream: {}", e))?;

    let mut collected_messages: Vec<serde_json::Value> = Vec::new();
    let mut stop_reason: Option<serde_json::Value> = None;
    let mut usage: Option<serde_json::Value> = None;

    while let Some(event_result) = stream.next().await {
        match event_result {
            Ok(event) => match event {
                StreamingEvent::Message(msg) => {
                    if let Ok(val) = serde_json::to_value(&msg) {
                        collected_messages.push(val);
                    }
                }
                StreamingEvent::StopReason(sr) => {
                    stop_reason = serde_json::to_value(&sr).ok();
                }
                StreamingEvent::Usage(u) => {
                    usage = serde_json::to_value(&u).ok();
                }
            },
            Err(e) => {
                // Stream errors after we've collected messages are non-fatal
                // (the server's cleanup_error happens here).  Only fail if
                // we have zero messages.
                tracing::debug!("Stream event error (may be non-fatal cleanup): {}", e);
                break;
            }
        }
    }

    if collected_messages.is_empty() {
        return Err("Stream completed without returning any messages".to_string());
    }

    // Assemble into LettaResponse shape
    Ok(serde_json::json!({
        "messages": collected_messages,
        "stop_reason": stop_reason.unwrap_or(serde_json::json!({"stop_reason": "end_turn"})),
        "usage": usage.unwrap_or(serde_json::json!({})),
    }))
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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for async_message operation",
    )?;
    let messages = require_field(
        request.messages,
        "messages are required for async_message operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for cancel_message operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let query = require_field(
        request.query,
        "query is required for search_messages operation",
    )?;

    let search_request = letta::types::MessageSearchRequest {
        query: Some(query),
        ..Default::default()
    };

    // search_messages (/v1/agents/messages/search) is a cloud-only endpoint.
    // On self-hosted servers it may hang indefinitely or return connection errors.
    // Apply a short timeout so callers get a clear error instead of blocking.
    let results = match tokio::time::timeout(
        std::time::Duration::from_secs(15),
        client.messages().search(search_request),
    )
    .await
    {
        Ok(Ok(results)) => results,
        Ok(Err(e)) => {
            let err_str = e.to_string();
            // Detect connection-refused / endpoint-not-found patterns
            if err_str.contains("error sending request")
                || err_str.contains("connection refused")
                || err_str.contains("Connection refused")
            {
                return Err(McpError::internal(
                    "search_messages is not supported on this server. \
                     This endpoint (/v1/agents/messages/search) is a Letta Cloud feature \
                     and is not available on self-hosted servers."
                        .to_string(),
                ));
            }
            return Err(sdk_err("search messages", e));
        }
        Err(_timeout) => {
            return Err(McpError::internal(
                "search_messages timed out after 15s. \
                 This endpoint (/v1/agents/messages/search) may not be available \
                 on self-hosted servers — it is a Letta Cloud feature."
                    .to_string(),
            ));
        }
    };

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for get_message operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for preview_payload operation",
    )?;
    let messages = require_field(
        request.messages,
        "messages are required for preview_payload operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
