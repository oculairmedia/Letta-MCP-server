//! Run and job execution management API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{LettaId, LettaMessageUnion, Run, Step};

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
            .get_with_query(&format!("v1/runs"), &[("agent_ids", id_list.join(","))])
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
            .get_with_query(
                &format!("v1/runs/active"),
                &[("agent_ids", id_list.join(","))],
            )
            .await
    }
}

/// Convenience methods for agent-specific run operations.
impl LettaClient {
    /// Get the run API for this client.
    pub fn runs(&self) -> RunApi {
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
