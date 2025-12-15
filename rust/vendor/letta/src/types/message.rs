//! Message-related types for the Letta API.

use crate::types::common::{LettaId, Timestamp};
use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Message role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// User message.
    User,
    /// Assistant message.
    Assistant,
    /// System message.
    System,
    /// Tool call result.
    Tool,
    /// Function call result (legacy).
    Function,
}

/// Message content variant - can be string or array.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContentVariant {
    /// Simple string content.
    String(String),
    /// Structured content items.
    Items(Vec<MessageContent>),
}

/// Message content item types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContentItem {
    /// Text content.
    Text {
        /// The text content.
        text: String,
    },
    /// Image content.
    Image {
        /// The source of the image.
        source: ImageContentSource,
    },
    /// Tool call content.
    ToolCall {
        /// Tool call information.
        #[serde(flatten)]
        tool_call: ToolCall,
    },
    /// Tool return content.
    ToolReturn {
        /// Tool return information.
        #[serde(flatten)]
        tool_return: ToolReturn,
    },
    /// Reasoning content.
    Reasoning {
        /// Reasoning text.
        reasoning: String,
    },
    /// Omitted reasoning content.
    OmittedReasoning {
        /// Message indicating reasoning was omitted.
        message: String,
    },
    /// Redacted reasoning content.
    RedactedReasoning {
        /// Number of characters redacted.
        redacted_chars: u32,
    },
}

/// Message content types (legacy, keeping for compatibility).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    /// Text content.
    Text {
        /// The text content.
        text: String,
    },
    /// Image content.
    Image {
        /// The source of the image.
        source: ImageContentSource,
    },
    /// Tool call content.
    ToolCall {
        /// Tool call information.
        #[serde(flatten)]
        tool_call: ToolCall,
    },
    /// Tool return content.
    ToolReturn {
        /// Tool return information.
        #[serde(flatten)]
        tool_return: ToolReturn,
    },
    /// Reasoning content.
    Reasoning {
        /// Reasoning text.
        reasoning: String,
    },
    /// Omitted reasoning content.
    OmittedReasoning {
        /// Message indicating reasoning was omitted.
        message: String,
    },
    /// Redacted reasoning content.
    RedactedReasoning {
        /// Number of characters redacted.
        redacted_chars: u32,
    },
}

/// Image URL data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    /// URL or base64 data.
    pub url: String,
    /// Image detail level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Image content source variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageContentSource {
    /// Base64 encoded image.
    Base64 {
        /// The media type for the image.
        media_type: String,
        /// The base64 encoded image data.
        data: String,
        /// What level of detail to use when processing and understanding the image.
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
    },
    /// Letta-hosted image (placeholder for future use).
    Letta {
        /// Image ID or reference in Letta's system.
        id: String,
    },
    /// URL-based image.
    Url {
        /// The URL of the image.
        url: String,
    },
}

/// Tool call information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool/function name.
    pub name: String,
    /// Function arguments as JSON string.
    pub arguments: String,
    /// Tool call ID.
    pub tool_call_id: String,
}

/// Message tool call (OpenAI format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageToolCall {
    /// Tool call ID.
    pub id: String,
    /// Function details.
    pub function: MessageToolCallFunction,
    /// Tool type (always "function").
    #[serde(rename = "type")]
    pub tool_type: String,
}

/// Message tool call function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageToolCallFunction {
    /// Function name.
    pub name: String,
    /// Function arguments as JSON string.
    pub arguments: String,
}

/// Message tool return.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageToolReturn {
    /// Return status.
    pub status: String,
    /// Stdout output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<Vec<String>>,
    /// Stderr output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<Vec<String>>,
}

/// Tool call function (for OpenAI-style messages).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    /// Function name.
    pub name: String,
    /// Function arguments as JSON string.
    pub arguments: String,
}

/// Tool return information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolReturn {
    /// Return status.
    pub status: ToolReturnStatus,
    /// Captured stdout from tool invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<Vec<String>>,
    /// Captured stderr from tool invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<Vec<String>>,
}

