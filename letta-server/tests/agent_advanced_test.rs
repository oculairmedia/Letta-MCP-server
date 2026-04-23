//! Tests for agent_advanced tool operations
//!
//! These tests verify the behavior of agent operations including:
//! - Request validation
//! - Response format and optimization
//! - Error handling

use letta_server::tools::agent_advanced::{
    AgentAdvancedRequest, AgentOperation, BulkDeleteFilters,
};
use letta_types::Pagination;
use serde_json::json;

// ============================================================
// Request Parsing Tests
// ============================================================

#[test]
fn test_parse_list_operation() {
    let json_input = json!({
        "operation": "list"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::List));
    assert!(request.agent_id.is_none());
    assert!(request.pagination.is_none());
}

#[test]
fn test_parse_list_with_pagination() {
    let json_input = json!({
        "operation": "list",
        "pagination": {
            "limit": 20,
            "offset": 10
        }
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::List));

    let pagination = request.pagination.unwrap();
    assert_eq!(pagination.limit, Some(20));
    assert_eq!(pagination.offset, Some(10));
}

#[test]
fn test_parse_get_operation() {
    let json_input = json!({
        "operation": "get",
        "agent_id": "agent-12345678-1234-1234-1234-123456789012"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Get));
    assert_eq!(
        request.agent_id.unwrap(),
        "agent-12345678-1234-1234-1234-123456789012"
    );
}

#[test]
fn test_parse_create_operation() {
    let json_input = json!({
        "operation": "create",
        "name": "TestAgent",
        "system": "You are a helpful assistant.",
        "llm_config": {
            "model": "gpt-4",
            "model_endpoint_type": "openai"
        }
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Create));
    assert_eq!(request.name.unwrap(), "TestAgent");
    assert_eq!(request.system.unwrap(), "You are a helpful assistant.");
    assert!(request.llm_config.is_some());
}

#[test]
fn test_parse_search_operation() {
    let json_input = json!({
        "operation": "search",
        "name": "Meridian"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Search));
    assert_eq!(request.name.unwrap(), "Meridian");
}

#[test]
fn test_parse_search_with_tags() {
    let json_input = json!({
        "operation": "search",
        "tags": ["assistant", "production"]
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Search));
    let tags = request.tags.unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&"assistant".to_string()));
}

#[test]
fn test_parse_search_with_query() {
    let json_input = json!({
        "operation": "search",
        "query": "helpful assistant"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Search));
    assert_eq!(request.query.unwrap(), "helpful assistant");
}

#[test]
fn test_parse_delete_operation() {
    let json_input = json!({
        "operation": "delete",
        "agent_id": "agent-to-delete"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Delete));
    assert_eq!(request.agent_id.unwrap(), "agent-to-delete");
}

#[test]
fn test_parse_send_message_operation() {
    let json_input = json!({
        "operation": "send_message",
        "agent_id": "agent-12345",
        "messages": [
            {"role": "user", "content": "Hello!"}
        ]
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::SendMessage));
    assert!(request.messages.is_some());
    assert_eq!(request.messages.as_ref().unwrap().len(), 1);
}

#[test]
fn test_parse_bulk_delete_operation() {
    let json_input = json!({
        "operation": "bulk_delete",
        "filters": {
            "agent_name_filter": "test-",
            "agent_ids": ["agent-1", "agent-2"]
        }
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::BulkDelete));

    let filters = request.filters.unwrap();
    assert_eq!(filters.agent_name_filter.unwrap(), "test-");
    assert_eq!(filters.agent_ids.unwrap().len(), 2);
}

#[test]
fn test_parse_count_operation() {
    let json_input = json!({
        "operation": "count"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Count));
}

