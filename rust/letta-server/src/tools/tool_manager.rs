//! Tool Manager Operations
//!
//! Consolidated tool for tool management operations using discriminator pattern.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::info;
use turbomcp::McpError;

use super::response_utils::{truncate_with_flag, truncate_with_suffix};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolOperation {
    List,
    Get,
    Create,
    Attach,
    BulkAttach,
    Update,
    Delete,
    Upsert,
    Detach,
    GenerateFromPrompt,
    GenerateSchema,
    RunFromSource,
    AddBaseTools,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ToolManagerRequest {
    pub operation: ToolOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<std::collections::HashMap<String, String>>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    // Pagination parameters for list operation (LMS-50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ToolManagerResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

/// Tool summary for list operation (LMS-50 optimization)
/// Excludes source_code, json_schema, and args_json_schema to reduce response size
#[derive(Debug, Serialize)]
pub struct ToolSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // Truncated to 100 chars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    // Metadata counts instead of full content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lines: Option<u32>,
}

pub async fn handle_tool_manager(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing tool operation");

    match request.operation {
        ToolOperation::List => handle_list_tools(client, request).await,
        ToolOperation::Get => handle_get_tool(client, request).await,
        ToolOperation::Create => handle_create_tool(client, request).await,
        ToolOperation::Attach => handle_attach_tool(client, request).await,
        ToolOperation::BulkAttach => handle_bulk_attach(client, request).await,
        ToolOperation::Update => handle_update_tool(client, request).await,
        ToolOperation::Delete => handle_delete_tool(client, request).await,
        ToolOperation::Upsert => handle_upsert_tool(client, request).await,
        ToolOperation::Detach => handle_detach_tool(client, request).await,
        ToolOperation::RunFromSource => handle_run_from_source(client, request).await,
        ToolOperation::AddBaseTools => handle_add_base_tools(client, request).await,
        ToolOperation::GenerateFromPrompt => Err(McpError::internal(
            "generate_from_prompt not available in SDK - requires custom implementation"
                .to_string(),
        )),
        ToolOperation::GenerateSchema => Err(McpError::internal(
            "generate_schema not available in SDK - requires custom implementation".to_string(),
        )),
    }
}

async fn handle_list_tools(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    // LMS-166: Pagination with default limit=25, max=100
    const DEFAULT_LIMIT: u32 = 25;
    const MAX_LIMIT: u32 = 100;

    let limit = request.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let offset = request.offset.unwrap_or(0);

    // Fetch offset + limit items so we can apply client-side offset
    // (SDK uses cursor-based pagination, not offset-based)
    let fetch_count = (offset + limit) as u32;
    let params = letta::types::ListToolsParams {
        limit: Some(fetch_count),
        ..Default::default()
    };

    let tools = client
        .tools()
        .list(Some(params))
        .await
        .map_err(|e| McpError::internal(format!("Failed to list tools: {}", e)))?;

    // Determine true total: if we got a full fetch, there may be more
    let fetched_count = tools.len() as u32;
    let total = if fetched_count < fetch_count {
        fetched_count
    } else {
        client.tools().count().await.unwrap_or(fetched_count)
    };

    // Apply client-side offset (SDK uses cursor, not offset)
    let paginated_tools: Vec<_> = tools
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    let summaries: Vec<ToolSummary> = paginated_tools.iter().map(|t| tool_to_summary(t)).collect();
    let returned = summaries.len();
    let has_more = total > (offset + returned as u32);

    let mut hints = vec!["Use 'get' with tool_id for full source code and schema".to_string()];
    if has_more {
        hints.push(format!(
            "Use offset={} for next page",
            offset + returned as u32
        ));
    }

    Ok(ToolManagerResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Returned {} of {} tools", returned, total),
        data: Some(serde_json::json!({
            "total": total,
            "returned": returned,
            "offset": offset,
            "limit": limit,
            "has_more": has_more,
            "tools": summaries,
            "hints": hints
        })),
        count: Some(returned),
    })
}

