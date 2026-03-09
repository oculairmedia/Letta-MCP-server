//! Tool Manager Operations
//!
//! Consolidated tool for tool management operations using discriminator pattern.

use crate::tools::response_utils::{ToolResponse, paginate};
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use futures::stream::{self, StreamExt};
use letta::LettaClient;
use letta::types::ListAgentsParams;
use letta::types::tool::ListToolsParams;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use turbomcp::McpError;

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

/// Tool manager request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ToolManagerRequest {
    /// The operation to perform (list, get, create, update, delete, upsert, attach, detach, bulk_attach, generate_from_prompt, generate_schema, run_from_source, add_base_tools)
    pub operation: ToolOperation,
    /// Tool ID (required for get, update, delete, attach, detach, bulk_attach)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_id: Option<String>,
    /// Agent ID (required for attach, detach, add_base_tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Agent IDs for bulk_attach operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// Filter agents by name pattern (for list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name_filter: Option<String>,
    /// Filter agents by tag (for list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tag_filter: Option<String>,
    /// Tool source code (required for create, upsert, generate_schema, run_from_source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// Source type: python, json_schema, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// Tags for filtering or categorizing tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Tool description (required for generate_from_prompt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON schema for the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<Value>,
    /// JSON schema for tool arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<Value>,
    /// Maximum characters in tool return value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Arguments for run_from_source operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>,
    /// Environment variables for run_from_source operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<std::collections::HashMap<String, String>>,
    /// Tool name (required for create, upsert, run_from_source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    /// Maximum number of results to return (for list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Number of results to skip (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// When false (default), returns minimal confirmation; when true, returns full state
    #[serde(default)]
    pub verbose: Option<bool>,
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
) -> Result<ToolResponse, McpError> {
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
) -> Result<ToolResponse, McpError> {
    let (limit, offset) = paginate(
        request.limit.map(|l| l as usize),
        request.offset.map(|o| o as usize),
        25,
        100,
    );
    let limit = limit as u32;
    let offset = offset as u32;
    let tag_filter = request.tags.clone();
    let name_filter = request.name.clone();

    let list_params = if name_filter.is_some() {
        Some(ListToolsParams {
            name: name_filter,
            ..Default::default()
        })
    } else {
        None
    };

    let tools = client
        .tools()
        .list(list_params)
        .await
        .map_err(|e| sdk_err("list tools", e))?;

    let filtered_tools: Vec<_> = if let Some(ref filter_tags) = tag_filter {
        tools
            .iter()
            .filter(|t| {
                if let Some(ref tool_tags) = t.tags {
                    filter_tags.iter().any(|ft| tool_tags.contains(ft))
                } else {
                    false
                }
            })
            .collect()
    } else {
        tools.iter().collect()
    };

    let total = filtered_tools.len();

    let paginated_tools: Vec<_> = filtered_tools
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    let summaries: Vec<ToolSummary> = paginated_tools.iter().map(|t| tool_to_summary(t)).collect();

    let returned = summaries.len();

    Ok(
        ToolResponse::success("list", format!("Returned {} of {} tools", returned, total))
            .with_json_data(serde_json::json!({
                "total": total,
                "returned": returned,
                "offset": offset,
                "limit": limit,
                "tools": summaries,
                "hints": vec!["Use 'get' with tool_id for full source code and schema"]
            }))
            .with_count(returned),
    )
}

async fn handle_get_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let tool_id = require_field(request.tool_id, "tool_id required")?;
    let letta_id = require_id(Some(tool_id), "tool_id")?;

    let mut tool = client
        .tools()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get tool", e))?;

    // LMS-50 optimization: Truncate source_code to 2000 chars
    const MAX_SOURCE_CODE_LENGTH: usize = 2000;

    let mut source_code_length = None;
    let mut source_code_truncated = false;
    let mut hint = None;

    if let Some(ref code) = tool.source_code {
        source_code_length = Some(code.len());
        if code.len() > MAX_SOURCE_CODE_LENGTH {
            let (truncated, _) = truncate_string(code, MAX_SOURCE_CODE_LENGTH);
            tool.source_code = Some(truncated);
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

    Ok(ToolResponse::success("get", "Tool retrieved successfully").with_json_data(tool_json))
}

async fn handle_create_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_code = require_field(request.source_code, "source_code required")?;

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
        .map_err(|e| sdk_err("create tool", e))?;

    Ok(ToolResponse::success("create", "Tool created successfully")
        .with_json_data(serde_json::to_value(tool)?))
}

