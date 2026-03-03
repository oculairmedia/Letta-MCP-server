//! Conversation management types for the Letta API.

use crate::types::common::{LettaId, Timestamp};
use serde::{Deserialize, Serialize};

/// Conversation resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Conversation ID.
    pub id: LettaId,
    /// Agent ID.
    pub agent_id: LettaId,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Optional summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Message IDs currently in context.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub in_context_message_ids: Vec<LettaId>,
    /// Block IDs isolated for this conversation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isolated_block_ids: Vec<LettaId>,
}

/// Request to create a conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    /// Optional summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Optional isolated block labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolated_block_labels: Option<Vec<String>>,
}

/// Request to update a conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConversationRequest {
    /// Optional summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Request to send messages within a conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationMessageRequest {
    /// Message array payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<serde_json::Value>,
    /// Single input payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    /// Maximum processing steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<i32>,
    /// Use assistant message format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assistant_message: Option<bool>,
    /// Assistant message tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_name: Option<String>,
    /// Assistant message kwarg name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_kwarg: Option<String>,
    /// Message return type filter payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_message_types: Option<serde_json::Value>,
    /// Enable reasoning/thinking mode value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_thinking: Option<String>,
    /// Client tools payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tools: Option<serde_json::Value>,
    /// Override model payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_model: Option<serde_json::Value>,
    /// Stream response flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
    /// Stream token chunks flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_tokens: Option<bool>,
    /// Include keepalive pings in stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pings: Option<bool>,
    /// Run in background mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
}

/// Parameters for listing conversations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConversationsParams {
    /// Agent ID to filter conversations by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Conversation ID cursor for pagination (returns conversations before this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Conversation ID cursor for pagination (returns conversations after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Maximum number of conversations to return (default: 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order by creation time ('asc' or 'desc', default: 'asc').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Field to sort by (default: 'created_at').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}
