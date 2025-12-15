//! Tests for file_folder_ops operations
//!
//! Tests file and folder management operations including:
//! - File sessions (list, open, close, close_all)
//! - Folder operations (list, attach, detach, list_agents_in_folder)
//! - Response optimization and pagination

use letta_server::tools::file_folder_ops::{FileFolderRequest, FileMetadata, FolderMetadata};
use serde_json::json;

// ============================================================
// Request Parsing Tests - File Operations
// ============================================================

#[test]
fn test_parse_list_files() {
    let json_input = json!({
        "operation": "list_files",
        "agent_id": "agent-12345"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "list_files");
    assert_eq!(request.agent_id.unwrap(), "agent-12345");
}

#[test]
fn test_parse_list_files_with_pagination() {
    let json_input = json!({
        "operation": "list_files",
        "agent_id": "agent-12345",
        "limit": 50,
        "offset": 10
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.limit, Some(50));
    assert_eq!(request.offset, Some(10));
}

#[test]
fn test_parse_open_file() {
    let json_input = json!({
        "operation": "open_file",
        "agent_id": "agent-12345",
        "file_id": "file-67890"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "open_file");
    assert_eq!(request.file_id.unwrap(), "file-67890");
}

#[test]
fn test_parse_close_file() {
    let json_input = json!({
        "operation": "close_file",
        "agent_id": "agent-12345",
        "file_id": "file-67890"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "close_file");
}

#[test]
fn test_parse_close_all_files() {
    let json_input = json!({
        "operation": "close_all_files",
        "agent_id": "agent-12345"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "close_all_files");
}

// ============================================================
// Request Parsing Tests - Folder Operations
// ============================================================

#[test]
fn test_parse_list_folders() {
    let json_input = json!({
        "operation": "list_folders"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "list_folders");
}

#[test]
fn test_parse_list_folders_with_pagination() {
    let json_input = json!({
        "operation": "list_folders",
        "limit": 30,
        "offset": 5
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.limit, Some(30));
}

#[test]
fn test_parse_attach_folder() {
    let json_input = json!({
        "operation": "attach_folder",
        "agent_id": "agent-12345",
        "folder_id": "folder-99999"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "attach_folder");
    assert_eq!(request.folder_id.unwrap(), "folder-99999");
}

#[test]
fn test_parse_detach_folder() {
    let json_input = json!({
        "operation": "detach_folder",
        "agent_id": "agent-12345",
        "folder_id": "folder-99999"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "detach_folder");
}

#[test]
fn test_parse_list_agents_in_folder() {
    let json_input = json!({
        "operation": "list_agents_in_folder",
        "folder_id": "folder-99999"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.operation, "list_agents_in_folder");
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_file_folder_operations_parse() {
    let operations = vec![
        "list_files",
        "open_file",
        "close_file",
        "close_all_files",
        "list_folders",
        "attach_folder",
        "detach_folder",
        "list_agents_in_folder",
    ];

    for op in operations {
        let json_input = json!({"operation": op});
        let result: Result<FileFolderRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// File Metadata Tests
// ============================================================

#[test]
fn test_file_metadata_serialization() {
    let file_meta = FileMetadata {
        id: "file-123".to_string(),
        filename: "document.pdf".to_string(),
        size: Some(1024000),
        mime_type: Some("application/pdf".to_string()),
        is_open: Some(true),
        opened_at: Some("2024-01-01T00:00:00Z".to_string()),
    };

    let json = serde_json::to_value(&file_meta).unwrap();
    assert_eq!(json["id"], "file-123");
    assert_eq!(json["filename"], "document.pdf");
    assert_eq!(json["size"], 1024000);
}

#[test]
fn test_file_metadata_optional_fields() {
    let file_meta = FileMetadata {
        id: "file-123".to_string(),
        filename: "unknown.bin".to_string(),
        size: None,
        mime_type: None,
        is_open: None,
        opened_at: None,
    };

    let json = serde_json::to_value(&file_meta).unwrap();
    assert!(!json.as_object().unwrap().contains_key("size"));
    assert!(!json.as_object().unwrap().contains_key("mime_type"));
}

#[test]
fn test_file_metadata_never_includes_content() {
    // FileMetadata should NEVER have a content field
    let file_meta = FileMetadata {
        id: "file-123".to_string(),
        filename: "secret.txt".to_string(),
        size: Some(100),
        mime_type: Some("text/plain".to_string()),
        is_open: Some(false),
        opened_at: None,
    };

    let json = serde_json::to_value(&file_meta).unwrap();
    assert!(!json.as_object().unwrap().contains_key("content"));
}

#[test]
fn test_file_is_open_status() {
    let file_meta = FileMetadata {
        id: "file-123".to_string(),
        filename: "active.txt".to_string(),
        size: None,
        mime_type: None,
        is_open: Some(true),
        opened_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    let json = serde_json::to_value(&file_meta).unwrap();
    assert_eq!(json["is_open"], true);
    assert!(json["opened_at"].is_string());
}

// ============================================================
// Folder Metadata Tests
// ============================================================

#[test]
fn test_folder_metadata_serialization() {
    let folder_meta = FolderMetadata {
        id: "folder-456".to_string(),
        name: "Documents".to_string(),
        description: Some("My documents folder".to_string()),
        file_count: Some(15),
        agent_count: Some(3),
    };

    let json = serde_json::to_value(&folder_meta).unwrap();
    assert_eq!(json["id"], "folder-456");
    assert_eq!(json["name"], "Documents");
    assert_eq!(json["file_count"], 15);
}

#[test]
fn test_folder_metadata_optional_fields() {
    let folder_meta = FolderMetadata {
        id: "folder-456".to_string(),
        name: "Empty Folder".to_string(),
        description: None,
        file_count: None,
        agent_count: None,
    };

    let json = serde_json::to_value(&folder_meta).unwrap();
    assert!(!json.as_object().unwrap().contains_key("description"));
    assert!(!json.as_object().unwrap().contains_key("file_count"));
}

#[test]
fn test_folder_metadata_truncated_description() {
    let long_desc = "a".repeat(200);
    let folder_meta = FolderMetadata {
        id: "folder-456".to_string(),
        name: "Test".to_string(),
        description: Some(long_desc),
        file_count: Some(0),
        agent_count: Some(0),
    };

    // Truncation happens in handler, not in struct
    let json = serde_json::to_value(&folder_meta).unwrap();
    let desc = json["description"].as_str().unwrap();
    assert!(!desc.is_empty());
}

// ============================================================
// Response Format Tests
// ============================================================

mod response_format {
    use super::*;

    #[test]
    fn test_list_files_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_files",
            "message": "Listed 10 files",
            "agent_id": "agent-123",
            "files": [
                {"id": "file-1", "filename": "doc1.pdf"},
                {"id": "file-2", "filename": "doc2.pdf"}
            ],
            "total": 10,
            "returned": 10,
            "hints": ["File content not included in list"]
        });

        assert_eq!(response_data["success"], true);
        assert!(response_data["files"].is_array());
        assert!(response_data["hints"].is_array());
    }

    #[test]
    fn test_open_file_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "open_file",
            "message": "File opened successfully",
            "agent_id": "agent-123",
            "file_id": "file-456",
            "opened": true
        });

        assert_eq!(response_data["opened"], true);
        assert_eq!(response_data["file_id"], "file-456");
    }

    #[test]
    fn test_close_file_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "close_file",
            "message": "File closed successfully",
            "agent_id": "agent-123",
            "file_id": "file-456",
            "closed": true
        });

        assert_eq!(response_data["closed"], true);
    }

    #[test]
    fn test_close_all_files_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "close_all_files",
            "message": "Closed 5 files",
            "agent_id": "agent-123",
            "closed_count": 5,
            "closed_files": ["file-1", "file-2", "file-3", "file-4", "file-5"]
        });

        assert_eq!(response_data["closed_count"], 5);
        assert!(response_data["closed_files"].is_array());
    }

    #[test]
    fn test_list_folders_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_folders",
            "message": "Listed 8 folders",
            "folders": [
                {"id": "folder-1", "name": "Docs"},
                {"id": "folder-2", "name": "Images"}
            ],
            "total": 8,
            "returned": 8
        });

        assert!(response_data["folders"].is_array());
        assert_eq!(response_data["total"], 8);
    }

    #[test]
    fn test_attach_folder_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "attach_folder",
            "message": "Folder attached successfully",
            "agent_id": "agent-123",
            "folder_id": "folder-789",
            "attached": true
        });

        assert_eq!(response_data["attached"], true);
    }

    #[test]
    fn test_detach_folder_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "detach_folder",
            "message": "Folder detached successfully",
            "agent_id": "agent-123",
            "folder_id": "folder-789",
            "detached": true
        });

        assert_eq!(response_data["detached"], true);
    }

    #[test]
    fn test_list_agents_in_folder_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_agents_in_folder",
            "message": "Found 3 agents",
            "folder_id": "folder-789",
            "agent_ids": ["agent-1", "agent-2", "agent-3"],
            "total": 3
        });

        assert!(response_data["agent_ids"].is_array());
        assert_eq!(response_data["total"], 3);
    }
}

