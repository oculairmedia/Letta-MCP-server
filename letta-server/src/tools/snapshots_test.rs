//! Snapshot tests for MCP response shapes
//!
//! These tests capture the JSON structure of MCP handler responses.
//! If a response shape changes (fields added/removed/renamed), the test will fail
//! and show a diff, requiring explicit snapshot update via `cargo insta review`.

#[cfg(test)]
mod tests {
    use crate::tools::response_utils::ToolResponse;
    use serde_json::json;

    #[test]
    fn test_tool_manager_list_response() {
        let response = ToolResponse::success("list", "Tools retrieved successfully")
            .with_data(json!({
                "tools": [
                    {
                        "id": "tool-123",
                        "name": "send_message",
                        "description": "Send a message to the user",
                        "source_type": "python",
                        "tags": ["core"],
                        "created_at": "2024-01-01T00:00:00Z",
                        "args_count": 2,
                        "source_lines": 15
                    }
                ]
            }))
            .unwrap()
            .with_count(1);
        
        insta::assert_json_snapshot!("tool_manager_list", response);
    }

    #[test]
    fn test_tool_manager_get_response() {
        let response = ToolResponse::success("get", "Tool retrieved successfully")
            .with_data(json!({
                "id": "tool-123",
                "name": "send_message",
                "description": "Send a message to the user",
                "source_type": "python",
                "source_code": "def send_message(message: str):\n    return message",
                "tags": ["core"],
                "created_at": "2024-01-01T00:00:00Z"
            }))
            .unwrap();
        
        insta::assert_json_snapshot!("tool_manager_get", response);
    }

    #[test]
    fn test_tool_manager_attach_response() {
        let response = ToolResponse::success("attach", "Tool attached successfully")
            .with_data(json!({
                "tool_id": "tool-123",
                "agent_id": "agent-456"
            }))
            .unwrap();
        
        insta::assert_json_snapshot!("tool_manager_attach", response);
    }

    #[test]
    fn test_tool_manager_bulk_attach_response() {
        let response = ToolResponse::success("bulk_attach", "Bulk attach completed")
            .with_data(json!({
                "tool_id": "tool-123",
                "successful": ["agent-456", "agent-789"],
                "failed": [],
                "total": 2
            }))
            .unwrap();
        
        insta::assert_json_snapshot!("tool_manager_bulk_attach", response);
    }

    #[test]
    fn test_agent_list_response() {
        let response = ToolResponse::success("list", "Agents retrieved")
            .with_data(json!({
                "agents": [{
                    "id": "agent-123",
                    "name": "Assistant",
                    "created_at": "2024-01-01T00:00:00Z"
                }]
            }))
            .unwrap()
            .with_count(1);
        
        insta::assert_json_snapshot!("agent_list", response);
    }

    #[test]
    fn test_memory_get_core_response() {
        let response = ToolResponse::success("get_core_memory", "Memory retrieved")
            .with_data(json!({
                "human": "User is a developer",
                "persona": "I am a helpful assistant"
            }))
            .unwrap();
        
        insta::assert_json_snapshot!("memory_get_core", response);
    }

    #[test]
    fn test_memory_search_archival_response() {
        let response = ToolResponse::success("search_archival", "Search completed")
            .with_data(json!({
                "results": [{
                    "id": "passage-123",
                    "text": "Important context",
                    "created_at": "2024-01-01T00:00:00Z"
                }]
            }))
            .unwrap()
            .with_count(1);
        
        insta::assert_json_snapshot!("memory_search_archival", response);
    }

    #[test]
    fn test_file_list_response() {
        let response = ToolResponse::success("list", "Files retrieved")
            .with_data(json!({
                "files": [{
                    "id": "file-123",
                    "name": "doc.txt",
                    "size": 1024
                }]
            }))
            .unwrap()
            .with_count(1);
        
        insta::assert_json_snapshot!("file_list", response);
    }

    #[test]
    fn test_error_response() {
        let response = ToolResponse::error("get", "Tool not found");
        insta::assert_json_snapshot!("error_response", response);
    }

    #[test]
    fn test_validation_error() {
        let response = ToolResponse::error("attach", "Missing required field: agent_id");
        insta::assert_json_snapshot!("validation_error", response);
    }
}
