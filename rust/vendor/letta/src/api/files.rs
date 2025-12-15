//! File session API endpoints for agents.
//!
//! This module provides LRU-based file session management for agents.
//! Files can be opened and closed in an agent's context, with automatic
//! eviction when the session limit is reached.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::file::PaginatedAgentFiles;
use crate::types::LettaId;

/// File session API operations for agents.
///
/// This API manages files that are currently "open" in an agent's context,
/// using an LRU (Least Recently Used) cache. When opening a file exceeds
/// the session limit, the least recently used files are automatically evicted.
#[derive(Debug)]
pub struct AgentFileApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentFileApi<'a> {
    /// Create a new agent file API instance.
    pub fn new(client: &'a LettaClient, agent_id: LettaId) -> Self {
        Self { client, agent_id }
    }

    /// List files currently in the agent's file session.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::LettaId;
    /// # use std::str::FromStr;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let agent_id = LettaId::from_str("agent-123")?;
    /// let files = client.agents().files(agent_id.clone()).list().await?;
    /// println!("Agent has {} files in session", files.files.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> LettaResult<PaginatedAgentFiles> {
        self.client
            .get(&format!("v1/agents/{}/files", self.agent_id))
            .await
    }

    /// Open a file in the agent's session.
    ///
    /// This adds the file to the agent's LRU file session cache. If the cache
    /// is full, the least recently used files will be automatically evicted.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to open
    ///
    /// # Returns
    ///
    /// Returns a list of file IDs that were evicted due to LRU cache limits.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::LettaId;
    /// # use std::str::FromStr;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let agent_id = LettaId::from_str("agent-123")?;
    /// let file_id = LettaId::from_str("file-456")?;
    /// let evicted = client.agents().files(agent_id).open(&file_id).await?;
    /// if !evicted.is_empty() {
    ///     println!("Evicted {} files due to LRU limit", evicted.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn open(&self, file_id: &LettaId) -> LettaResult<Vec<String>> {
        self.client
            .post(
                &format!("v1/agents/{}/files/{}/open", self.agent_id, file_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Close a specific file in the agent's session.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to close
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::LettaId;
    /// # use std::str::FromStr;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let agent_id = LettaId::from_str("agent-123")?;
    /// let file_id = LettaId::from_str("file-456")?;
    /// client.agents().files(agent_id).close(&file_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close(&self, file_id: &LettaId) -> LettaResult<()> {
        self.client
            .post(
                &format!("v1/agents/{}/files/{}/close", self.agent_id, file_id),
                &serde_json::json!({}),
            )
            .await
            .map(|_: serde_json::Value| ())
    }

    /// Close all files in the agent's session.
    ///
    /// # Returns
    ///
    /// Returns a list of file IDs that were closed.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::LettaId;
    /// # use std::str::FromStr;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let agent_id = LettaId::from_str("agent-123")?;
    /// let closed = client.agents().files(agent_id).close_all().await?;
    /// println!("Closed {} files", closed.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close_all(&self) -> LettaResult<Vec<String>> {
        self.client
            .post(
                &format!("v1/agents/{}/files/close-all", self.agent_id),
                &serde_json::json!({}),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_agent_file_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let agent_id = "agent-550e8400-e29b-41d4-a716-446655440000"
            .parse()
            .unwrap();
        let _api = AgentFileApi::new(&client, agent_id);
    }
}