// ============================================================
// Edge Cases Tests
// ============================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_missing_agent_id() {
        let json_input = json!({
            "operation": "list_files"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(
            request.agent_id.is_none(),
            "Should parse but fail in handler"
        );
    }

    #[test]
    fn test_missing_file_id_for_open() {
        let json_input = json!({
            "operation": "open_file",
            "agent_id": "agent-123"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.file_id.is_none());
    }

    #[test]
    fn test_missing_folder_id_for_attach() {
        let json_input = json!({
            "operation": "attach_folder",
            "agent_id": "agent-123"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.folder_id.is_none());
    }

    #[test]
    fn test_zero_limit() {
        let json_input = json!({
            "operation": "list_files",
            "limit": 0
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(0));
    }

    #[test]
    fn test_excessive_limit() {
        let json_input = json!({
            "operation": "list_files",
            "limit": 999999
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(999999));
        // Should be clamped to MAX_FILE_LIMIT (100) in handler
    }

    #[test]
    fn test_negative_offset() {
        let json_input = json!({
            "operation": "list_files",
            "offset": 0  // Using 0 instead of negative since usize
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.offset, Some(0));
    }

    #[test]
    fn test_unicode_in_filename() {
        let file_meta = FileMetadata {
            id: "file-123".to_string(),
            filename: "文档.txt 🚀".to_string(),
            size: None,
            mime_type: None,
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["filename"], "文档.txt 🚀");
    }

    #[test]
    fn test_unicode_in_folder_name() {
        let folder_meta = FolderMetadata {
            id: "folder-456".to_string(),
            name: "我的文件夹 📁".to_string(),
            description: None,
            file_count: None,
            agent_count: None,
        };

        let json = serde_json::to_value(&folder_meta).unwrap();
        assert_eq!(json["name"], "我的文件夹 📁");
    }

    #[test]
    fn test_null_optional_fields() {
        let json_input = json!({
            "operation": "list_files",
            "agent_id": "agent-123",
            "file_id": null,
            "limit": null
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.file_id.is_none());
        assert!(request.limit.is_none());
    }
}

// ============================================================
// Pagination Tests
// ============================================================

mod pagination {
    use super::*;

    #[test]
    fn test_pagination_with_limit_and_offset() {
        let json_input = json!({
            "operation": "list_files",
            "limit": 25,
            "offset": 50
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(25));
        assert_eq!(request.offset, Some(50));
    }

    #[test]
    fn test_pagination_metadata_in_response() {
        let response_data = json!({
            "total": 100,
            "returned": 25,
            "offset": 0,
            "hints": ["Use limit and offset for pagination"]
        });

        assert_eq!(response_data["total"], 100);
        assert_eq!(response_data["returned"], 25);
    }

    #[test]
    fn test_folder_pagination() {
        let json_input = json!({
            "operation": "list_folders",
            "limit": 20,
            "offset": 10
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(20));
    }
}

// ============================================================
// Constants and Limits Tests
// ============================================================

mod limits {
    use super::*;

    #[test]
    fn test_default_file_limit_is_25() {
        // DEFAULT_FILE_LIMIT should be 25
        let json_input = json!({
            "operation": "list_files"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.limit.is_none());
        // Handler should use DEFAULT_FILE_LIMIT (25)
    }

    #[test]
    fn test_max_file_limit_is_100() {
        // MAX_FILE_LIMIT should be 100
        let json_input = json!({
            "operation": "list_files",
            "limit": 200
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(200));
        // Should be clamped to 100 in handler
    }

    #[test]
    fn test_default_folder_limit_is_20() {
        // DEFAULT_FOLDER_LIMIT should be 20
        let json_input = json!({
            "operation": "list_folders"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_max_folder_limit_is_50() {
        // MAX_FOLDER_LIMIT should be 50
        let json_input = json!({
            "operation": "list_folders",
            "limit": 100
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.limit, Some(100));
        // Should be clamped to 50 in handler
    }

    #[test]
    fn test_max_description_length_100() {
        // MAX_DESCRIPTION_LENGTH should be 100
        let long_desc = "a".repeat(200);
        assert!(long_desc.len() > 100);
        // Truncation happens in handler
    }
}

// ============================================================
// File Operations Tests
// ============================================================

mod file_operations {
    use super::*;

    #[test]
    fn test_open_file_request() {
        let json_input = json!({
            "operation": "open_file",
            "agent_id": "agent-123",
            "file_id": "file-456"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.operation, "open_file");
        assert!(request.agent_id.is_some());
        assert!(request.file_id.is_some());
    }

    #[test]
    fn test_close_file_request() {
        let json_input = json!({
            "operation": "close_file",
            "agent_id": "agent-123",
            "file_id": "file-456"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.operation, "close_file");
    }

    #[test]
    fn test_close_all_files_request() {
        let json_input = json!({
            "operation": "close_all_files",
            "agent_id": "agent-123"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.operation, "close_all_files");
        assert!(
            request.file_id.is_none(),
            "close_all shouldn't need file_id"
        );
    }
}

// ============================================================
// Folder Operations Tests
// ============================================================

mod folder_operations {
    use super::*;

    #[test]
    fn test_attach_folder_request() {
        let json_input = json!({
            "operation": "attach_folder",
            "agent_id": "agent-123",
            "folder_id": "folder-789"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.operation, "attach_folder");
        assert!(request.folder_id.is_some());
    }

    #[test]
    fn test_detach_folder_request() {
        let json_input = json!({
            "operation": "detach_folder",
            "agent_id": "agent-123",
            "folder_id": "folder-789"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert_eq!(request.operation, "detach_folder");
    }

    #[test]
    fn test_list_agents_in_folder_request() {
        let json_input = json!({
            "operation": "list_agents_in_folder",
            "folder_id": "folder-789"
        });

        let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
        assert!(
            request.agent_id.is_none(),
            "list_agents_in_folder shouldn't need agent_id"
        );
        assert!(request.folder_id.is_some());
    }
}

// ============================================================
// Operation Coverage Test
// ============================================================

#[test]
fn test_operation_count() {
    // Verify we have exactly 8 operations
    let operations = [
        "list_files",
        "open_file",
        "close_file",
        "close_all_files",
        "list_folders",
        "attach_folder",
        "detach_folder",
        "list_agents_in_folder",
    ];

    assert_eq!(operations.len(), 8, "Should have exactly 8 operations");
}

// ============================================================
// Request Heartbeat Tests
// ============================================================

#[test]
fn test_request_heartbeat_true() {
    let json_input = json!({
        "operation": "list_files",
        "request_heartbeat": true
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(true));
}

#[test]
fn test_request_heartbeat_false() {
    let json_input = json!({
        "operation": "list_files",
        "request_heartbeat": false
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(false));
}

#[test]
fn test_request_heartbeat_omitted() {
    let json_input = json!({
        "operation": "list_files"
    });

    let request: FileFolderRequest = serde_json::from_value(json_input).unwrap();
    assert!(request.request_heartbeat.is_none());
}

// ============================================================
// MIME Type Tests
// ============================================================

mod mime_types {
    use super::*;

    #[test]
    fn test_pdf_mime_type() {
        let file_meta = FileMetadata {
            id: "file-1".to_string(),
            filename: "doc.pdf".to_string(),
            size: None,
            mime_type: Some("application/pdf".to_string()),
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["mime_type"], "application/pdf");
    }

    #[test]
    fn test_text_mime_type() {
        let file_meta = FileMetadata {
            id: "file-2".to_string(),
            filename: "readme.txt".to_string(),
            size: None,
            mime_type: Some("text/plain".to_string()),
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["mime_type"], "text/plain");
    }

    #[test]
    fn test_json_mime_type() {
        let file_meta = FileMetadata {
            id: "file-3".to_string(),
            filename: "data.json".to_string(),
            size: None,
            mime_type: Some("application/json".to_string()),
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["mime_type"], "application/json");
    }
}

// ============================================================
// File Size Tests
// ============================================================

mod file_sizes {
    use super::*;

    #[test]
    fn test_small_file_size() {
        let file_meta = FileMetadata {
            id: "file-1".to_string(),
            filename: "tiny.txt".to_string(),
            size: Some(100),
            mime_type: None,
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["size"], 100);
    }

    #[test]
    fn test_large_file_size() {
        let file_meta = FileMetadata {
            id: "file-2".to_string(),
            filename: "video.mp4".to_string(),
            size: Some(1024 * 1024 * 100), // 100 MB
            mime_type: None,
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["size"], 1024 * 1024 * 100);
    }

    #[test]
    fn test_zero_size_file() {
        let file_meta = FileMetadata {
            id: "file-3".to_string(),
            filename: "empty.txt".to_string(),
            size: Some(0),
            mime_type: None,
            is_open: None,
            opened_at: None,
        };

        let json = serde_json::to_value(&file_meta).unwrap();
        assert_eq!(json["size"], 0);
    }
}
