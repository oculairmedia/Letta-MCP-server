//! MCP Operations
//!
//! Consolidated tool for MCP server lifecycle management.

use letta::{
    types::tool::{McpServerConfig, TestMcpServerRequest, UpdateMcpServerRequest},
    LettaClient,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum McpOperation {
    Add,
    Update,
    Delete,
    Test,
    Connect,
    Resync,
    Execute,
    ListServers,
    ListTools,
    RegisterTool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct McpOpsRequest {
    pub operation: McpOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_args: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct McpOpsResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,
}

// Constants for response size optimization
const DEFAULT_SERVERS_LIMIT: usize = 20;
const MAX_SERVERS_LIMIT: usize = 50;
const DEFAULT_TOOLS_LIMIT: usize = 30;
const MAX_TOOLS_LIMIT: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 80;
const MAX_OUTPUT_LENGTH: usize = 3000;

pub async fn handle_mcp_ops(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing MCP operation");

    match request.operation {
        McpOperation::Add => handle_add_server(client, request).await,
        McpOperation::Update => handle_update_server(client, request).await,
        McpOperation::Delete => handle_delete_server(client, request).await,
        McpOperation::Test => handle_test_server(client, request).await,
        McpOperation::Connect => handle_connect_server(client, request).await,
        McpOperation::Resync => handle_resync_server(client, request).await,
        McpOperation::Execute => handle_execute_tool(client, request).await,
        McpOperation::ListServers => handle_list_servers(client, request).await,
        McpOperation::ListTools => handle_list_tools(client, request).await,
        McpOperation::RegisterTool => handle_register_tool(client, request).await,
    }
}

/// Truncate a string to max_length characters, adding "..." if truncated
fn truncate_string(s: &str, max_length: usize) -> (String, bool) {
    if s.len() <= max_length {
        (s.to_string(), false)
    } else {
        let truncated = format!("{}...", &s[..max_length.saturating_sub(3)]);
        (truncated, true)
    }
}

/// Extract pagination parameters from request
fn get_pagination_params(
    pagination: &Option<Value>,
    default_limit: usize,
    max_limit: usize,
) -> (usize, usize) {
    let limit = pagination
        .as_ref()
        .and_then(|p| p.get("limit"))
        .and_then(|l| l.as_u64())
        .map(|l| l as usize)
        .unwrap_or(default_limit)
        .min(max_limit);

    let offset = pagination
        .as_ref()
        .and_then(|p| p.get("offset"))
        .and_then(|o| o.as_u64())
        .map(|o| o as usize)
        .unwrap_or(0);

    (limit, offset)
}

async fn handle_add_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_config_value = request
        .server_config
        .ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to McpServerConfig
    let server_config: McpServerConfig = serde_json::from_value(server_config_value)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    // Extract server name and type from the enum variant
    let (server_name, server_type) = match &server_config {
        McpServerConfig::Sse(config) => (config.server_name.clone(), "sse"),
        McpServerConfig::Stdio(config) => (config.server_name.clone(), "stdio"),
        McpServerConfig::StreamableHttp(config) => (config.server_name.clone(), "http"),
    };

    let _result = client
        .tools()
        .add_mcp_server(server_config)
        .await
        .map_err(|e| McpError::internal(format!("Failed to add MCP server: {}", e)))?;

    // Don't echo back full config - return minimal response
    let mut summary = serde_json::Map::new();
    summary.insert("server_name".to_string(), Value::String(server_name));
    summary.insert(
        "server_type".to_string(),
        Value::String(server_type.to_string()),
    );

    Ok(McpOpsResponse {
        success: true,
        operation: "add".to_string(),
        message: "MCP server added successfully".to_string(),
        data: Some(Value::Object(summary)),
        servers: None,
        tools: None,
        server_name: None,
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: Some(vec!["Use 'test' operation to verify connection".to_string()]),
    })
}

async fn handle_update_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let server_config_value = request
        .server_config
        .ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to UpdateMcpServerRequest
    let update_request: UpdateMcpServerRequest = serde_json::from_value(server_config_value)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    let _result = client
        .tools()
        .update_mcp_server(&server_name, update_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to update MCP server: {}", e)))?;

    // Don't echo back full config - return minimal response
    let mut summary = serde_json::Map::new();
    summary.insert(
        "server_name".to_string(),
        Value::String(server_name.clone()),
    );

    Ok(McpOpsResponse {
        success: true,
        operation: "update".to_string(),
        message: "MCP server updated successfully".to_string(),
        data: Some(Value::Object(summary)),
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: Some(vec!["Use 'test' operation to verify connection".to_string()]),
    })
}

async fn handle_delete_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    client
        .tools()
        .delete_mcp_server(&server_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete MCP server: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "delete".to_string(),
        message: "MCP server deleted successfully".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: None,
    })
}