/// Tool return status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolReturnStatus {
    /// Tool executed successfully.
    Success,
    /// Tool execution failed.
    Error,
}

/// Reasoning message source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningMessageSource {
    /// Reasoning from a model with native reasoning capabilities.
    ReasonerModel,
    /// Reasoning derived via prompting.
    NonReasonerModel,
}

/// Hidden reasoning message state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HiddenReasoningMessageState {
    /// Content was redacted by the provider.
    Redacted,
    /// Content was omitted by the API.
    Omitted,
}

/// Full message representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Message role (required).
    pub role: MessageRole,
    /// Message content as array of content items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<MessageContentItem>>,
    /// For user/assistant: participant name. For tool/function: function name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls (for assistant messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<MessageToolCall>>,
    /// Tool call ID (for tool messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Tool returns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_returns: Option<Vec<MessageToolReturn>>,
    /// Group ID for multi-agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<LettaId>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Batch item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_item_id: Option<LettaId>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the message was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the message was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Request to create a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// Message role.
    pub role: MessageRole,
    /// Message content.
    pub content: String,
    /// Optional name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls (for assistant role).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call ID (for tool role).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Query parameters for listing messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListMessagesParams {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Include tool calls in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tool_calls: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_message_serialization() {
        let message = Message {
            id: Some(LettaId::from_str("message-550e8400-e29b-41d4-a716-446655440000").unwrap()),
            organization_id: None,
            agent_id: Some(
                LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440001").unwrap(),
            ),
            role: MessageRole::User,
            content: Some(vec![MessageContentItem::Text {
                text: "Hello, world!".to_string(),
            }]),
            name: None,
            tool_calls: None,
            tool_call_id: None,
            model: None,
            step_id: None,
            otid: None,
            tool_returns: None,
            group_id: None,
            sender_id: None,
            batch_item_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(message.role, deserialized.role);
    }

    #[test]
    fn test_message_content_variants() {
        let text_content = MessageContentItem::Text {
            text: "Hello".to_string(),
        };
        let json = serde_json::to_string(&text_content).unwrap();
        assert!(json.contains("\"type\":\"text\""));

        let tool_call = MessageContentItem::ToolCall {
            tool_call: ToolCall {
                name: "get_weather".to_string(),
                arguments: r#"{"location": "Seattle"}"#.to_string(),
                tool_call_id: "call-123".to_string(),
            },
        };
        let json = serde_json::to_string(&tool_call).unwrap();
        assert!(json.contains("\"type\":\"tool_call\""));
    }

    #[test]
    fn test_message_role_serialization() {
        assert_eq!(
            serde_json::to_string(&MessageRole::User).unwrap(),
            "\"user\""
        );
        assert_eq!(
            serde_json::to_string(&MessageRole::Assistant).unwrap(),
            "\"assistant\""
        );
        assert_eq!(
            serde_json::to_string(&MessageRole::Tool).unwrap(),
            "\"tool\""
        );
    }
}

// =============================================================================
// Letta-specific Message Types
// =============================================================================

/// Message type values used by Letta.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// System message.
    #[serde(rename = "system_message")]
    SystemMessage,
    /// User message.
    #[serde(rename = "user_message")]
    UserMessage,
    /// Assistant response message.
    #[serde(rename = "assistant_message")]
    AssistantMessage,
    /// Reasoning/thinking message.
    #[serde(rename = "reasoning_message")]
    ReasoningMessage,
    /// Hidden/redacted reasoning message.
    #[serde(rename = "hidden_reasoning_message")]
    HiddenReasoningMessage,
    /// Tool call invocation message.
    #[serde(rename = "tool_call_message")]
    ToolCallMessage,
    /// Tool execution result message.
    #[serde(rename = "tool_return_message")]
    ToolReturnMessage,
    /// Stop reason indicator.
    #[serde(rename = "stop_reason")]
    StopReason,
    /// Usage statistics.
    #[serde(rename = "usage_statistics")]
    UsageStatistics,
    /// Any other message type.
    #[serde(other)]
    Other,
}

