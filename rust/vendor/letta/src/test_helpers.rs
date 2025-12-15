//! Test helper utilities for the Letta client library.
//!
//! This module provides convenient helper functions for writing tests with the Letta API.
//! It includes builders, preset configurations, and utilities to reduce boilerplate in tests.

#![cfg(test)]

use crate::types::*;
use crate::LettaId;
use std::str::FromStr;

/// Test helper functions for agents.
pub mod agents {
    use super::*;

    /// Create a test agent request with minimal configuration.
    ///
    /// # Example
    /// ```no_run
    /// use letta::test_helpers::agents::test_agent;
    ///
    /// let request = test_agent("TestBot");
    /// // Creates an agent with name "TestBot" and sensible test defaults
    /// ```
    pub fn test_agent(name: impl Into<String>) -> CreateAgentRequest {
        CreateAgentRequest {
            name: Some(name.into()),
            agent_type: Some(AgentType::MemGPT),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            ..Default::default()
        }
    }

    /// Create a test agent with custom LLM configuration.
    pub fn test_agent_with_llm(
        name: impl Into<String>,
        llm_config: LLMConfig,
    ) -> CreateAgentRequest {
        CreateAgentRequest {
            name: Some(name.into()),
            agent_type: Some(AgentType::MemGPT),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            llm_config: Some(llm_config),
            ..Default::default()
        }
    }

    /// Create a minimal agent request for testing specific features.
    pub fn minimal_agent() -> CreateAgentRequest {
        CreateAgentRequest {
            agent_type: Some(AgentType::MemGPT),
            ..Default::default()
        }
    }
}

/// Test helper functions for memory blocks.
pub mod memory {
    use super::*;

    /// Create a test human memory block.
    pub fn test_human_block(name: impl Into<String>) -> Block {
        Block::human(format!("The human's name is {}", name.into())).with_limit(2000)
    }

    /// Create a test persona memory block.
    pub fn test_persona_block(description: impl Into<String>) -> Block {
        Block::persona(format!(
            "I am a helpful AI assistant. {}",
            description.into()
        ))
        .with_limit(2000)
    }

    /// Create a test archival memory request.
    pub fn test_archival_memory(text: impl Into<String>) -> CreateArchivalMemoryRequest {
        CreateArchivalMemoryRequest { text: text.into() }
    }

    /// Create multiple test archival memories.
    pub fn test_archival_memories(texts: Vec<&str>) -> Vec<CreateArchivalMemoryRequest> {
        texts
            .into_iter()
            .map(|text| CreateArchivalMemoryRequest {
                text: text.to_string(),
            })
            .collect()
    }
}

/// Test helper functions for tools.
pub mod tools {
    use super::*;
    use serde_json::json;

    /// Create a simple echo tool for testing.
    pub fn test_echo_tool(name: impl Into<String>) -> CreateToolRequest {
        let name_str = name.into();
        CreateToolRequest {
            description: Some("Echoes back the input message".to_string()),
            source_code: format!(
                r#"
def {}(message: str) -> str:
    """
    Echo the input message.
    
    Args:
        message: The message to echo
    
    Returns:
        str: The echoed message
    """
    return message
"#,
                name_str
            ),
            json_schema: Some(json!({
                "name": name_str,
                "description": "Echoes back the input message",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "The message to echo"
                        }
                    },
                    "required": ["message"]
                }
            })),
            args_json_schema: Some(json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo"
                    }
                },
                "required": ["message"]
            })),
            ..Default::default()
        }
    }

    /// Create a calculator tool for testing.
    pub fn test_calculator_tool() -> CreateToolRequest {
        CreateToolRequest {
            description: Some("Performs basic arithmetic operations".to_string()),
            source_code: r#"
def calculator(operation: str, a: float, b: float) -> float:
    """
    Perform arithmetic operations.
    
    Args:
        operation: The arithmetic operation to perform (add, subtract, multiply, divide)
        a: First operand
        b: Second operand
    
    Returns:
        float: The result of the operation
    """
    if operation == "add":
        return a + b
    elif operation == "subtract":
        return a - b
    elif operation == "multiply":
        return a * b
    elif operation == "divide":
        if b == 0:
            raise ValueError("Cannot divide by zero")
        return a / b
    else:
        raise ValueError(f"Unknown operation: {operation}")
"#
            .to_string(),
            json_schema: Some(json!({
                "name": "calculator",
                "description": "Performs basic arithmetic operations",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "operation": {
                            "type": "string",
                            "enum": ["add", "subtract", "multiply", "divide"],
                            "description": "The arithmetic operation to perform"
                        },
                        "a": {
                            "type": "number",
                            "description": "First operand"
                        },
                        "b": {
                            "type": "number",
                            "description": "Second operand"
                        }
                    },
                    "required": ["operation", "a", "b"]
                }
            })),
            args_json_schema: Some(json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"],
                        "description": "The arithmetic operation to perform"
                    },
                    "a": {
                        "type": "number",
                        "description": "First operand"
                    },
                    "b": {
                        "type": "number",
                        "description": "Second operand"
                    }
                },
                "required": ["operation", "a", "b"]
            })),
            ..Default::default()
        }
    }

    /// Create a simple no-op tool that does nothing.
    pub fn test_noop_tool() -> CreateToolRequest {
        CreateToolRequest {
            description: Some("A tool that does nothing".to_string()),
            source_code: r#"
def noop() -> str:
    """
    Do nothing and return success.
    
    Returns:
        str: Always returns 'success'
    """
    return "success"
"#
            .to_string(),
            json_schema: Some(json!({
                "name": "noop",
                "description": "A tool that does nothing",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            })),
            args_json_schema: Some(json!({
                "type": "object",
                "properties": {}
            })),
            ..Default::default()
        }
    }

    /// Create a test tool rule.
    pub fn test_tool_rule(tool_name: impl Into<String>) -> ToolRule {
        ToolRule::init(tool_name)
    }

    /// Create a test conditional tool rule.
    pub fn test_conditional_rule(tool_name: impl Into<String>) -> ConditionalToolRule {
        ConditionalToolRule::builder()
            .tool_name(tool_name.into())
            .prompt_template("Should I use this tool? Answer yes or no.".to_string())
            .default_child("fallback_tool".to_string())
            .build()
    }
}

