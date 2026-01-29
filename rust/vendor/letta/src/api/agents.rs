//! Agent API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::{PaginatedStream, PaginationExt};
use crate::types::{
    AgentState, AgentsSearchRequest, AgentsSearchResponse, CreateAgentRequest, ImportAgentRequest,
    LettaId, ListAgentsParams, PaginationParams,
};
use reqwest::multipart::{Form, Part};
use std::path::Path;

/// Agent API operations.
#[derive(Debug)]
pub struct AgentApi<'a> {
    client: &'a LettaClient,
}

impl<'a> AgentApi<'a> {
    /// Create a new agent API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all agents.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListAgentsParams>) -> LettaResult<Vec<AgentState>> {
        self.client
            .get_with_query("v1/agents/", &params.unwrap_or_default())
            .await
    }

    /// Create a new agent.
    ///
    /// # Arguments
    ///
    /// * `request` - The agent creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: CreateAgentRequest) -> LettaResult<AgentState> {
        self.client.post("v1/agents", &request).await
    }

    /// Create a new agent with optional project context.
    ///
    /// # Arguments
    ///
    /// * `request` - The agent creation request
    /// * `project_id` - Optional project ID to associate the agent with (sent as X-Project header)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_with_project(
        &self,
        request: CreateAgentRequest,
        project_id: &str,
    ) -> LettaResult<AgentState> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Project",
            project_id.parse().map_err(|_| {
                crate::error::LettaError::validation("Invalid X-Project header value")
            })?,
        );

        self.client
            .post_with_headers("v1/agents", &request, headers)
            .await
    }

    /// Get a specific agent by ID.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, agent_id: &LettaId) -> LettaResult<AgentState> {
        self.client.get(&format!("v1/agents/{}", agent_id)).await
    }

    /// Delete an agent by ID.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, agent_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/agents/{}", agent_id))
            .await
    }

    /// Update an existing agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to update
    /// * `request` - The update request with fields to modify
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        agent_id: &LettaId,
        request: crate::types::UpdateAgentRequest,
    ) -> LettaResult<AgentState> {
        self.client
            .patch(&format!("v1/agents/{}", agent_id), &request)
            .await
    }

    /// Get context window information for an agent.
    ///
    /// Returns information about the agent's context window usage.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_context(&self, agent_id: &LettaId) -> LettaResult<serde_json::Value> {
        self.client
            .get(&format!("v1/agents/{}/context", agent_id))
            .await
    }

    /// Reset an agent's message history.
    ///
    /// This clears all conversation messages for the agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to reset
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn reset_messages(&self, agent_id: &LettaId) -> LettaResult<()> {
        self.client
            .post(
                &format!("v1/agents/{}/reset-messages", agent_id),
                &serde_json::json!({}),
            )
            .await
            .map(|_: serde_json::Value| ())
    }

    /// Get file session API for this agent.
    ///
    /// Returns an API instance for managing files in the agent's session.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent
    pub fn files(&self, agent_id: LettaId) -> crate::api::AgentFileApi {
        crate::api::AgentFileApi::new(self.client, agent_id)
    }

    /// Get folder API for this agent.
    ///
    /// Returns an API instance for managing folder attachments.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent
    pub fn folders(&self, agent_id: LettaId) -> crate::api::AgentFolderApi {
        crate::api::AgentFolderApi::new(self.client, agent_id)
    }

    /// Summarize an agent's conversation history to a target message length.
    ///
    /// This endpoint summarizes the current message history for a given agent,
    /// truncating and compressing it down to the specified `max_message_length`.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose conversation to summarize
    /// * `max_message_length` - Maximum number of messages to retain after summarization
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn summarize_agent_conversation(
        &self,
        agent_id: &LettaId,
        max_message_length: u32,
    ) -> LettaResult<AgentState> {
        // Empty body for POST with query params
        self.client
            .post(
                &format!(
                    "v1/agents/{}/summarize?max_message_length={}",
                    agent_id, max_message_length
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Get the count of all agents associated with a given user.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn count(&self) -> LettaResult<u32> {
        self.client.get("v1/agents/count").await
    }

    /// Export the serialized JSON representation of an agent, formatted with indentation.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to export
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn export_file(&self, agent_id: &LettaId) -> LettaResult<String> {
        // The export endpoint returns a JSON object, but we need to return it as a string
        let json_value: serde_json::Value = self
            .client
            .get(&format!("v1/agents/{}/export", agent_id))
            .await?;

        // Serialize the JSON value to a string
        Ok(serde_json::to_string(&json_value)?)
    }

    /// Import an agent from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the agent file to import
    /// * `request` - Import parameters
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn import_file(
        &self,
        file_path: &Path,
        request: ImportAgentRequest,
    ) -> LettaResult<AgentState> {
        // Read the file
        let file_content = tokio::fs::read(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("agent.json");

        // Create multipart form
        let file_part = Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/json")?;

        let form = Form::new().part("file", file_part);

        // Add query parameters to the form
        let mut params = Vec::new();

        if let Some(append_copy_suffix) = request.append_copy_suffix {
            params.push(("append_copy_suffix", append_copy_suffix.to_string()));
        }
        if let Some(override_existing_tools) = request.override_existing_tools {
            params.push((
                "override_existing_tools",
                override_existing_tools.to_string(),
            ));
        }
        if let Some(project_id) = request.project_id {
            params.push(("project_id", project_id.to_string()));
        }
        if let Some(strip_messages) = request.strip_messages {
            params.push(("strip_messages", strip_messages.to_string()));
        }

        // Build the path with query parameters
        let mut path = String::from("v1/agents/import");
        if !params.is_empty() {
            path.push('?');
            path.push_str(&serde_urlencoded::to_string(&params)?);
        }

        self.client.post_multipart(&path, form).await
    }

    /// Search for agents using various criteria.
    ///
    /// # Arguments
    ///
    /// * `request` - Search parameters and filters
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn search(&self, request: AgentsSearchRequest) -> LettaResult<AgentsSearchResponse> {
        self.client.post("v1/agents/search", &request).await
    }

    /// List groups that an agent belongs to.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose groups to list
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_groups(&self, agent_id: &LettaId) -> LettaResult<Vec<crate::types::Group>> {
        self.client
            .get(&format!("v1/agents/{}/groups", agent_id))
            .await
    }

    /// List agents with pagination support.
    ///
    /// Returns a stream that automatically fetches subsequent pages as needed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use letta::{LettaClient, ClientConfig};
    /// # use letta::types::PaginationParams;
    /// # use futures::StreamExt;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    /// // Get all agents, fetching pages automatically
    /// let mut stream = client.agents().paginated(None);
    ///
    /// while let Some(agent) = stream.next().await {
    ///     let agent = agent?;
    ///     println!("Agent: {}", agent.name);
    /// }
    ///
    /// // Or collect all agents at once
    /// let all_agents = client.agents()
    ///     .paginated(Some(PaginationParams::new().limit(50)))
    ///     .collect()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<AgentState> {
        let client = self.client.clone();

        // Convert PaginationParams to ListAgentsParams
        let list_params = params.as_ref().map(|p| ListAgentsParams {
            before: p.before.clone(),
            after: p.after.clone(),
            limit: p.limit,
            ..Default::default()
        });

        PaginatedStream::new_with_id_cursor(
            params,
            move |page_params| {
                let client = client.clone();
                let mut effective_params = list_params.clone().unwrap_or_default();

                // Update pagination fields from page_params
                if let Some(p) = page_params {
                    effective_params.before = p.before;
                    effective_params.after = p.after;
                    effective_params.limit = p.limit;
                }

                async move { client.agents().list(Some(effective_params)).await }
            },
            |agent| &agent.id,
        )
    }
}

impl<'a> PaginationExt for AgentApi<'a> {
    type Item = AgentState;

    fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<Self::Item> {
        self.paginated(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_agent_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = AgentApi::new(&client);
    }
}
