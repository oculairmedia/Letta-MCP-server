//! Folder API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::agent::AgentState;
use crate::types::folder::{CreateFolderRequest, Folder, ListFoldersParams, UpdateFolderRequest};
use crate::types::memory::Passage;
use crate::types::source::FileMetadata;
use crate::types::LettaId;
use bytes::Bytes;
use reqwest::multipart::{Form, Part};

/// Folder API operations.
#[derive(Debug)]
pub struct FolderApi<'a> {
    client: &'a LettaClient,
}

impl<'a> FolderApi<'a> {
    /// Create a new folder API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all folders.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListFoldersParams>) -> LettaResult<Vec<Folder>> {
        self.client
            .get_with_query("v1/folders/", &params.unwrap_or_default())
            .await
    }

    /// Create a new folder.
    ///
    /// # Arguments
    ///
    /// * `request` - The folder creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: CreateFolderRequest) -> LettaResult<Folder> {
        self.client.post("v1/folders/", &request).await
    }

    /// Get a folder by ID.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, folder_id: &LettaId) -> LettaResult<Folder> {
        self.client.get(&format!("v1/folders/{}", folder_id)).await
    }

    /// Update a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to update
    /// * `request` - The update request with fields to modify
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        folder_id: &LettaId,
        request: UpdateFolderRequest,
    ) -> LettaResult<Folder> {
        self.client
            .patch(&format!("v1/folders/{}", folder_id), &request)
            .await
    }

    /// Delete a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, folder_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/folders/{}", folder_id))
            .await
    }

    /// Get the count of all folders.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn count(&self) -> LettaResult<u32> {
        self.client.get("v1/folders/count").await
    }

    /// Get a folder by name.
    ///
    /// # Arguments
    ///
    /// * `folder_name` - The name of the folder to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_by_name(&self, folder_name: &str) -> LettaResult<Folder> {
        self.client
            .get(&format!("v1/folders/name/{}", folder_name))
            .await
    }

    /// Retrieve metadata for folders.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_metadata(&self) -> LettaResult<serde_json::Value> {
        self.client.get("v1/folders/metadata").await
    }

    /// Upload a file to a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to upload to
    /// * `file_name` - The name of the file
    /// * `file_data` - The file data as bytes
    /// * `content_type` - Optional content type (MIME type)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn upload_file(
        &self,
        folder_id: &LettaId,
        file_name: String,
        file_data: Bytes,
        content_type: Option<String>,
    ) -> LettaResult<FileMetadata> {
        let form = Form::new();
        let mut part = Part::bytes(file_data.to_vec()).file_name(file_name);

        if let Some(ct) = content_type {
            part = part.mime_str(&ct)?;
        }

        let form = form.part("file", part);

        self.client
            .post_multipart(&format!("v1/folders/{}/upload", folder_id), form)
            .await
    }

    /// List agents in a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_agents(&self, folder_id: &LettaId) -> LettaResult<Vec<String>> {
        self.client
            .get(&format!("v1/folders/{}/agents", folder_id))
            .await
    }

    /// List passages in a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_passages(&self, folder_id: &LettaId) -> LettaResult<Vec<Passage>> {
        self.client
            .get(&format!("v1/folders/{}/passages", folder_id))
            .await
    }

    /// List files in a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_files(&self, folder_id: &LettaId) -> LettaResult<Vec<FileMetadata>> {
        self.client
            .get(&format!("v1/folders/{}/files", folder_id))
            .await
    }

    /// Delete a file from a folder.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder
    /// * `file_id` - The ID of the file to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete_file(&self, folder_id: &LettaId, file_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/folders/{}/{}", folder_id, file_id))
            .await
    }

    /// Get agent-folder relationship sub-API for a specific agent.
    pub fn agent(&self, agent_id: LettaId) -> AgentFolderApi {
        AgentFolderApi::new(self.client, agent_id)
    }
}

/// Agent-folder relationship API operations.
#[derive(Debug)]
pub struct AgentFolderApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentFolderApi<'a> {
    /// Create a new agent-folder API instance.
    pub fn new(client: &'a LettaClient, agent_id: LettaId) -> Self {
        Self { client, agent_id }
    }

    /// Attach a folder to the agent.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to attach
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn attach(&self, folder_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch_no_body(&format!(
                "v1/agents/{}/folders/attach/{}",
                self.agent_id, folder_id
            ))
            .await
    }

    /// Detach a folder from the agent.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to detach
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn detach(&self, folder_id: &LettaId) -> LettaResult<AgentState> {
        self.client
            .patch_no_body(&format!(
                "v1/agents/{}/folders/detach/{}",
                self.agent_id, folder_id
            ))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_folder_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = FolderApi::new(&client);
    }

    #[test]
    fn test_agent_folder_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let agent_id = "agent-550e8400-e29b-41d4-a716-446655440000"
            .parse()
            .unwrap();
        let _api = AgentFolderApi::new(&client, agent_id);
    }
}
