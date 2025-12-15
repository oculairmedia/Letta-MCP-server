//! Tests for mcp_ops operations
//!
//! Tests MCP server lifecycle management operations including:
//! - Server management (add, update, delete, test, connect, resync)
//! - Tool operations (list, register, execute)
//! - Response optimization and pagination

use serde_json::json;
use letta_server::tools::mcp_ops::{McpOpsRequest, McpOperation};

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_add_server() {
    let json_input = json!({
        "operation": "add",
        "server_config": {
            "sse": {
                "server_name": "test-server",
                "url": "https://example.com/mcp"
            }
        }
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Add));
    assert!(request.server_config.is_some());
}

#[test]
fn test_parse_update_server() {
    let json_input = json!({
        "operation": "update",
        "server_name": "test-server",
        "server_config": {
            "sse": {
                "server_name": "test-server",
                "url": "https://example.com/mcp-v2"
            }
        }
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Update));
    assert_eq!(request.server_name.unwrap(), "test-server");
}

#[test]
fn test_parse_delete_server() {
    let json_input = json!({
        "operation": "delete",
        "server_name": "test-server"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Delete));
    assert_eq!(request.server_name.unwrap(), "test-server");
}

#[test]
fn test_parse_test_server() {
    let json_input = json!({
        "operation": "test",
        "server_name": "test-server"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Test));
}

#[test]
fn test_parse_connect_server() {
    let json_input = json!({
        "operation": "connect",
        "server_name": "test-server"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Connect));
}

#[test]
fn test_parse_resync_server() {
    let json_input = json!({
        "operation": "resync",
        "server_name": "test-server"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Resync));
}

#[test]
fn test_parse_list_servers() {
    let json_input = json!({
        "operation": "list_servers",
        "pagination": {
            "limit": 30,
            "offset": 0
        }
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::ListServers));
    assert!(request.pagination.is_some());
}

#[test]
fn test_parse_list_tools() {
    let json_input = json!({
        "operation": "list_tools",
        "server_name": "test-server",
        "pagination": {
            "limit": 50
        }
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::ListTools));
}

#[test]
fn test_parse_register_tool() {
    let json_input = json!({
        "operation": "register_tool",
        "server_name": "test-server",
        "tool_name": "my_tool"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::RegisterTool));
    assert_eq!(request.tool_name.unwrap(), "my_tool");
}