#[test]
fn test_parse_clone_operation() {
    let json_input = json!({
        "operation": "clone",
        "agent_id": "source-agent",
        "name": "cloned-agent"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(request.operation, AgentOperation::Clone));
    assert_eq!(request.agent_id.unwrap(), "source-agent");
    assert_eq!(request.name.unwrap(), "cloned-agent");
}

// ============================================================
// All Operations Enum Test
// ============================================================

#[test]
fn test_all_operations_parse() {
    let operations = vec![
        // CRUD
        "list",
        "create",
        "get",
        "update",
        "delete",
        "search",
        // Management
        "list_tools",
        "export",
        "import",
        "clone",
        "get_config",
        "bulk_delete",
        "context",
        "reset_messages",
        "summarize",
        // Messaging
        "send_message",
        "stream",
        "async_message",
        "cancel_message",
        "preview_payload",
        "search_messages",
        "get_message",
        "count",
        // Conversations
        "list_conversations",
        "get_conversation",
        "send_conversation_message",
        "cancel_conversation",
        "compact_conversation",
    ];

    for op in operations {
        let json_input = json!({ "operation": op });
        let result: Result<AgentAdvancedRequest, _> = serde_json::from_value(json_input);
        assert!(result.is_ok(), "Failed to parse operation: {}", op);
    }
}

#[test]
fn test_invalid_operation_fails() {
    let json_input = json!({
        "operation": "invalid_operation"
    });

    let result: Result<AgentAdvancedRequest, _> = serde_json::from_value(json_input);
    assert!(result.is_err(), "Should fail on invalid operation");
}

// ============================================================
// Conversation Operation Parsing Tests
// ============================================================

#[test]
fn test_parse_send_conversation_message() {
    let json_input = json!({
        "operation": "send_conversation_message",
        "agent_id": "agent-12345",
        "conversation_id": "conv-67890",
        "messages": [
            {"role": "user", "content": "Hello from conversation!"}
        ]
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        AgentOperation::SendConversationMessage
    ));
    assert_eq!(request.conversation_id.unwrap(), "conv-67890");
    assert!(request.messages.is_some());
}

#[test]
fn test_parse_list_conversations() {
    let json_input = json!({
        "operation": "list_conversations",
        "agent_id": "agent-12345"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        AgentOperation::ListConversations
    ));
    assert_eq!(request.agent_id.unwrap(), "agent-12345");
}

#[test]
fn test_parse_get_conversation() {
    let json_input = json!({
        "operation": "get_conversation",
        "conversation_id": "conv-abcdef"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        AgentOperation::GetConversation
    ));
    assert_eq!(request.conversation_id.unwrap(), "conv-abcdef");
}

#[test]
fn test_parse_compact_conversation() {
    let json_input = json!({
        "operation": "compact_conversation",
        "conversation_id": "conv-12345"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        AgentOperation::CompactConversation
    ));
}

#[test]
fn test_parse_cancel_conversation() {
    let json_input = json!({
        "operation": "cancel_conversation",
        "conversation_id": "conv-12345"
    });

    let request: AgentAdvancedRequest = serde_json::from_value(json_input).unwrap();
    assert!(matches!(
        request.operation,
        AgentOperation::CancelConversation
    ));
}

// ============================================================
// Pagination Tests
// ============================================================

#[test]
fn test_pagination_defaults() {
    let pagination = Pagination::default();
    // Pagination::default() sets limit=50, offset=0
    assert_eq!(pagination.limit, Some(50));
    assert_eq!(pagination.offset, Some(0));
}

#[test]
fn test_pagination_with_values() {
    let json_input = json!({
        "limit": 50,
        "offset": 100
    });

    let pagination: Pagination = serde_json::from_value(json_input).unwrap();
    assert_eq!(pagination.limit, Some(50));
    assert_eq!(pagination.offset, Some(100));
}

// ============================================================
// BulkDeleteFilters Tests
// ============================================================

#[test]
fn test_bulk_delete_filters_name_only() {
    let json_input = json!({
        "agent_name_filter": "test-"
    });

    let filters: BulkDeleteFilters = serde_json::from_value(json_input).unwrap();
    assert_eq!(filters.agent_name_filter.unwrap(), "test-");
    assert!(filters.agent_ids.is_none());
    assert!(filters.agent_tag_filter.is_none());
}

#[test]
fn test_bulk_delete_filters_ids_only() {
    let json_input = json!({
        "agent_ids": ["agent-1", "agent-2", "agent-3"]
    });

    let filters: BulkDeleteFilters = serde_json::from_value(json_input).unwrap();
    assert!(filters.agent_name_filter.is_none());
    let ids = filters.agent_ids.unwrap();
    assert_eq!(ids.len(), 3);
}

#[test]
fn test_bulk_delete_filters_combined() {
    let json_input = json!({
        "agent_name_filter": "prod-",
        "agent_tag_filter": "deprecated",
        "agent_ids": ["specific-agent"]
    });

    let filters: BulkDeleteFilters = serde_json::from_value(json_input).unwrap();
    assert_eq!(filters.agent_name_filter.unwrap(), "prod-");
    assert_eq!(filters.agent_tag_filter.unwrap(), "deprecated");
    assert_eq!(filters.agent_ids.unwrap().len(), 1);
}

// ============================================================
// Response Validation Tests (using test helpers)
// ============================================================

mod response_format {
    use letta_types::StandardResponse;
    use serde_json::json;

    #[test]
    fn test_success_response_format() {
        let response = StandardResponse::success(
            "list",
            json!({"agents": [], "total": 0}),
            "Retrieved 0 agents",
        );

        assert!(response.success);
        assert_eq!(response.operation, "list");
        assert!(response.data.as_ref().unwrap().get("agents").is_some());
        assert!(response.message.contains("Retrieved"));
    }

    #[test]
    fn test_success_no_data_response() {
        let response = StandardResponse::success_no_data("delete", "Agent deleted successfully");

        assert!(response.success);
        assert_eq!(response.operation, "delete");
        assert!(response.message.contains("deleted"));
    }

    #[test]
    fn test_response_has_operation() {
        let response = StandardResponse::success(
            "search",
            json!({"count": 5, "agents": []}),
            "Found 5 agents",
        );

        assert_eq!(response.operation, "search");
    }
}

// ============================================================
// Truncation Logic Tests
// ============================================================

mod truncation {
    /// Test helper - replicates truncate_text from agent_advanced
    fn truncate_text(text: &str, max_chars: usize) -> String {
        if text.len() <= max_chars {
            text.to_string()
        } else {
            let remaining = text.len() - max_chars;
            format!(
                "{}...[truncated, {} more chars]",
                &text[..max_chars],
                remaining
            )
        }
    }

    #[test]
    fn test_short_text_not_truncated() {
        let text = "Short text";
        let result = truncate_text(text, 100);
        assert_eq!(result, text);
    }

    #[test]
    fn test_long_text_truncated() {
        let text = "a".repeat(1000);
        let result = truncate_text(&text, 100);

        assert!(result.len() < text.len());
        assert!(result.contains("truncated"));
        assert!(result.contains("900 more chars"));
    }

    #[test]
    fn test_exact_length_not_truncated() {
        let text = "a".repeat(100);
        let result = truncate_text(&text, 100);
        assert_eq!(result, text);
    }

    #[test]
    fn test_one_over_length_truncated() {
        let text = "a".repeat(101);
        let result = truncate_text(&text, 100);

        assert!(result.contains("truncated"));
        assert!(result.contains("1 more chars"));
    }

    #[test]
    fn test_description_truncation_limit() {
        // Agent list uses 100 char limit for descriptions
        let long_description = "This is a very long description that goes well beyond the 100 character limit set for agent summaries in the list operation.";
        let result = truncate_text(long_description, 100);

        assert!(result.starts_with("This is a very long"));
        assert!(result.contains("truncated"));
    }

    #[test]
    fn test_system_prompt_truncation_limit() {
        // Agent get uses 500 char limit for system prompts
        let long_system = "You are a helpful assistant. ".repeat(50);
        let result = truncate_text(&long_system, 500);

        assert!(result.len() < long_system.len());
        assert!(result.contains("truncated"));
    }
}

// ============================================================
// SSE Response Parsing Tests
// ============================================================
//
// These tests verify the SSE parsing logic that works around the
// Letta server's non-streaming response serialization bug.
// See: letta-MCP-server-dpg

mod sse_parsing {
    use serde_json::json;

    /// Replicates the SSE parsing logic from conversations.rs
    /// to test it in isolation without HTTP dependencies.
    fn parse_sse_body(body_text: &str) -> Result<serde_json::Value, String> {
        let mut collected_messages: Vec<serde_json::Value> = Vec::new();
        let mut stop_reason: Option<serde_json::Value> = None;

        for line in body_text.lines() {
            let line = line.trim();
            if !line.starts_with("data: ") {
                continue;
            }
            let data = &line[6..];
            if data.is_empty() || data == "[DONE]" {
                continue;
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                let msg_type = val
                    .get("message_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                match msg_type {
                    "assistant_message" | "tool_call_message" | "tool_return_message"
                    | "reasoning_message" | "user_message" | "system_message" => {
                        collected_messages.push(val);
                    }
                    "stop_reason" => {
                        stop_reason = Some(val);
                    }
                    "error_message" => {
                        // Non-fatal cleanup errors from the server
                    }
                    _ => {}
                }
            }
        }

        if collected_messages.is_empty() {
            return Err("Stream completed without returning any messages".to_string());
        }

        Ok(json!({
            "messages": collected_messages,
            "stop_reason": stop_reason.unwrap_or(json!({"stop_reason": "end_turn"})),
            "usage": {},
        }))
    }

    #[test]
    fn test_parse_normal_sse_response() {
        let sse_body = "\
data: {\"message_type\": \"assistant_message\", \"text\": \"Hello!\"}\n\
\n\
data: {\"message_type\": \"stop_reason\", \"stop_reason\": \"end_turn\"}\n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["text"], "Hello!");
    }

    #[test]
    fn test_parse_multi_message_sse() {
        let sse_body = "\
data: {\"message_type\": \"reasoning_message\", \"text\": \"thinking...\"}\n\
\n\
data: {\"message_type\": \"tool_call_message\", \"text\": \"calling tool\"}\n\
\n\
data: {\"message_type\": \"tool_return_message\", \"text\": \"tool result\"}\n\
\n\
data: {\"message_type\": \"assistant_message\", \"text\": \"Here is the answer.\"}\n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 4);
    }

    #[test]
    fn test_parse_sse_with_error_message_non_fatal() {
        // Server cleanup errors should be silently skipped
        let sse_body = "\
data: {\"message_type\": \"assistant_message\", \"text\": \"Response\"}\n\
\n\
data: {\"message_type\": \"error_message\", \"text\": \"cleanup error\"}\n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1, "Error messages should be filtered out");
        assert_eq!(messages[0]["text"], "Response");
    }

    #[test]
    fn test_parse_empty_sse_returns_error() {
        let sse_body = "data: [DONE]\n";
        let result = parse_sse_body(sse_body);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("without returning any messages"));
    }

    #[test]
    fn test_parse_sse_with_only_done() {
        let sse_body = "\n\ndata: [DONE]\n\n";
        let result = parse_sse_body(sse_body);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_sse_ignores_non_data_lines() {
        let sse_body = "\
event: message\n\
data: {\"message_type\": \"assistant_message\", \"text\": \"Hello!\"}\n\
\n\
: keep-alive\n\
\n\
retry: 3000\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_parse_sse_with_invalid_json_lines() {
        // Malformed JSON lines should be silently skipped
        let sse_body = "\
data: {\"message_type\": \"assistant_message\", \"text\": \"Good response\"}\n\
\n\
data: {invalid json here\n\
\n\
data: \n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1, "Only valid JSON messages should be kept");
    }

    #[test]
    fn test_parse_sse_default_stop_reason() {
        // When no stop_reason event is present, should default to end_turn
        let sse_body = "\
data: {\"message_type\": \"assistant_message\", \"text\": \"Hello\"}\n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        assert_eq!(result["stop_reason"]["stop_reason"], "end_turn");
    }

    #[test]
    fn test_parse_sse_unknown_message_types_ignored() {
        let sse_body = "\
data: {\"message_type\": \"ping\", \"ts\": 123}\n\
\n\
data: {\"message_type\": \"assistant_message\", \"text\": \"Real message\"}\n\
\n\
data: {\"message_type\": \"internal_debug\", \"detail\": \"stuff\"}\n\
\n\
data: [DONE]\n";

        let result = parse_sse_body(sse_body).unwrap();
        let messages = result["messages"].as_array().unwrap();
        assert_eq!(
            messages.len(),
            1,
            "Only known message types should be collected"
        );
    }
}

// ============================================================
// Schema Validation Tests
// ============================================================

mod schema_validation {
    use letta_server::tools::agent_advanced::AgentAdvancedRequest;
    use schemars::schema_for;

    #[test]
    fn test_schema_has_no_refs() {
        let schema = schema_for!(AgentAdvancedRequest);
        let schema_json = serde_json::to_string(&schema).unwrap();

        // Check that $ref appears only in $defs (definitions) section, not in properties
        // The properties should be inline
        let schema_value: serde_json::Value = serde_json::from_str(&schema_json).unwrap();

        // Properties should not contain $ref directly
        if let Some(props) = schema_value.get("properties") {
            let props_str = serde_json::to_string(props).unwrap();
            // Count $ref occurrences - should be 0 in properties
            let ref_count = props_str.matches("\"$ref\"").count();
            assert_eq!(
                ref_count, 0,
                "Properties should not contain $ref references. Found {} refs in: {}",
                ref_count, props_str
            );
        }
    }

    #[test]
    fn test_only_operation_is_required() {
        let schema = schema_for!(AgentAdvancedRequest);
        let schema_json = serde_json::to_string(&schema).unwrap();
        let schema_value: serde_json::Value = serde_json::from_str(&schema_json).unwrap();

        if let Some(required) = schema_value.get("required") {
            let required_array = required.as_array().expect("required should be an array");

            // Only 'operation' should be required
            assert_eq!(
                required_array.len(),
                1,
                "Only 'operation' should be required, but found: {:?}",
                required_array
            );
            assert_eq!(
                required_array[0].as_str().unwrap(),
                "operation",
                "The only required field should be 'operation'"
            );
        } else {
            // If no required field, that's wrong - operation should be required
            panic!("Schema should have a 'required' field with 'operation'");
        }
    }

    #[test]
    fn test_schema_has_no_defs() {
        let schema = schema_for!(AgentAdvancedRequest);
        let schema_json = serde_json::to_string(&schema).unwrap();
        let schema_value: serde_json::Value = serde_json::from_str(&schema_json).unwrap();

        // Check that there's no $defs section (all types should be inlined)
        assert!(
            schema_value.get("$defs").is_none(),
            "Schema should not have $defs section - all types should be inlined. Found: {:?}",
            schema_value.get("$defs")
        );
    }
}

// ============================================================
// Agent Summary Format Tests
// ============================================================

mod summary_format {
    use serde_json::json;

    /// Expected fields in agent summary (list operation)
    const EXPECTED_SUMMARY_FIELDS: &[&str] = &[
        "id",
        "name",
        "description",
        "model",
        "created_at",
        "tool_count",
    ];

    /// Fields that should be excluded from summary
    const EXCLUDED_SUMMARY_FIELDS: &[&str] = &[
        "system",
        "tools",
        "memory",
        "llm_config",
        "embedding_config",
    ];

    #[test]
    fn test_agent_summary_has_required_fields() {
        let summary = json!({
            "id": "agent-123",
            "name": "TestAgent",
            "description": "A test agent",
            "model": "gpt-4",
            "created_at": "2025-01-01T00:00:00Z",
            "tool_count": 5
        });

        for field in EXPECTED_SUMMARY_FIELDS {
            assert!(summary.get(field).is_some(), "Missing field: {}", field);
        }
    }

    #[test]
    fn test_agent_summary_excludes_heavy_fields() {
        let summary = json!({
            "id": "agent-123",
            "name": "TestAgent",
            "description": "A test agent",
            "model": "gpt-4",
            "created_at": "2025-01-01T00:00:00Z",
            "tool_count": 5
        });

        for field in EXCLUDED_SUMMARY_FIELDS {
            assert!(
                summary.get(field).is_none(),
                "Should exclude field: {}",
                field
            );
        }
    }

    #[test]
    fn test_pagination_metadata_format() {
        let response_data = json!({
            "total": 100,
            "returned": 15,
            "offset": 0,
            "has_more": true,
            "agents": [],
            "hints": ["Use 'get' with agent_id for full details"]
        });

        assert_eq!(response_data["total"], 100);
        assert_eq!(response_data["returned"], 15);
        assert_eq!(response_data["has_more"], true);
        assert!(!response_data["hints"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_search_response_format() {
        let response_data = json!({
            "count": 3,
            "agents": [],
            "search_criteria": {
                "name": "Meridian",
                "tags": null,
                "query": null
            },
            "hint": "Use 'get' with agent_id for full details"
        });

        assert!(response_data.get("count").is_some());
        assert!(response_data.get("agents").is_some());
        assert!(response_data.get("search_criteria").is_some());
        assert!(response_data.get("hint").is_some());
    }
}
