//! MCP Operations
//!
//! Consolidated tool for MCP server lifecycle management.

use letta::{
    types::mcp_server::McpToolExecuteRequestV2,
    types::tool::{McpServerConfig, TestMcpServerRequest, UpdateMcpServerRequest},
    LettaClient,
};
use crate::tools::id_utils::parse_letta_id;
use crate::tools::validation_utils::require_field;
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
    AttachMcpServer,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct McpOpsRequest {
    pub operation: McpOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_args: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Serialize, Default)]
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
    pub mcp_server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_id: Option<String>,
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

impl McpOpsResponse {
    /// Create a success response with operation and message
    fn success(operation: &str, message: &str) -> Self {
        Self {
            success: true,
            operation: operation.to_string(),
            message: message.to_string(),
            data: None,
            servers: None,
            tools: None,
            server_name: None,
            mcp_server_id: None,
            tool_name: None,
            tool_id: None,
            total: None,
            returned: None,
            truncated: None,
            output_length: None,
            hints: None,
        }
    }

    fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    fn with_server_name(mut self, name: String) -> Self {
        self.server_name = Some(name);
        self
    }

    fn with_hint(mut self, hint: &str) -> Self {
        self.hints
            .get_or_insert_with(Vec::new)
            .push(hint.to_string());
        self
    }
}

// Constants for response size optimization
const DEFAULT_SERVERS_LIMIT: usize = 20;
const MAX_SERVERS_LIMIT: usize = 50;
const DEFAULT_TOOLS_LIMIT: usize = 30;
const MAX_TOOLS_LIMIT: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 80;
const MAX_OUTPUT_LENGTH: usize = 3000;

// Common hint messages
mod hints {
    pub const TEST_CONNECTION: &str = "Use 'test' operation to verify connection";
}

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
        McpOperation::AttachMcpServer => handle_attach_mcp_server(client, request).await,
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
    let server_config_value = require_field(request.server_config, "server_config required")?;

    // Deserialize Value to McpServerConfig
    let server_config: McpServerConfig = serde_json::from_value(server_config_value)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    // Extract server name and type from the enum variant
    let (server_name, server_type) = match &server_config {
        McpServerConfig::Sse(config) => (config.server_name.clone(), "sse"),
        McpServerConfig::Stdio(config) => (config.server_name.clone(), "stdio"),
        McpServerConfig::StreamableHttp(config) => (config.server_name.clone(), "http"),
    };

    client
        .tools()
        .add_mcp_server(server_config)
        .await
        .map_err(|e| McpError::internal(format!("Failed to add MCP server: {}", e)))?;

    // Use json! macro instead of manual Map::insert - fewer allocations
    let summary = serde_json::json!({
        "server_name": server_name,
        "server_type": server_type,
    });

    Ok(
        McpOpsResponse::success("add", "MCP server added successfully")
            .with_data(summary)
            .with_hint(hints::TEST_CONNECTION),
    )
}

async fn handle_update_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = require_field(request.server_name, "server_name required")?;
    let server_config_value = require_field(request.server_config, "server_config required")?;

    // Deserialize Value to UpdateMcpServerRequest
    let update_request: UpdateMcpServerRequest = serde_json::from_value(server_config_value)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    client
        .tools()
        .update_mcp_server(&server_name, update_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to update MCP server: {}", e)))?;

    let summary = serde_json::json!({ "server_name": &server_name });

    Ok(
        McpOpsResponse::success("update", "MCP server updated successfully")
            .with_data(summary)
            .with_server_name(server_name)
            .with_hint(hints::TEST_CONNECTION),
    )
}

async fn handle_delete_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = require_field(request.server_name, "server_name required")?;

    client
        .tools()
        .delete_mcp_server(&server_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete MCP server: {}", e)))?;

    Ok(
        McpOpsResponse::success("delete", "MCP server deleted successfully")
            .with_server_name(server_name),
    )
}