/// Union type for all Letta message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum LettaMessageUnion {
    /// System message from Letta.
    SystemMessage(SystemMessage),
    /// User message to Letta.
    UserMessage(UserMessage),
    /// Assistant response from Letta.
    AssistantMessage(AssistantMessage),
    /// Internal reasoning/thinking message.
    ReasoningMessage(ReasoningMessage),
    /// Hidden or redacted reasoning message.
    HiddenReasoningMessage(HiddenReasoningMessage),
    /// Tool call invocation message.
    ToolCallMessage(ToolCallMessage),
    /// Tool execution result message.
    ToolReturnMessage(ToolReturnMessage),
}

/// System message from Letta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// System message content.
    pub content: String,
}

/// User message to Letta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// User message content.
    pub content: String,
}

/// Assistant message from Letta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// Assistant response content.
    pub content: String,
}

/// Reasoning message (internal thoughts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// Source of reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ReasoningMessageSource>,
    /// Reasoning content (internal thoughts).
    pub reasoning: String,
    /// Model-generated signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Hidden reasoning message (redacted thoughts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenReasoningMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// State of hidden reasoning.
    pub state: HiddenReasoningMessageState,
    /// Hidden reasoning content (redacted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_reasoning: Option<String>,
}

/// Tool call message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// Tool call information.
    pub tool_call: ToolCall,
}

/// Tool return message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolReturnMessage {
    /// Message ID.
    pub id: LettaId,
    /// Message timestamp.
    pub date: Timestamp,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<LettaId>,
    /// Tool return value as string.
    pub tool_return: String,
    /// Status of the tool call.
    pub status: ToolReturnStatus,
    /// Tool call ID this is responding to.
    pub tool_call_id: String,
    /// Captured stdout from tool invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<Vec<String>>,
    /// Captured stderr from tool invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<Vec<String>>,
}

/// Request to create a message for Letta agents.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct MessageCreate {
    /// Message role.
    pub role: MessageRole,
    /// Message content (can be string or complex content).
    pub content: MessageCreateContent,
    /// Optional participant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Offline threading ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<LettaId>,
    /// Batch item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_item_id: Option<LettaId>,
    /// Group ID for multi-agent conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<LettaId>,
}

impl Default for MessageCreate {
    fn default() -> Self {
        Self {
            role: MessageRole::User,
            content: MessageCreateContent::String(String::new()),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }
    }
}

impl MessageCreate {
    /// Create a simple user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: MessageCreateContent::String(content.into()),
            ..Default::default()
        }
    }

    /// Create a simple assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: MessageCreateContent::String(content.into()),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }
    }

    /// Create a simple system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: MessageCreateContent::String(content.into()),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }
    }
}

/// Content for message creation (can be simple string or complex types).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageCreateContent {
    /// Simple text string.
    String(String),
    /// Complex content parts for multi-modal messages.
    ContentParts(Vec<ContentPart>),
}

impl From<String> for MessageCreateContent {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for MessageCreateContent {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Vec<String>> for MessageCreateContent {
    fn from(parts: Vec<String>) -> Self {
        Self::ContentParts(
            parts
                .into_iter()
                .map(|text| ContentPart::Text(TextContent { text }))
                .collect(),
        )
    }
}

/// Individual content part for multi-modal messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    /// Text content part.
    Text(TextContent),
    /// Image content part.
    Image(ImageContent),
}

/// Text content part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    /// The text content.
    pub text: String,
}

/// Image content part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    /// Image URL information.
    pub image_url: ImageUrl,
}

/// Response from creating/sending messages to Letta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LettaResponse {
    /// Messages in the conversation.
    pub messages: Vec<LettaMessageUnion>,
    /// Reason processing stopped.
    pub stop_reason: LettaStopReason,
    /// Token usage statistics.
    pub usage: LettaUsageStatistics,
}

/// Stop reason types supported by Letta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReasonType {
    /// End of conversation turn.
    EndTurn,
    /// Error occurred.
    Error,
    /// Invalid tool call.
    InvalidToolCall,
    /// Maximum steps reached.
    MaxSteps,
    /// No tool call made.
    NoToolCall,
    /// Tool rule triggered.
    ToolRule,
}

