//! Voice API endpoints (beta).

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{LettaId, VoiceChatCompletionRequest, VoiceChatCompletionResponse};

/// Voice API operations (beta).
#[derive(Debug)]
pub struct VoiceApi<'a> {
    client: &'a LettaClient,
}

impl<'a> VoiceApi<'a> {
    /// Create a new voice API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Create voice chat completions for an agent.
    ///
    /// **Note**: This is a beta endpoint and the exact request/response structure
    /// is not well-documented. The endpoint accepts and returns generic JSON data.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent for voice chat
    /// * `request` - The voice chat completion request
    /// * `user_id` - Optional user ID to include in the request headers
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_voice_chat_completions(
        &self,
        agent_id: &LettaId,
        request: VoiceChatCompletionRequest,
        user_id: Option<&str>,
    ) -> LettaResult<VoiceChatCompletionResponse> {
        let path = format!("v1/voice-beta/{}/chat/completions", agent_id);

        if let Some(user_id) = user_id {
            // Use post_with_headers when we have a user_id
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "user-id",
                user_id.parse().map_err(|_| {
                    crate::error::LettaError::validation("Invalid user-id header value")
                })?,
            );

            self.client
                .post_with_headers(&path, &request, headers)
                .await
        } else {
            self.client.post(&path, &request).await
        }
    }
}

/// Convenience method for voice operations.
impl LettaClient {
    /// Get the voice API for this client.
    pub fn voice(&self) -> VoiceApi {
        VoiceApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_voice_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = VoiceApi::new(&client);
    }
}
