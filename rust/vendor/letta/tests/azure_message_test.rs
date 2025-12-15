//! Test Azure message creation specifically.

use letta::types::{AgentType, CreateAgentRequest};
use letta::{ClientConfig, LettaClient};

#[tokio::test]
async fn test_azure_message_creation() {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a fresh Azure agent for testing
    println!("Creating fresh Azure agent for testing...");
    let create_request = CreateAgentRequest::builder()
        .name("Fresh Azure Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("azure/gpt-4o")
        .embedding("azure/text-embedding-3-small")
        .build();

    let agent = match client.agents().create(create_request).await {
        Ok(agent) => agent,
        Err(e) => {
            println!("Failed to create Azure agent: {:?}", e);
            println!("Skipping Azure message test");
            return;
        }
    };

    let agent_id = &agent.id;
    println!("Created agent: {} ({})", agent.name, agent_id);

    println!("Testing message creation with Azure agent...");
    let message_request = letta::types::CreateMessagesRequest {
        messages: vec![letta::types::MessageCreate {
            role: letta::types::MessageRole::User,
            content: letta::types::MessageCreateContent::String("Hello from Rust SDK!".to_string()),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }],
        max_steps: Some(2),
        ..Default::default()
    };

    match client.messages().create(agent_id, message_request).await {
        Ok(response) => {
            println!("✅ Success!");
            println!("Messages: {}", response.messages.len());
            for (i, msg) in response.messages.iter().enumerate() {
                use letta::types::LettaMessageUnion;
                let type_str = match msg {
                    LettaMessageUnion::SystemMessage(_) => "System",
                    LettaMessageUnion::UserMessage(_) => "User",
                    LettaMessageUnion::AssistantMessage(_) => "Assistant",
                    LettaMessageUnion::ReasoningMessage(_) => "Reasoning",
                    LettaMessageUnion::HiddenReasoningMessage(_) => "HiddenReasoning",
                    LettaMessageUnion::ToolCallMessage(_) => "ToolCall",
                    LettaMessageUnion::ToolReturnMessage(_) => "ToolReturn",
                };
                println!("  Message {}: Type={}", i, type_str);
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
            // Clean up the agent before failing
            let _ = client.agents().delete(agent_id).await;
            panic!("Message creation failed: {:?}", e);
        }
    }

    // Clean up: delete the test agent
    println!("Cleaning up test agent...");
    match client.agents().delete(agent_id).await {
        Ok(_) => println!("✅ Test agent deleted"),
        Err(e) => println!("Warning: Failed to delete test agent: {:?}", e),
    }
}