/// Reason why message processing stopped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LettaStopReason {
    /// The reason processing stopped.
    pub stop_reason: StopReasonType,
}

/// Token usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LettaUsageStatistics {
    /// Tokens used for completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i32>,
    /// Tokens used for prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i32>,
    /// Total tokens used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i32>,
    /// Step count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_count: Option<i32>,
    /// Messages generated per step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_messages: Option<Vec<Vec<Message>>>,
    /// Background task run IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_ids: Option<Vec<LettaId>>,
}

/// Parameters for creating messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct CreateMessagesRequest {
    /// Messages to send.
    pub messages: Vec<MessageCreate>,
    /// Maximum processing steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<i32>,
    /// Use assistant message format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assistant_message: Option<bool>,
    /// Assistant message tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_name: Option<String>,
    /// Assistant message tool kwargs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_kwargs: Option<String>,
    /// Filter response message types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_message_types: Option<Vec<MessageType>>,
}

/// Parameters for listing messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListMessagesRequest {
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Maximum number of messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Filter by group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<LettaId>,
    /// Use assistant message format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assistant_message: Option<bool>,
    /// Assistant message tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_name: Option<String>,
    /// Assistant message tool kwargs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_kwargs: Option<String>,
}

// =============================================================================
// Message Update Types
// =============================================================================

/// Request to modify/update a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum UpdateMessageRequest {
    /// Update a system message.
    #[serde(rename = "system_message")]
    SystemMessage(UpdateSystemMessage),
    /// Update a user message.
    #[serde(rename = "user_message")]
    UserMessage(UpdateUserMessage),
    /// Update a reasoning message.
    #[serde(rename = "reasoning_message")]
    ReasoningMessage(UpdateReasoningMessage),
    /// Update an assistant message.
    #[serde(rename = "assistant_message")]
    AssistantMessage(UpdateAssistantMessage),
}

/// Update system message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSystemMessage {
    /// Updated message content.
    pub content: String,
}

/// Update user message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserMessage {
    /// Updated message content.
    pub content: UpdateUserMessageContent,
}

/// Content for updating user messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserMessageContent {
    /// Simple text content.
    String(String),
    /// Complex content parts.
    ContentParts(Vec<ContentPart>),
}

/// Update reasoning message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReasoningMessage {
    /// Updated reasoning content.
    pub reasoning: String,
}

/// Update assistant message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAssistantMessage {
    /// Updated message content.
    pub content: UpdateAssistantMessageContent,
}

/// Content for updating assistant messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAssistantMessageContent {
    /// Simple text content.
    String(String),
    /// Complex content parts.
    ContentParts(Vec<ContentPart>),
}

// Note: Async message/job types (Run, JobStatus, JobType) have been moved to types/runs.rs

/// Request to cancel agent runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelAgentRunRequest {
    /// Optional list of run IDs to cancel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_ids: Option<Vec<String>>,
}

/// Search mode for message search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageSearchMode {
    /// Vector search only.
    Vector,
    /// Full-text search only.
    Fts,
    /// Hybrid search combining vector and FTS.
    Hybrid,
}

impl Default for MessageSearchMode {
    fn default() -> Self {
        Self::Hybrid
    }
}

/// Request for searching messages across organization.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct MessageSearchRequest {
    /// Text query for full-text search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Search mode to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<MessageSearchMode>,

    /// Filter messages by role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<MessageRole>>,

    /// Filter messages by project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// Filter messages by template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    /// Maximum number of results to return (1-100, default: 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Filter messages created after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,

    /// Filter messages created on or before this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
}

/// Result from a message search operation with scoring details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSearchResult {
    /// The embedded content (LLM-friendly).
    pub embedded_text: String,

    /// The raw message object.
    pub message: LettaMessageUnion,

    /// Full-text search rank position if FTS was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fts_rank: Option<i32>,

    /// Vector search rank position if vector search was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_rank: Option<i32>,

    /// Reciprocal Rank Fusion combined score.
    pub rrf_score: f64,
}