/// Test helper functions for messages.
pub mod messages {
    use super::*;

    /// Create a simple test user message.
    pub fn test_user_message(content: impl Into<String>) -> MessageCreate {
        MessageCreate::user(content)
    }

    /// Create a test message with a specific role.
    pub fn test_message(role: MessageRole, content: impl Into<String>) -> MessageCreate {
        MessageCreate {
            role,
            content: MessageCreateContent::String(content.into()),
            ..Default::default()
        }
    }

    /// Create a test messages request with a single message.
    pub fn test_messages_request(message: MessageCreate) -> CreateMessagesRequest {
        CreateMessagesRequest {
            messages: vec![message],
            ..Default::default()
        }
    }

    /// Create a test messages request with multiple messages.
    pub fn test_conversation(messages: Vec<(&str, MessageRole)>) -> CreateMessagesRequest {
        let messages = messages
            .into_iter()
            .map(|(content, role)| MessageCreate {
                role,
                content: MessageCreateContent::String(content.to_string()),
                ..Default::default()
            })
            .collect();

        CreateMessagesRequest {
            messages,
            ..Default::default()
        }
    }

    /// Create a multi-part message with text and image.
    pub fn test_multipart_message(text: &str, image_url: &str) -> MessageCreate {
        MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::ContentParts(vec![
                ContentPart::Text(TextContent {
                    text: text.to_string(),
                }),
                ContentPart::Image(ImageContent {
                    image_url: ImageUrl {
                        url: image_url.to_string(),
                        detail: None,
                    },
                }),
            ]),
            ..Default::default()
        }
    }
}

/// Test helper functions for sources.
pub mod sources {
    use super::*;

    /// Create a test source.
    pub fn test_source(name: impl Into<String>) -> CreateSourceRequest {
        CreateSourceRequest {
            name: name.into(),
            description: Some("Test source for unit tests".to_string()),
            metadata: None,
            embedding: None,
            embedding_config: None,
            embedding_chunk_size: None,
            instructions: None,
        }
    }

    /// Create a test source upload.
    pub fn test_source_upload(
        name: impl Into<String>,
        content: &str,
    ) -> (CreateSourceRequest, Vec<u8>) {
        let source = test_source(name);
        let file_content = content.as_bytes().to_vec();
        (source, file_content)
    }
}

/// Utility functions for generating test data.
pub mod data {
    use super::*;
    use chrono::Utc;

    /// Generate a unique test name with a timestamp and random suffix.
    pub fn unique_name(prefix: &str) -> String {
        let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let random_suffix = uuid::Uuid::new_v4().simple().to_string();
        format!("{}-{}-{}", prefix, timestamp, &random_suffix[..8])
    }

