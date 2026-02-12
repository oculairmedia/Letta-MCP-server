//! MCP server v2 management types for the Letta API.

use crate::types::common::LettaId;
use serde::{Deserialize, Serialize};

/// MCP transport discriminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpServerTransportType {
    /// Server-sent events transport.
    Sse,
    /// Stdio transport.
    Stdio,
    /// Streamable HTTP transport.
    StreamableHttp,
}

/// SSE transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSseMcpServerConfig {
    /// Transport type marker.
    pub mcp_server_type: McpServerTransportType,
    /// Endpoint URL.
    pub server_url: String,
    /// Optional auth header name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    /// Optional auth token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Optional custom headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

/// Streamable HTTP transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamableHttpMcpServerConfig {
    /// Transport type marker.
    pub mcp_server_type: McpServerTransportType,
    /// Endpoint URL.
    pub server_url: String,
    /// Optional auth header name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    /// Optional auth token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Optional custom headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

/// Stdio transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStdioMcpServerConfig {
    /// Transport type marker.
    pub mcp_server_type: McpServerTransportType,
    /// Command to execute.
    pub command: String,
    /// Command arguments.
    pub args: Vec<String>,
    /// Optional environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Union transport configuration for create/update requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServerConfigV2 {
    /// SSE transport config.
    Sse(CreateSseMcpServerConfig),
    /// Streamable HTTP transport config.
    StreamableHttp(CreateStreamableHttpMcpServerConfig),
    /// Stdio transport config.
    Stdio(CreateStdioMcpServerConfig),
}

/// Request to create an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMcpServerRequestV2 {
    /// Server display name.
    pub server_name: String,
    /// Transport configuration payload.
    pub config: McpServerConfigV2,
}

/// Request to update an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMcpServerRequestV2 {
    /// Optional updated server name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// Transport configuration payload.
    pub config: McpServerConfigV2,
}

/// MCP server object returned by v2 endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerSchemaV2 {
    /// Server ID.
    pub id: LettaId,
    /// Transport type string.
    pub server_type: String,
    /// Server name.
    pub server_name: String,
    /// Optional URL when remote transport is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// Optional stdio config payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdio_config: Option<serde_json::Value>,
    /// Optional metadata payload.
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata_")]
    pub metadata: Option<serde_json::Value>,
}

/// Request body for MCP tool execution.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McpToolExecuteRequestV2 {
    /// Tool arguments payload.
    #[serde(default)]
    pub args: serde_json::Map<String, serde_json::Value>,
}

/// MCP tool execution response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolExecutionResultV2 {
    /// Execution status string.
    pub status: String,
    /// Optional tool return payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func_return: Option<serde_json::Value>,
    /// Optional agent state payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_state: Option<serde_json::Value>,
    /// Optional stdout entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<Vec<String>>,
    /// Optional stderr entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<Vec<String>>,
    /// Optional sandbox config fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_config_fingerprint: Option<String>,
}
