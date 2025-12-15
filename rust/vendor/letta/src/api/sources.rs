//! Source API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::agent::AgentState;
use crate::types::memory::Passage;
use crate::types::source::{
    CreateSourceRequest, FileMetadata, FileUploadResponse, GetFileParams, ListFilesParams,
    ListPassagesParams, Source, UpdateSourceRequest,
};
use crate::types::{LettaId, PaginationParams};
use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use serde_json::Value;

/// Source API operations.
#[derive(Debug)]
pub struct SourceApi<'a> {
    client: &'a LettaClient,
}

impl<'a> SourceApi<'a> {
    /// Create a new source API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all sources.
    pub async fn list(&self) -> LettaResult<Vec<Source>> {
        self.client.get("v1/sources/").await
    }

    /// Create a new source.
    pub async fn create(&self, request: CreateSourceRequest) -> LettaResult<Source> {
        self.client.post("v1/sources/", &request).await
    }

    /// Get a source by ID.
    pub async fn get(&self, source_id: &LettaId) -> LettaResult<Source> {
        self.client.get(&format!("v1/sources/{}", source_id)).await
    }

    /// Update a source.
    pub async fn update(
        &self,
        source_id: &LettaId,
        request: UpdateSourceRequest,
    ) -> LettaResult<Source> {
        self.client
            .patch(&format!("v1/sources/{}", source_id), &request)
            .await
    }

    /// Delete a source.
    pub async fn delete(&self, source_id: &LettaId) -> LettaResult<Option<Value>> {
        self.client
            .delete(&format!("v1/sources/{}", source_id))
            .await
    }

    /// Get source count.
    pub async fn count(&self) -> LettaResult<i32> {
        self.client.get("v1/sources/count").await
    }

    /// Get source ID by name.
    pub async fn get_by_name(&self, source_name: &str) -> LettaResult<String> {
        self.client
            .get(&format!("v1/sources/name/{}", source_name))
            .await
    }

    /// Upload a file to a source.
    /// Returns either a job response (local) or file metadata (cloud).
    pub async fn upload_file(
        &self,
        source_id: &LettaId,
        file_name: String,
        file_data: Bytes,
        content_type: Option<String>,
    ) -> LettaResult<FileUploadResponse> {
        let form = Form::new();
        let mut part = Part::bytes(file_data.to_vec()).file_name(file_name);

        if let Some(ct) = content_type {
            part = part.mime_str(&ct)?;
        }

        let form = form.part("file", part);

        self.client
            .post_multipart(&format!("v1/sources/{}/upload", source_id), form)
            .await
    }

    /// List files in a source.
    pub async fn list_files(
        &self,
        source_id: &LettaId,
        params: Option<ListFilesParams>,
    ) -> LettaResult<Vec<FileMetadata>> {
        self.client
            .get_with_query(&format!("v1/sources/{}/files", source_id), &params)
            .await
    }

    /// Get file metadata.
    pub async fn get_file(
        &self,
        source_id: &LettaId,
        file_id: &LettaId,
        params: Option<GetFileParams>,
    ) -> LettaResult<FileMetadata> {
        self.client
            .get_with_query(
                &format!("v1/sources/{}/files/{}", source_id, file_id),
                &params,
            )
            .await
    }

    /// Delete a file from a source.
    pub async fn delete_file(&self, source_id: &LettaId, file_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/sources/{}/{}", source_id, file_id))
            .await
    }

    /// List passages from a source.
    pub async fn list_passages(
        &self,
        source_id: &LettaId,
        params: Option<ListPassagesParams>,
    ) -> LettaResult<Vec<Passage>> {
        self.client
            .get_with_query(&format!("v1/sources/{}/passages", source_id), &params)
            .await
    }

    /// Get agent source sub-API for a specific agent.
    pub fn agent_sources(&self, agent_id: LettaId) -> AgentSourceApi {
        AgentSourceApi::new(self.client, agent_id)
    }

    /// Get a paginated stream of files for a source.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all files using async iteration.
    ///
    /// # Arguments
    ///
    /// * `source_id` - The ID of the source
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::client::{ClientConfig, LettaClient};
    /// # use letta::{types::PaginationParams, pagination::PaginationExt};
    /// # use letta::types::LettaId;
    /// # use futures::StreamExt;
    /// # use std::str::FromStr;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let source_id = LettaId::from_str("source-123")?;
    ///
    /// let mut stream = client.sources().paginated_files(&source_id, None);
    /// while let Some(file) = stream.next().await {
    ///     let file = file?;
    ///     println!("File: {} - Created: {:?}",
    ///         file.file_name.as_deref().unwrap_or("unnamed"),
    ///         file.created_at);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated_files(
        &self,
        source_id: &LettaId,
        params: Option<PaginationParams>,
    ) -> PaginatedStream<FileMetadata> {
        let client = self.client.clone();
        let source_id = source_id.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            let source_id = source_id.clone();
            async move {
                let list_params = params.map(|p| ListFilesParams {
                    after: p.after,
                    limit: p.limit.map(|l| l as i32),
                    include_content: None,
                });

                client.sources().list_files(&source_id, list_params).await
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |file: &FileMetadata| {
            file.id
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_default()
        })
    }

    /// Get a paginated stream of passages for a source.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all passages using async iteration.
    ///
    /// # Arguments
    ///
    /// * `source_id` - The ID of the source
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::client::{ClientConfig, LettaClient};
    /// # use letta::{types::PaginationParams, pagination::PaginationExt};
    /// # use letta::types::LettaId;
    /// # use futures::StreamExt;
    /// # use std::str::FromStr;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// let source_id = LettaId::from_str("source-123")?;
    ///
    /// let mut stream = client.sources().paginated_passages(&source_id, None);
    /// while let Some(passage) = stream.next().await {
    ///     let passage = passage?;
    ///     println!("Passage: {}", passage.text);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated_passages(
        &self,
        source_id: &LettaId,
        params: Option<PaginationParams>,
    ) -> PaginatedStream<Passage> {
        let client = self.client.clone();
        let source_id = source_id.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            let source_id = source_id.clone();
            async move {
                let list_params = params.map(|p| ListPassagesParams {
                    after: p.after,
                    before: p.before,
                    limit: p.limit.map(|l| l as i32),
                });

                client
                    .sources()
                    .list_passages(&source_id, list_params)
                    .await
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |passage: &Passage| {
            passage.id.to_string()
        })
    }
}

/// Agent source sub-API operations.
#[derive(Debug)]
pub struct AgentSourceApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentSourceApi<'a> {
    /// Create a new agent source API instance.
    pub fn new(client: &'a LettaClient, agent_id: LettaId) -> Self {
        Self { client, agent_id }
    }

    /// List sources attached to the agent.
    pub async fn list(&self) -> LettaResult<Vec<Source>> {
        self.client
            .get(&format!("v1/agents/{}/sources", self.agent_id))
            .await
    }

    /// Attach a source to the agent.
    pub async fn attach(&self, source_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch(
                &format!("v1/agents/{}/sources/attach/{}", self.agent_id, source_id),
                &(),
            )
            .await
    }

    /// Detach a source from the agent.
    pub async fn detach(&self, source_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch(
                &format!("v1/agents/{}/sources/detach/{}", self.agent_id, source_id),
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
    fn test_source_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = SourceApi::new(&client);
    }

    #[test]
    fn test_agent_source_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = AgentSourceApi::new(
            &client,
            LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440000").unwrap(),
        );
    }
}
