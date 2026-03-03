//! Source Manager Operations
//!
//! Consolidated tool for source management operations.

use crate::tools::response_utils::{ToolResponse, paginate};
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use serde::{Deserialize, Serialize};
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceOperation {
    List,
    Get,
    Create,
    Update,
    Delete,
    Attach,
    Detach,
    ListAttached,
    Upload,
    DeleteFiles,
    ListFiles,
    Count,
    ListAgentsUsing,
    // Note: ListFolders and GetFolderContents have been moved to letta_file_folder_ops tool
}

/// Source manager request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SourceManagerRequest {
    /// The operation to perform (list, get, create, update, delete, count, attach, detach, list_attached, upload, delete_files, list_files, list_agents_using)
    pub operation: SourceOperation,
    /// Source ID (required for get, update, delete, attach, detach, upload, delete_files, list_files, list_agents_using)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// Agent ID (required for attach, detach, list_attached)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Source name (required for create)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// File ID (required for delete_files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// File name (required for upload)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Base64-encoded file data (required for upload)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// MIME content type for upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Maximum number of results to return (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Include full content in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    /// When false (default), returns minimal confirmation; when true, returns full state
    #[serde(default)]
    pub verbose: Option<bool>,
}

/// Pagination metadata for list operations
#[derive(Debug, Serialize)]
pub struct PaginationMetadata {
    pub total: usize,
    pub returned: usize,
    pub limit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Optimized source summary (excludes full file/agent arrays)
#[derive(Debug, Serialize)]
pub struct SourceSummary {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // Truncated to 100 chars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    // Counts instead of full arrays
    pub file_count: u32,
    pub attached_agent_count: u32,
}

/// Optimized file summary (never includes content)
#[derive(Debug, Serialize)]
pub struct FileSummary {
    pub id: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<String>,
}

/// Minimal agent reference (ID and name only)
#[derive(Debug, Serialize)]
pub struct AgentReference {
    pub id: String,
    pub name: String,
}

/// Minimal file upload response
#[derive(Debug, Serialize)]
pub struct FileUploadSummary {
    pub success: bool,
    pub file_id: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// Truncate a string to a maximum length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...[truncated]", &s[..max_len])
    }
}

pub async fn handle_source_manager(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing source operation");

    match request.operation {
        SourceOperation::List => handle_list_sources(client, request).await,
        SourceOperation::Get => handle_get_source(client, request).await,
        SourceOperation::Create => handle_create_source(client, request).await,
        SourceOperation::Update => handle_update_source(client, request).await,
        SourceOperation::Delete => handle_delete_source(client, request).await,
        SourceOperation::Attach => handle_attach_source(client, request).await,
        SourceOperation::Detach => handle_detach_source(client, request).await,
        SourceOperation::Count => handle_count_sources(client, request).await,
        SourceOperation::ListAttached => handle_list_attached(client, request).await,
        SourceOperation::ListFiles => handle_list_files(client, request).await,
        SourceOperation::Upload => handle_upload_file(client, request).await,
        SourceOperation::DeleteFiles => handle_delete_file(client, request).await,
        SourceOperation::ListAgentsUsing => handle_list_agents_using(client, request).await,
    }
}

async fn handle_list_sources(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let (page_limit, _) = paginate(request.limit.map(|l| l as usize), None, 20, 100);
    let limit = page_limit as i32;

    let all_sources = client
        .sources()
        .list()
        .await
        .map_err(|e| sdk_err("list sources", e))?;

    let total = all_sources.len();

    // Take only up to limit
    let sources_to_return: Vec<_> = all_sources.into_iter().take(limit as usize).collect();
    let returned = sources_to_return.len();

    // Convert to optimized summaries
    let summaries: Vec<SourceSummary> = sources_to_return
        .into_iter()
        .map(|source| {
            let description = source.description.map(|d| truncate_string(&d, 100));

            SourceSummary {
                id: source.id.map(|id| id.to_string()).unwrap_or_default(),
                name: source.name,
                description,
                created_at: source.created_at.map(|t| t.to_string()),
                updated_at: source.updated_at.map(|t| t.to_string()),
                file_count: 0, // Note: Would need additional API call to get accurate count
                attached_agent_count: 0, // Note: Would need additional API call to get accurate count
            }
        })
        .collect();

    let pagination = PaginationMetadata {
        total,
        returned,
        limit,
        hint: if total > returned {
            Some(format!(
                "Showing {} of {} sources. Use limit parameter to see more (max 100).",
                returned, total
            ))
        } else {
            None
        },
    };

    Ok(ToolResponse::success(
        "list",
        &format!("Found {} sources, returning {}", total, returned),
    )
    .with_json_data(serde_json::to_value(&summaries)?)
    .with_count(total)
    .with_extra(serde_json::to_value(&pagination).unwrap()))
}

