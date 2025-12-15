//! Group and multi-agent conversation API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    CreateMessagesRequest, Group, GroupCreate, GroupUpdate, GroupsListRequest, LettaId,
    LettaResponse, MessageCreate,
};
use crate::{LettaError, LettaMessageUnion, MessageStream, StreamingEvent};
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use reqwest::header::HeaderMap;

/// Group API operations.
#[derive(Debug)]
pub struct GroupApi<'a> {
    client: &'a LettaClient,
}

impl<'a> GroupApi<'a> {
    /// Create a new group API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all groups.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<GroupsListRequest>) -> LettaResult<Vec<Group>> {
        self.client
            .get_with_query("v1/groups", &params.unwrap_or_default())
            .await
    }

    /// Create a new group.
    ///
    /// # Arguments
    ///
    /// * `request` - The group creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: GroupCreate) -> LettaResult<Group> {
        self.client.post("v1/groups", &request).await
    }

    /// Get a specific group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, group_id: &LettaId) -> LettaResult<Group> {
        self.client.get(&format!("v1/groups/{}", group_id)).await
    }

    /// Update a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to update
    /// * `request` - The group update request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(&self, group_id: &LettaId, request: GroupUpdate) -> LettaResult<Group> {
        self.client
            .patch(&format!("v1/groups/{}", group_id), &request)
            .await
    }

    /// Delete a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn delete(&self, group_id: &LettaId) -> LettaResult<String> {
        self.client.delete(&format!("v1/groups/{}", group_id)).await
    }

    /// Send a message to a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to send the message to
    /// * `messages` - The messages to send
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn send_message(
        &self,
        group_id: &LettaId,
        messages: Vec<MessageCreate>,
    ) -> LettaResult<LettaResponse> {
        let request = CreateMessagesRequest {
            messages,
            ..Default::default()
        };
        self.client
            .post(&format!("v1/groups/{}/messages", group_id), &request)
            .await
    }

    /// Process a user message and return the group’s responses. This endpoint accepts a message from a user and processes it through agents in the group based on the specified pattern. It will stream the steps of the response always, and stream the tokens if ‘stream_tokens’ is set to True.
    ///
    /// This method uses Server-Sent Events (SSE) to stream the response, allowing
    /// for real-time updates as the agent processes the messages.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the agent group to send messages to
    /// * `request` - The message creation request with messages and options
    /// * `stream_tokens` - Whether to stream individual tokens (true) or complete messages (false)
    ///
    /// # Returns
    ///
    /// A stream of [`StreamingEvent`] items that can be consumed asynchronously.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn send_message_streaming(
        &self,
        group_id: &LettaId,
        request: CreateMessagesRequest,
        stream_tokens: bool,
    ) -> LettaResult<MessageStream> {
        // Build the URL with streaming endpoint
        let url = self
            .client
            .base_url()
            .join(&format!("v1/groups/{}/messages/stream", group_id))?;

        // Add query parameter for token streaming
        let url = if stream_tokens {
            url::Url::parse_with_params(url.as_str(), &[("stream_tokens", "true")])?
        } else {
            url
        };

        // Build headers
        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "text/event-stream".parse().unwrap());

        // Send the request
        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        // Create the event stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|result| async move {
                match result {
                    Ok(event) => {
                        // Skip events without data
                        if event.data.is_empty() || event.data == "[DONE]" {
                            return None;
                        }

                        // Parse the event data
                        match serde_json::from_str::<StreamingEvent>(&event.data) {
                            Ok(parsed) => Some(Ok(parsed)),
                            Err(e) => {
                                // Skip parsing errors (like the Python SDK)
                                eprintln!("Failed to parse SSE event: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => Some(Err(LettaError::streaming(format!(
                        "SSE stream error: {}",
                        e
                    )))),
                }
            });

        Ok(Box::pin(stream))
    }

    /// Update a message.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the agent that owns the message
    /// * `message_id` - The ID of the message to update
    /// * `request` - The update request with the new message content
    ///
    /// # Returns
    ///
    /// The updated message as a [`LettaMessageUnion`].
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update_message(
        &self,
        group_id: &LettaId,
        message_id: &LettaId,
        request: crate::types::UpdateMessageRequest,
    ) -> LettaResult<LettaMessageUnion> {
        self.client
            .patch(
                &format!("v1/groups/{}/messages/{}", group_id, message_id),
                &request,
            )
            .await
    }

    /// Reset a group's message history.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the agent whose messages to reset
    /// * `add_default_initial_messages` - Whether to add default initial messages
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn reset(
        &self,
        group_id: &LettaId,
        add_default_initial_messages: Option<bool>,
    ) -> LettaResult<crate::types::AgentState> {
        let mut body = serde_json::Map::new();
        if let Some(add_default) = add_default_initial_messages {
            body.insert(
                "add_default_initial_messages".to_string(),
                serde_json::Value::Bool(add_default),
            );
        }

        self.client
            .patch(&format!("v1/agents/{}/reset-messages", group_id), &body)
            .await
    }

    /// Retrieve message history for an agent group
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the group to send the message to
    /// * `messages` - The messages to send
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_messages(
        &self,
        group_id: &LettaId,
        messages: Vec<MessageCreate>,
    ) -> LettaResult<LettaResponse> {
        let request = CreateMessagesRequest {
            messages,
            ..Default::default()
        };
        self.client
            .get_with_query(&format!("v1/groups/{}/messages", group_id), &request)
            .await
    }
}

/// Convenience methods for group operations.
impl LettaClient {
    /// Get the group API for this client.
    pub fn groups(&self) -> GroupApi {
        GroupApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_group_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = GroupApi::new(&client);
    }
}