async fn handle_test_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_config_value = request
        .server_config
        .ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to McpServerConfig for the flattened TestMcpServerRequest
    let config: McpServerConfig = serde_json::from_value(server_config_value)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    let test_request = TestMcpServerRequest { config };

    let start_time = std::time::Instant::now();
    let result = client
        .tools()
        .test_mcp_server(test_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to test MCP server: {}", e)))?;
    let connection_time_ms = start_time.elapsed().as_millis() as i64;

    // Build compact response - only tool names, not full definitions
    let mut test_result = serde_json::Map::new();
    test_result.insert("status".to_string(), Value::String("success".to_string()));
    test_result.insert(
        "connection_time_ms".to_string(),
        Value::Number(connection_time_ms.into()),
    );

    // Extract tool names only from the result
    let result_value = serde_json::to_value(&result)?;
    if let Some(tools) = result_value.get("tools").and_then(|t| t.as_array()) {
        let tool_names: Vec<String> = tools
            .iter()
            .filter_map(|tool| {
                tool.get("name")
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        test_result.insert(
            "tools_available".to_string(),
            Value::Number(tool_names.len().into()),
        );
        test_result.insert("tool_names".to_string(), serde_json::to_value(tool_names)?);
    }

    Ok(McpOpsResponse {
        success: true,
        operation: "test".to_string(),
        message: "MCP server connection successful".to_string(),
        data: Some(Value::Object(test_result)),
        servers: None,
        tools: None,
        server_name: None,
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: None,
    })
}

async fn handle_connect_server(
    _client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    // TODO: Implement when SDK adds connect_mcp_server support
    // For now, return a placeholder response
    Ok(McpOpsResponse {
        success: false,
        operation: "connect".to_string(),
        message: "Connect operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: None,
    })
}

async fn handle_resync_server(
    _client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    // TODO: Implement when SDK adds resync support
    // For now, return a placeholder response with summary format
    Ok(McpOpsResponse {
        success: false,
        operation: "resync".to_string(),
        message: "Resync operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: Some(vec![
            "This operation will return summary with counts when implemented".to_string(),
        ]),
    })
}

async fn handle_execute_tool(
    _client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let tool_name = request
        .tool_name
        .ok_or_else(|| McpError::invalid_request("tool_name required".to_string()))?;

    // TODO: Implement when SDK adds tool execution support
    // For now, return a placeholder response
    // When implemented, this should truncate output to MAX_OUTPUT_LENGTH
    Ok(McpOpsResponse {
        success: false,
        operation: "execute".to_string(),
        message: "Execute operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: Some(tool_name),
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: Some(vec![format!(
            "Output will be truncated to {} characters when implemented",
            MAX_OUTPUT_LENGTH
        )]),
    })
}

async fn handle_list_servers(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let result = client
        .tools()
        .list_mcp_servers()
        .await
        .map_err(|e| McpError::internal(format!("Failed to list MCP servers: {}", e)))?;

    // SDK returns object with server names as keys
    let all_servers: Vec<Value> = if let Value::Object(servers_map) = serde_json::to_value(&result)?
    {
        servers_map
            .into_iter()
            .map(|(name, config)| {
                let mut server_summary = serde_json::Map::new();
                server_summary.insert("name".to_string(), Value::String(name.clone()));

                // Extract minimal info, exclude full server_config and oauth_config
                if let Value::Object(config_obj) = config {
                    // Extract server type if available
                    if let Some(config_type) = config_obj.get("config").and_then(|c| c.as_object())
                    {
                        if config_type.contains_key("command") {
                            server_summary.insert(
                                "server_type".to_string(),
                                Value::String("stdio".to_string()),
                            );
                        } else if config_type.contains_key("url") {
                            server_summary.insert(
                                "server_type".to_string(),
                                Value::String("http".to_string()),
                            );
                        } else {
                            server_summary.insert(
                                "server_type".to_string(),
                                Value::String("unknown".to_string()),
                            );
                        }
                    }

                    // Extract status if available
                    if let Some(status) = config_obj.get("status") {
                        server_summary.insert("status".to_string(), status.clone());
                    } else {
                        server_summary
                            .insert("status".to_string(), Value::String("unknown".to_string()));
                    }

                    // Extract tool count if available
                    if let Some(tools) = config_obj.get("tools").and_then(|t| t.as_array()) {
                        server_summary
                            .insert("tool_count".to_string(), Value::Number(tools.len().into()));
                    } else {
                        server_summary.insert("tool_count".to_string(), Value::Number(0.into()));
                    }

                    // Extract last_connected if available
                    if let Some(last_connected) = config_obj.get("last_connected") {
                        server_summary.insert("last_connected".to_string(), last_connected.clone());
                    }
                }

                Value::Object(server_summary)
            })
            .collect()
    } else {
        vec![]
    };

    let total_count = all_servers.len();
    let (limit, offset) = get_pagination_params(
        &request.pagination,
        DEFAULT_SERVERS_LIMIT,
        MAX_SERVERS_LIMIT,
    );

    // Apply pagination
    let paginated_servers: Vec<Value> = all_servers.into_iter().skip(offset).take(limit).collect();

    let returned_count = paginated_servers.len();
    let has_more = offset + returned_count < total_count;

    let mut hints = vec![];
    if has_more {
        hints.push(format!(
            "Showing {} of {} servers. Use pagination to see more.",
            returned_count, total_count
        ));
    }
    hints.push("Use 'test' operation with server_name for full config".to_string());

    Ok(McpOpsResponse {
        success: true,
        operation: "list_servers".to_string(),
        message: format!("Found {} MCP servers", total_count),
        data: None,
        servers: Some(paginated_servers),
        tools: None,
        server_name: None,
        tool_name: None,
        total: Some(total_count),
        returned: Some(returned_count),
        truncated: None,
        output_length: None,
        hints: Some(hints),
    })
}

