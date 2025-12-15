//! Tool-related types.

use crate::types::common::{LettaId, Metadata, Timestamp};
use bon::Builder;
use serde::{Deserialize, Serialize};

/// Tool type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Core Letta tools.
    #[serde(rename = "letta_core")]
    LettaCore,
    /// Memory core tools.
    #[serde(rename = "letta_memory_core")]
    LettaMemoryCore,
    /// Multi-agent core tools.
    #[serde(rename = "letta_multi_agent_core")]
    LettaMultiAgentCore,
    /// Sleeptime core tools.
    #[serde(rename = "letta_sleeptime_core")]
    LettaSleeptimeCore,
    /// Voice sleeptime core tools.
    #[serde(rename = "letta_voice_sleeptime_core")]
    LettaVoiceSleeptimeCore,
    /// Built-in tools.
    #[serde(rename = "letta_builtin")]
    LettaBuiltin,
    /// Files core tools.
    #[serde(rename = "letta_files_core")]
    LettaFilesCore,
    /// External Composio tools.
    #[serde(rename = "external_composio")]
    ExternalComposio,
    /// External LangChain tools.
    #[serde(rename = "external_langchain")]
    ExternalLangchain,
    /// External MCP tools.
    #[serde(rename = "external_mcp")]
    ExternalMcp,
    /// Custom tools.
    Custom,
    /// Other tool types.
    #[serde(other)]
    Other,
}

/// Source type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// Python source code.
    Python,
    /// JavaScript source code.
    JavaScript,
    /// Other source types.
    #[serde(other)]
    Other,
}

/// Pip requirement for a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipRequirement {
    /// Package name.
    pub package: String,
    /// Version constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Tool type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<ToolType>,
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Source type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Tool name.
    pub name: String,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    /// Args JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<serde_json::Value>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Tool metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata_")]
    pub metadata: Option<Metadata>,
    /// When the tool was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the tool was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Tool creation request.
///
/// # Important Requirements
///
/// Letta has strict validation requirements for tool creation:
///
/// 1. **Docstring is MANDATORY**: The Python function in `source_code` MUST have a docstring,
///    even if you provide `json_schema` and `args_json_schema`.
///
/// 2. **Parameter Documentation**: The docstring MUST include an `Args:` section that documents
///    each parameter. Without this, you'll get errors like:
///    `"Parameter 'param_name' in function 'function_name' lacks a description in the docstring"`
///
/// 3. **Returns Documentation**: The docstring should include a `Returns:` section.
///
/// 4. **Schema Requirements**: For best results, provide both:
///    - `json_schema`: The full OpenAI-style function schema
///    - `args_json_schema`: Just the parameters/arguments schema
///
/// # Example
///
/// ```python
/// def my_tool(message: str) -> str:
///     """
///     Process the given message.
///     
///     Args:
///         message: The message to process
///     
///     Returns:
///         str: The processed message
///     """
///     return f"Processed: {message}"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct CreateToolRequest {
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Source code.
    ///
    /// The Python function code. MUST include a properly formatted docstring with Args and Returns sections.
    pub source_code: String,
    /// Source type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /// JSON schema.
    ///
    /// Full OpenAI-style function schema including name, description, and parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    /// Args JSON schema.
    ///
    /// Schema for just the function arguments/parameters. Should match the parameters
    /// section of json_schema. Required for proper validation even if json_schema is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<serde_json::Value>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
}

/// Query parameters for listing tools.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListToolsParams {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Filter by tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Tool update request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateToolRequest {
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
    /// Tool metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata_")]
    pub metadata: Option<Metadata>,
}

/// MCP server type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpServerType {
    /// Server-sent events.
    Sse,
    /// Standard I/O.
    Stdio,
    /// Streamable HTTP.
    StreamableHttp,
}

/// Configuration for an MCP server using SSE.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SseServerConfig {
    /// The name of the server.
    pub server_name: String,
    /// Server type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<McpServerType>,
    /// The URL of the server (MCP SSE client will connect to this URL).
    pub server_url: String,
    /// The name of the authentication header (e.g., 'Authorization').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    /// The authentication token or API key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Custom HTTP headers to include with SSE requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

/// Configuration for an MCP server using STDIO.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StdioServerConfig {
    /// The name of the server.
    pub server_name: String,
    /// Server type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<McpServerType>,
    /// The command to run (MCP 'local' client will run this command).
    pub command: String,
    /// The arguments to pass to the command.
    pub args: Vec<String>,
    /// Environment variables to set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Configuration for an MCP server using Streamable HTTP.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamableHttpServerConfig {
    /// The name of the server.
    pub server_name: String,
    /// Server type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<McpServerType>,
    /// The URL path for the streamable HTTP server (e.g., 'example/mcp').
    pub server_url: String,
    /// The name of the authentication header (e.g., 'Authorization').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
    /// The authentication token or API key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Custom HTTP headers to include with streamable HTTP requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

