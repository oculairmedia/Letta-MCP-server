//! Message API endpoints.

use crate::api::endpoints;
use crate::client::LettaClient;
use crate::error::{LettaError, LettaResult};
use crate::pagination::PaginatedStream;
use crate::types::{
    CreateMessagesRequest, LettaId, LettaMessageUnion, LettaResponse, LettaStopReason,
    LettaUsageStatistics, ListMessagesRequest, PaginationParams,
};
use eventsource_stream::Eventsource;
use futures::stream::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Streaming event types from the message stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamingEvent {
    /// A message from the agent.
    Message(LettaMessageUnion),
    /// Stop reason indicating why processing stopped.
    StopReason(LettaStopReason),
    /// Usage statistics for the conversation.
    Usage(LettaUsageStatistics),
}

/// Streaming response type containing the event stream.
pub type MessageStream = Pin<Box<dyn Stream<Item = LettaResult<StreamingEvent>> + Send>>;

/// Message API operations.
#[derive(Debug)]
pub struct MessageApi<'a> {
    client: &'a LettaClient,
}

impl<'a> MessageApi<'a> {
    /// Create a new message API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List messages for an agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to list
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        agent_id: &LettaId,
        params: Option<ListMessagesRequest>,
    ) -> LettaResult<Vec<LettaMessageUnion>> {
        self.client
            .get_with_query(
                &endpoints::agents::messages::list(agent_id),
                &params.unwrap_or_default(),
            )
            .await
    }

    /// Send messages to an agent and get response.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
    /// * `request` - The message creation request with messages and options
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
    ) -> LettaResult<LettaResponse> {
        // Use post_no_retry: sending messages is non-idempotent — retrying on
        // transient server errors (e.g. 500 during response serialization) would
        // create duplicate messages on the agent.
        self.client
            .post_no_retry(&endpoints::agents::messages::send(agent_id), &request)
            .await
    }

    /// Reset an agent's message history.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to reset
    /// * `add_default_initial_messages` - Whether to add default initial messages
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn reset(
        &self,
        agent_id: &LettaId,
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
            .patch(&endpoints::agents::reset_messages(agent_id), &body)
            .await
    }

    /// Send messages to an agent and stream the response.
    ///
    /// This method uses Server-Sent Events (SSE) to stream the response, allowing
    /// for real-time updates as the agent processes the messages.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use letta::types::{MessageCreate, MessageRole, CreateMessagesRequest, MessageCreateContent};
    /// # use letta::api::messages::StreamingEvent;
    /// # use letta::LettaId;
    /// # use std::str::FromStr;
    /// # use futures::StreamExt;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = letta::LettaClient::new(
    /// #     letta::client::ClientConfig::new("http://localhost:8283")?
    /// # )?;
    /// let mut stream = client
    ///     .messages()
    ///     .create_stream(
    ///         &LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap(),
    ///         CreateMessagesRequest {
    ///             messages: vec![MessageCreate {
    ///                 role: MessageRole::User,
    ///                 content: "Hello!".into(),
    ///                 ..Default::default()
    ///             }],
    ///             ..Default::default()
    ///         },
    ///         false,
    ///     )
    ///     .await?;
    ///
    /// while let Some(event) = stream.next().await {
    ///     match event? {
    ///         StreamingEvent::Message(msg) => println!("Message: {:?}", msg),
    ///         StreamingEvent::StopReason(reason) => println!("Stop: {:?}", reason),
    ///         StreamingEvent::Usage(usage) => println!("Usage: {:?}", usage),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_stream(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
        stream_tokens: bool,
    ) -> LettaResult<MessageStream> {
        // Build the URL with streaming endpoint
        let url = self
            .client
            .base_url()
            .join(&endpoints::agents::messages::stream(agent_id))?;

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
    /// * `agent_id` - The ID of the agent that owns the message
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
    pub async fn update(
        &self,
        agent_id: &LettaId,
        message_id: &LettaId,
        request: crate::types::UpdateMessageRequest,
    ) -> LettaResult<LettaMessageUnion> {
        self.client
            .patch(
                &endpoints::agents::messages::get(agent_id, message_id),
                &request,
            )
            .await
    }

    /// Create messages asynchronously and return a run object.
    ///
    /// This method submits messages for processing in the background and returns
    /// immediately with a run ID that can be used to check the status.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
    /// * `request` - The message creation request with messages and options
    ///
    /// # Returns
    ///
    /// A [`crate::types::Run`] object containing the run ID and status information.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_async(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
    ) -> LettaResult<crate::types::Run> {
        // Non-idempotent: avoid retry to prevent duplicate message processing.
        self.client
            .post_no_retry(&endpoints::agents::messages::send_async(agent_id), &request)
            .await
    }

    /// Cancel agent runs.
    ///
    /// Cancels runs associated with an agent. If run_ids are provided, cancels those specific runs.
    /// Note: To cancel active runs, Redis is required.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose runs to cancel
    /// * `request` - Optional request specifying which runs to cancel
    ///
    /// # Returns
    ///
    /// A JSON response object confirming the cancellation.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn cancel(
        &self,
        agent_id: &LettaId,
        request: Option<crate::types::CancelAgentRunRequest>,
    ) -> LettaResult<serde_json::Value> {
        self.client
            .post(
                &endpoints::agents::messages::cancel(agent_id),
                &request.unwrap_or_default(),
            )
            .await
    }

    /// Preview the raw LLM request payload without sending it.
    ///
    /// This endpoint processes the message through the agent loop up until the LLM request,
    /// then returns the raw request payload that would be sent to the LLM provider.
    /// Useful for debugging and inspection.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to preview messages for
    /// * `request` - The message creation request with messages and options
    ///
    /// # Returns
    ///
    /// A JSON object containing the raw LLM request payload.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn preview(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
    ) -> LettaResult<serde_json::Value> {
        self.client
            .post(&endpoints::agents::messages::preview(agent_id), &request)
            .await
    }

    /// Search messages across the entire organization.
    ///
    /// Returns messages with FTS/vector ranks and total RRF score.
    /// This is a cloud-only feature.
    ///
    /// # Arguments
    ///
    /// * `request` - The search request with query and filters
    ///
    /// # Returns
    ///
    /// A vector of [`crate::types::MessageSearchResult`] containing matching messages with scores.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use letta::types::{MessageSearchRequest, MessageSearchMode, MessageRole};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = letta::LettaClient::new(
    /// #     letta::client::ClientConfig::new("http://localhost:8283")?
    /// # )?;
    /// let results = client
    ///     .messages()
    ///     .search(MessageSearchRequest {
    ///         query: Some("error handling".to_string()),
    ///         search_mode: Some(MessageSearchMode::Hybrid),
    ///         roles: Some(vec![MessageRole::User, MessageRole::Assistant]),
    ///         limit: Some(10),
    ///         ..Default::default()
    ///     })
    ///     .await?;
    ///
    /// for result in results {
    ///     println!("Message: {} (RRF: {})", result.embedded_text, result.rrf_score);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(
        &self,
        request: crate::types::MessageSearchRequest,
    ) -> LettaResult<Vec<crate::types::MessageSearchResult>> {
        self.client
            .post(endpoints::agents::MESSAGE_SEARCH, &request)
            .await
    }

    /// List messages with pagination support.
    ///
    /// Returns a stream that automatically fetches subsequent pages as needed.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to list
    /// * `params` - Optional pagination parameters
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::{PaginationParams, LettaId};
    /// # use futures::StreamExt;
    /// # use std::str::FromStr;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// # let agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap();
    /// // Get all messages, fetching pages automatically
    /// let mut stream = client.messages().paginated(&agent_id, None);
    ///
    /// while let Some(message) = stream.next().await {
    ///     let message = message?;
    ///     println!("Message: {:?}", message);
    /// }
    ///
    /// // Or collect all messages at once
    /// let all_messages = client.messages()
    ///     .paginated(&agent_id, Some(PaginationParams::new().limit(50)))
    ///     .collect()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(
        &self,
        agent_id: &LettaId,
        params: Option<PaginationParams>,
    ) -> PaginatedStream<LettaMessageUnion> {
        let client = self.client.clone();
        let agent_id = agent_id.clone();

        // Convert PaginationParams to ListMessagesRequest
        let list_params = params.as_ref().map(|p| ListMessagesRequest {
            before: p.before.clone(),
            after: p.after.clone(),
            limit: p.limit.map(|l| l as i32),
            ..Default::default()
        });

        PaginatedStream::new_with_id_cursor(
            params,
            move |page_params| {
                let client = client.clone();
                let agent_id = agent_id.clone();
                let mut effective_params = list_params.clone().unwrap_or_default();

                // Update pagination fields from page_params
                if let Some(p) = page_params {
                    effective_params.before = p.before;
                    effective_params.after = p.after;
                    effective_params.limit = p.limit.map(|l| l as i32);
                }

                async move {
                    client
                        .messages()
                        .list(&agent_id, Some(effective_params))
                        .await
                }
            },
            |message| match message {
                LettaMessageUnion::SystemMessage(msg) => &msg.id,
                LettaMessageUnion::UserMessage(msg) => &msg.id,
                LettaMessageUnion::AssistantMessage(msg) => &msg.id,
                LettaMessageUnion::ReasoningMessage(msg) => &msg.id,
                LettaMessageUnion::HiddenReasoningMessage(msg) => &msg.id,
                LettaMessageUnion::ToolCallMessage(msg) => &msg.id,
                LettaMessageUnion::ToolReturnMessage(msg) => &msg.id,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_message_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = MessageApi::new(&client);
    }
}