async fn handle_test_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_config_value = require_field(request.server_config, "server_config required")?;

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

    // Extract tool names only from the result
    let result_value = serde_json::to_value(&result)?;
    let tool_names: Vec<&str> = result_value
        .get("tools")
        .and_then(|t| t.as_array())
        .map(|tools| {
            tools
                .iter()
                .filter_map(|tool| tool.get("name").and_then(|n| n.as_str()))
                .collect()
        })
        .unwrap_or_default();

    // Use json! macro for compact response
    let test_result = serde_json::json!({
        "status": "success",
        "connection_time_ms": connection_time_ms,
        "tools_available": tool_names.len(),
        "tool_names": tool_names,
    });

    Ok(McpOpsResponse::success("test", "MCP server connection successful").with_data(test_result))
}

async fn handle_connect_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let mcp_server_id =
        require_field(request.mcp_server_id, "mcp_server_id required for connect")?;
    let letta_mcp_server_id = parse_letta_id(&mcp_server_id, "mcp_server_id")?;

    let result = client
        .mcp_servers()
        .connect(&letta_mcp_server_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to connect MCP server: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "connect".into(),
        message: "MCP server connected successfully".into(),
        data: Some(result),
        mcp_server_id: Some(mcp_server_id),
        ..Default::default()
    })
}

async fn handle_resync_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let mcp_server_id = require_field(request.mcp_server_id, "mcp_server_id required for resync")?;
    let letta_mcp_server_id = parse_letta_id(&mcp_server_id, "mcp_server_id")?;

    let result = client
        .mcp_servers()
        .refresh(&letta_mcp_server_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to refresh MCP server tools: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "resync".into(),
        message: "MCP server tools refreshed successfully".into(),
        data: Some(result),
        mcp_server_id: Some(mcp_server_id),
        ..Default::default()
    })
}

