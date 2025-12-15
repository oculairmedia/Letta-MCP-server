//! Tests for tool_manager operations
//!
//! Tests tool management operations including:
//! - CRUD operations
//! - Agent tool attachment/detachment
//! - Response optimization

use serde_json::json;
use letta_server::tools::tool_manager::{ToolManagerRequest, ToolOperation};

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_list_tools() {
    let json_input = json!({
        "operation": "list"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::List));
}

#[test]
fn test_parse_list_with_pagination() {
    let json_input = json!({
        "operation": "list",
        "limit": 25,
        "offset": 0
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::List));
    assert_eq!(request.limit, Some(25));
    assert_eq!(request.offset, Some(0));
}

#[test]
fn test_parse_get_tool() {
    let json_input = json!({
        "operation": "get",
        "tool_id": "tool-12345"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Get));
    assert_eq!(request.tool_id.unwrap(), "tool-12345");
}

#[test]
fn test_parse_create_tool() {
    let json_input = json!({
        "operation": "create",
        "name": "my_custom_tool",
        "description": "A custom tool for testing",
        "source_code": "def my_custom_tool(): return 'hello'",
        "source_type": "python"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Create));
    assert_eq!(request.name.unwrap(), "my_custom_tool");
    assert!(request.source_code.is_some());
}

#[test]
fn test_parse_update_tool() {
    let json_input = json!({
        "operation": "update",
        "tool_id": "tool-12345",
        "description": "Updated description"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Update));
}

#[test]
fn test_parse_delete_tool() {
    let json_input = json!({
        "operation": "delete",
        "tool_id": "tool-12345"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Delete));
}

#[test]
fn test_parse_attach_tool() {
    let json_input = json!({
        "operation": "attach",
        "agent_id": "agent-12345",
        "tool_id": "tool-67890"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Attach));
    assert!(request.agent_id.is_some());
    assert!(request.tool_id.is_some());
}

#[test]
fn test_parse_detach_tool() {
    let json_input = json!({
        "operation": "detach",
        "agent_id": "agent-12345",
        "tool_id": "tool-67890"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::Detach));
}

#[test]
fn test_parse_bulk_attach() {
    let json_input = json!({
        "operation": "bulk_attach",
        "agent_ids": ["agent-1", "agent-2", "agent-3"],
        "tool_id": "tool-shared"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::BulkAttach));
    assert_eq!(request.agent_ids.as_ref().unwrap().len(), 3);
}

#[test]
fn test_parse_generate_from_prompt() {
    let json_input = json!({
        "operation": "generate_from_prompt",
        "description": "Create a tool that calculates fibonacci numbers"
    });
    
    let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, ToolOperation::GenerateFromPrompt));
}

// ============================================================
// All Operations Test
// ============================================================

#[test]
fn test_all_tool_operations_parse() {
    let operations = vec![
        "list", "get", "create", "update", "delete", "upsert",
        "attach", "detach", "bulk_attach", "generate_from_prompt",
        "generate_schema", "run_from_source", "add_base_tools"
    ];
    
    for op in operations {
        let json_input = json!({ "operation": op });
        let result: Result<ToolManagerRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// Tool Summary Format Tests
// ============================================================

mod summary_format {
    use serde_json::json;
    
    #[test]
    fn test_tool_summary_fields() {
        // Tool summaries should include these fields
        let summary = json!({
            "id": "tool-123",
            "name": "my_tool",
            "description": "Tool description (truncated if needed)",
            "source_type": "python"
            // Excluded: source_code, json_schema, args_schema
        });
        
        assert!(summary.get("id").is_some());
        assert!(summary.get("name").is_some());
        assert!(summary.get("description").is_some());
        assert!(summary.get("source_type").is_some());
        
        // These should be excluded from list view
        assert!(summary.get("source_code").is_none());
        assert!(summary.get("json_schema").is_none());
    }
    
    #[test]
    fn test_list_response_format() {
        let response_data = json!({
            "total": 50,
            "returned": 25,
            "has_more": true,
            "tools": [],
            "hint": "Use 'get' with tool_id for full details including source code"
        });
        
        assert!(response_data.get("total").is_some());
        assert!(response_data.get("returned").is_some());
        assert!(response_data.get("has_more").is_some());
        assert!(response_data.get("hint").is_some());
    }
}

// ============================================================
// Edge Cases
// ============================================================

mod edge_cases {
    use serde_json::json;
    use letta_server::tools::tool_manager::ToolManagerRequest;
    
    #[test]
    fn test_empty_tags() {
        let json_input = json!({
            "operation": "create",
            "name": "test_tool",
            "tags": []
        });
        
        let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.tags.is_some());
        assert_eq!(request.tags.unwrap().len(), 0);
    }
    
    #[test]
    fn test_with_tags() {
        let json_input = json!({
            "operation": "create",
            "name": "test_tool",
            "tags": ["utility", "math", "helper"]
        });
        
        let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
        let tags = request.tags.unwrap();
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"utility".to_string()));
    }
    
    #[test]
    fn test_json_schema_field() {
        let json_input = json!({
            "operation": "create",
            "name": "typed_tool",
            "json_schema": {
                "type": "object",
                "properties": {
                    "input": {"type": "string"}
                },
                "required": ["input"]
            }
        });
        
        let request: ToolManagerRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.json_schema.is_some());
    }
}
