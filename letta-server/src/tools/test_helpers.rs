//! Test helpers and mock infrastructure for tool testing
//!
//! Provides utilities for unit testing tool handlers without requiring
//! a live Letta server connection.

use letta_types::StandardResponse;
use serde_json::{json, Value};

/// Create a mock agent state for testing
pub fn mock_agent(id: &str, name: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "description": "Test agent description",
        "system": "You are a helpful assistant.",
        "agent_type": "memgpt",
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z",
        "llm_config": {
            "model": "gpt-4",
            "model_endpoint_type": "openai"
        },
        "embedding_config": {
            "model": "text-embedding-3-small",
            "model_endpoint_type": "openai"
        },
        "tools": [],
        "memory": {
            "core_memory": {
                "persona": "I am a helpful assistant.",
                "human": "The user is testing."
            }
        }
    })
}

/// Create a mock agent with custom fields
pub fn mock_agent_full(
    id: &str,
    name: &str,
    description: Option<&str>,
    system: Option<&str>,
    model: &str,
    tool_count: usize,
) -> Value {
    let tools: Vec<Value> = (0..tool_count)
        .map(|i| json!({"id": format!("tool-{}", i), "name": format!("tool_{}", i)}))
        .collect();

    json!({
        "id": id,
        "name": name,
        "description": description,
        "system": system.unwrap_or("You are a helpful assistant."),
        "agent_type": "memgpt",
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z",
        "llm_config": {
            "model": model,
            "model_endpoint_type": "openai"
        },
        "embedding_config": {
            "model": "text-embedding-3-small",
            "model_endpoint_type": "openai"
        },
        "tools": tools,
        "memory": {
            "core_memory": {
                "persona": "I am a helpful assistant.",
                "human": "The user is testing."
            }
        }
    })
}

/// Create a mock memory block
pub fn mock_block(id: &str, label: &str, value: &str) -> Value {
    json!({
        "id": id,
        "label": label,
        "description": format!("{} block for testing", label),
        "value": value,
        "is_template": false,
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z"
    })
}

/// Create a mock tool
pub fn mock_tool(id: &str, name: &str, description: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "description": description,
        "source_type": "python",
        "source_code": "def {}(): pass".replace("{}", name),
        "json_schema": {
            "type": "object",
            "properties": {}
        },
        "created_at": "2025-01-01T00:00:00Z"
    })
}

/// Create a mock source
pub fn mock_source(id: &str, name: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "description": "Test source",
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z"
    })
}

/// Create a mock passage
pub fn mock_passage(id: &str, text: &str, source: Option<&str>) -> Value {
    json!({
        "id": id,
        "text": text,
        "source": source,
        "created_at": "2025-01-01T00:00:00Z"
    })
}

/// Create a mock job
pub fn mock_job(id: &str, status: &str) -> Value {
    json!({
        "id": id,
        "status": status,
        "created_at": "2025-01-01T00:00:00Z",
        "updated_at": "2025-01-01T00:00:00Z",
        "result": null
    })
}

/// Create a mock MCP server config
pub fn mock_mcp_server(name: &str, server_type: &str) -> Value {
    json!({
        "name": name,
        "server_type": server_type,
        "config": {
            "url": format!("http://localhost:3000/{}", name)
        },
        "created_at": "2025-01-01T00:00:00Z"
    })
}

/// Create a mock message
pub fn mock_message(id: &str, role: &str, content: &str) -> Value {
    json!({
        "id": id,
        "role": role,
        "text": content,
        "created_at": "2025-01-01T00:00:00Z"
    })
}

/// Assert that a StandardResponse indicates success
pub fn assert_success(response: &StandardResponse) {
    assert!(
        response.success,
        "Expected success but got failure: {:?}",
        response.message
    );
}

/// Assert that a StandardResponse indicates failure
pub fn assert_failure(response: &StandardResponse) {
    assert!(
        !response.success,
        "Expected failure but got success: {:?}",
        response.message
    );
}

/// Assert that response data contains expected field
pub fn assert_has_field(response: &StandardResponse, field: &str) {
    let data = response.data.as_ref().expect("Response data is None");
    assert!(
        data.get(field).is_some(),
        "Expected field '{}' not found in response data: {:?}",
        field,
        data
    );
}