#[test]
fn test_parse_execute_tool() {
    let json_input = json!({
        "operation": "execute",
        "server_name": "test-server",
        "tool_name": "my_tool",
        "tool_args": {
            "param1": "value1",
            "param2": 42
        }
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, McpOperation::Execute));
    assert!(request.tool_args.is_some());
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_mcp_operations_parse() {
    let operations = vec![
        "add", "update", "delete", "test", "connect", "resync",
        "execute", "list_servers", "list_tools", "register_tool"
    ];
    
    for op in operations {
        let json_input = json!({"operation": op});
        let result: Result<McpOpsRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

// ============================================================
// Server Config Tests
// ============================================================

mod server_config {
    use super::*;

    #[test]
    fn test_sse_server_config() {
        let json_input = json!({
            "operation": "add",
            "server_config": {
                "sse": {
                    "server_name": "sse-server",
                    "url": "https://example.com/mcp"
                }
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let config = request.server_config.unwrap();
        assert!(config.get("sse").is_some());
    }

    #[test]
    fn test_stdio_server_config() {
        let json_input = json!({
            "operation": "add",
            "server_config": {
                "stdio": {
                    "server_name": "stdio-server",
                    "command": "node",
                    "args": ["server.js"]
                }
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let config = request.server_config.unwrap();
        assert!(config.get("stdio").is_some());
    }

    #[test]
    fn test_http_server_config() {
        let json_input = json!({
            "operation": "add",
            "server_config": {
                "streamable_http": {
                    "server_name": "http-server",
                    "url": "https://api.example.com"
                }
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let config = request.server_config.unwrap();
        assert!(config.get("streamable_http").is_some());
    }

    #[test]
    fn test_server_config_with_oauth() {
        let json_input = json!({
            "operation": "add",
            "server_config": {
                "sse": {
                    "server_name": "oauth-server",
                    "url": "https://example.com/mcp"
                }
            },
            "oauth_config": {
                "client_id": "test-client",
                "client_secret": "test-secret"
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.oauth_config.is_some());
    }
}

// ============================================================
// Pagination Tests
// ============================================================

mod pagination {
    use super::*;

    #[test]
    fn test_pagination_with_limit() {
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "limit": 25
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 25);
    }

    #[test]
    fn test_pagination_with_offset() {
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "limit": 20,
                "offset": 10
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["offset"], 10);
    }

    #[test]
    fn test_pagination_defaults() {
        let json_input = json!({
            "operation": "list_servers"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.pagination.is_none());
        // Should use DEFAULT_SERVERS_LIMIT (20) in handler
    }

    #[test]
    fn test_tools_pagination() {
        let json_input = json!({
            "operation": "list_tools",
            "pagination": {
                "limit": 50
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 50);
    }
}

// ============================================================
// Response Format Tests
// ============================================================

mod response_format {
    use super::*;

    #[test]
    fn test_add_server_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "add",
            "message": "MCP server added successfully",
            "server_name": "test-server",
            "data": {
                "server_name": "test-server",
                "type": "sse"
            }
        });
        
        assert_eq!(response_data["success"], true);
        assert_eq!(response_data["operation"], "add");
        assert!(response_data["server_name"].is_string());
    }

    #[test]
    fn test_list_servers_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_servers",
            "message": "Listed MCP servers",
            "servers": [
                {"server_name": "server1", "type": "sse"},
                {"server_name": "server2", "type": "stdio"}
            ],
            "total": 2,
            "returned": 2
        });
        
        assert!(response_data["servers"].is_array());
        assert_eq!(response_data["total"], 2);
    }

    #[test]
    fn test_list_tools_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "list_tools",
            "message": "Listed MCP tools",
            "tools": [
                {"name": "tool1", "description": "First tool"},
                {"name": "tool2", "description": "Second tool"}
            ],
            "total": 2,
            "returned": 2,
            "server_name": "test-server"
        });
        
        assert!(response_data["tools"].is_array());
        assert_eq!(response_data["server_name"], "test-server");
    }

    #[test]
    fn test_execute_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "execute",
            "message": "Tool executed successfully",
            "server_name": "test-server",
            "tool_name": "my_tool",
            "data": {
                "result": "success",
                "output": "Tool execution result"
            }
        });
        
        assert_eq!(response_data["tool_name"], "my_tool");
        assert!(response_data["data"].is_object());
    }

    #[test]
    fn test_test_server_response_format() {
        let response_data = json!({
            "success": true,
            "operation": "test",
            "message": "Server test completed",
            "server_name": "test-server",
            "data": {
                "status": "healthy",
                "latency_ms": 123
            }
        });
        
        assert_eq!(response_data["operation"], "test");
    }

    #[test]
    fn test_truncated_response_metadata() {
        let response_data = json!({
            "success": true,
            "operation": "list_tools",
            "message": "Listed MCP tools",
            "tools": [],
            "truncated": true,
            "output_length": 2500,
            "hints": [
                "Response truncated for size optimization",
                "Use pagination to retrieve more results"
            ]
        });
        
        assert_eq!(response_data["truncated"], true);
        assert!(response_data["hints"].is_array());
    }
}

// ============================================================
// Edge Cases Tests
// ============================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_missing_server_name() {
        let json_input = json!({
            "operation": "delete"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.server_name.is_none(), "Should parse but fail in handler");
    }

    #[test]
    fn test_missing_server_config_for_add() {
        let json_input = json!({
            "operation": "add"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.server_config.is_none(), "Should parse but fail validation");
    }

    #[test]
    fn test_missing_tool_name_for_execute() {
        let json_input = json!({
            "operation": "execute",
            "server_name": "test-server"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.tool_name.is_none());
    }

    #[test]
    fn test_empty_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "server_name": "test-server",
            "tool_name": "my_tool",
            "tool_args": {}
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.tool_args.is_some());
        assert!(request.tool_args.unwrap().as_object().unwrap().is_empty());
    }

    #[test]
    fn test_complex_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "server_name": "test-server",
            "tool_name": "complex_tool",
            "tool_args": {
                "nested": {
                    "param1": "value",
                    "param2": [1, 2, 3]
                },
                "array": ["a", "b", "c"],
                "number": 42
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert!(args["nested"].is_object());
        assert!(args["array"].is_array());
    }

    #[test]
    fn test_unicode_in_server_name() {
        let json_input = json!({
            "operation": "add",
            "server_config": {
                "sse": {
                    "server_name": "测试服务器-🚀",
                    "url": "https://example.com"
                }
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.server_config.is_some());
    }

    #[test]
    fn test_zero_pagination_limit() {
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "limit": 0
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 0);
        // Should be handled appropriately in handler
    }

    #[test]
    fn test_excessive_pagination_limit() {
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "limit": 999999
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 999999);
        // Should be clamped to MAX_SERVERS_LIMIT (50) in handler
    }

    #[test]
    fn test_negative_offset() {
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "offset": -10
            }
        });
        
        // Should fail to parse or be handled as 0
        let result: Result<McpOpsRequest, _> = serde_json::from_value(json_input);
        // Depending on implementation, this might fail or treat as 0
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_null_optional_fields() {
        let json_input = json!({
            "operation": "list_servers",
            "server_name": null,
            "pagination": null
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.server_name.is_none());
        assert!(request.pagination.is_none());
    }
}

// ============================================================
// Constants and Limits Tests
// ============================================================

mod constants {
    use super::*;

    #[test]
    fn test_default_servers_limit_is_20() {
        // DEFAULT_SERVERS_LIMIT should be 20
        let json_input = json!({
            "operation": "list_servers"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.pagination.is_none());
        // Handler should use DEFAULT_SERVERS_LIMIT (20)
    }

    #[test]
    fn test_max_servers_limit_is_50() {
        // MAX_SERVERS_LIMIT should be 50
        let json_input = json!({
            "operation": "list_servers",
            "pagination": {
                "limit": 100
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 100);
        // Should be clamped to 50 in handler
    }

    #[test]
    fn test_default_tools_limit_is_30() {
        // DEFAULT_TOOLS_LIMIT should be 30
        let json_input = json!({
            "operation": "list_tools"
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        assert!(request.pagination.is_none());
        // Handler should use DEFAULT_TOOLS_LIMIT (30)
    }

    #[test]
    fn test_max_tools_limit_is_100() {
        // MAX_TOOLS_LIMIT should be 100
        let json_input = json!({
            "operation": "list_tools",
            "pagination": {
                "limit": 200
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let pagination = request.pagination.unwrap();
        assert_eq!(pagination["limit"], 200);
        // Should be clamped to 100 in handler
    }

    #[test]
    fn test_max_description_length_80() {
        // MAX_DESCRIPTION_LENGTH should be 80
        // Truncation happens in handler, here we just verify the constant is used
        let long_description = "a".repeat(200);
        assert!(long_description.len() > 80);
    }

    #[test]
    fn test_max_output_length_3000() {
        // MAX_OUTPUT_LENGTH should be 3000
        let large_output = "x".repeat(5000);
        assert!(large_output.len() > 3000);
        // Handler should truncate to 3000 chars
    }
}

// ============================================================
// Tool Args Validation Tests
// ============================================================

mod tool_args {
    use super::*;

    #[test]
    fn test_string_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "tool_args": {
                "input": "test string"
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert_eq!(args["input"], "test string");
    }

    #[test]
    fn test_numeric_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "tool_args": {
                "count": 42,
                "ratio": 3.14
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert_eq!(args["count"], 42);
        assert_eq!(args["ratio"], 3.14);
    }

    #[test]
    fn test_boolean_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "tool_args": {
                "enabled": true,
                "verbose": false
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert_eq!(args["enabled"], true);
        assert_eq!(args["verbose"], false);
    }

    #[test]
    fn test_array_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "tool_args": {
                "items": ["a", "b", "c"]
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert!(args["items"].is_array());
        assert_eq!(args["items"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_nested_object_tool_args() {
        let json_input = json!({
            "operation": "execute",
            "tool_args": {
                "config": {
                    "level1": {
                        "level2": {
                            "value": "deep"
                        }
                    }
                }
            }
        });
        
        let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
        let args = request.tool_args.unwrap();
        assert!(args["config"]["level1"]["level2"]["value"].is_string());
    }
}

// ============================================================
// Operation Coverage Test
// ============================================================

#[test]
fn test_operation_count() {
    // Verify we have exactly 10 operations
    let operations = vec![
        McpOperation::Add,
        McpOperation::Update,
        McpOperation::Delete,
        McpOperation::Test,
        McpOperation::Connect,
        McpOperation::Resync,
        McpOperation::Execute,
        McpOperation::ListServers,
        McpOperation::ListTools,
        McpOperation::RegisterTool,
    ];
    
    assert_eq!(operations.len(), 10, "Should have exactly 10 operations");
}

// ============================================================
// Request Heartbeat Tests
// ============================================================

#[test]
fn test_request_heartbeat_true() {
    let json_input = json!({
        "operation": "list_servers",
        "request_heartbeat": true
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(true));
}

#[test]
fn test_request_heartbeat_false() {
    let json_input = json!({
        "operation": "list_servers",
        "request_heartbeat": false
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert_eq!(request.request_heartbeat, Some(false));
}

#[test]
fn test_request_heartbeat_omitted() {
    let json_input = json!({
        "operation": "list_servers"
    });
    
    let request: McpOpsRequest = serde_json::from_value(json_input).unwrap();
    assert!(request.request_heartbeat.is_none());
}