async fn handle_get_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let mut tool = client
        .tools()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get tool: {}", e)))?;

    // LMS-50 optimization: Truncate source_code to 2000 chars
    const MAX_SOURCE_CODE_LENGTH: usize = 2000;

    let mut source_code_length = None;
    let mut source_code_truncated = false;
    let mut hint = None;

    if let Some(ref code) = tool.source_code {
        source_code_length = Some(code.len());
        let (truncated, was_truncated) = truncate_with_flag(code, MAX_SOURCE_CODE_LENGTH);
        if was_truncated {
            tool.source_code = Some(truncated.into_owned());
            source_code_truncated = true;
            hint = Some("Full source available via direct API call if needed".to_string());
        }
    }

    let mut tool_json = serde_json::to_value(tool)?;

    // Add metadata about truncation
    if let Some(obj) = tool_json.as_object_mut() {
        if let Some(len) = source_code_length {
            obj.insert("source_code_length".to_string(), serde_json::json!(len));
        }
        obj.insert(
            "source_code_truncated".to_string(),
            serde_json::json!(source_code_truncated),
        );
        if let Some(h) = hint {
            obj.insert("hint".to_string(), serde_json::json!(h));
        }
    }

    Ok(ToolManagerResponse {
        success: true,
        operation: "get".to_string(),
        message: "Tool retrieved successfully".to_string(),
        data: Some(tool_json),
        count: None,
    })
}

async fn handle_create_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let source_code = request
        .source_code
        .ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;

    // Parse source_type if provided
    let source_type = request
        .source_type
        .and_then(|s| match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        });

    let create_request = letta::types::tool::CreateToolRequest {
        source_code,
        description: request.description,
        json_schema: request.json_schema,
        args_json_schema: request.args_json_schema,
        source_type,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
    };

    let tool = client
        .tools()
        .create(create_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "create".to_string(),
        message: "Tool created successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_attach_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let agent_state = client
        .memory()
        .attach_tool_to_agent(&letta_agent_id, &letta_tool_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to attach tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "attach".to_string(),
        message: "Tool attached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_bulk_attach(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let agent_ids = request
        .agent_ids
        .ok_or_else(|| McpError::invalid_request("agent_ids required".to_string()))?;

    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    // Validate all agent IDs upfront, separate valid from invalid
    let mut valid_agents: Vec<(String, letta::types::LettaId)> = Vec::new();
    let mut errors: Vec<serde_json::Value> = Vec::new();

    for agent_id in &agent_ids {
        match letta::types::LettaId::from_str(agent_id) {
            Ok(letta_agent_id) => valid_agents.push((agent_id.clone(), letta_agent_id)),
            Err(e) => errors.push(serde_json::json!({
                "agent_id": agent_id,
                "success": false,
                "error": format!("Invalid agent_id: {}", e)
            })),
        }
    }

    // Concurrent attach instead of sequential loop
    let attach_futures: Vec<_> = valid_agents
        .into_iter()
        .map(|(agent_id_str, letta_agent_id)| {
            let client = client.clone();
            let tool_id = letta_tool_id.clone();
            async move {
                match client
                    .memory()
                    .attach_tool_to_agent(&letta_agent_id, &tool_id)
                    .await
                {
                    Ok(agent_state) => Ok(serde_json::json!({
                        "agent_id": agent_id_str,
                        "success": true,
                        "data": agent_state
                    })),
                    Err(e) => Err(serde_json::json!({
                        "agent_id": agent_id_str,
                        "success": false,
                        "error": e.to_string()
                    })),
                }
            }
        })
        .collect();

    let attach_results = futures::future::join_all(attach_futures).await;

    let mut results: Vec<serde_json::Value> = Vec::new();
    for result in attach_results {
        match result {
            Ok(v) => results.push(v),
            Err(v) => errors.push(v),
        }
    }

    Ok(ToolManagerResponse {
        success: errors.is_empty(),
        operation: "bulk_attach".to_string(),
        message: format!(
            "Attached to {} agents, {} errors",
            results.len(),
            errors.len()
        ),
        data: Some(serde_json::json!({
            "results": results,
            "errors": errors
        })),
        count: Some(results.len()),
    })
}

async fn handle_update_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let update_request = letta::types::tool::UpdateToolRequest {
        description: request.description,
        source_code: request.source_code,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
        metadata: None,
    };

    let tool = client
        .tools()
        .update(&letta_id, update_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to update tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "update".to_string(),
        message: "Tool updated successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_delete_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    client
        .tools()
        .delete(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "delete".to_string(),
        message: "Tool deleted successfully".to_string(),
        data: None,
        count: None,
    })
}

