//! MCP server v2 API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::mcp_server::{
    CreateMcpServerRequestV2, ListMcpServersParams, McpServerSchemaV2, McpToolExecuteRequestV2,
    McpToolExecutionResultV2, UpdateMcpServerRequestV2,
};
use crate::types::{LettaId, Tool};

/// MCP server v2 API operations.
#[derive(Debug)]
pub struct McpServerApi<'a> {
    client: &'a LettaClient,
}

impl<'a> McpServerApi<'a> {
    /// Create a new MCP server API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Create an MCP server.
    pub async fn create(
        &self,
        request: CreateMcpServerRequestV2,
    ) -> LettaResult<McpServerSchemaV2> {
        self.client.post("v1/mcp-servers/", &request).await
    }

    /// List MCP servers.
    pub async fn list(
        &self,
        params: Option<ListMcpServersParams>,
    ) -> LettaResult<Vec<McpServerSchemaV2>> {
        self.client
            .get_with_query("v1/mcp-servers/", &params.unwrap_or_default())
            .await
    }

    /// Get an MCP server by ID.
    pub async fn get(&self, mcp_server_id: &LettaId) -> LettaResult<McpServerSchemaV2> {
        self.client
            .get(&format!("v1/mcp-servers/{}", mcp_server_id))
            .await
    }

    /// MCP server patch endpoint.
    pub async fn update(
        &self,
        mcp_server_id: &LettaId,
        request: UpdateMcpServerRequestV2,
    ) -> LettaResult<McpServerSchemaV2> {
        self.client
            .patch(&format!("v1/mcp-servers/{}", mcp_server_id), &request)
            .await
    }

    /// MCP server removal endpoint.
    pub async fn delete(&self, mcp_server_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/mcp-servers/{}", mcp_server_id))
            .await
    }

    /// Connect to an MCP server by ID.
    pub async fn connect(&self, mcp_server_id: &LettaId) -> LettaResult<serde_json::Value> {
        self.client
            .get(&format!("v1/mcp-servers/connect/{}", mcp_server_id))
            .await
    }

    /// Refresh tools for an MCP server by ID.
    pub async fn refresh(&self, mcp_server_id: &LettaId) -> LettaResult<serde_json::Value> {
        self.client
            .patch_no_body(&format!("v1/mcp-servers/{}/refresh", mcp_server_id))
            .await
    }

    /// List tools for an MCP server by ID.
    pub async fn list_tools(&self, mcp_server_id: &LettaId) -> LettaResult<Vec<Tool>> {
        self.client
            .get(&format!("v1/mcp-servers/{}/tools", mcp_server_id))
            .await
    }

    /// Get a tool from an MCP server.
    pub async fn get_tool(&self, mcp_server_id: &LettaId, tool_id: &LettaId) -> LettaResult<Tool> {
        self.client
            .get(&format!(
                "v1/mcp-servers/{}/tools/{}",
                mcp_server_id, tool_id
            ))
            .await
    }

    /// Execute a tool on an MCP server.
    pub async fn run_tool(
        &self,
        mcp_server_id: &LettaId,
        tool_id: &LettaId,
        request: McpToolExecuteRequestV2,
    ) -> LettaResult<McpToolExecutionResultV2> {
        self.client
            .post(
                &format!("v1/mcp-servers/{}/tools/{}/run", mcp_server_id, tool_id),
                &request,
            )
            .await
    }
}

impl LettaClient {
    /// Get the MCP server v2 API.
    pub fn mcp_servers(&self) -> McpServerApi<'_> {
        McpServerApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_mcp_server_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = McpServerApi::new(&client);
    }
}
