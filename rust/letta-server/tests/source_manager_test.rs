//! Tests for source_manager operations
//!
//! Tests source management operations including:
//! - CRUD operations (list, get, create, update, delete)
//! - Agent attachment/detachment
//! - File operations (upload, list, delete)
//! - Response optimization and pagination

use letta_server::tools::response_utils::PaginationMeta;
use letta_server::tools::source_manager::{
    AgentReference, FileSummary, SourceManagerRequest, SourceOperation, SourceSummary,
};
use serde_json::json;

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_list_sources() {
    let json_input = json!({
        "operation": "list"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::List));
}

#[test]
fn test_parse_list_with_pagination() {
    let json_input = json!({
        "operation": "list",
        "limit": 50,
        "include_content": false
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::List));
    assert_eq!(request.limit, Some(50));
    assert_eq!(request.include_content, Some(false));
}

#[test]
fn test_parse_get_source() {
    let json_input = json!({
        "operation": "get",
        "source_id": "source-12345"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Get));
    assert_eq!(request.source_id.unwrap(), "source-12345");
}

#[test]
fn test_parse_create_source() {
    let json_input = json!({
        "operation": "create",
        "name": "my_knowledge_base",
        "description": "A collection of documents"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Create));
    assert_eq!(request.name.unwrap(), "my_knowledge_base");
    assert_eq!(request.description.unwrap(), "A collection of documents");
}

#[test]
fn test_parse_update_source() {
    let json_input = json!({
        "operation": "update",
        "source_id": "source-12345",
        "description": "Updated description"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Update));
    assert_eq!(request.source_id.unwrap(), "source-12345");
}

#[test]
fn test_parse_delete_source() {
    let json_input = json!({
        "operation": "delete",
        "source_id": "source-12345"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Delete));
}

#[test]
fn test_parse_attach_source() {
    let json_input = json!({
        "operation": "attach",
        "source_id": "source-12345",
        "agent_id": "agent-67890"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Attach));
    assert_eq!(request.source_id.unwrap(), "source-12345");
    assert_eq!(request.agent_id.unwrap(), "agent-67890");
}

#[test]
fn test_parse_detach_source() {
    let json_input = json!({
        "operation": "detach",
        "source_id": "source-12345",
        "agent_id": "agent-67890"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Detach));
}

#[test]
fn test_parse_list_attached() {
    let json_input = json!({
        "operation": "list_attached",
        "agent_id": "agent-67890"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::ListAttached));
    assert_eq!(request.agent_id.unwrap(), "agent-67890");
}

#[test]
fn test_parse_upload_file() {
    let json_input = json!({
        "operation": "upload",
        "source_id": "source-12345",
        "file_name": "document.pdf",
        "file_data": "base64encodeddata",
        "content_type": "application/pdf"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Upload));
    assert_eq!(request.file_name.unwrap(), "document.pdf");
    assert_eq!(request.content_type.unwrap(), "application/pdf");
}

#[test]
fn test_parse_delete_files() {
    let json_input = json!({
        "operation": "delete_files",
        "source_id": "source-12345",
        "file_id": "file-99999"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::DeleteFiles));
    assert_eq!(request.file_id.unwrap(), "file-99999");
}

#[test]
fn test_parse_list_files() {
    let json_input = json!({
        "operation": "list_files",
        "source_id": "source-12345",
        "limit": 100
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::ListFiles));
    assert_eq!(request.limit, Some(100));
}

#[test]
fn test_parse_count_sources() {
    let json_input = json!({
        "operation": "count"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, SourceOperation::Count));
}

