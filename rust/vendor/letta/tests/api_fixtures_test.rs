//! API Response Fixture Tests
//!
//! These tests ensure our deserializers can handle all known API response shapes,
//! including edge cases like null responses discovered in production.

use letta::types::agent::AgentState;
use letta::types::memory::Passage;
use letta::types::tool::Tool;

#[test]
fn test_attach_tool_null_response() {
    // Issue #134: attach/detach operations sometimes return null instead of AgentState
    let json = include_str!("fixtures/attach_tool_null.json");
    let result: Result<Option<AgentState>, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Failed to deserialize null response: {:?}",
        result.err()
    );
    assert!(result.unwrap().is_none(), "Expected None for null response");
}

#[test]
fn test_attach_tool_valid_response() {
    let json = include_str!("fixtures/attach_tool_valid.json");
    let result: Result<AgentState, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Failed to deserialize valid AgentState: {:?}",
        result.err()
    );

    let agent = result.unwrap();
    assert_eq!(agent.name, "Test Agent");
    assert!(!agent.tools.is_empty(), "Expected tools to be present");
}

#[test]
fn test_list_agents_response() {
    let json = include_str!("fixtures/list_agents_response.json");
    let result: Result<Vec<AgentState>, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Failed to deserialize agent list: {:?}",
        result.err()
    );

    let agents = result.unwrap();
    assert_eq!(agents.len(), 2, "Expected 2 agents");
    assert_eq!(agents[0].name, "Agent One");
    assert_eq!(agents[1].name, "Agent Two");
}

#[test]
fn test_list_tools_response() {
    let json = include_str!("fixtures/list_tools_response.json");
    let result: Result<Vec<Tool>, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Failed to deserialize tool list: {:?}",
        result.err()
    );

    let tools = result.unwrap();
    assert_eq!(tools.len(), 2, "Expected 2 tools");
    assert_eq!(tools[0].name, "send_message");
    assert_eq!(tools[1].name, "core_memory_append");
}

#[test]
fn test_archival_memory_search_response() {
    let json = include_str!("fixtures/archival_memory_search.json");
    let result: Result<Vec<Passage>, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Failed to deserialize passage list: {:?}",
        result.err()
    );

    let passages = result.unwrap();
    assert_eq!(passages.len(), 2, "Expected 2 passages");
    assert!(passages[0].text.contains("Important context"));
}