    /// Generate a test UUID string.
    pub fn test_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Generate a test LettaId with a given prefix.
    pub fn test_id(prefix: &str) -> LettaId {
        LettaId::new_prefixed(prefix, uuid::Uuid::new_v4())
    }

    /// Generate a bare test LettaId.
    pub fn test_bare_id() -> LettaId {
        LettaId::new_bare(uuid::Uuid::new_v4())
    }

    /// Parse a test LettaId from a string (panics on error for tests).
    pub fn parse_id(id: &str) -> LettaId {
        LettaId::from_str(id).expect("Invalid test ID")
    }

    /// Generate test metadata.
    pub fn test_metadata() -> serde_json::Value {
        serde_json::json!({
            "test": true,
            "created_by": "test_helpers",
            "timestamp": Utc::now().to_rfc3339(),
        })
    }
}

/// Test helper functions for pagination.
pub mod pagination {
    use super::*;

    /// Create test pagination params with a limit.
    pub fn test_pagination(limit: u32) -> PaginationParams {
        PaginationParams {
            limit: Some(limit),
            ..Default::default()
        }
    }

    /// Create test pagination params with all fields.
    pub fn test_full_pagination(limit: u32, after: &str) -> PaginationParams {
        PaginationParams {
            limit: Some(limit),
            after: Some(after.to_string()),
            ascending: Some(false),
            ..Default::default()
        }
    }
}

/// Test assertions and validators.
pub mod assertions {
    use super::*;

    /// Assert that an agent has the expected name.
    pub fn assert_agent_name(agent: &AgentState, expected_name: &str) {
        assert_eq!(agent.name.as_str(), expected_name, "Agent name mismatch");
    }

    /// Assert that a message has the expected role.
    pub fn assert_message_role(message: &Message, expected_role: MessageRole) {
        assert_eq!(message.role, expected_role, "Message role mismatch");
    }

    /// Assert that a tool has the expected name.
    pub fn assert_tool_name(tool: &Tool, expected_name: &str) {
        assert_eq!(tool.name.as_str(), expected_name, "Tool name mismatch");
    }

    /// Assert that a block has the expected label.
    pub fn assert_block_label(block: &Block, expected_label: &str) {
        assert_eq!(block.label.as_str(), expected_label, "Block label mismatch");
    }
}

/// Common test fixtures and scenarios.
pub mod fixtures {
    use super::*;

    /// Create a complete test agent with memory blocks and tools.
    pub fn complete_test_agent(name: &str) -> CreateAgentRequest {
        let mut request = agents::test_agent(name);

        // Add memory blocks
        request.memory_blocks = Some(vec![
            memory::test_human_block("TestUser"),
            memory::test_persona_block("I help with testing."),
        ]);

        // Add tools
        request.tools = Some(vec!["send_message".to_string()]);

        // Add tool rules
        request.tool_rules = Some(vec![tools::test_tool_rule("send_message")]);

        request
    }

    /// Create a test agent configured for streaming.
    pub fn streaming_test_agent(name: &str) -> CreateAgentRequest {
        let mut request = agents::test_agent(name);
        request.model = Some("gpt-4".to_string());
        request
    }

    /// Create a test agent with custom metadata.
    pub fn agent_with_metadata(
        name: &str,
        metadata_value: serde_json::Value,
    ) -> CreateAgentRequest {
        let mut request = agents::test_agent(name);
        let mut metadata = Metadata::new();
        if let serde_json::Value::Object(map) = metadata_value {
            for (k, v) in map {
                metadata.insert(k, v);
            }
        }
        request.metadata = Some(metadata);
        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_names_are_unique() {
        let name1 = data::unique_name("test");
        let name2 = data::unique_name("test");
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_agent_helpers() {
        let agent = agents::test_agent("TestBot");
        assert_eq!(agent.name.as_ref().unwrap(), "TestBot");
        assert_eq!(agent.agent_type, Some(AgentType::MemGPT));
    }

    #[test]
    fn test_memory_helpers() {
        let human = memory::test_human_block("Alice");
        assert!(human.value.contains("Alice"));
        assert_eq!(human.label, "human");
    }

    #[test]
    fn test_tool_helpers() {
        let echo = tools::test_echo_tool("my_echo");
        assert!(echo.source_code.contains("my_echo"));
        assert!(echo.source_code.contains("Echo the input message"));
    }

    #[test]
    fn test_message_helpers() {
        let msg = messages::test_user_message("Hello");
        assert_eq!(msg.role, MessageRole::User);

        if let MessageCreateContent::String(content) = &msg.content {
            assert_eq!(content, "Hello");
        } else {
            panic!("Expected string content");
        }
    }
}
