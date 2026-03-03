//! Conversation API endpoints.

use crate::client::LettaClient;
use crate::error::{LettaError, LettaResult};
use crate::types::{
    Conversation, ConversationMessageRequest, CreateConversationRequest, LettaId,
    LettaMessageUnion, LettaResponse, ListConversationsParams, UpdateConversationRequest,
};
use eventsource_stream::Eventsource;
use futures::stream::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Streaming event types for conversation streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConversationStreamingEvent {
    /// A conversation message payload.
    Message(LettaMessageUnion),
    /// Unstructured stream event payload.
    Raw(serde_json::Value),
}

/// Conversation stream type.
pub type ConversationStream =
    Pin<Box<dyn Stream<Item = LettaResult<ConversationStreamingEvent>> + Send>>;

/// Conversation API operations.
#[derive(Debug)]
pub struct ConversationApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ConversationApi<'a> {
    /// Create a new conversation API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List conversations for an agent.
    pub async fn list(
        &self,
        agent_id: &LettaId,
        params: Option<ListConversationsParams>,
    ) -> LettaResult<Vec<Conversation>> {
        let mut p = params.unwrap_or_default();
        p.agent_id = Some(agent_id.to_string());
        self.client.get_with_query("v1/conversations/", &p).await
    }

    /// Create a conversation.
    pub async fn create(&self, request: CreateConversationRequest) -> LettaResult<Conversation> {
        self.client.post("v1/conversations/", &request).await
    }

    /// Get a conversation by ID.
    pub async fn get(&self, conversation_id: &LettaId) -> LettaResult<Conversation> {
        self.client
            .get(&format!("v1/conversations/{}", conversation_id))
            .await
    }

    /// Apply a partial mutation to a conversation resource.
    pub async fn update(
        &self,
        conversation_id: &LettaId,
        request: UpdateConversationRequest,
    ) -> LettaResult<Conversation> {
        self.client
            .patch(&format!("v1/conversations/{}", conversation_id), &request)
            .await
    }

    /// Delete a conversation.
    pub async fn delete(&self, conversation_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/conversations/{}", conversation_id))
            .await
    }

    /// Cancel a conversation.
    pub async fn cancel(&self, conversation_id: &LettaId) -> LettaResult<serde_json::Value> {
        self.client
            .post(&format!("v1/conversations/{}/cancel", conversation_id), &())
            .await
    }

    /// Compact a conversation.
    pub async fn compact(
        &self,
        conversation_id: &LettaId,
        body: Option<serde_json::Value>,
    ) -> LettaResult<serde_json::Value> {
        let payload = body.unwrap_or_default();
        self.client
            .post(
                &format!("v1/conversations/{}/compact", conversation_id),
                &payload,
            )
            .await
    }

    /// List messages in a conversation.
    pub async fn list_messages(
        &self,
        conversation_id: &LettaId,
    ) -> LettaResult<Vec<LettaMessageUnion>> {
        self.client
            .get(&format!("v1/conversations/{}/messages", conversation_id))
            .await
    }

    /// Send a message in a conversation.
    pub async fn send_message(
        &self,
        conversation_id: &LettaId,
        request: ConversationMessageRequest,
    ) -> LettaResult<LettaResponse> {
        self.client
            .post(
                &format!("v1/conversations/{}/messages", conversation_id),
                &request,
            )
            .await
    }

    /// Stream a conversation response.
    pub async fn stream(
        &self,
        conversation_id: &LettaId,
        request: ConversationMessageRequest,
    ) -> LettaResult<ConversationStream> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/conversations/{}/stream", conversation_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert(
            "Content-Type",
            "application/json"
                .parse::<reqwest::header::HeaderValue>()
                .map_err(|e| LettaError::config(e.to_string()))?,
        );
        headers.insert(
            "Accept",
            "text/event-stream"
                .parse::<reqwest::header::HeaderValue>()
                .map_err(|e| LettaError::config(e.to_string()))?,
        );

        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|result| async move {
                match result {
                    Ok(event) => {
                        if event.data.is_empty() || event.data == "[DONE]" {
                            return None;
                        }

                        let parsed = serde_json::from_str::<LettaMessageUnion>(&event.data)
                            .map(ConversationStreamingEvent::Message)
                            .or_else(|_| {
                                serde_json::from_str::<serde_json::Value>(&event.data)
                                    .map(ConversationStreamingEvent::Raw)
                            })
                            .map_err(|e| {
                                LettaError::config(format!(
                                    "Failed to parse conversation stream event: {}",
                                    e
                                ))
                            });

                        Some(parsed)
                    }
                    Err(e) => Some(Err(LettaError::config(format!(
                        "Conversation stream error: {}",
                        e
                    )))),
                }
            });

        Ok(Box::pin(stream))
    }
}

impl LettaClient {
    /// Get the conversation API.
    pub fn conversations(&self) -> ConversationApi<'_> {
        ConversationApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_conversation_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ConversationApi::new(&client);
    }
}
