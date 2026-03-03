//! Integration test for file/folder response optimizations
//!
//! Tests that file/folder operations properly limit and paginate responses
//! Validates requirements from LMS-54
//!
//! Note: FileFolderResponse was replaced by the unified ToolResponse struct
//! (see response_utils.rs). These tests validate the public types that remain:
//! FileFolderRequest, FileMetadata, FolderMetadata.

use letta_server::tools::file_folder_ops::{FileFolderRequest, FileMetadata, FolderMetadata};

#[test]
fn test_file_metadata_excludes_content() {
    // Verify FileMetadata struct doesn't have content field
    let file_meta = FileMetadata {
        id: "file-123".to_string(),
        filename: "test.txt".to_string(),
        size: Some(1024),
        mime_type: Some("text/plain".to_string()),
        is_open: Some(false),
        opened_at: None,
    };

    // Serialize to JSON and verify no content field exists
    let json = serde_json::to_value(&file_meta).unwrap();
    assert!(json.get("content").is_none());
    assert!(json.get("data").is_none());
    assert_eq!(json.get("id").unwrap().as_str().unwrap(), "file-123");
}

#[test]
fn test_file_metadata_skips_none_fields() {
    // Verify serde skip_serializing_if works for optional fields
    let file_meta = FileMetadata {
        id: "file-456".to_string(),
        filename: "minimal.txt".to_string(),
        size: None,
        mime_type: None,
        is_open: None,
        opened_at: None,
    };

    let json = serde_json::to_value(&file_meta).unwrap();
    // Required fields always present
    assert!(json.get("id").is_some());
    assert!(json.get("filename").is_some());
    // Optional None fields should be omitted
    assert!(json.get("size").is_none());
    assert!(json.get("mime_type").is_none());
    assert!(json.get("is_open").is_none());
    assert!(json.get("opened_at").is_none());
}

#[test]
fn test_folder_metadata_structure() {
    let folder = FolderMetadata {
        id: "folder-123".to_string(),
        name: "Test Folder".to_string(),
        description: Some("A test folder".to_string()),
        file_count: Some(50),
        agent_count: Some(10),
    };

    let json = serde_json::to_value(&folder).unwrap();
    assert_eq!(json.get("id").unwrap().as_str().unwrap(), "folder-123");
    assert_eq!(json.get("name").unwrap().as_str().unwrap(), "Test Folder");
    assert_eq!(json.get("file_count").unwrap().as_i64().unwrap(), 50);
    assert_eq!(json.get("agent_count").unwrap().as_i64().unwrap(), 10);
}

#[test]
fn test_folder_metadata_skips_none_fields() {
    let folder = FolderMetadata {
        id: "folder-456".to_string(),
        name: "Minimal Folder".to_string(),
        description: None,
        file_count: None,
        agent_count: None,
    };

    let json = serde_json::to_value(&folder).unwrap();
    assert!(json.get("id").is_some());
    assert!(json.get("name").is_some());
    assert!(json.get("description").is_none());
    assert!(json.get("file_count").is_none());
    assert!(json.get("agent_count").is_none());
}

#[test]
fn test_folder_metadata_long_description() {
    // Test that folder descriptions can hold long strings
    // (truncation happens in handle_list_folders, not in the struct)
    let long_desc = "a".repeat(200);
    let folder = FolderMetadata {
        id: "folder-123".to_string(),
        name: "Test Folder".to_string(),
        description: Some(long_desc.clone()),
        file_count: Some(50),
        agent_count: Some(10),
    };

    assert!(folder.description.is_some());
    assert_eq!(folder.description.unwrap().len(), 200);
}

#[test]
fn test_file_folder_request_defaults() {
    // Test that request parsing handles missing optional fields
    let request = FileFolderRequest {
        operation: "list_files".to_string(),
        agent_id: Some("agent-123".to_string()),
        file_id: None,
        folder_id: None,
        limit: None,
        offset: None,
        request_heartbeat: None,
        verbose: None,
    };

    assert_eq!(request.operation, "list_files");
    assert!(request.limit.is_none());
    assert!(request.offset.is_none());
}

#[test]
fn test_file_folder_request_with_pagination() {
    // Test request with pagination parameters
    let request = FileFolderRequest {
        operation: "list_folders".to_string(),
        agent_id: None,
        file_id: None,
        folder_id: None,
        limit: Some(25),
        offset: Some(50),
        request_heartbeat: None,
        verbose: None,
    };

    assert_eq!(request.limit, Some(25));
    assert_eq!(request.offset, Some(50));
}

#[test]
fn test_file_folder_request_json_roundtrip() {
    // Test serialization/deserialization roundtrip
    let request = FileFolderRequest {
        operation: "open_file".to_string(),
        agent_id: Some("agent-123".to_string()),
        file_id: Some("file-456".to_string()),
        folder_id: None,
        limit: None,
        offset: None,
        request_heartbeat: None,
        verbose: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    let parsed: FileFolderRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.operation, "open_file");
    assert_eq!(parsed.agent_id, Some("agent-123".to_string()));
    assert_eq!(parsed.file_id, Some("file-456".to_string()));
    // None fields should not appear in JSON
    assert!(!json.contains("folder_id"));
}