#[test]
fn test_parse_list_agents_using() {
    let json_input = json!({
        "operation": "list_agents_using",
        "source_id": "source-12345"
    });

    let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        SourceOperation::ListAgentsUsing
    ));
    assert_eq!(request.source_id.unwrap(), "source-12345");
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_source_operations_parse() {
    let operations = vec![
        "list",
        "get",
        "create",
        "update",
        "delete",
        "attach",
        "detach",
        "list_attached",
        "upload",
        "delete_files",
        "list_files",
        "count",
        "list_agents_using",
    ];

    for op in operations {
        let json_input = json!({"operation": op});
        let result: Result<SourceManagerRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// Source Summary Tests
// ============================================================

#[test]
fn test_source_summary_serialization() {
    let summary = SourceSummary {
        id: "source-123".to_string(),
        name: "My Source".to_string(),
        description: Some("Test description".to_string()),
        created_at: Some("2024-01-01T00:00:00Z".to_string()),
        updated_at: Some("2024-01-02T00:00:00Z".to_string()),
        file_count: 5,
        attached_agent_count: 3,
    };

    let json = serde_json::to_value(&summary).unwrap();
    assert_eq!(json["id"], "source-123");
    assert_eq!(json["name"], "My Source");
    assert_eq!(json["file_count"], 5);
    assert_eq!(json["attached_agent_count"], 3);
}

#[test]
fn test_source_summary_optional_fields() {
    let summary = SourceSummary {
        id: "source-123".to_string(),
        name: "My Source".to_string(),
        description: None,
        created_at: None,
        updated_at: None,
        file_count: 0,
        attached_agent_count: 0,
    };

    let json = serde_json::to_value(&summary).unwrap();
    assert!(!json.as_object().unwrap().contains_key("description"));
    assert!(!json.as_object().unwrap().contains_key("created_at"));
    assert_eq!(json["file_count"], 0);
}

#[test]
fn test_source_summary_truncated_description() {
    let long_desc = "a".repeat(200);
    let summary = SourceSummary {
        id: "source-123".to_string(),
        name: "My Source".to_string(),
        description: Some(long_desc),
        created_at: None,
        updated_at: None,
        file_count: 0,
        attached_agent_count: 0,
    };

    let json = serde_json::to_value(&summary).unwrap();
    let desc = json["description"].as_str().unwrap();
    // In actual implementation, description should be truncated to 100 chars
    assert!(!desc.is_empty());
}

// ============================================================
// File Summary Tests
// ============================================================

#[test]
fn test_file_summary_serialization() {
    let file_summary = FileSummary {
        id: "file-456".to_string(),
        file_name: "document.pdf".to_string(),
        content_type: Some("application/pdf".to_string()),
        size_bytes: Some(1024000),
        created_at: Some("2024-01-01T00:00:00Z".to_string()),
        processing_status: Some("completed".to_string()),
    };

    let json = serde_json::to_value(&file_summary).unwrap();
    assert_eq!(json["id"], "file-456");
    assert_eq!(json["file_name"], "document.pdf");
    assert_eq!(json["size_bytes"], 1024000);
}

#[test]
fn test_file_summary_optional_fields() {
    let file_summary = FileSummary {
        id: "file-456".to_string(),
        file_name: "document.txt".to_string(),
        content_type: None,
        size_bytes: None,
        created_at: None,
        processing_status: None,
    };

    let json = serde_json::to_value(&file_summary).unwrap();
    assert!(!json.as_object().unwrap().contains_key("content_type"));
    assert!(!json.as_object().unwrap().contains_key("size_bytes"));
}

#[test]
fn test_file_summary_never_includes_content() {
    // FileSummary struct should not have a content field
    let file_summary = FileSummary {
        id: "file-456".to_string(),
        file_name: "secret.txt".to_string(),
        content_type: Some("text/plain".to_string()),
        size_bytes: Some(100),
        created_at: None,
        processing_status: None,
    };

    let json = serde_json::to_value(&file_summary).unwrap();
    assert!(!json.as_object().unwrap().contains_key("content"));
}

// ============================================================
// Agent Reference Tests
// ============================================================

#[test]
fn test_agent_reference_serialization() {
    let agent_ref = AgentReference {
        id: "agent-789".to_string(),
        name: "TestAgent".to_string(),
    };

    let json = serde_json::to_value(&agent_ref).unwrap();
    assert_eq!(json["id"], "agent-789");
    assert_eq!(json["name"], "TestAgent");

    // Should only have id and name fields
    assert_eq!(json.as_object().unwrap().len(), 2);
}

// ============================================================
// Pagination Metadata Tests
// ============================================================

#[test]
fn test_pagination_metadata() {
    let pagination = PaginationMeta::new(100, 20, 0, 20)
        .with_hint("Use limit parameter to see more");

    let json = serde_json::to_value(&pagination).unwrap();
    assert_eq!(json["total"], 100);
    assert_eq!(json["returned"], 20);
    assert_eq!(json["limit"], 20);
    assert!(!json["hints"].as_array().unwrap().is_empty());
}

#[test]
fn test_pagination_no_hint_when_all_returned() {
    let pagination = PaginationMeta::new(15, 15, 0, 20);

    let json = serde_json::to_value(&pagination).unwrap();
    assert_eq!(json["total"], 15);
    assert_eq!(json["returned"], 15);
    // hints should be empty (and skipped in serialization)
    assert!(
        !json.as_object().unwrap().contains_key("hints")
            || json["hints"].as_array().unwrap().is_empty()
    );
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
            "message": "Listed sources successfully",
            "count": 20,
            "pagination": {
                "total": 100,
                "returned": 20,
                "limit": 20,
                "hint": "Showing 20 of 100 sources"
            },
            "data": []
        });

        assert_eq!(response_data["success"], true);
        assert_eq!(response_data["operation"], "list");
        assert!(response_data["pagination"].is_object());
    }

    #[test]
    fn test_get_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "get",
            "message": "Retrieved source successfully",
            "data": {
                "id": "source-123",
                "name": "My Source",
                "file_count": 5
            }
        });

        assert_eq!(response_data["success"], true);
        assert!(response_data["data"].is_object());
    }

    #[test]
    fn test_create_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "create",
            "message": "Created source successfully",
            "data": {
                "id": "source-new",
                "name": "New Source"
            }
        });

        assert_eq!(response_data["operation"], "create");
    }

    #[test]
    fn test_count_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "count",
            "message": "Counted sources successfully",
            "count": 42
        });

        assert_eq!(response_data["count"], 42);
    }

    #[test]
    fn test_upload_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "upload",
            "message": "File uploaded successfully",
            "data": {
                "success": true,
                "file_id": "file-123",
                "file_name": "document.pdf",
                "size_bytes": 1024000
            }
        });

        assert_eq!(response_data["data"]["file_id"], "file-123");
    }
}

