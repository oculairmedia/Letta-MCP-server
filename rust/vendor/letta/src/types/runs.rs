//! Run and Job management types for the Letta API.

use std::fmt;

use crate::types::common::{LettaId, Timestamp};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// Status of a job/run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    /// Job was created.
    #[default]
    Created,
    /// Job is running.
    Running,
    /// Job completed successfully.
    Completed,
    /// Job failed.
    Failed,
    /// Job is pending.
    Pending,
    /// Job was cancelled.
    Cancelled,
    /// Job expired.
    Expired,
}

/// Type of job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobType {
    /// Regular job.
    Job,
    /// Run (message processing).
    Run,
    /// Batch job.
    Batch,
}

/// Base job representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// The unique identifier of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// The status of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// The type of job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<JobType>,
    /// When the job was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the job was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Timestamp>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the job was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Job metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Callback URL for completion notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// When callback was last attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_sent_at: Option<Timestamp>,
    /// HTTP status code from callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_status_code: Option<i32>,
    /// Error message from callback attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_error: Option<String>,
}

/// Representation of a run for async message processing.
/// Extends Job with request configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    /// The unique identifier of the run.
    pub id: LettaId,
    /// The status of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// The type of job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<JobType>,
    /// When the job was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the job was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Timestamp>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the run was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Job metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Callback URL for completion notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// When callback was last attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_sent_at: Option<Timestamp>,
    /// HTTP status code from callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_status_code: Option<i32>,
    /// Error message from callback attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_error: Option<String>,
    /// Request configuration for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_config: Option<RunRequestConfig>,
}

/// Configuration for a run request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunRequestConfig {
    /// Agent ID for the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
    /// Messages to process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::types::message::MessageCreate>>,
    /// Maximum number of steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<i32>,
    /// Whether to use assistant message format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assistant_message: Option<bool>,
    /// Assistant message tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_name: Option<String>,
    /// Assistant message tool kwargs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_kwarg: Option<String>,
    /// Message types to include in response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_message_types: Option<Vec<crate::types::message::MessageType>>,
}

/// Query parameters for listing runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRunsParams {
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Maximum number of runs to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// Filter by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
}

/// Run status update request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRunStatus {
    /// New status for the run.
    pub status: JobStatus,
}

/// Query parameters for listing steps.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListStepsParams {
    /// Filter by run ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<LettaId>,
    /// Filter by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of steps to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Filter by tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Step in a run execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// Step ID.
    pub id: LettaId,
    /// Run ID this step belongs to.
    pub run_id: LettaId,
    /// Step execution order.
    pub step_number: i32,
    /// Step name or type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Messages produced in this step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::types::message::LettaMessageUnion>>,
    /// Tool calls in this step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<crate::types::message::ToolCall>>,
    /// Token usage for this step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<StepUsage>,
    /// When the step was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Provider trace information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_trace: Option<ProviderTrace>,
}

/// Token usage for a step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepUsage {
    /// Completion tokens used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i32>,
    /// Prompt tokens used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i32>,
    /// Total tokens used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i32>,
}

/// Provider execution trace information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderTrace {
    /// Provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Model used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Execution duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    /// Raw provider response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<serde_json::Value>,
}

/// Step feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepFeedback {
    /// Positive feedback.
    Positive,
    /// Negative feedback.
    Negative,
}

impl fmt::Display for StepFeedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => f.write_str("positive"),
            Self::Negative => f.write_str("negative"),
        }
    }
}

/// Batch job for processing multiple requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    /// Base job fields.
    #[serde(flatten)]
    pub job: Job,
    /// Number of requests in the batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i32>,
    /// Number of completed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_count: Option<i32>,
    /// Number of failed requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
}

/// Batch request for processing multiple agent messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LettaBatchRequest {
    /// The messages to be sent to the agent.
    pub messages: Vec<crate::types::message::MessageCreate>,
    /// Maximum number of steps the agent should take to process the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<i32>,
    /// Whether the server should parse specific tool call arguments as AssistantMessage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_assistant_message: Option<bool>,
    /// The name of the designated message tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_name: Option<String>,
    /// The name of the message argument in the designated message tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message_tool_kwarg: Option<String>,
    /// Only return specified message types in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_message_types: Option<Vec<crate::types::message::MessageType>>,
    /// The ID of the agent to send this batch request for.
    pub agent_id: LettaId,
}

/// Request to create a batch job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatch {
    /// List of requests to be processed in batch.
    pub requests: Vec<LettaBatchRequest>,
    /// Optional URL to call via POST when the batch completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_status_serialization() {
        assert_eq!(
            serde_json::to_string(&JobStatus::Running).unwrap(),
            "\"running\""
        );
        assert_eq!(
            serde_json::to_string(&JobStatus::Completed).unwrap(),
            "\"completed\""
        );
    }

    #[test]
    fn test_job_type_serialization() {
        assert_eq!(serde_json::to_string(&JobType::Run).unwrap(), "\"run\"");
        assert_eq!(serde_json::to_string(&JobType::Batch).unwrap(), "\"batch\"");
    }

    #[test]
    fn test_step_feedback_serialization() {
        assert_eq!(
            serde_json::to_string(&StepFeedback::Positive).unwrap(),
            "\"positive\""
        );
        assert_eq!(
            serde_json::to_string(&StepFeedback::Negative).unwrap(),
            "\"negative\""
        );
    }
}