async fn handle_get_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    let source = client
        .sources()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get source", e))?;

    Ok(
        ToolResponse::success("get", "Source retrieved successfully")
            .with_json_data(serde_json::to_value(source)?),
    )
}

async fn handle_create_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let name = require_field(request.name, "name required")?;

    let create_request = if let Some(desc) = request.description {
        letta::types::source::CreateSourceRequest::builder()
            .name(name)
            .description(desc)
            .build()
    } else {
        letta::types::source::CreateSourceRequest::builder()
            .name(name)
            .build()
    };

    let source = client
        .sources()
        .create(create_request)
        .await
        .map_err(|e| sdk_err("create source", e))?;

    Ok(
        ToolResponse::success("create", "Source created successfully")
            .with_json_data(serde_json::to_value(source)?),
    )
}

async fn handle_update_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    let update_request = letta::types::source::UpdateSourceRequest {
        name: request.name,
        description: request.description,
        ..Default::default()
    };

    let source = client
        .sources()
        .update(&letta_id, update_request)
        .await
        .map_err(|e| sdk_err("update source", e))?;

    Ok(
        ToolResponse::success("update", "Source updated successfully")
            .with_json_data(serde_json::to_value(source)?),
    )
}

async fn handle_delete_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    client
        .sources()
        .delete(&letta_id)
        .await
        .map_err(|e| sdk_err("delete source", e))?;

    Ok(ToolResponse::success(
        "delete",
        "Source deleted successfully",
    ))
}

async fn handle_attach_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id required")?;
    let source_id = require_field(request.source_id, "source_id required")?;

    let letta_agent_id = require_id(Some(agent_id), "agent_id")?;
    let letta_source_id = require_id(Some(source_id.clone()), "source_id")?;

    let agent_state = client
        .sources()
        .agent_sources(letta_agent_id)
        .attach(&letta_source_id)
        .await
        .map_err(|e| sdk_err("attach source", e))?;

    Ok(
        ToolResponse::success("attach", "Source attached successfully")
            .with_json_data(serde_json::to_value(agent_state)?),
    )
}

async fn handle_detach_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id required")?;
    let source_id = require_field(request.source_id, "source_id required")?;

    let letta_agent_id = require_id(Some(agent_id), "agent_id")?;
    let letta_source_id = require_id(Some(source_id.clone()), "source_id")?;

    let agent_state = client
        .sources()
        .agent_sources(letta_agent_id)
        .detach(&letta_source_id)
        .await
        .map_err(|e| sdk_err("detach source", e))?;

    Ok(
        ToolResponse::success("detach", "Source detached successfully")
            .with_json_data(serde_json::to_value(agent_state)?),
    )
}

async fn handle_count_sources(
    client: &LettaClient,
    _request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let count = client
        .sources()
        .count()
        .await
        .map_err(|e| sdk_err("count sources", e))?;

    Ok(
        ToolResponse::success("count", format!("Total sources: {}", count))
            .with_json_data(serde_json::json!({"count": count}))
            .with_count(count as usize),
    )
}

async fn handle_list_attached(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id required")?;

    let letta_agent_id = require_id(Some(agent_id), "agent_id")?;

    let sources = client
        .sources()
        .agent_sources(letta_agent_id)
        .list()
        .await
        .map_err(|e| sdk_err("list attached sources", e))?;

    // Return lightweight summaries (id, name, file_count only)
    let summaries: Vec<serde_json::Value> = sources
        .into_iter()
        .map(|source| {
            serde_json::json!({
                "id": source.id.map(|id| id.to_string()).unwrap_or_default(),
                "name": source.name,
                "file_count": 0, // Note: Would need additional API call for accurate count
            })
        })
        .collect();

    Ok(ToolResponse::success(
        "list_attached",
        format!("Found {} attached sources", summaries.len()),
    )
    .with_json_data(serde_json::to_value(&summaries)?)
    .with_count(summaries.len()))
}

