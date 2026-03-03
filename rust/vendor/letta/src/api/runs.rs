//! Run and job execution management API endpoints.

use crate::client::LettaClient;
use crate::error::{LettaError, LettaResult};
use crate::types::{LettaId, LettaMessageUnion, Run, RunMetrics, Step, UsageStatistics};
use eventsource_stream::Eventsource;
use futures::stream::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Streaming event types for run streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStreamingEvent {
    /// A run message payload.
    Message(LettaMessageUnion),
    /// Unstructured stream event payload.
    Raw(serde_json::Value),
}

/// Run stream type.
pub type RunStream = Pin<Box<dyn Stream<Item = LettaResult<RunStreamingEvent>> + Send>>;

/// Run API operations.
#[derive(Debug)]
pub struct RunApi<'a> {
    client: &'a LettaClient,
}

impl<'a> RunApi<'a> {
    /// Create a new run API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all runs.
    ///
    /// # Arguments
    ///
    /// * `agent_ids` - The agent IDs associated with the run
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, agent_ids: &[LettaId]) -> LettaResult<Vec<Run>> {
        let id_list = agent_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>();
        self.client
            .get_with_query("v1/runs/", &[("agent_ids", id_list.join(","))])
            .await
    }

    /// Get a specific run.
    ///
    /// # Arguments
    ///
    /// * `run_id` - The ID of the run to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, run_id: &LettaId) -> LettaResult<Run> {
        self.client.get(&format!("v1/runs/{}", run_id)).await
    }

    /// Get messages for a run.
    ///
    /// # Arguments
    ///
    /// * `run_id` - The ID of the run whose messages to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_messages(&self, run_id: &LettaId) -> LettaResult<Vec<LettaMessageUnion>> {
        self.client
            .get(&format!("v1/runs/{}/messages", run_id))
            .await
    }

    /// Get steps for a run.
    ///
    /// # Arguments
    ///
    /// * `run_id` - The ID of the run whose steps to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_steps(&self, run_id: &LettaId) -> LettaResult<Vec<Step>> {
        self.client.get(&format!("v1/runs/{}/steps", run_id)).await
    }

    /// List active runs for an agent.
    ///
    /// # Arguments
    ///
    /// * `agent_ids` - The IDs of the agents whose runs to list
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_active(&self, agent_ids: &[LettaId]) -> LettaResult<Vec<Run>> {
        let id_list = agent_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>();
        self.client
            .get_with_query("v1/runs/active/", &[("agent_ids", id_list.join(","))])
            .await
    }

    /// Get run metrics payload.
    pub async fn get_metrics(&self, run_id: &LettaId) -> LettaResult<RunMetrics> {
        self.client
            .get(&format!("v1/runs/{}/metrics", run_id))
            .await
    }

    /// Get run usage payload.
    pub async fn get_usage(&self, run_id: &LettaId) -> LettaResult<UsageStatistics> {
        self.client.get(&format!("v1/runs/{}/usage", run_id)).await
    }

    /// Get run trace payload.
    pub async fn get_trace(&self, run_id: &LettaId) -> LettaResult<serde_json::Value> {
        self.client.get(&format!("v1/runs/{}/trace", run_id)).await
    }

    /// Stream run events via Server-Sent Events.
    pub async fn stream(&self, run_id: &LettaId) -> LettaResult<RunStream> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/runs/{}/stream", run_id))?;

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
            .json(&serde_json::json!({}))
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
                            .map(RunStreamingEvent::Message)
                            .or_else(|_| {
                                serde_json::from_str::<serde_json::Value>(&event.data)
                                    .map(RunStreamingEvent::Raw)
                            })
                            .map_err(|e| {
                                LettaError::config(format!(
                                    "Failed to parse run stream event: {}",
                                    e
                                ))
                            });

                        Some(parsed)
                    }
                    Err(e) => Some(Err(LettaError::config(format!(
                        "Run stream error: {}",
                        e
                    )))),
                }
            });

        Ok(Box::pin(stream))
    }
}

/// Convenience methods for agent-specific run operations.
impl LettaClient {
    /// Get the run API for this client.
    pub fn runs(&self) -> RunApi<'_> {
        RunApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_run_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = RunApi::new(&client);
    }
}
