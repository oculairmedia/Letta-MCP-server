//! Batch-related types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{LettaId, Message, MessageCreateContent, MessageRole};

/// Batch run status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    /// Batch has been created.
    Created,
    /// Batch is running.
    Running,
    /// Batch completed successfully.
    Completed,
    /// Batch failed.
    Failed,
    /// Batch was cancelled.
    Cancelled,
}

impl std::fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Running => write!(f, "running"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Batch job type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchJobType {
    /// Regular job.
    Job,
    /// Batch job.
    Batch,
}

/// Batch run information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRun {
    /// ID of the user who created this batch.
    pub created_by_id: Option<String>,
    /// ID of the user who last updated this batch.
    pub last_updated_by_id: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Batch status.
    pub status: BatchStatus,
    /// Completion timestamp.
    pub completed_at: Option<String>,
    /// Batch metadata.
    #[serde(default)]
    pub metadata: Value,
    /// Job type.
    pub job_type: BatchJobType,
    /// Batch ID (prefixed with "job-" or "batch-").
    pub id: LettaId,
    /// Callback URL for webhook notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// Timestamp when callback was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_sent_at: Option<String>,
    /// HTTP status code from callback response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_status_code: Option<u16>,
    /// Error message from callback if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_error: Option<String>,
}

/// Message request for batch creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessageRequest {
    /// Messages to send.
    pub messages: Vec<BatchMessage>,
    /// Agent ID to send messages to.
    pub agent_id: LettaId,
}

/// Message to create in a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessage {
    /// Message role (user, assistant, system, tool).
    pub role: MessageRole,
    /// Message content - can be a string or array of content parts.
    pub content: MessageCreateContent,
    /// Optional message name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional tool calls for assistant messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<super::MessageToolCall>>,
    /// Optional tool call ID for tool messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl BatchMessage {
    /// Create a simple user message.
    pub fn user(content: impl Into<MessageCreateContent>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
}

/// Request to create a batch of messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    /// List of message requests.
    pub requests: Vec<BatchMessageRequest>,
    /// Optional callback URL for webhook notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

/// Parameters for listing batch messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListBatchMessagesParams {
    /// Maximum number of messages to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor for pagination (message ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Filter by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
    /// Sort in descending order (default true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_descending: Option<bool>,
}

/// Response containing batch messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessagesResponse {
    /// List of messages.
    pub messages: Vec<Message>,
}
