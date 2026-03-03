//! Job Monitor Operations
//!
//! Consolidated tool for job monitoring operations with response size optimizations.

use letta::LettaClient;
use crate::tools::response_utils::ToolResponse;
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobOperation {
    List,
    Get,
    Cancel,
    ListActive,
}

/// Job monitor request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobMonitorRequest {
    /// The operation to perform (list, get, cancel, list_active)
    pub operation: JobOperation,
    /// Job ID (required for get, cancel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Maximum number of results to return (for list, list_active)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    /// When false (default), returns minimal confirmation; when true, returns full state
    #[serde(default)]
    pub verbose: Option<bool>,
}


/// Simplified job summary for list operations
#[derive(Debug, Serialize)]
struct JobSummary {
    id: Option<String>,
    job_type: Option<String>,
    status: Option<String>,
    created_at: Option<String>,
    completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress_percent: Option<u8>,
}

/// Truncated job details for get operation
#[derive(Debug, Serialize)]
struct TruncatedJobDetails {
    id: Option<String>,
    job_type: Option<String>,
    status: Option<String>,
    created_at: Option<String>,
    completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_error: Option<TruncatedField>,
}

#[derive(Debug, Serialize)]
struct TruncatedField {
    value: String,
    truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_length: Option<usize>,
}

/// Cancel operation minimal response
#[derive(Debug, Serialize)]
struct CancelResponse {
    success: bool,
    job_id: String,
    message: String,
}

const DEFAULT_LIST_LIMIT: i32 = 20;
const MAX_CALLBACK_ERROR_LENGTH: usize = 1000;
const MAX_METADATA_LENGTH: usize = 2000;

pub async fn handle_job_monitor(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<ToolResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing job operation");

    match request.operation {
        JobOperation::List => handle_list_jobs(client, request).await,
        JobOperation::Get => handle_get_job(client, request).await,
        JobOperation::Cancel => handle_cancel_job(client, request).await,
        JobOperation::ListActive => handle_list_active_jobs(client, request).await,
    }
}

