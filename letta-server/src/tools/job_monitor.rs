//! Job Monitor Operations
//!
//! Consolidated tool for job monitoring operations with response size optimizations.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobMonitorRequest {
    pub operation: JobOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct JobMonitorResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,
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
) -> Result<JobMonitorResponse, McpError> {
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
) -> Result<JobMonitorResponse, McpError> {
    let limit = request.limit.unwrap_or(DEFAULT_LIST_LIMIT);

    let jobs = client
        .jobs()
        .list(None, Some(limit), None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list jobs: {}", e)))?;

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

    Ok(JobMonitorResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Returned {} jobs", returned),
        data: Some(serde_json::to_value(&summaries)?),
        count: Some(returned),
        total: None, // API doesn't provide total count
        returned: Some(returned),
        hints: Some(vec![
            "Use 'get' operation with job_id for full details".to_string()
        ]),
    })
}

async fn handle_get_job(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<JobMonitorResponse, McpError> {
    let job_id = request
        .job_id
        .ok_or_else(|| McpError::invalid_request("job_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&job_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid job_id: {}", e)))?;

    let job = client
        .jobs()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get job: {}", e)))?;

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
    if details
        .callback_error
        .as_ref()
        .map_or(false, |e| e.truncated)
    {
        hints.push("Error details truncated; use direct API for full error".to_string());
    }

    Ok(JobMonitorResponse {
        success: true,
        operation: "get".to_string(),
        message: "Job retrieved successfully".to_string(),
        data: Some(serde_json::to_value(details)?),
        count: None,
        total: None,
        returned: None,
        hints: if hints.is_empty() { None } else { Some(hints) },
    })
}

async fn handle_cancel_job(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<JobMonitorResponse, McpError> {
    let job_id = request
        .job_id
        .ok_or_else(|| McpError::invalid_request("job_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&job_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid job_id: {}", e)))?;

    // Get current status before canceling
    let job = client
        .jobs()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get job status: {}", e)))?;

    let previous_status = job
        .status
        .as_ref()
        .map(|s| format!("{:?}", s).to_lowercase());

    client
        .jobs()
        .delete(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to cancel job: {}", e)))?;

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

    Ok(JobMonitorResponse {
        success: true,
        operation: "cancel".to_string(),
        message: cancel_response.message.clone(),
        data: Some(serde_json::to_value(cancel_response)?),
        count: None,
        total: None,
        returned: None,
        hints: None,
    })
}

async fn handle_list_active_jobs(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<JobMonitorResponse, McpError> {
    let limit = request.limit.unwrap_or(DEFAULT_LIST_LIMIT);

    let jobs = client
        .jobs()
        .list_active(None, Some(limit))
        .await
        .map_err(|e| McpError::internal(format!("Failed to list active jobs: {}", e)))?;

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

    Ok(JobMonitorResponse {
        success: true,
        operation: "list_active".to_string(),
        message: format!("Found {} active jobs", returned),
        data: Some(serde_json::to_value(&summaries)?),
        count: Some(returned),
        total: None,
        returned: Some(returned),
        hints: Some(vec![
            "Active jobs are those with status 'pending' or 'running'".to_string(),
            "Use 'get' operation with job_id for full details".to_string(),
        ]),
    })
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