async fn handle_list_files(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    let (page_limit, _) = paginate(request.limit.map(|l| l as usize), None, 25, 100);
    let limit = page_limit as i32;

    // NEVER include content by default - override user request if they try
    let include_content = false;

    let params = Some(letta::types::source::ListFilesParams {
        limit: Some(limit),
        after: None,
        include_content: Some(include_content),
    });

    let files = client
        .sources()
        .list_files(&letta_id, params)
        .await
        .map_err(|e| sdk_err("list files", e))?;

    let total = files.len();

    // Convert to file summaries (never include content)
    let summaries: Vec<FileSummary> = files
        .into_iter()
        .map(|file| FileSummary {
            id: file.id.map(|id| id.to_string()).unwrap_or_default(),
            file_name: file.file_name.unwrap_or_else(|| "unknown".to_string()),
            content_type: file.file_type,
            size_bytes: file.file_size,
            created_at: file.created_at.map(|t| t.to_string()),
            processing_status: file.processing_status.map(|s| format!("{:?}", s)),
        })
        .collect();

    let pagination = PaginationMetadata {
        total,
        returned: summaries.len(),
        limit,
        hint: Some(format!(
            "File content is NEVER included in list operations. Use individual file retrieval to get content. Showing {} files (limit: {}).",
            summaries.len(),
            limit
        )),
    };

    Ok(ToolResponse::success(
        "list_files",
        format!("Found {} files (content not included)", total),
    )
    .with_json_data(serde_json::to_value(&summaries)?)
    .with_count(total)
    .with_extra(serde_json::to_value(&pagination).unwrap()))
}

async fn handle_upload_file(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let file_name = require_field(request.file_name, "file_name required")?;
    let file_data_b64 = require_field(request.file_data, "file_data required (base64 encoded)")?;

    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    // Decode base64 file data
    use base64::{Engine as _, engine::general_purpose};
    let file_bytes = general_purpose::STANDARD
        .decode(&file_data_b64)
        .map_err(|e| McpError::invalid_request(format!("Invalid base64 file_data: {}", e)))?;

    let file_size = file_bytes.len();

    let response = client
        .sources()
        .upload_file(
            &letta_id,
            file_name.clone(),
            bytes::Bytes::from(file_bytes),
            request.content_type.clone(),
        )
        .await
        .map_err(|e| sdk_err("upload file", e))?;

    // Return minimal summary - don't echo back file content
    // FileUploadResponse can be either Job or FileMetadata
    let (file_id, actual_size, actual_content_type) = match response {
        letta::types::source::FileUploadResponse::Job(job) => (
            job.id.to_string(),
            Some(file_size as i64),
            request.content_type,
        ),
        letta::types::source::FileUploadResponse::FileMetadata(metadata) => (
            metadata
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            metadata.file_size,
            metadata.file_type.or(request.content_type),
        ),
    };

    let upload_summary = FileUploadSummary {
        success: true,
        file_id,
        file_name: file_name.clone(),
        size_bytes: actual_size,
        content_type: actual_content_type,
    };

    Ok(ToolResponse::success(
        "upload",
        format!(
            "File '{}' uploaded successfully ({} bytes)",
            file_name,
            actual_size.unwrap_or(file_size as i64)
        ),
    )
    .with_json_data(serde_json::to_value(&upload_summary)?))
}

async fn handle_delete_file(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let file_id = require_field(request.file_id, "file_id required")?;

    let letta_source_id = require_id(Some(source_id.clone()), "source_id")?;
    let letta_file_id = require_id(Some(file_id), "file_id")?;

    client
        .sources()
        .delete_file(&letta_source_id, &letta_file_id)
        .await
        .map_err(|e| sdk_err("delete file", e))?;

    Ok(ToolResponse::success(
        "delete_files",
        "File deleted successfully",
    ))
}

async fn handle_list_agents_using(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<ToolResponse, McpError> {
    let source_id = require_field(request.source_id, "source_id required")?;
    let letta_id = require_id(Some(source_id.clone()), "source_id")?;

    let list_params = letta::types::ListAgentsParams {
        limit: Some(50),
        ..Default::default()
    };
    let agents = client
        .agents()
        .list(Some(list_params))
        .await
        .map_err(|e| sdk_err("list agents", e))?;

    // Filter agents that have this source attached
    let mut agents_using = Vec::new();
    for agent in agents {
        // Check if this agent has the source attached
        let sources = client
            .sources()
            .agent_sources(agent.id.clone())
            .list()
            .await
            .map_err(|e| sdk_err("check agent sources", e))?;

        for source in sources {
            if let Some(sid) = &source.id {
                if sid == &letta_id {
                    agents_using.push(agent.clone());
                    break;
                }
            }
        }
    }

    // Return only IDs and names - not full agent objects!
    let agent_refs: Vec<AgentReference> = agents_using
        .into_iter()
        .map(|agent| AgentReference {
            id: agent.id.to_string(),
            name: agent.name,
        })
        .collect();

    let agent_count = agent_refs.len();

    Ok(ToolResponse::success(
        "list_agents_using",
        format!("Found {} agents using this source", agent_count),
    )
    .with_json_data(serde_json::json!({
        "source_id": source_id,
        "agent_count": agent_count,
        "agents": agent_refs,
    }))
    .with_count(agent_count))
}