/// MCP server configuration (union type).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServerConfig {
    /// SSE server configuration.
    Sse(SseServerConfig),
    /// STDIO server configuration.
    Stdio(StdioServerConfig),
    /// Streamable HTTP server configuration.
    StreamableHttp(StreamableHttpServerConfig),
}

/// Update request for SSE MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSseMcpServer {
    /// The name of the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// The URL of the server (MCP SSE client will connect to this URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// The access token or API key for the MCP server (used for SSE authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Update request for Streamable HTTP MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UpdateStreamableHttpMcpServer {
    /// The name of the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// The URL of the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// The access token or API key for the MCP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Update MCP server request (union type).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMcpServerRequest {
    /// Update SSE server.
    Sse(UpdateSseMcpServer),
    /// Update Streamable HTTP server.
    StreamableHttp(UpdateStreamableHttpMcpServer),
}

/// MCP tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    /// Tool name.
    pub name: String,
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input schema.
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
    /// Tool annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
}

/// Tool annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolAnnotations {
    /// Additional annotations.
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}

/// Request to test an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMcpServerRequest {
    /// Server configuration to test.
    #[serde(flatten)]
    pub config: McpServerConfig,
}

/// Request to run a tool from source code.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunToolFromSourceRequest {
    /// Source code of the function to run.
    pub source_code: String,
    /// Arguments to pass to the tool.
    pub args: serde_json::Value,
    /// Environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<std::collections::HashMap<String, String>>,
    /// Tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source type (e.g., "python").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /// Args JSON schema for validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<serde_json::Value>,
    /// Full JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
}

/// Response from running a tool from source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunToolFromSourceResponse {
    /// Response ID (may be "null" string from API).
    pub id: String,
    /// Date of execution.
    pub date: String,
    /// Tool return value.
    pub tool_return: String,
    /// Execution status.
    pub status: ToolExecutionStatus,
    /// Tool call ID (may be "null" string from API).
    pub tool_call_id: String,
    /// Tool name.
    pub name: Option<String>,
    /// Message type.
    pub message_type: String,
    /// Optional trace ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otid: Option<String>,
    /// Sender ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// Step ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    /// Standard output.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stdout: Option<Vec<String>>,
    /// Standard error.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stderr: Option<Vec<String>>,
}

/// Tool execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolExecutionStatus {
    /// Tool executed successfully.
    Success,
    /// Tool execution failed.
    Error,
}

// Composio Integration Types

/// Authentication mode for Composio apps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppAuthSchemeAuthMode {
    /// OAuth 2.0 authentication.
    Oauth2,
    /// API key authentication.
    ApiKey,
    /// Basic authentication.
    Basic,
    /// No authentication required.
    NoAuth,
}

/// Field type for authentication scheme fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSchemeFieldType {
    /// Text field.
    Text,
    /// Password field.
    Password,
    /// Select field.
    Select,
}

/// Authentication scheme field configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSchemeField {
    /// Field name.
    pub name: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Field description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Field type.
    #[serde(rename = "type")]
    pub field_type: AuthSchemeFieldType,
    /// Whether the field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Field options (for select type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// Authentication scheme for a Composio app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAuthScheme {
    /// Scheme identifier.
    pub scheme_name: String,
    /// Authentication mode.
    pub auth_mode: AppAuthSchemeAuthMode,
    /// Authentication fields.
    pub fields: Vec<AuthSchemeField>,
    /// Proxy configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<serde_json::Value>,
}

/// Composio app model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppModel {
    /// App name.
    pub name: String,
    /// App key.
    pub key: String,
    /// App ID.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// App description.
    pub description: String,
    /// App categories.
    pub categories: Vec<String>,
    /// App metadata.
    pub meta: serde_json::Value,
    /// App logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Documentation URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    /// App group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// App status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Whether the app is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether the app requires no authentication.
    #[serde(rename = "noAuth", skip_serializing_if = "Option::is_none")]
    pub no_auth: Option<bool>,
    /// Authentication schemes.
    #[serde(rename = "authSchemes", skip_serializing_if = "Option::is_none")]
    pub auth_schemes: Option<Vec<AppAuthScheme>>,
    /// Test connectors.
    #[serde(rename = "testConnectors", skip_serializing_if = "Option::is_none")]
    pub test_connectors: Option<Vec<serde_json::Value>>,
    /// Triggers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<serde_json::Value>>,
    /// Documentation sections.
    #[serde(rename = "docs_sections", skip_serializing_if = "Option::is_none")]
    pub docs_sections: Option<serde_json::Value>,
}

