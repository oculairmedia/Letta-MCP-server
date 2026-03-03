//! File and Folder Management Hub
//!
//! Provides unified interface for file session management and folder operations.
//! Implements 8 operations:
//! - File Sessions: list_files, open_file, close_file, close_all_files
//! - Folders: list_folders, attach_folder, detach_folder, list_agents_in_folder
//!
//! Response size optimizations (LMS-54):
//! - list_files: Default limit=25, NEVER includes file content
//! - list_folders: Default limit=20, truncates descriptions
//! - open_file: Returns minimal confirmation (content retrieval via separate API)
//! - All list operations include pagination metadata

use crate::tools::response_utils::{ToolResponse, paginate};
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use turbomcp::McpError;

const MAX_DESCRIPTION_LENGTH: usize = 100;

/// Truncate a string to a maximum length with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...[truncated]", &s[..max_len.saturating_sub(15)])
    }
}

/// File/folder operation request
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FileFolderRequest {
    /// Operation to perform
    pub operation: String,

    /// Agent ID (required for agent-specific operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// File ID (required for open_file, close_file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// Folder ID (required for attach/detach/list_agents_in_folder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,

    /// Maximum number of results to return (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    /// Number of results to skip (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,

    #[serde(default)]
    pub verbose: Option<bool>,
}

/// File metadata (optimized for list operations - no content)
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FileMetadata {
    pub id: String,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
    // Note: File content is NEVER included in list operations
}

/// Folder metadata
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FolderMetadata {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<i32>,
}

/// Agent reference
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AgentReference {
    pub id: String,
}

/// Handle letta_file_folder_ops tool requests
pub async fn handle_file_folder_ops(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let operation = request.operation.as_str();
    info!(operation = %operation, "Executing file/folder operation");

    match operation {
        "list_files" => handle_list_files(client, request).await,
        "open_file" => handle_open_file(client, request).await,
        "close_file" => handle_close_file(client, request).await,
        "close_all_files" => handle_close_all_files(client, request).await,
        "list_folders" => handle_list_folders(client, request).await,
        "attach_folder" => handle_attach_folder(client, request).await,
        "detach_folder" => handle_detach_folder(client, request).await,
        "list_agents_in_folder" => handle_list_agents_in_folder(client, request).await,
        _ => {
            error!(operation = %operation, "Unknown operation");
            Err(McpError::invalid_request(format!(
                "Unknown operation: {}",
                operation
            )))
        }
    }
}

/// List files for an agent
async fn handle_list_files(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let (limit, offset) = paginate(request.limit, request.offset, 25, 100);

    // Use SDK to list agent files
    let result = client
        .agents()
        .files(letta_agent_id)
        .list()
        .await
        .map_err(|e| sdk_err("list files", e))?;

    let total = result.files.len();

    // Apply pagination - NEVER include file content in list operations
    let files: Vec<FileMetadata> = result
        .files
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|f| FileMetadata {
            id: f.id.to_string(),
            filename: f.filename.clone(),
            size: Some(f.size),
            mime_type: Some(f.mime_type.clone()),
            is_open: Some(f.is_open),
            opened_at: f.opened_at.clone(),
        })
        .collect();

    let returned = files.len();
    let mut hints = vec!["File content is NEVER included in list operations".to_string()];

    if total > offset + returned {
        hints.push(format!(
            "More files available. Use offset={} to see next page",
            offset + returned
        ));
    }

    Ok(ToolResponse::success(
        "list_files",
        format!("Returned {} of {} files", returned, total),
    )
    .with_extra(serde_json::json!({
        "agent_id": agent_id,
        "total": total,
        "returned": returned,
        "offset": offset,
        "hints": hints,
        "files": files,
    })))
}

/// Open a file for an agent
async fn handle_open_file(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;
    let file_id = require_field(request.file_id, "file_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_file_id = require_id(Some(file_id.clone()), "file_id")?;

    // Use SDK to open file - returns array of evicted file IDs
    let evicted = client
        .agents()
        .files(letta_agent_id)
        .open(&letta_file_id)
        .await
        .map_err(|e| sdk_err("open file", e))?;

    // Note: The SDK open() method marks the file as open in the agent's context
    // It does NOT return file content. Content retrieval would require a separate API call.
    let hints = vec![
        "File marked as open in agent context. Content retrieval requires separate API call."
            .to_string(),
    ];

    Ok(
        ToolResponse::success("open_file", "File opened successfully").with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "file_id": file_id,
                "opened": true,
                "evicted_files": evicted,
                "hints": hints,
            }),
        ),
    )
}

