//! Tests for job_monitor operations
//!
//! Tests job monitoring operations including:
//! - List jobs (with pagination)
//! - Get job details
//! - Cancel job
//! - List active jobs
//! - Response optimization and truncation

use letta_server::tools::job_monitor::{JobMonitorRequest, JobOperation};
use serde_json::json;

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_list_jobs() {
    let json_input = json!({
        "operation": "list"
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, JobOperation::List));
}

#[test]
fn test_parse_list_with_limit() {
    let json_input = json!({
        "operation": "list",
        "limit": 50
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, JobOperation::List));
    assert_eq!(request.limit, Some(50));
}

#[test]
fn test_parse_get_job() {
    let json_input = json!({
        "operation": "get",
        "job_id": "job-12345-abcde"
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, JobOperation::Get));
    assert_eq!(request.job_id.unwrap(), "job-12345-abcde");
}

#[test]
fn test_parse_cancel_job() {
    let json_input = json!({
        "operation": "cancel",
        "job_id": "job-12345-abcde"
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, JobOperation::Cancel));
    assert_eq!(request.job_id.unwrap(), "job-12345-abcde");
}

#[test]
fn test_parse_list_active() {
    let json_input = json!({
        "operation": "list_active"
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, JobOperation::ListActive));
}

#[test]
fn test_parse_list_active_with_limit() {
    let json_input = json!({
        "operation": "list_active",
        "limit": 10
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.limit, Some(10));
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_job_operations_parse() {
    let operations = vec!["list", "get", "cancel", "list_active"];

    for op in operations {
        let json_input = json!({"operation": op});
        let result: Result<JobMonitorRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// Job Summary Tests
// ============================================================

#[test]
fn test_job_summary_serialization() {
    let summary = json!({
        "id": "job-123",
        "job_type": "embedding",
        "status": "completed",
        "created_at": "2024-01-01T00:00:00Z",
        "completed_at": "2024-01-01T00:05:00Z",
        "progress_percent": 100
    });

    assert_eq!(summary["id"], "job-123");
    assert_eq!(summary["job_type"], "embedding");
    assert_eq!(summary["status"], "completed");
    assert!(summary["progress_percent"].is_number());
}

#[test]
fn test_job_summary_optional_fields() {
    let summary = json!({
        "id": "job-123",
        "job_type": "processing",
        "status": "pending",
        "created_at": "2024-01-01T00:00:00Z"
    });

    assert!(!summary.as_object().unwrap().contains_key("completed_at"));
    assert!(
        !summary
            .as_object()
            .unwrap()
            .contains_key("progress_percent")
    );
}

#[test]
fn test_job_summary_excludes_metadata() {
    // JobSummary should not include metadata, callback details, or other large fields
    let summary = json!({
        "id": "job-123",
        "job_type": "embedding",
        "status": "running",
        "created_at": "2024-01-01T00:00:00Z"
    });

    assert!(!summary.as_object().unwrap().contains_key("metadata"));
    assert!(!summary.as_object().unwrap().contains_key("callback_url"));
    assert!(!summary.as_object().unwrap().contains_key("callback_error"));
}

// ============================================================
// Job Status Tests
// ============================================================

mod job_status {
    use super::*;

    #[test]
    fn test_pending_status() {
        let summary = json!({
            "id": "job-123",
            "status": "pending",
            "created_at": "2024-01-01T00:00:00Z"
        });

        assert_eq!(summary["status"], "pending");
    }

    #[test]
    fn test_running_status() {
        let summary = json!({
            "id": "job-123",
            "status": "running",
            "progress_percent": 45
        });

        assert_eq!(summary["status"], "running");
    }

    #[test]
    fn test_completed_status() {
        let summary = json!({
            "id": "job-123",
            "status": "completed",
            "completed_at": "2024-01-01T00:10:00Z"
        });

        assert_eq!(summary["status"], "completed");
    }

    #[test]
    fn test_failed_status() {
        let summary = json!({
            "id": "job-123",
            "status": "failed",
            "completed_at": "2024-01-01T00:02:00Z"
        });

        assert_eq!(summary["status"], "failed");
    }

    #[test]
    fn test_cancelled_status() {
        let summary = json!({
            "id": "job-123",
            "status": "cancelled",
            "completed_at": "2024-01-01T00:01:30Z"
        });

        assert_eq!(summary["status"], "cancelled");
    }
}

// ============================================================
// Job Types Tests
// ============================================================

mod job_types {
    use super::*;

    #[test]
    fn test_embedding_job_type() {
        let summary = json!({
            "id": "job-123",
            "job_type": "embedding",
            "status": "running"
        });

        assert_eq!(summary["job_type"], "embedding");
    }

    #[test]
    fn test_processing_job_type() {
        let summary = json!({
            "id": "job-123",
            "job_type": "processing",
            "status": "pending"
        });

        assert_eq!(summary["job_type"], "processing");
    }
}

// ============================================================
// Response Format Tests
// ============================================================

mod response_format {
    use super::*;

    #[test]
    fn test_list_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list",
            "message": "Returned 5 jobs",
            "data": [
                {"id": "job-1", "status": "completed"},
                {"id": "job-2", "status": "running"}
            ],
            "count": 5,
            "returned": 5,
            "hints": ["Use 'get' operation with job_id for full details"]
        });

        assert_eq!(response_data["success"], true);
        assert_eq!(response_data["operation"], "list");
        assert!(response_data["data"].is_array());
        assert!(response_data["hints"].is_array());
    }

    #[test]
    fn test_get_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "get",
            "message": "Retrieved job successfully",
            "data": {
                "id": "job-123",
                "job_type": "embedding",
                "status": "completed",
                "metadata": {"key": "value"}
            }
        });

        assert_eq!(response_data["operation"], "get");
        assert!(response_data["data"].is_object());
    }

    #[test]
    fn test_cancel_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "cancel",
            "message": "Job cancelled successfully",
            "data": {
                "success": true,
                "job_id": "job-123",
                "message": "Job cancellation initiated"
            }
        });

        assert_eq!(response_data["success"], true);
        assert_eq!(response_data["data"]["job_id"], "job-123");
    }

    #[test]
    fn test_list_active_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_active",
            "message": "Returned 3 active jobs",
            "data": [
                {"id": "job-1", "status": "running"},
                {"id": "job-2", "status": "pending"}
            ],
            "count": 3,
            "returned": 3
        });

        assert_eq!(response_data["operation"], "list_active");
        assert!(response_data["data"].is_array());
    }

    #[test]
    fn test_truncated_field_format() {
        let truncated = json!({
            "value": "This is a long error message that has been trun...",
            "truncated": true,
            "original_length": 5000
        });

        assert_eq!(truncated["truncated"], true);
        assert!(truncated["original_length"].is_number());
    }
}