async fn handle_attach_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id.clone(), "agent_id required")?;
    let tool_id = require_field(request.tool_id.clone(), "tool_id required")?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_tool_id = require_id(Some(tool_id.clone()), "tool_id")?;

    let agent_state = client
        .memory()
        .attach_tool_to_agent(&letta_agent_id, &letta_tool_id)
        .await
        .map_err(|e| sdk_err("attach tool", e))?;

    let tool_count = agent_state.as_ref().map(|s| s.tools.len()).unwrap_or(0);

    let data = if verbose {
        agent_state.as_ref().map(serde_json::to_value).transpose()?
    } else {
        Some(create_compact_attach_response(
            &agent_id, &tool_id, tool_count,
        ))
    };

    let mut resp = ToolResponse::success(
        "attach",
        format!(
            "Tool attached successfully. Agent now has {} tools.",
            tool_count
        ),
    );
    if let Some(d) = data {
        resp = resp.with_json_data(d);
    }
    Ok(resp)
}

async fn handle_bulk_attach(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let tool_id = require_field(request.tool_id.clone(), "tool_id required")?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_tool_id = require_id(Some(tool_id.clone()), "tool_id")?;

    let agent_ids = if let Some(ids) = request.agent_ids {
        if !ids.is_empty() {
            ids
        } else {
            let filter_name = request.agent_name_filter.clone();
            let filter_tag = request.agent_tag_filter.clone();

            if filter_name.is_none() && filter_tag.is_none() {
                return Err(McpError::invalid_request(
                    "Either agent_ids or agent_name_filter/agent_tag_filter required".to_string(),
                ));
            }

            let list_params = ListAgentsParams {
                name: filter_name,
                tags: filter_tag.map(|tag| vec![tag]),
                ..Default::default()
            };

            let agents = client
                .agents()
                .list(Some(list_params))
                .await
                .map_err(|e| sdk_err("list agents", e))?;

            if agents.is_empty() {
                return Err(McpError::invalid_request(
                    "No agents matched the provided filters".to_string(),
                ));
            }

            agents.iter().map(|a| a.id.to_string()).collect()
        }
    } else {
        let filter_name = request.agent_name_filter.clone();
        let filter_tag = request.agent_tag_filter.clone();

        if filter_name.is_none() && filter_tag.is_none() {
            return Err(McpError::invalid_request(
                "Either agent_ids or agent_name_filter/agent_tag_filter required".to_string(),
            ));
        }

        let list_params = ListAgentsParams {
            name: filter_name,
            tags: filter_tag.map(|tag| vec![tag]),
            ..Default::default()
        };

        let agents = client
            .agents()
            .list(Some(list_params))
            .await
            .map_err(|e| sdk_err("list agents", e))?;

        if agents.is_empty() {
            return Err(McpError::invalid_request(
                "No agents matched the provided filters".to_string(),
            ));
        }

        agents.iter().map(|a| a.id.to_string()).collect()
    };

    // Parallel fan-out: attach tool to all agents concurrently
    let attach_results: Vec<(String, Result<_, _>)> = stream::iter(agent_ids)
        .map(|agent_id| {
            let letta_tool_id = &letta_tool_id;
            async move {
                let result = match require_id(Some(agent_id.clone()), "agent_id") {
                    Ok(letta_agent_id) => client
                        .memory()
                        .attach_tool_to_agent(&letta_agent_id, letta_tool_id)
                        .await
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e.to_string()),
                };
                (agent_id, result)
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    let mut results = Vec::new();
    let mut errors = Vec::new();
    for (agent_id, result) in attach_results {
        match result {
            Ok(agent_state) => {
                let tool_count = agent_state.as_ref().map(|s| s.tools.len()).unwrap_or(0);
                if verbose {
                    results.push(serde_json::json!({
                        "agent_id": agent_id,
                        "success": true,
                        "tool_count": tool_count,
                        "data": agent_state
                    }));
                } else {
                    results.push(serde_json::json!({
                        "agent_id": agent_id,
                        "success": true,
                        "tool_count": tool_count
                    }));
                }
            }
            Err(e) => {
                errors.push(serde_json::json!({
                    "agent_id": agent_id,
                    "success": false,
                    "error": e
                }));
            }
        }
    }

    let resp = if errors.is_empty() {
        ToolResponse::success(
            "bulk_attach",
            format!(
                "Attached to {} agents, {} errors",
                results.len(),
                errors.len()
            ),
        )
    } else {
        ToolResponse::error(
            "bulk_attach",
            format!(
                "Attached to {} agents, {} errors",
                results.len(),
                errors.len()
            ),
        )
    };
    Ok(resp
        .with_json_data(serde_json::json!({
            "tool_id": tool_id,
            "results": results,
            "errors": errors
        }))
        .with_count(results.len()))
}

async fn handle_update_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let tool_id = require_field(request.tool_id, "tool_id required")?;
    let letta_id = require_id(Some(tool_id), "tool_id")?;

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
        .map_err(|e| sdk_err("update tool", e))?;

    Ok(ToolResponse::success("update", "Tool updated successfully")
        .with_json_data(serde_json::to_value(tool)?))
}

async fn handle_delete_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let tool_id = require_field(request.tool_id, "tool_id required")?;
    let letta_id = require_id(Some(tool_id), "tool_id")?;

    client
        .tools()
        .delete(&letta_id)
        .await
        .map_err(|e| sdk_err("delete tool", e))?;

    Ok(ToolResponse::success("delete", "Tool deleted successfully"))
}

