//! Archive API endpoints.

use crate::api::endpoints;
use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::agent::AgentState;
use crate::types::archive::{
    Archive, ArchiveCreateRequest, ArchivePassagesResponse, ArchiveUpdateRequest,
    ListArchivesParams, PassageBatchCreateRequest, PassageCreateRequest,
};
use crate::types::memory::Passage;
use crate::types::LettaId;

/// Archive API operations.
#[derive(Debug)]
pub struct ArchiveApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ArchiveApi<'a> {
    /// Create a new archive API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all archives.
    pub async fn list(&self, params: Option<ListArchivesParams>) -> LettaResult<Vec<Archive>> {
        self.client
            .get_with_query(endpoints::archives::LIST, &params.unwrap_or_default())
            .await
    }

    /// Create a new archive.
    pub async fn create(&self, request: ArchiveCreateRequest) -> LettaResult<Archive> {
        self.client
            .post(endpoints::archives::CREATE, &request)
            .await
    }

    /// Get an archive by ID.
    pub async fn get(&self, archive_id: &LettaId) -> LettaResult<Archive> {
        self.client.get(&endpoints::archives::get(archive_id)).await
    }

    /// Partial archive metadata mutation endpoint.
    pub async fn update(
        &self,
        archive_id: &LettaId,
        request: ArchiveUpdateRequest,
    ) -> LettaResult<Archive> {
        self.client
            .patch(&endpoints::archives::update(archive_id), &request)
            .await
    }

    /// Archive deletion endpoint.
    pub async fn delete(&self, archive_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&endpoints::archives::delete(archive_id))
            .await
    }

    /// List agents attached to an archive.
    pub async fn list_agents(&self, archive_id: &LettaId) -> LettaResult<Vec<AgentState>> {
        self.client
            .get(&endpoints::archives::list_agents(archive_id))
            .await
    }

    /// Create a passage inside an archive.
    pub async fn create_passage(
        &self,
        archive_id: &LettaId,
        request: PassageCreateRequest,
    ) -> LettaResult<Passage> {
        self.client
            .post(&endpoints::archives::passages::create(archive_id), &request)
            .await
    }

    /// Batch-create passages inside an archive.
    pub async fn create_passages(
        &self,
        archive_id: &LettaId,
        request: PassageBatchCreateRequest,
    ) -> LettaResult<ArchivePassagesResponse> {
        self.client
            .post(
                &endpoints::archives::passages::batch_create(archive_id),
                &request,
            )
            .await
    }

    /// Archive passage deletion endpoint.
    pub async fn delete_passage(
        &self,
        archive_id: &LettaId,
        passage_id: &LettaId,
    ) -> LettaResult<()> {
        self.client
            .delete_no_response(&endpoints::archives::passages::delete(
                archive_id, passage_id,
            ))
            .await
    }

    /// List passages in an archive.
    pub async fn list_passages(&self, archive_id: &LettaId) -> LettaResult<Vec<Passage>> {
        self.client
            .get(&endpoints::archives::passages::list(archive_id))
            .await
    }

    /// Get archive sub-API for agent-specific archive operations.
    pub fn agent_archives(&self, agent_id: LettaId) -> AgentArchiveApi<'_> {
        AgentArchiveApi::new(self.client, agent_id)
    }
}

/// Agent archive sub-API operations.
#[derive(Debug)]
pub struct AgentArchiveApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentArchiveApi<'a> {
    /// Create a new agent archive API instance.
    pub fn new(client: &'a LettaClient, agent_id: LettaId) -> Self {
        Self { client, agent_id }
    }

    /// Attach an archive to an agent.
    pub async fn attach(&self, archive_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch(
                &endpoints::agents::archives::attach(&self.agent_id, archive_id),
                &(),
            )
            .await
    }

    /// Detach an archive from an agent.
    pub async fn detach(&self, archive_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch(
                &endpoints::agents::archives::detach(&self.agent_id, archive_id),
                &(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use std::str::FromStr;

    #[test]
    fn test_archive_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ArchiveApi::new(&client);
    }

    #[test]
    fn test_agent_archive_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = AgentArchiveApi::new(
            &client,
            LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440000").unwrap(),
        );
    }
}