async fn handle_list_jobs(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<ToolResponse, McpError> {
    let limit = request.limit.unwrap_or(DEFAULT_LIST_LIMIT);

    let jobs = client
        .jobs()
        .list(None, Some(limit), None)
        .await
        .map_err(|e| sdk_err("list jobs", e))?;

    // Convert to summaries (exclude metadata, callback details, etc.)
    let summaries: Vec<JobSummary> = jobs
        .iter()
        .map(|job| JobSummary {
            id: job.id.as_ref().map(|id| id.to_string()),
            job_type: job
                .job_type
                .as_ref()
                .map(|jt| format!("{:?}", jt).to_lowercase()),
            status: job
                .status
                .as_ref()
                .map(|s| format!("{:?}", s).to_lowercase()),
            created_at: job.created_at.as_ref().map(|ts| ts.to_string()),
            completed_at: job.completed_at.as_ref().map(|ts| ts.to_string()),
            progress_percent: None, // Could be calculated from metadata if available
        })
        .collect();

    let returned = summaries.len();

    Ok(ToolResponse::success("list", format!("Returned {} jobs", returned))
        .with_json_data(serde_json::to_value(&summaries)?)
        .with_count(returned)
        .with_extra(serde_json::json!({
            "returned": returned,
            "hints": vec!["Use 'get' operation with job_id for full details"]
        })))
}

async fn handle_get_job(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<ToolResponse, McpError> {
    let job_id = require_field(request.job_id, "job_id required")?;
    let letta_id = require_id(Some(job_id), "job_id")?;

    let job = client
        .jobs()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get job", e))?;

    // Truncate large fields
    let truncated_metadata = truncate_json_field(&job.metadata, MAX_METADATA_LENGTH);
    let truncated_callback_error = job
        .callback_error
        .as_ref()
        .map(|err| truncate_string_field(err, MAX_CALLBACK_ERROR_LENGTH));

    let details = TruncatedJobDetails {
        id: job.id.as_ref().map(|id| id.to_string()),
        job_type: job
            .job_type
            .as_ref()
            .map(|jt| format!("{:?}", jt).to_lowercase()),
        status: job
            .status
            .as_ref()
            .map(|s| format!("{:?}", s).to_lowercase()),
        created_at: job.created_at.as_ref().map(|ts| ts.to_string()),
        completed_at: job.completed_at.as_ref().map(|ts| ts.to_string()),
        metadata: truncated_metadata,
        callback_url: job.callback_url.clone(),
        callback_error: truncated_callback_error,
    };

    let mut hints = Vec::new();
    if let Some(ref meta) = details.metadata {
        if let Some(obj) = meta.as_object() {
            if obj.contains_key("truncated") {
                hints.push("Some fields were truncated due to size limits".to_string());
            }
        }
    }
    if details.callback_error.as_ref().is_some_and(|e| e.truncated) {
        hints.push("Error details truncated; use direct API for full error".to_string());
    }

    let mut resp = ToolResponse::success("get", "Job retrieved successfully")
        .with_json_data(serde_json::to_value(details)?);
    if !hints.is_empty() {
        resp = resp.with_extra(serde_json::json!({ "hints": hints }));
    }
    Ok(resp)
}

async fn handle_cancel_job(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<ToolResponse, McpError> {
    let job_id = require_field(request.job_id, "job_id required")?;
    let letta_id = require_id(Some(job_id.clone()), "job_id")?;

    // Get current status before canceling
    let job = client
        .jobs()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get job status", e))?;

    let previous_status = job
        .status
        .as_ref()
        .map(|s| format!("{:?}", s).to_lowercase());

    client
        .jobs()
        .delete(&letta_id)
        .await
        .map_err(|e| sdk_err("cancel job", e))?;

    // Minimal response
    let cancel_response = CancelResponse {
        success: true,
        job_id: job_id.clone(),
        message: format!(
            "Job {} cancelled (was: {})",
            job_id,
            previous_status.unwrap_or_else(|| "unknown".to_string())
        ),
    };

    Ok(ToolResponse::success("cancel", cancel_response.message.clone())
        .with_json_data(serde_json::to_value(cancel_response)?))
}

async fn handle_list_active_jobs(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<ToolResponse, McpError> {
    let limit = request.limit.unwrap_or(DEFAULT_LIST_LIMIT);

    let jobs = client
        .jobs()
        .list_active(None, Some(limit))
        .await
        .map_err(|e| sdk_err("list active jobs", e))?;

    // Convert to summaries
    let summaries: Vec<JobSummary> = jobs
        .iter()
        .map(|job| JobSummary {
            id: job.id.as_ref().map(|id| id.to_string()),
            job_type: job
                .job_type
                .as_ref()
                .map(|jt| format!("{:?}", jt).to_lowercase()),
            status: job
                .status
                .as_ref()
                .map(|s| format!("{:?}", s).to_lowercase()),
            created_at: job.created_at.as_ref().map(|ts| ts.to_string()),
            completed_at: job.completed_at.as_ref().map(|ts| ts.to_string()),
            progress_percent: None,
        })
        .collect();

    let returned = summaries.len();

    Ok(ToolResponse::success("list_active", format!("Found {} active jobs", returned))
        .with_json_data(serde_json::to_value(&summaries)?)
        .with_count(returned)
        .with_extra(serde_json::json!({
            "returned": returned,
            "hints": [
                "Active jobs are those with status 'pending' or 'running'",
                "Use 'get' operation with job_id for full details"
            ]
        })))
}

/// Truncate a JSON field if it exceeds max_length when serialized
fn truncate_json_field(field: &Option<Value>, max_length: usize) -> Option<Value> {
    field.as_ref().and_then(|val| {
        let serialized = serde_json::to_string(val).ok()?;
        if serialized.len() <= max_length {
            Some(val.clone())
        } else {
            // Create truncated version
            let truncated = &serialized[..max_length];
            let result = serde_json::json!({
                "truncated": true,
                "original_length": serialized.len(),
                "preview": truncated,
            });
            Some(result)
        }
    })
}

/// Truncate a string field
fn truncate_string_field(field: &str, max_length: usize) -> TruncatedField {
    if field.len() <= max_length {
        TruncatedField {
            value: field.to_string(),
            truncated: false,
            original_length: None,
        }
    } else {
        TruncatedField {
            value: field[..max_length].to_string(),
            truncated: true,
            original_length: Some(field.len()),
        }
    }
}