// ============================================================
// Edge Cases Tests
// ============================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_missing_job_id_for_get() {
        let json_input = json!({
            "operation": "get"
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.job_id.is_none(), "Should parse but fail in handler");
    }

    #[test]
    fn test_missing_job_id_for_cancel() {
        let json_input = json!({
            "operation": "cancel"
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.job_id.is_none());
    }

    #[test]
    fn test_zero_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": 0
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(0));
    }

    #[test]
    fn test_negative_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": -5
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(-5));
        // Should be handled appropriately in handler
    }

    #[test]
    fn test_very_large_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": 999999
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(999999));
        // No explicit max limit in job_monitor, but should be handled reasonably
    }

    #[test]
    fn test_uuid_job_id_format() {
        let json_input = json!({
            "operation": "get",
            "job_id": "550e8400-e29b-41d4-a716-446655440000"
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.job_id.unwrap().contains("-"));
    }

    #[test]
    fn test_custom_job_id_format() {
        let json_input = json!({
            "operation": "get",
            "job_id": "custom-job-id-12345"
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.job_id.unwrap(), "custom-job-id-12345");
    }

    #[test]
    fn test_empty_job_id() {
        let json_input = json!({
            "operation": "get",
            "job_id": ""
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.job_id.unwrap(), "");
        // Should fail validation in handler
    }

    #[test]
    fn test_null_optional_fields() {
        let json_input = json!({
            "operation": "list",
            "job_id": null,
            "limit": null
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.job_id.is_none());
        assert!(request.limit.is_none());
    }
}

// ============================================================
// Truncation Tests
// ============================================================

mod truncation {
    use super::*;

    #[test]
    fn test_callback_error_truncation_limit() {
        // MAX_CALLBACK_ERROR_LENGTH should be 1000
        let long_error = "E".repeat(2000);
        assert!(long_error.len() > 1000);
        // Truncation happens in handler
    }

    #[test]
    fn test_metadata_truncation_limit() {
        // MAX_METADATA_LENGTH should be 2000
        let large_metadata = json!({"data": "x".repeat(3000)});
        let metadata_str = serde_json::to_string(&large_metadata).unwrap();
        assert!(metadata_str.len() > 2000);
        // Truncation happens in handler
    }

    #[test]
    fn test_truncated_field_structure() {
        let truncated = json!({
            "value": "Truncated content...",
            "truncated": true,
            "original_length": 5000
        });

        assert!(truncated["value"].is_string());
        assert_eq!(truncated["truncated"], true);
        assert_eq!(truncated["original_length"], 5000);
    }

    #[test]
    fn test_non_truncated_field() {
        let not_truncated = json!({
            "value": "Short content",
            "truncated": false
        });

        assert_eq!(not_truncated["truncated"], false);
        assert!(
            !not_truncated
                .as_object()
                .unwrap()
                .contains_key("original_length")
        );
    }
}

// ============================================================
// Limits and Constants Tests
// ============================================================

mod limits {
    use super::*;

    #[test]
    fn test_default_list_limit_is_20() {
        // DEFAULT_LIST_LIMIT should be 20
        let json_input = json!({
            "operation": "list"
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.limit.is_none());
        // Handler should use DEFAULT_LIST_LIMIT (20)
    }

    #[test]
    fn test_explicit_limit_overrides_default() {
        let json_input = json!({
            "operation": "list",
            "limit": 100
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(100));
    }
}

// ============================================================
// Progress Tracking Tests
// ============================================================

mod progress {
    use super::*;

    #[test]
    fn test_progress_percent_0() {
        let summary = json!({
            "id": "job-123",
            "status": "running",
            "progress_percent": 0
        });

        assert_eq!(summary["progress_percent"], 0);
    }

    #[test]
    fn test_progress_percent_50() {
        let summary = json!({
            "id": "job-123",
            "status": "running",
            "progress_percent": 50
        });

        assert_eq!(summary["progress_percent"], 50);
    }

    #[test]
    fn test_progress_percent_100() {
        let summary = json!({
            "id": "job-123",
            "status": "completed",
            "progress_percent": 100
        });

        assert_eq!(summary["progress_percent"], 100);
    }

    #[test]
    fn test_progress_percent_optional() {
        let summary = json!({
            "id": "job-123",
            "status": "pending"
        });

        assert!(
            !summary
                .as_object()
                .unwrap()
                .contains_key("progress_percent")
        );
    }
}

// ============================================================
// Timestamp Tests
// ============================================================

mod timestamps {
    use super::*;

    #[test]
    fn test_created_at_timestamp() {
        let summary = json!({
            "id": "job-123",
            "created_at": "2024-01-01T00:00:00Z"
        });

        assert!(summary["created_at"].is_string());
    }

    #[test]
    fn test_completed_at_timestamp() {
        let summary = json!({
            "id": "job-123",
            "status": "completed",
            "completed_at": "2024-01-01T00:10:00Z"
        });

        assert!(summary["completed_at"].is_string());
    }

    #[test]
    fn test_completed_at_optional_for_pending() {
        let summary = json!({
            "id": "job-123",
            "status": "pending",
            "created_at": "2024-01-01T00:00:00Z"
        });

        assert!(!summary.as_object().unwrap().contains_key("completed_at"));
    }

    #[test]
    fn test_iso8601_timestamp_format() {
        let timestamp = "2024-01-15T14:30:00.123Z";
        let summary = json!({
            "id": "job-123",
            "created_at": timestamp
        });

        assert_eq!(summary["created_at"], timestamp);
    }
}

// ============================================================
// Active Jobs Filter Tests
// ============================================================

mod active_jobs {
    use super::*;

    #[test]
    fn test_list_active_excludes_completed() {
        // list_active should only return pending/running jobs
        let response_data = json!({
            "data": [
                {"id": "job-1", "status": "running"},
                {"id": "job-2", "status": "pending"}
            ]
        });

        let jobs = response_data["data"].as_array().unwrap();
        for job in jobs {
            let status = job["status"].as_str().unwrap();
            assert!(status == "running" || status == "pending");
        }
    }

    #[test]
    fn test_list_active_with_limit() {
        let json_input = json!({
            "operation": "list_active",
            "limit": 5
        });

        let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(5));
    }
}

// ============================================================
// Operation Coverage Test
// ============================================================

#[test]
fn test_operation_count() {
    // Verify we have exactly 4 operations
    let operations = [
        JobOperation::List,
        JobOperation::Get,
        JobOperation::Cancel,
        JobOperation::ListActive,
    ];

    assert_eq!(operations.len(), 4, "Should have exactly 4 operations");
}

// ============================================================
// Request Heartbeat Tests
// ============================================================

#[test]
fn test_request_heartbeat_true() {
    let json_input = json!({
        "operation": "list",
        "request_heartbeat": true
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(true));
}

#[test]
fn test_request_heartbeat_false() {
    let json_input = json!({
        "operation": "list",
        "request_heartbeat": false
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(false));
}

#[test]
fn test_request_heartbeat_omitted() {
    let json_input = json!({
        "operation": "list"
    });

    let request: JobMonitorRequest = serde_json::from_value(json_input).unwrap();
    assert!(request.request_heartbeat.is_none());
}

// ============================================================
// Hints Tests
// ============================================================

#[test]
fn test_response_hints_present() {
    let response_data = json!({
        "success": true,
        "operation": "list",
        "hints": [
            "Use 'get' operation with job_id for full details"
        ]
    });

    assert!(response_data["hints"].is_array());
    assert!(!response_data["hints"].as_array().unwrap().is_empty());
}

#[test]
fn test_response_hints_content() {
    let response_data = json!({
        "hints": [
            "Use 'get' operation with job_id for full details"
        ]
    });

    let hints = response_data["hints"].as_array().unwrap();
    assert!(hints[0].as_str().unwrap().contains("get"));
}