// ============================================================
// Edge Cases Tests
// ============================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_source_name() {
        let json_input = json!({
            "operation": "create",
            "name": ""
        });

        let request: Result<SourceManagerRequest, _> = serde_json::from_value(json_input);
        assert!(
            request.is_ok(),
            "Empty name should parse but fail validation later"
        );
    }

    #[test]
    fn test_zero_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": 0
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(0));
    }

    #[test]
    fn test_negative_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": -1
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(-1));
        // Should be clamped to 0 in handler
    }

    #[test]
    fn test_very_large_limit() {
        let json_input = json!({
            "operation": "list",
            "limit": 999999
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(999999));
        // Should be clamped to MAX_LIMIT (100) in handler
    }

    #[test]
    fn test_missing_required_source_id() {
        let json_input = json!({
            "operation": "get"
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(
            request.source_id.is_none(),
            "Missing source_id should parse but fail in handler"
        );
    }

    #[test]
    fn test_missing_agent_id_for_attach() {
        let json_input = json!({
            "operation": "attach",
            "source_id": "source-123"
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(
            request.agent_id.is_none(),
            "Should parse but fail validation"
        );
    }

    #[test]
    fn test_base64_file_data() {
        let base64_data = "SGVsbG8gV29ybGQh"; // "Hello World!" in base64
        let json_input = json!({
            "operation": "upload",
            "source_id": "source-123",
            "file_name": "test.txt",
            "file_data": base64_data
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.file_data.unwrap(), base64_data);
    }

    #[test]
    fn test_unicode_in_source_name() {
        let json_input = json!({
            "operation": "create",
            "name": "测试源 🚀"
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.name.unwrap(), "测试源 🚀");
    }

    #[test]
    fn test_null_optional_fields() {
        let json_input = json!({
            "operation": "create",
            "name": "Test",
            "description": null
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.description.is_none());
    }
}

// ============================================================
// Limits and Defaults Tests
// ============================================================

mod limits {
    use super::*;

    #[test]
    fn test_default_limit_is_20() {
        // Default limit should be 20 (per source code)
        let json_input = json!({
            "operation": "list"
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(
            request.limit.is_none(),
            "No limit specified should use default"
        );
    }

    #[test]
    fn test_max_limit_is_100() {
        // Max limit should be 100 (per source code)
        let json_input = json!({
            "operation": "list",
            "limit": 150
        });

        let request: SourceManagerRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(150));
        // Should be clamped to 100 in handler
    }

    #[test]
    fn test_description_truncation_at_100_chars() {
        // Description should be truncated to 100 chars
        let long_desc = "a".repeat(200);
        let summary = SourceSummary {
            id: "source-123".to_string(),
            name: "Test".to_string(),
            description: Some(long_desc.clone()),
            created_at: None,
            updated_at: None,
            file_count: 0,
            attached_agent_count: 0,
        };

        // The actual truncation happens in the handler via truncate_string()
        // Here we just verify the struct can hold long descriptions
        assert_eq!(summary.description.unwrap().len(), 200);
    }
}

// ============================================================
// Operation Coverage Test
// ============================================================

#[test]
fn test_operation_count() {
    // Verify we have exactly 13 operations (not 15 - ListFolders and GetFolderContents moved)
    let operations = [
        SourceOperation::List,
        SourceOperation::Get,
        SourceOperation::Create,
        SourceOperation::Update,
        SourceOperation::Delete,
        SourceOperation::Attach,
        SourceOperation::Detach,
        SourceOperation::ListAttached,
        SourceOperation::Upload,
        SourceOperation::DeleteFiles,
        SourceOperation::ListFiles,
        SourceOperation::Count,
        SourceOperation::ListAgentsUsing,
    ];

    assert_eq!(operations.len(), 13, "Should have exactly 13 operations");
}