/// Assert that response data has expected count
pub fn assert_count(response: &StandardResponse, field: &str, expected: usize) {
    let data = response.data.as_ref().expect("Response data is None");
    let arr = data
        .get(field)
        .and_then(|v| v.as_array())
        .expect(&format!("Expected array field '{}'", field));
    assert_eq!(
        arr.len(),
        expected,
        "Expected {} items in '{}', got {}",
        expected,
        field,
        arr.len()
    );
}

/// Extract a string field from response data
pub fn get_string_field(response: &StandardResponse, field: &str) -> Option<String> {
    let data = response.data.as_ref()?;
    data.get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract a numeric field from response data
pub fn get_number_field(response: &StandardResponse, field: &str) -> Option<u64> {
    let data = response.data.as_ref()?;
    data.get(field).and_then(|v| v.as_u64())
}

/// Generate a list of mock agents
pub fn mock_agent_list(count: usize) -> Vec<Value> {
    (0..count)
        .map(|i| {
            mock_agent_full(
                &format!("agent-{:08}", i),
                &format!("Agent {}", i),
                Some(&format!("Description for agent {}", i)),
                None,
                if i % 2 == 0 { "gpt-4" } else { "claude-3" },
                i % 5,
            )
        })
        .collect()
}

/// Generate a list of mock blocks
pub fn mock_block_list(count: usize) -> Vec<Value> {
    let labels = ["persona", "human", "system", "custom"];
    (0..count)
        .map(|i| {
            mock_block(
                &format!("block-{:08}", i),
                labels[i % labels.len()],
                &format!("Value content for block {}", i),
            )
        })
        .collect()
}

/// Generate a list of mock tools
pub fn mock_tool_list(count: usize) -> Vec<Value> {
    (0..count)
        .map(|i| {
            mock_tool(
                &format!("tool-{:08}", i),
                &format!("tool_{}", i),
                &format!("Description for tool {}", i),
            )
        })
        .collect()
}

/// Generate a list of mock sources
pub fn mock_source_list(count: usize) -> Vec<Value> {
    (0..count)
        .map(|i| mock_source(&format!("source-{:08}", i), &format!("Source {}", i)))
        .collect()
}

/// Test helper for validating truncation
pub fn is_truncated(text: &str) -> bool {
    text.contains("truncated") || text.ends_with("...")
}

/// Test helper for checking pagination hints
/// Test helper for checking pagination hints
pub fn has_pagination_hint(response: &StandardResponse) -> bool {
    response
        .data
        .as_ref()
        .and_then(|d| d.get("hints"))
        .and_then(|v| v.as_array())
        .map(|arr| !arr.is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_agent_creation() {
        let agent = mock_agent("agent-123", "TestAgent");
        assert_eq!(agent["id"], "agent-123");
        assert_eq!(agent["name"], "TestAgent");
        assert!(agent["llm_config"].is_object());
    }

    #[test]
    fn test_mock_agent_full() {
        let agent = mock_agent_full(
            "agent-456",
            "FullAgent",
            Some("Custom description"),
            Some("Custom system prompt"),
            "claude-3",
            3,
        );
        assert_eq!(agent["id"], "agent-456");
        assert_eq!(agent["description"], "Custom description");
        assert_eq!(agent["tools"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_mock_agent_list() {
        let agents = mock_agent_list(10);
        assert_eq!(agents.len(), 10);

        // Check alternating models
        assert_eq!(agents[0]["llm_config"]["model"], "gpt-4");
        assert_eq!(agents[1]["llm_config"]["model"], "claude-3");
    }

    #[test]
    fn test_mock_block() {
        let block = mock_block("block-123", "persona", "Test persona content");
        assert_eq!(block["id"], "block-123");
        assert_eq!(block["label"], "persona");
        assert_eq!(block["value"], "Test persona content");
    }

    #[test]
    fn test_mock_tool() {
        let tool = mock_tool("tool-123", "my_tool", "A test tool");
        assert_eq!(tool["id"], "tool-123");
        assert_eq!(tool["name"], "my_tool");
        assert!(tool["source_code"].as_str().unwrap().contains("my_tool"));
    }

    #[test]
    fn test_is_truncated() {
        assert!(is_truncated("Some text...[truncated, 100 more chars]"));
        assert!(is_truncated("Short text..."));
        assert!(!is_truncated("Normal text without truncation"));
    }
}
