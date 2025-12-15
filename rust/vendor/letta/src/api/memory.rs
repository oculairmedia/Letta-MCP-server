//! Memory API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::{
    memory::{
        ArchivalMemoryQueryParams, Block, CreateArchivalMemoryRequest, Memory, Passage,
        UpdateArchivalMemoryRequest, UpdateMemoryBlockRequest,
    },
    LettaId, PaginationParams,
};

/// Memory API operations.
#[derive(Debug)]
pub struct MemoryApi<'a> {
    client: &'a LettaClient,
}

impl<'a> MemoryApi<'a> {
    /// Create a new memory API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Get the core memory of an agent.
    pub async fn get_core_memory(&self, agent_id: &LettaId) -> LettaResult<Memory> {
        let url = format!("/v1/agents/{}/core-memory", agent_id);
        self.client.get(&url).await
    }

    /// Get all core memory blocks for an agent.
    pub async fn list_core_memory_blocks(&self, agent_id: &LettaId) -> LettaResult<Vec<Block>> {
        let url = format!("/v1/agents/{}/core-memory/blocks", agent_id);
        self.client.get(&url).await
    }

    /// Get a specific core memory block by label.
    pub async fn get_core_memory_block(
        &self,
        agent_id: &LettaId,
        block_label: &str,
    ) -> LettaResult<Block> {
        let url = format!("/v1/agents/{}/core-memory/blocks/{}", agent_id, block_label);
        self.client.get(&url).await
    }

    /// Update a core memory block.
    pub async fn update_core_memory_block(
        &self,
        agent_id: &LettaId,
        block_label: &str,
        request: UpdateMemoryBlockRequest,
    ) -> LettaResult<Block> {
        let url = format!("/v1/agents/{}/core-memory/blocks/{}", agent_id, block_label);
        self.client.patch(&url, &request).await
    }

    /// Attach a memory block to an agent.
    pub async fn attach_memory_block(
        &self,
        agent_id: &LettaId,
        block_id: &LettaId,
    ) -> LettaResult<crate::types::agent::AgentState> {
        let url = format!(
            "/v1/agents/{}/core-memory/blocks/attach/{}",
            agent_id, block_id
        );
        self.client.patch_no_body(&url).await
    }

    /// Detach a memory block from an agent.
    pub async fn detach_memory_block(
        &self,
        agent_id: &LettaId,
        block_id: &LettaId,
    ) -> LettaResult<crate::types::agent::AgentState> {
        let url = format!(
            "/v1/agents/{}/core-memory/blocks/detach/{}",
            agent_id, block_id
        );
        self.client.patch_no_body(&url).await
    }

    // Archival Memory API

    /// List archival memory passages for an agent.
    pub async fn list_archival_memory(
        &self,
        agent_id: &LettaId,
        params: Option<ArchivalMemoryQueryParams>,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory", agent_id);
        if let Some(params) = params {
            let query = serde_urlencoded::to_string(&params)?;
            if !query.is_empty() {
                let url_with_query = format!("{}?{}", url, query);
                return self.client.get(&url_with_query).await;
            }
        }
        self.client.get(&url).await
    }

    /// Create a new archival memory passage.
    pub async fn create_archival_memory(
        &self,
        agent_id: &LettaId,
        request: CreateArchivalMemoryRequest,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory", agent_id);
        self.client.post(&url, &request).await
    }

    /// Update an archival memory passage.
    pub async fn update_archival_memory(
        &self,
        agent_id: &LettaId,
        memory_id: &LettaId,
        request: UpdateArchivalMemoryRequest,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory/{}", agent_id, memory_id);
        self.client.patch(&url, &request).await
    }

    /// Delete an archival memory passage.
    pub async fn delete_archival_memory(
        &self,
        agent_id: &LettaId,
        memory_id: &LettaId,
    ) -> LettaResult<serde_json::Value> {
        let url = format!("/v1/agents/{}/archival-memory/{}", agent_id, memory_id);
        self.client.delete(&url).await
    }

    // Agent Tools API

    /// List tools attached to an agent.
    pub async fn list_agent_tools(
        &self,
        agent_id: &LettaId,
    ) -> LettaResult<Vec<crate::types::tool::Tool>> {
        let url = format!("/v1/agents/{}/tools", agent_id);
        self.client.get(&url).await
    }

    /// Attach a tool to an agent.
    pub async fn attach_tool_to_agent(
        &self,
        agent_id: &LettaId,
        tool_id: &LettaId,
    ) -> LettaResult<crate::types::agent::AgentState> {
        let url = format!("/v1/agents/{}/tools/attach/{}", agent_id, tool_id);
        self.client.patch_no_body(&url).await
    }

    /// Detach a tool from an agent.
    pub async fn detach_tool_from_agent(
        &self,
        agent_id: &LettaId,
        tool_id: &LettaId,
    ) -> LettaResult<crate::types::agent::AgentState> {
        let url = format!("/v1/agents/{}/tools/detach/{}", agent_id, tool_id);
        self.client.patch_no_body(&url).await
    }

    /// List archival memory with pagination support.
    ///
    /// Returns a stream that automatically fetches subsequent pages as needed.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose archival memory to list
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
    /// // Get all archival memory passages, fetching pages automatically
    /// let mut stream = client.memory().archival_paginated(&agent_id, None);
    ///
    /// while let Some(passage) = stream.next().await {
    ///     let passage = passage?;
    ///     println!("Passage: {}", passage.text);
    /// }
    ///
    /// // Or search with pagination
    /// let all_passages = client.memory()
    ///     .archival_paginated(&agent_id, Some(PaginationParams::new().limit(50)))
    ///     .collect()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn archival_paginated(
        &self,
        agent_id: &LettaId,
        params: Option<PaginationParams>,
    ) -> PaginatedStream<Passage> {
        let client = self.client.clone();
        let agent_id = agent_id.clone();

        // Convert PaginationParams to ArchivalMemoryQueryParams
        let query_params = params.as_ref().map(|p| ArchivalMemoryQueryParams {
            before: p.before.clone(),
            after: p.after.clone(),
            limit: p.limit,
            ..Default::default()
        });

        PaginatedStream::new_with_id_cursor(
            params,
            move |page_params| {
                let client = client.clone();
                let agent_id = agent_id.clone();
                let mut effective_params = query_params.clone().unwrap_or_default();

                // Update pagination fields from page_params
                if let Some(p) = page_params {
                    effective_params.before = p.before;
                    effective_params.after = p.after;
                    effective_params.limit = p.limit;
                }

                async move {
                    client
                        .memory()
                        .list_archival_memory(&agent_id, Some(effective_params))
                        .await
                }
            },
            |passage| &passage.id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_memory_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = MemoryApi::new(&client);
    }
}