async fn handle_list_tools(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    let result = client
        .tools()
        .list_mcp_tools_by_server(&server_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list MCP tools: {}", e)))?;

    let all_tools: Vec<Value> = if let Value::Array(arr) = serde_json::to_value(&result)? {
        arr.into_iter()
            .map(|tool| {
                if let Value::Object(mut tool_obj) = tool {
                    let mut simplified = serde_json::Map::new();

                    // Include name
                    if let Some(name) = tool_obj.remove("name") {
                        simplified.insert("name".to_string(), name);
                    }

                    // Add server_name for context
                    simplified.insert(
                        "server_name".to_string(),
                        Value::String(server_name.clone()),
                    );

                    // Include description but truncate to MAX_DESCRIPTION_LENGTH
                    if let Some(Value::String(desc)) = tool_obj.remove("description") {
                        let (truncated_desc, was_truncated) =
                            truncate_string(&desc, MAX_DESCRIPTION_LENGTH);
                        simplified.insert("description".to_string(), Value::String(truncated_desc));
                        if was_truncated {
                            simplified
                                .insert("description_truncated".to_string(), Value::Bool(true));
                        }
                    }

                    // EXCLUDE inputSchema - don't include it

                    Value::Object(simplified)
                } else {
                    tool
                }
            })
            .collect()
    } else if let Value::Object(obj) = serde_json::to_value(&result)? {
        if let Some(Value::Array(tools)) = obj.get("tools") {
            tools.clone()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let total_count = all_tools.len();
    let (limit, offset) =
        get_pagination_params(&request.pagination, DEFAULT_TOOLS_LIMIT, MAX_TOOLS_LIMIT);

    // Apply pagination
    let paginated_tools: Vec<Value> = all_tools.into_iter().skip(offset).take(limit).collect();

    let returned_count = paginated_tools.len();
    let has_more = offset + returned_count < total_count;

    let mut hints = vec![];
    if has_more {
        hints.push(format!(
            "Showing {} of {} tools. Use pagination to see more.",
            returned_count, total_count
        ));
    }

    Ok(McpOpsResponse {
        success: true,
        operation: "list_tools".to_string(),
        message: format!("Found {} tools on server {}", total_count, server_name),
        data: None,
        servers: None,
        tools: Some(paginated_tools),
        server_name: Some(server_name),
        tool_name: None,
        total: Some(total_count),
        returned: Some(returned_count),
        truncated: None,
        output_length: None,
        hints: if hints.is_empty() { None } else { Some(hints) },
    })
}

async fn handle_register_tool(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = request
        .server_name
        .ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let tool_name = request
        .tool_name
        .ok_or_else(|| McpError::invalid_request("tool_name required".to_string()))?;

    let result = client
        .tools()
        .add_mcp_tool(&server_name, &tool_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to register MCP tool: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "register_tool".to_string(),
        message: format!(
            "Tool {} from {} registered successfully in Letta",
            tool_name, server_name
        ),
        data: Some(serde_json::to_value(&result)?),
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: Some(tool_name),
        total: None,
        returned: None,
        truncated: None,
        output_length: None,
        hints: None,
    })
}
