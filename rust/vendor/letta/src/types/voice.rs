//! Voice-related types.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
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
/// Letta 0.16.x documents this beta endpoint as returning `None`, while older
/// deployments may return an object. Keep both shapes acceptable.
#[derive(Debug, Clone)]
pub struct VoiceChatCompletionResponse {
    /// Response data as JSON when the server returns an object/value.
    pub data: Option<Value>,
}

impl<'de> Deserialize<'de> for VoiceChatCompletionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self {
            data: if value.is_null() { None } else { Some(value) },
        })
    }
}

impl Serialize for VoiceChatCompletionResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.data {
            Some(value) => value.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}