async fn handle_execute_tool(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let mcp_server_id = require_field(request.mcp_server_id, "mcp_server_id required for execute")?;
    let tool_id = require_field(request.tool_id, "tool_id required for execute")?;

    let letta_mcp_server_id = parse_letta_id(&mcp_server_id, "mcp_server_id")?;
    let letta_tool_id = parse_letta_id(&tool_id, "tool_id")?;

    let args = match request.tool_args {
        Some(Value::Object(map)) => map,
        Some(_) => {
            return Err(McpError::invalid_request(
                "tool_args must be a JSON object".to_string(),
            ));
        }
        None => serde_json::Map::new(),
    };

    let exec_request = McpToolExecuteRequestV2 { args };

    let result = client
        .mcp_servers()
        .run_tool(&letta_mcp_server_id, &letta_tool_id, exec_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to execute MCP tool: {}", e)))?;

    let mut output = serde_json::to_value(&result)?;
    let mut truncated = false;
    let mut output_length = None;

    if let Some(func_return) = output.get_mut("func_return") {
        if let Ok(serialized) = serde_json::to_string(func_return) {
            let len = serialized.len();
            output_length = Some(len);
            if len > MAX_OUTPUT_LENGTH {
                let preview = &serialized[..MAX_OUTPUT_LENGTH];
                *func_return = serde_json::json!({
                    "truncated": true,
                    "original_length": len,
                    "preview": preview,
                });
                truncated = true;
            }
        }
    }

    Ok(McpOpsResponse {
        success: true,
        operation: "execute".into(),
        message: "MCP tool executed successfully".into(),
        data: Some(output),
        mcp_server_id: Some(mcp_server_id),
        tool_id: Some(tool_id),
        truncated: Some(truncated),
        output_length,
        hints: if truncated {
            Some(vec![format!(
                "Output truncated to {} characters",
                MAX_OUTPUT_LENGTH
            )])
        } else {
            None
        },
        ..Default::default()
    })
}

async fn handle_list_servers(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let result = client
        .mcp_servers()
        .list()
        .await
        .map_err(|e| McpError::internal(format!("Failed to list MCP servers: {}", e)))?;

    let all_servers: Vec<Value> = result
        .into_iter()
        .map(|server| {
            serde_json::json!({
                "id": server.id,
                "name": server.server_name,
                "mcp_server_type": server.mcp_server_type,
                "server_url": server.server_url,
            })
        })
        .collect();

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
    hints.push("Use 'test' operation with server_name for full config".into());

    Ok(McpOpsResponse {
        success: true,
        operation: "list_servers".into(),
        message: format!("Found {} MCP servers", total_count),
        servers: Some(paginated_servers),
        total: Some(total_count),
        returned: Some(returned_count),
        hints: Some(hints),
        ..Default::default()
    })
}

async fn handle_list_tools(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let (all_tools, used_server_name, used_mcp_server_id) =
        if let Some(mcp_server_id) = request.mcp_server_id {
            let letta_mcp_server_id = parse_letta_id(&mcp_server_id, "mcp_server_id")?;

            let tools = client
                .mcp_servers()
                .list_tools(&letta_mcp_server_id)
                .await
                .map_err(|e| McpError::internal(format!("Failed to list MCP tools: {}", e)))?;

            let mapped: Vec<Value> = tools
                .into_iter()
                .map(|tool| {
                    let description = tool.description.clone().unwrap_or_default();
                    let (desc, truncated) = truncate_string(&description, MAX_DESCRIPTION_LENGTH);
                    let mut summary = serde_json::json!({
                        "id": tool.id,
                        "name": tool.name,
                        "description": desc,
                        "source_type": tool.source_type,
                        "tool_type": tool.tool_type,
                    });
                    if truncated {
                        summary["description_truncated"] = serde_json::json!(true);
                    }
                    summary
                })
                .collect();

            (mapped, None, Some(mcp_server_id))
        } else {
            let server_name = request.server_name.ok_or_else(|| {
                McpError::invalid_request("mcp_server_id or server_name required")
            })?;

            let result = client
                .tools()
                .list_mcp_tools_by_server(&server_name)
                .await
                .map_err(|e| McpError::internal(format!("Failed to list MCP tools: {}", e)))?;

            let mapped: Vec<Value> = if let Value::Array(arr) = serde_json::to_value(&result)? {
                arr.into_iter()
                    .map(|tool| {
                        if let Value::Object(mut tool_obj) = tool {
                            let name = tool_obj.remove("name");
                            let description = tool_obj
                                .remove("description")
                                .and_then(|d| d.as_str().map(String::from));

                            let (desc, truncated) = match description {
                                Some(d) => truncate_string(&d, MAX_DESCRIPTION_LENGTH),
                                None => (String::new(), false),
                            };

                            let mut summary = serde_json::json!({
                                "name": name,
                                "server_name": &server_name,
                                "description": desc,
                            });

                            if truncated {
                                summary["description_truncated"] = serde_json::json!(true);
                            }
                            summary
                        } else {
                            tool
                        }
                    })
                    .collect()
            } else if let Value::Object(obj) = serde_json::to_value(&result)? {
                obj.get("tools")
                    .and_then(|t| t.as_array())
                    .cloned()
                    .unwrap_or_default()
            } else {
                vec![]
            };

            (mapped, Some(server_name), None)
        };

    let total_count = all_tools.len();
    let (limit, offset) =
        get_pagination_params(&request.pagination, DEFAULT_TOOLS_LIMIT, MAX_TOOLS_LIMIT);

    // Apply pagination
    let paginated_tools: Vec<Value> = all_tools.into_iter().skip(offset).take(limit).collect();

    let returned_count = paginated_tools.len();
    let has_more = offset + returned_count < total_count;

    let hints = if has_more {
        Some(vec![format!(
            "Showing {} of {} tools. Use pagination to see more.",
            returned_count, total_count
        )])
    } else {
        None
    };

    Ok(McpOpsResponse {
        success: true,
        operation: "list_tools".into(),
        message: format!("Found {} MCP tools", total_count),
        tools: Some(paginated_tools),
        server_name: used_server_name,
        mcp_server_id: used_mcp_server_id,
        total: Some(total_count),
        returned: Some(returned_count),
        hints,
        ..Default::default()
    })
}

async fn handle_register_tool(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = require_field(request.server_name, "server_name required")?;
    let tool_name = require_field(request.tool_name, "tool_name required")?;

    let result = client
        .tools()
        .add_mcp_tool(&server_name, &tool_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to register MCP tool: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "register_tool".into(),
        message: format!(
            "Tool {} from {} registered successfully in Letta",
            tool_name, server_name
        ),
        data: Some(serde_json::to_value(&result)?),
        server_name: Some(server_name),
        tool_name: Some(tool_name),
        ..Default::default()
    })
}