/// Close a specific file
async fn handle_close_file(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;
    let file_id = require_field(request.file_id, "file_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_file_id = require_id(Some(file_id.clone()), "file_id")?;

    // Use SDK to close file
    client
        .agents()
        .files(letta_agent_id)
        .close(&letta_file_id)
        .await
        .map_err(|e| sdk_err("close file", e))?;

    // Minimal response as per LMS-54 requirements
    Ok(
        ToolResponse::success("close_file", "File closed successfully").with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "file_id": file_id,
                "closed": true,
            }),
        ),
    )
}

/// Close all files for an agent
async fn handle_close_all_files(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;

    // Use SDK to close all files - returns array of closed file IDs
    let closed = client
        .agents()
        .files(letta_agent_id)
        .close_all()
        .await
        .map_err(|e| sdk_err("close all files", e))?;

    let count = closed.len();

    // Minimal response - just file IDs, not full metadata (LMS-54)
    Ok(
        ToolResponse::success("close_all_files", format!("Closed {} files", count)).with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "closed_count": count,
                "closed_files": closed,
            }),
        ),
    )
}

/// List all folders
async fn handle_list_folders(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let (limit, offset) = paginate(request.limit, request.offset, 20, 50);

    // Use SDK to list folders with server-side limit
    let result = client
        .folders()
        .list(Some(letta::types::folder::ListFoldersParams {
            limit: Some(limit as u32 + offset as u32),
            ..Default::default()
        }))
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list folders - SDK error details");
            sdk_err("list folders", e)
        })?;

    let total = result.len();

    // Apply pagination and truncate descriptions (LMS-54)
    let folders: Vec<FolderMetadata> = result
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|f| FolderMetadata {
            id: f.id.to_string(),
            name: f.name.clone(),
            description: f
                .description
                .as_ref()
                .map(|d| truncate_string(d, MAX_DESCRIPTION_LENGTH)),
            file_count: None,  // Not included in SDK response
            agent_count: None, // Not included in SDK response
        })
        .collect();

    let returned = folders.len();
    let mut hints = Vec::new();

    if total > offset + returned {
        hints.push(format!(
            "More folders available. Use offset={} to see next page",
            offset + returned
        ));
    }

    Ok(ToolResponse::success(
        "list_folders",
        format!("Returned {} of {} folders", returned, total),
    )
    .with_extra(serde_json::json!({
        "total": total,
        "returned": returned,
        "offset": offset,
        "hints": if hints.is_empty() { None } else { Some(hints) },
        "folders": folders,
    })))
}

/// Attach folder to agent
async fn handle_attach_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;
    let folder_id = require_field(request.folder_id, "folder_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_folder_id = require_id(Some(folder_id.clone()), "folder_id")?;

    // Use SDK to attach folder - returns AgentState
    let _agent_state = client
        .folders()
        .agent(letta_agent_id)
        .attach(&letta_folder_id)
        .await
        .map_err(|e| sdk_err("attach folder", e))?;

    // Minimal response - don't include full agent state (LMS-54)
    Ok(
        ToolResponse::success("attach_folder", "Folder attached to agent successfully").with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "folder_id": folder_id,
                "attached": true,
            }),
        ),
    )
}

/// Detach folder from agent
async fn handle_detach_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required")?;
    let folder_id = require_field(request.folder_id, "folder_id is required")?;

    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_folder_id = require_id(Some(folder_id.clone()), "folder_id")?;

    // Use SDK to detach folder - returns AgentState
    let _agent_state = client
        .folders()
        .agent(letta_agent_id)
        .detach(&letta_folder_id)
        .await
        .map_err(|e| sdk_err("detach folder", e))?;

    // Minimal response - don't include full agent state (LMS-54)
    Ok(
        ToolResponse::success("detach_folder", "Folder detached from agent successfully")
            .with_extra(serde_json::json!({
                "agent_id": agent_id,
                "folder_id": folder_id,
                "detached": true,
            })),
    )
}

/// List agents in a specific folder
async fn handle_list_agents_in_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<ToolResponse, McpError> {
    let folder_id = require_field(request.folder_id, "folder_id is required")?;

    let letta_folder_id = require_id(Some(folder_id.clone()), "folder_id")?;

    // Use SDK to list agents in folder - returns Vec<String>
    let agent_ids = client
        .folders()
        .list_agents(&letta_folder_id)
        .await
        .map_err(|e| sdk_err("list agents in folder", e))?;

    // Return IDs only - already optimized (LMS-54)
    let agents: Vec<AgentReference> = agent_ids
        .iter()
        .map(|id| AgentReference { id: id.clone() })
        .collect();

    let count = agent_ids.len();

    Ok(ToolResponse::success(
        "list_agents_in_folder",
        format!("Found {} agents in folder", count),
    )
    .with_extra(serde_json::json!({
        "folder_id": folder_id,
        "agent_ids": agent_ids,
        "agents": agents,
    })))
}
