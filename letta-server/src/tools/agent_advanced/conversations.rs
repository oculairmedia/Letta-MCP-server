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
        .list(&letta_agent_id)
        .await
        .map_err(|e| sdk_err("list conversations", e))?;

    Ok(StandardResponse::success(
        "list_conversations",
        serde_json::json!({
            "count": conversations.len(),
            "conversations": conversations,
        }),
        "Conversations listed successfully",
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
        request.conversation_id,
        "conversation_id is required for send_conversation_message",
    )?;
    let messages = require_field(
        request.messages,
        "messages is required for send_conversation_message",
    )?;
    let letta_conversation_id = require_id(Some(conversation_id), "conversation_id")?;

    let message_values: Vec<Value> = messages
        .into_iter()
        .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
        .collect();

    let message_request = letta::types::ConversationMessageRequest {
        messages: Some(Value::Array(message_values)),
        input: None,
        max_steps: None,
        use_assistant_message: None,
        assistant_message_tool_name: None,
        assistant_message_tool_kwarg: None,
        include_return_message_types: None,
        enable_thinking: None,
        client_tools: None,
        override_model: None,
        streaming: request.stream,
        stream_tokens: None,
        include_pings: None,
        background: None,
    };

    let response = client
        .conversations()
        .send_message(&letta_conversation_id, message_request)
        .await
        .map_err(|e| sdk_err("send conversation message", e))?;

    Ok(StandardResponse::success(
        "send_conversation_message",
        serde_json::to_value(response)?,
        "Conversation message sent successfully",
    ))
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