async fn handle_upsert_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_code = require_field(request.source_code, "source_code required")?;

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
        .map_err(|e| sdk_err("upsert tool", e))?;

    Ok(
        ToolResponse::success("upsert", "Tool upserted successfully")
            .with_json_data(serde_json::to_value(tool)?),
    )
}

async fn handle_detach_tool(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id.clone(), "agent_id required")?;
    let tool_id = require_field(request.tool_id.clone(), "tool_id required")?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_tool_id = require_id(Some(tool_id.clone()), "tool_id")?;

    let agent_state = client
        .memory()
        .detach_tool_from_agent(&letta_agent_id, &letta_tool_id)
        .await
        .map_err(|e| sdk_err("detach tool", e))?;

    let tool_count = agent_state.as_ref().map(|s| s.tools.len()).unwrap_or(0);

    let data = if verbose {
        agent_state.as_ref().map(serde_json::to_value).transpose()?
    } else {
        Some(create_compact_detach_response(
            &agent_id, &tool_id, tool_count,
        ))
    };

    let mut resp = ToolResponse::success(
        "detach",
        format!(
            "Tool detached successfully. Agent now has {} tools.",
            tool_count
        ),
    );
    if let Some(d) = data {
        resp = resp.with_json_data(d);
    }
    Ok(resp)
}

async fn handle_run_from_source(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_code = require_field(request.source_code, "source_code required")?;
    let args = require_field(request.args, "args required (JSON object)")?;

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
        .map_err(|e| sdk_err("run tool from source", e))?;

    // LMS-50 optimization: Truncate output to 2000 chars
    const MAX_OUTPUT_LENGTH: usize = 2000;

    let mut response_json = serde_json::to_value(response)?;

    // Check if output exists and truncate if needed
    if let Some(obj) = response_json.as_object_mut()
        && let Some(output) = obj.get("output").and_then(|v| v.as_str())
    {
        let output_length = output.len();
        let (truncated_output, is_truncated) = truncate_string(output, MAX_OUTPUT_LENGTH);

        obj.insert("output".to_string(), serde_json::json!(truncated_output));
        obj.insert(
            "output_length".to_string(),
            serde_json::json!(output_length),
        );
        obj.insert("truncated".to_string(), serde_json::json!(is_truncated));
    }

    Ok(
        ToolResponse::success("run_from_source", "Tool executed successfully")
            .with_json_data(response_json),
    )
}

async fn handle_add_base_tools(
    client: &LettaClient,
    _request: ToolManagerRequest,
) -> Result<ToolResponse, McpError> {
    let tools = client
        .tools()
        .upsert_base_tools()
        .await
        .map_err(|e| sdk_err("add base tools", e))?;

    // LMS-50 optimization: Return names only, not full definitions
    let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();

    Ok(ToolResponse::success(
        "add_base_tools",
        format!("Added {} base tools", tools.len()),
    )
    .with_json_data(serde_json::json!({
        "tools_added": tools.len(),
        "tool_names": tool_names,
        "hint": "Use 'list' operation to see tool details"
    }))
    .with_count(tools.len()))
}

// ========================================
// Helper Functions for LMS-50/LMS-113 Optimizations
// ========================================

fn create_compact_attach_response(agent_id: &str, tool_id: &str, tool_count: usize) -> Value {
    serde_json::json!({
        "agent_id": agent_id,
        "tool_id": tool_id,
        "tool_count": tool_count
    })
}

fn create_compact_detach_response(agent_id: &str, tool_id: &str, tool_count: usize) -> Value {
    serde_json::json!({
        "agent_id": agent_id,
        "tool_id": tool_id,
        "tool_count": tool_count
    })
}

fn truncate_string(s: &str, max_len: usize) -> (String, bool) {
    if s.len() <= max_len {
        (s.to_string(), false)
    } else {
        let truncated = s.chars().take(max_len).collect::<String>();
        (format!("{}...[truncated]", truncated), true)
    }
}

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
    let description = tool.description.as_ref().map(|d| {
        let (truncated, _) = truncate_string(d, 100);
        truncated
    });

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
