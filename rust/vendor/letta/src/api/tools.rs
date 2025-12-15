//! Tool API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::tool::{
    CreateToolRequest, ListToolsParams, McpServerConfig, McpTool, RunToolFromSourceRequest,
    RunToolFromSourceResponse, TestMcpServerRequest, Tool, UpdateMcpServerRequest,
    UpdateToolRequest,
};
use crate::types::{LettaId, PaginationParams};

/// Tool API operations.
#[derive(Debug)]
pub struct ToolApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ToolApi<'a> {
    /// Create a new tool API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all tools.
    pub async fn list(&self, params: Option<ListToolsParams>) -> LettaResult<Vec<Tool>> {
        let url = "/v1/tools/";
        if let Some(params) = params {
            let query = serde_urlencoded::to_string(&params)?;
            if !query.is_empty() {
                let url_with_query = format!("{}?{}", url, query);
                return self.client.get(&url_with_query).await;
            }
        }
        self.client.get(url).await
    }

    /// Create a new tool.
    pub async fn create(&self, request: CreateToolRequest) -> LettaResult<Tool> {
        self.client.post("/v1/tools/", &request).await
    }

    /// Get a specific tool by ID.
    pub async fn get(&self, tool_id: &LettaId) -> LettaResult<Tool> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.get(&url).await
    }

    /// Update a tool.
    pub async fn update(&self, tool_id: &LettaId, request: UpdateToolRequest) -> LettaResult<Tool> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.patch(&url, &request).await
    }

    /// Delete a tool.
    pub async fn delete(&self, tool_id: &LettaId) -> LettaResult<()> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.delete(&url).await
    }

    /// Get count of tools.
    pub async fn count(&self) -> LettaResult<u32> {
        let response: serde_json::Value = self.client.get("/v1/tools/count").await?;
        // The response is just a bare number
        response.as_u64().map(|v| v as u32).ok_or_else(|| {
            crate::error::LettaError::validation("Invalid count response - expected number")
        })
    }

    /// Upsert a tool (create or update).
    pub async fn upsert(&self, request: CreateToolRequest) -> LettaResult<Tool> {
        self.client.put("/v1/tools/", &request).await
    }

    // MCP Server Management

    /// Get a list of all configured MCP servers.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_mcp_servers(
        &self,
    ) -> LettaResult<std::collections::HashMap<String, McpServerConfig>> {
        self.client.get("v1/tools/mcp/servers").await
    }

    /// Get a list of all configured MCP servers with optional user context.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Optional user ID (sent as user-id query parameter)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_mcp_servers_with_user(
        &self,
        user_id: &str,
    ) -> LettaResult<std::collections::HashMap<String, McpServerConfig>> {
        let url = format!("v1/tools/mcp/servers?user-id={}", user_id);
        self.client.get(&url).await
    }

    /// Add a new MCP server to the Letta MCP server config.
    ///
    /// # Arguments
    ///
    /// * `config` - The MCP server configuration
    ///
    /// # Returns
    ///
    /// Returns a list of MCP server configurations.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn add_mcp_server(
        &self,
        config: McpServerConfig,
    ) -> LettaResult<Vec<McpServerConfig>> {
        self.client.put("v1/tools/mcp/servers", &config).await
    }

    /// Get a list of tools for a specific MCP server.
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the MCP server
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_mcp_tools_by_server(&self, server_name: &str) -> LettaResult<Vec<McpTool>> {
        self.client
            .get(&format!("v1/tools/mcp/servers/{}/tools", server_name))
            .await
    }

    /// Add an MCP tool to Letta from a specific MCP server.
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the MCP server
    /// * `tool_name` - The name of the MCP tool
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn add_mcp_tool(&self, server_name: &str, tool_name: &str) -> LettaResult<Tool> {
        self.client
            .post(
                &format!("v1/tools/mcp/servers/{}/{}", server_name, tool_name),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete an MCP server.
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the MCP server to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete_mcp_server(&self, server_name: &str) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/tools/mcp/servers/{}", server_name))
            .await
    }

    /// Update an MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `server_name` - The name of the MCP server to update
    /// * `request` - The update request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update_mcp_server(
        &self,
        server_name: &str,
        request: UpdateMcpServerRequest,
    ) -> LettaResult<McpServerConfig> {
        self.client
            .patch(&format!("v1/tools/mcp/servers/{}", server_name), &request)
            .await
    }

    /// Test an MCP server connection.
    ///
    /// # Arguments
    ///
    /// * `request` - The test request containing the server configuration
    ///
    /// # Returns
    ///
    /// Returns a list of MCP tools available on the server.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn test_mcp_server(
        &self,
        request: TestMcpServerRequest,
    ) -> LettaResult<Vec<McpTool>> {
        self.client
            .post("v1/tools/mcp/servers/test", &request)
            .await
    }

    /// Run a tool from source code without creating it first.
    ///
    /// This endpoint allows you to execute a tool directly from source code
    /// without needing to create and store it first. Useful for one-off tool
    /// executions or testing.
    ///
    /// # Arguments
    ///
    /// * `request` - The request containing source code and arguments
    ///
    /// # Returns
    ///
    /// Returns the tool execution result including output, status, and any stdout/stderr.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if:
    /// - The source code has validation errors
    /// - The tool execution fails
    /// - The request fails or response cannot be parsed
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::{LettaClient, RunToolFromSourceRequest, SourceType};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = LettaClient::local()?;
    /// let request = RunToolFromSourceRequest {
    ///     source_code: r#"
    /// def add_numbers(a: float, b: float) -> float:
    ///     """Add two numbers.
    ///     
    ///     Args:
    ///         a: First number
    ///         b: Second number
    ///     
    ///     Returns:
    ///         float: Sum of the numbers
    ///     """
    ///     return a + b
    /// "#.to_string(),
    ///     args: json!({ "a": 5, "b": 3 }),
    ///     source_type: Some(SourceType::Python),
    ///     ..Default::default()
    /// };
    ///
    /// let result = client.tools().run_from_source(request).await?;
    /// assert_eq!(result.tool_return, "8");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_from_source(
        &self,
        request: RunToolFromSourceRequest,
    ) -> LettaResult<RunToolFromSourceResponse> {
        self.client.post("/v1/tools/run", &request).await
    }

    // Composio Integration

    /// List all available Composio apps.
    ///
    /// Returns a list of all Composio applications that can be integrated
    /// with Letta. Each app provides a set of actions that can be converted
    /// into Letta tools.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_composio_apps(&self) -> LettaResult<Vec<crate::types::tool::AppModel>> {
        self.client.get("/v1/tools/composio/apps").await
    }

    /// List all actions for a specific Composio app.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the Composio app to get actions for
    ///
    /// # Returns
    ///
    /// Returns a list of actions available for the specified Composio app.
    /// Each action can be converted into a Letta tool.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_composio_actions(
        &self,
        app_name: &str,
    ) -> LettaResult<Vec<crate::types::tool::ActionModel>> {
        self.client
            .get(&format!("/v1/tools/composio/apps/{}/actions", app_name))
            .await
    }

    /// Add a Composio action as a Letta tool.
    ///
    /// Converts a Composio action into a Letta tool that can be attached
    /// to agents and executed.
    ///
    /// # Arguments
    ///
    /// * `action_name` - The name of the Composio action to add as a tool
    ///
    /// # Returns
    ///
    /// Returns the created [`Tool`] object.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn add_composio_tool(&self, action_name: &str) -> LettaResult<Tool> {
        self.client
            .post(
                &format!("/v1/tools/composio/{}", action_name),
                &serde_json::json!({}),
            )
            .await
    }

    /// Upsert base tools.
    ///
    /// Adds or updates the default set of base tools in the Letta system.
    /// This is typically used during initial setup or to refresh the base
    /// tool set.
    ///
    /// # Returns
    ///
    /// Returns a list of all base tools that were added or updated.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn upsert_base_tools(&self) -> LettaResult<Vec<Tool>> {
        self.client
            .post("/v1/tools/add-base-tools", &serde_json::json!({}))
            .await
    }

    /// Get a paginated stream of tools.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all tools using async iteration.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::client::{ClientConfig, LettaClient};
    /// # use letta::{types::PaginationParams, pagination::PaginationExt};
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    ///
    /// let mut stream = client.tools().paginated(None);
    /// while let Some(tool) = stream.next().await {
    ///     let tool = tool?;
    ///     println!("Tool: {} - {}", tool.name, tool.description.as_deref().unwrap_or(""));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<Tool> {
        let client = self.client.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            async move {
                let list_params = params.map(|p| ListToolsParams {
                    after: p.after,
                    before: p.before,
                    limit: p.limit.map(|l| l as u32),
                    name: None,
                });

                client.tools().list(list_params).await
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |tool: &Tool| {
            tool.id
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_tool_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ToolApi::new(&client);
    }
}
