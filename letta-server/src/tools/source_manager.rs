//! Source Manager Operations
//!
//! Consolidated tool for source management operations.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SourceManagerRequest {
    pub operation: SourceOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>, // Base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SourceManagerResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMetadata>,
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
) -> Result<SourceManagerResponse, McpError> {
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
) -> Result<SourceManagerResponse, McpError> {
    // Default limit: 20, max limit: 100
    const DEFAULT_LIMIT: i32 = 20;
    const MAX_LIMIT: i32 = 100;

    let limit = request.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

    let all_sources = client
        .sources()
        .list()
        .await
        .map_err(|e| McpError::internal(format!("Failed to list sources: {}", e)))?;

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
                "Showing {} of {} sources. Use limit parameter to see more (max {}).",
                returned, total, MAX_LIMIT
            ))
        } else {
            None
        },
    };

    Ok(SourceManagerResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Found {} sources, returning {}", total, returned),
        data: Some(serde_json::to_value(&summaries)?),
        count: Some(total),
        pagination: Some(pagination),
    })
}

async fn handle_get_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let source = client
        .sources()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "get".to_string(),
        message: "Source retrieved successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
        pagination: None,
    })
}

async fn handle_create_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let name = request
        .name
        .ok_or_else(|| McpError::invalid_request("name required".to_string()))?;

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
        .map_err(|e| McpError::internal(format!("Failed to create source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "create".to_string(),
        message: "Source created successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
        pagination: None,
    })
}

async fn handle_update_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let update_request = letta::types::source::UpdateSourceRequest {
        name: request.name,
        description: request.description,
        ..Default::default()
    };

    let source = client
        .sources()
        .update(&letta_id, update_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to update source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "update".to_string(),
        message: "Source updated successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
        pagination: None,
    })
}

async fn handle_delete_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    client
        .sources()
        .delete(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "delete".to_string(),
        message: "Source deleted successfully".to_string(),
        data: None,
        count: None,
        pagination: None,
    })
}

async fn handle_attach_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let agent_state = client
        .sources()
        .agent_sources(letta_agent_id)
        .attach(&letta_source_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to attach source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "attach".to_string(),
        message: "Source attached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
        pagination: None,
    })
}

async fn handle_detach_source(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let agent_state = client
        .sources()
        .agent_sources(letta_agent_id)
        .detach(&letta_source_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to detach source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "detach".to_string(),
        message: "Source detached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
        pagination: None,
    })
}

async fn handle_count_sources(
    client: &LettaClient,
    _request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let count = client
        .sources()
        .count()
        .await
        .map_err(|e| McpError::internal(format!("Failed to count sources: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "count".to_string(),
        message: format!("Total sources: {}", count),
        data: Some(serde_json::json!({"count": count})),
        count: Some(count as usize),
        pagination: None,
    })
}

async fn handle_list_attached(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let sources = client
        .sources()
        .agent_sources(letta_agent_id)
        .list()
        .await
        .map_err(|e| McpError::internal(format!("Failed to list attached sources: {}", e)))?;

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

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_attached".to_string(),
        message: format!("Found {} attached sources", summaries.len()),
        data: Some(serde_json::to_value(&summaries)?),
        count: Some(summaries.len()),
        pagination: None,
    })
}

async fn handle_list_files(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    // Default limit: 25, max limit: 100
    const DEFAULT_LIMIT: i32 = 25;
    const MAX_LIMIT: i32 = 100;

    let limit = request.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

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
        .map_err(|e| McpError::internal(format!("Failed to list files: {}", e)))?;

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
        hint: Some(format!("File content is NEVER included in list operations. Use individual file retrieval to get content. Showing {} files (limit: {}).", summaries.len(), limit)),
    };

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_files".to_string(),
        message: format!("Found {} files (content not included)", total),
        data: Some(serde_json::to_value(&summaries)?),
        count: Some(total),
        pagination: Some(pagination),
    })
}

async fn handle_upload_file(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let file_name = request
        .file_name
        .ok_or_else(|| McpError::invalid_request("file_name required".to_string()))?;
    let file_data_b64 = request.file_data.ok_or_else(|| {
        McpError::invalid_request("file_data required (base64 encoded)".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    // Decode base64 file data
    use base64::{engine::general_purpose, Engine as _};
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
        .map_err(|e| McpError::internal(format!("Failed to upload file: {}", e)))?;

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

    Ok(SourceManagerResponse {
        success: true,
        operation: "upload".to_string(),
        message: format!(
            "File '{}' uploaded successfully ({} bytes)",
            file_name,
            actual_size.unwrap_or(file_size as i64)
        ),
        data: Some(serde_json::to_value(&upload_summary)?),
        count: None,
        pagination: None,
    })
}

async fn handle_delete_file(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let file_id = request
        .file_id
        .ok_or_else(|| McpError::invalid_request("file_id required".to_string()))?;

    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;
    let letta_file_id = letta::types::LettaId::from_str(&file_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid file_id: {}", e)))?;

    client
        .sources()
        .delete_file(&letta_source_id, &letta_file_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete file: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "delete_files".to_string(),
        message: "File deleted successfully".to_string(),
        data: None,
        count: None,
        pagination: None,
    })
}

async fn handle_list_agents_using(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<SourceManagerResponse, McpError> {
    let source_id = request
        .source_id
        .ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    // Get all agents and filter by those using this source
    let agents = client
        .agents()
        .list(None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

    // Filter agents that have this source attached
    let mut agents_using = Vec::new();
    for agent in agents {
        // Check if this agent has the source attached
        let sources = client
            .sources()
            .agent_sources(agent.id.clone())
            .list()
            .await
            .map_err(|e| McpError::internal(format!("Failed to check agent sources: {}", e)))?;

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

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_agents_using".to_string(),
        message: format!("Found {} agents using this source", agent_count),
        data: Some(serde_json::json!({
            "source_id": source_id,
            "agent_count": agent_count,
            "agents": agent_refs,
        })),
        count: Some(agent_count),
        pagination: None,
    })
}