/// Action parameters/response model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionParametersModel {
    /// Schema properties.
    pub properties: serde_json::Map<String, serde_json::Value>,
    /// Schema title.
    pub title: String,
    /// Schema type.
    #[serde(rename = "type")]
    pub schema_type: String,
    /// Required fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// Schema examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<serde_json::Value>>,
}

/// Composio action model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModel {
    /// Action name.
    pub name: String,
    /// Action description.
    pub description: String,
    /// Action parameters schema.
    pub parameters: ActionParametersModel,
    /// Action response schema.
    pub response: ActionParametersModel,
    /// Parent app name.
    #[serde(rename = "appName")]
    pub app_name: String,
    /// Parent app ID.
    #[serde(rename = "appId")]
    pub app_id: String,
    /// Action version.
    pub version: String,
    /// Available versions.
    #[serde(rename = "availableVersions")]
    pub available_versions: Vec<String>,
    /// Action tags.
    pub tags: Vec<String>,
    /// App logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Display name.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the action is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tool_serialization() {
        let tool = Tool {
            id: Some(LettaId::from_str("tool-550e8400-e29b-41d4-a716-446655440000").unwrap()),
            tool_type: Some(ToolType::Custom),
            name: "calculator".to_string(),
            description: Some("Basic calculator tool".to_string()),
            source_type: Some(SourceType::Python),
            organization_id: None,
            tags: Some(vec!["math".to_string(), "utility".to_string()]),
            source_code: Some("def calculate(x, y): return x + y".to_string()),
            json_schema: Some(serde_json::json!({
                "name": "calculator",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "x": {"type": "number"},
                        "y": {"type": "number"}
                    }
                }
            })),
            args_json_schema: None,
            return_char_limit: Some(1000),
            pip_requirements: None,
            created_by_id: None,
            last_updated_by_id: None,
            metadata: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: Tool = serde_json::from_str(&json).unwrap();
        assert_eq!(tool.name, deserialized.name);
    }

    #[test]
    fn test_tool_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ToolType::LettaCore).unwrap(),
            "\"letta_core\""
        );
        assert_eq!(
            serde_json::to_string(&ToolType::Custom).unwrap(),
            "\"custom\""
        );
        assert_eq!(
            serde_json::to_string(&ToolType::LettaFilesCore).unwrap(),
            "\"letta_files_core\""
        );
    }

    #[test]
    fn test_create_tool_request() {
        let request = CreateToolRequest {
            description: Some("My custom tool".to_string()),
            source_code: "def my_tool(): pass".to_string(),
            source_type: Some(SourceType::Python),
            json_schema: Some(serde_json::json!({
                "name": "my_tool",
                "parameters": {"type": "object"}
            })),
            tags: Some(vec!["custom".to_string()]),
            return_char_limit: None,
            pip_requirements: None,
            args_json_schema: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"source_code\":\"def my_tool(): pass\""));
    }

    #[test]
    fn test_mcp_server_configs() {
        // Test SSE server config
        let sse_config = SseServerConfig {
            server_name: "test-sse".to_string(),
            server_type: Some(McpServerType::Sse),
            server_url: "https://example.com/sse".to_string(),
            auth_header: Some("Authorization".to_string()),
            auth_token: Some("Bearer token123".to_string()),
            custom_headers: None,
        };

        let json = serde_json::to_string(&sse_config).unwrap();
        assert!(json.contains("\"server_name\":\"test-sse\""));
        assert!(json.contains("\"type\":\"sse\""));

        // Test STDIO server config
        let stdio_config = StdioServerConfig {
            server_name: "test-stdio".to_string(),
            server_type: Some(McpServerType::Stdio),
            command: "node".to_string(),
            args: vec!["server.js".to_string()],
            env: Some(std::collections::HashMap::from([(
                "NODE_ENV".to_string(),
                "production".to_string(),
            )])),
        };

        let json = serde_json::to_string(&stdio_config).unwrap();
        assert!(json.contains("\"command\":\"node\""));

        // Test union type serialization
        let config = McpServerConfig::Sse(sse_config);
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"server_name\":\"test-sse\""));

        // Test deserialization
        let deserialized: McpServerConfig = serde_json::from_str(&json).unwrap();
        match deserialized {
            McpServerConfig::Sse(config) => assert_eq!(config.server_name, "test-sse"),
            _ => panic!("Expected SSE config"),
        }
    }

    #[test]
    fn test_mcp_tool() {
        let tool = McpTool {
            name: "calculator".to_string(),
            description: Some("Performs basic calculations".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "x": {"type": "number"},
                    "y": {"type": "number"},
                    "operation": {"type": "string", "enum": ["add", "subtract"]}
                }
            }),
            annotations: None,
        };

        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("\"name\":\"calculator\""));
        assert!(json.contains("\"input_schema\""));
    }
}
