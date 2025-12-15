//! Voice-related types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request for voice chat completions.
///
/// Note: The exact structure is not well-documented in the API.
/// This uses a generic JSON value to accommodate any request format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatCompletionRequest {
    /// Request data as JSON.
    #[serde(flatten)]
    pub data: Value,
}

/// Response from voice chat completions.
///
/// Note: The exact structure is not well-documented in the API.
/// This uses a generic JSON value to accommodate any response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChatCompletionResponse {
    /// Response data as JSON.
    #[serde(flatten)]
    pub data: Value,
}