async fn handle_upsert_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let source_code = request
        .source_code
        .ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;

    // Parse source_type if provided
    let source_type = request
        .source_type
        .and_then(|s| match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        });

    let upsert_request = letta::types::tool::CreateToolRequest {
        source_code,
        description: request.description,
        json_schema: request.json_schema,
        args_json_schema: request.args_json_schema,
        source_type,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
    };

    let tool = client
        .tools()
        .upsert(upsert_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to upsert tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "upsert".to_string(),
        message: "Tool upserted successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_detach_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let tool_id = request
        .tool_id
        .ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let agent_state = client
        .memory()
        .detach_tool_from_agent(&letta_agent_id, &letta_tool_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to detach tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "detach".to_string(),
        message: "Tool detached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_run_from_source(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let source_code = request
        .source_code
        .ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;
    let args = request
        .args
        .ok_or_else(|| McpError::invalid_request("args required (JSON object)".to_string()))?;

    // Parse source_type if provided
    let source_type = request
        .source_type
        .and_then(|s| match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        });

    let run_request = letta::types::tool::RunToolFromSourceRequest {
        source_code,
        args,
        env_vars: request.env_vars,
        name: request.name,
        source_type,
        args_json_schema: request.args_json_schema,
        json_schema: request.json_schema,
        pip_requirements: None,
    };

    let response = client
        .tools()
        .run_from_source(run_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to run tool from source: {}", e)))?;

    // LMS-50 optimization: Truncate output to 2000 chars
    const MAX_OUTPUT_LENGTH: usize = 2000;

    let mut response_json = serde_json::to_value(response)?;

    // Check if output exists and truncate if needed
    if let Some(obj) = response_json.as_object_mut() {
        if let Some(output) = obj.get("output").and_then(|v| v.as_str()) {
            let output_length = output.len();
            let (truncated_output, is_truncated) = truncate_with_flag(output, MAX_OUTPUT_LENGTH);

            obj.insert("output".to_string(), serde_json::json!(truncated_output));
            obj.insert(
                "output_length".to_string(),
                serde_json::json!(output_length),
            );
            obj.insert("truncated".to_string(), serde_json::json!(is_truncated));
        }
    }

    Ok(ToolManagerResponse {
        success: true,
        operation: "run_from_source".to_string(),
        message: "Tool executed successfully".to_string(),
        data: Some(response_json),
        count: None,
    })
}

async fn handle_add_base_tools(
    client: &LettaClient,
    _request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let tools = client
        .tools()
        .upsert_base_tools()
        .await
        .map_err(|e| McpError::internal(format!("Failed to add base tools: {}", e)))?;

    // LMS-50 optimization: Return names only, not full definitions
    let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();

    Ok(ToolManagerResponse {
        success: true,
        operation: "add_base_tools".to_string(),
        message: format!("Added {} base tools", tools.len()),
        data: Some(serde_json::json!({
            "tools_added": tools.len(),
            "tool_names": tool_names,
            "hint": "Use 'list' operation to see tool details"
        })),
        count: Some(tools.len()),
    })
}

// ========================================
// Helper Functions for LMS-50 Optimizations
// ========================================

/// Count the number of lines in a string
fn count_lines(s: &str) -> u32 {
    s.lines().count() as u32
}

/// Count the number of properties in a JSON schema
fn count_json_properties(schema: &Option<Value>) -> Option<u32> {
    schema.as_ref().and_then(|s| {
        s.get("properties")
            .and_then(|p| p.as_object())
            .map(|obj| obj.len() as u32)
    })
}

/// Convert Tool to ToolSummary for list operation
fn tool_to_summary(tool: &letta::types::tool::Tool) -> ToolSummary {
    let description = tool
        .description
        .as_ref()
        .map(|d| truncate_with_suffix(d, 100).into_owned());

    let source_lines = tool.source_code.as_ref().map(|code| count_lines(code));
    let args_count = count_json_properties(&tool.args_json_schema);

    ToolSummary {
        id: tool.id.as_ref().map(|id| id.to_string()),
        name: tool.name.clone(),
        description,
        source_type: tool
            .source_type
            .as_ref()
            .map(|st| format!("{:?}", st).to_lowercase()),
        tags: tool.tags.clone(),
        created_at: tool.created_at.as_ref().map(|ts| ts.to_string()),
        args_count,
        source_lines,
    }
}