async fn handle_attach_mcp_server(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<McpOpsResponse, McpError> {
    let server_name = require_field(request.server_name, "server_name required")?;
    let agent_id = require_field(request.agent_id, "agent_id required for attach_mcp_server")?;

    let letta_agent_id = parse_letta_id(&agent_id, "agent_id")?;

    let mcp_tools = client
        .tools()
        .list_mcp_tools_by_server(&server_name)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list MCP tools: {}", e)))?;

    let total_discovered = mcp_tools.len();
    if total_discovered == 0 {
        return Ok(McpOpsResponse::success(
            "attach_mcp_server",
            &format!("No tools found on MCP server '{}'", server_name),
        )
        .with_data(serde_json::json!({
            "agent_id": agent_id,
            "total_discovered": 0,
            "registered": 0,
            "attached": 0,
            "failed": 0,
            "tools": []
        }))
        .with_server_name(server_name));
    }

    let mut tools_results = Vec::new();
    let mut registered_count = 0u32;
    let mut attached_count = 0u32;
    let mut failed_count = 0u32;

    for mcp_tool in &mcp_tools {
        let tool_name = &mcp_tool.name;

        match client.tools().add_mcp_tool(&server_name, tool_name).await {
            Ok(registered_tool) => {
                registered_count += 1;
                let tool_id = registered_tool.id.as_ref().map(|id| id.to_string());

                if let Some(ref tid) = registered_tool.id {
                    match client
                        .memory()
                        .attach_tool_to_agent(&letta_agent_id, tid)
                        .await
                    {
                        Ok(_) => {
                            attached_count += 1;
                            tools_results.push(serde_json::json!({
                                "name": tool_name,
                                "tool_id": tool_id,
                                "status": "attached"
                            }));
                        }
                        Err(e) => {
                            failed_count += 1;
                            tools_results.push(serde_json::json!({
                                "name": tool_name,
                                "tool_id": tool_id,
                                "status": "register_ok_attach_failed",
                                "error": e.to_string()
                            }));
                        }
                    }
                } else {
                    failed_count += 1;
                    tools_results.push(serde_json::json!({
                        "name": tool_name,
                        "tool_id": null,
                        "status": "registered_no_id",
                        "error": "Registered tool has no ID"
                    }));
                }
            }
            Err(e) => {
                failed_count += 1;
                tools_results.push(serde_json::json!({
                    "name": tool_name,
                    "tool_id": null,
                    "status": "register_failed",
                    "error": e.to_string()
                }));
            }
        }
    }

    let message = format!(
        "Attached {} of {} tools from '{}' to agent",
        attached_count, total_discovered, server_name
    );

    Ok(McpOpsResponse {
        success: failed_count == 0,
        operation: "attach_mcp_server".into(),
        message,
        data: Some(serde_json::json!({
            "agent_id": agent_id,
            "total_discovered": total_discovered,
            "registered": registered_count,
            "attached": attached_count,
            "failed": failed_count,
            "tools": tools_results
        })),
        server_name: Some(server_name),
        hints: Some(vec!["Use 'list_tools' to verify attached tools".into()]),
        ..Default::default()
    })
}
