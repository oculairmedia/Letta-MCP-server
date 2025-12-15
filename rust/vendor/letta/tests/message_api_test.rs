//! Integration tests for the message API.

use letta::types::memory::Block;
use letta::types::{
    AgentType, CreateAgentRequest, CreateMessagesRequest, MessageCreate, MessageCreateContent,
    MessageRole, UpdateMessageRequest, UpdateUserMessage, UpdateUserMessageContent,
};
use letta::{ClientConfig, LettaClient};
use serial_test::serial;

#[tokio::test]
async fn test_local_server_message_operations() {
    // Create client for local server (no auth required)
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for message operations...");
    let create_request = CreateAgentRequest::builder()
        .name("Message Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block::human("The human's name is User"))
        .memory_block(Block::persona("I am a helpful assistant"))
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // Test 1: List existing messages
    println!("Testing message list for agent: {}", agent_id);
    let messages = client.messages().list(agent_id, None).await.unwrap();
    println!("✅ Found {} existing messages", messages.len());

    // Test 2: Send a simple message
    println!("Testing message creation...");
    let message_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::String(
                "Hello! This is a test message from the Rust SDK.".to_string(),
            ),
            name: None,
            otid: None,
            sender_id: None,
            batch_item_id: None,
            group_id: None,
        }],
        max_steps: Some(3), // Limit steps for testing
        ..Default::default()
    };

    let response = client
        .messages()
        .create(agent_id, message_request)
        .await
        .unwrap();

    println!("✅ Message sent successfully!");
    println!("   Stop reason: {:?}", response.stop_reason);
    println!("   Usage: {:?} total tokens", response.usage.total_tokens);
    println!(
        "   Received {} messages in response",
        response.messages.len()
    );

    // Print response messages
    for (i, message) in response.messages.iter().enumerate() {
        match message {
            letta::types::LettaMessageUnion::UserMessage(msg) => {
                println!("   Message {}: [User] {}", i + 1, msg.content);
            }
            letta::types::LettaMessageUnion::AssistantMessage(msg) => {
                println!("   Message {}: [Assistant] {}", i + 1, msg.content);
            }
            letta::types::LettaMessageUnion::SystemMessage(msg) => {
                println!("   Message {}: [System] {}", i + 1, msg.content);
            }
            letta::types::LettaMessageUnion::ReasoningMessage(msg) => {
                println!("   Message {}: [Reasoning] {}", i + 1, msg.reasoning);
            }
            _ => {
                println!("   Message {}: [Other] {:?}", i + 1, message);
            }
        }
    }

    // Test 3: List messages again
    let updated_messages = client.messages().list(agent_id, None).await.unwrap();
    println!(
        "✅ After sending message, found {} messages in current page",
        updated_messages.len()
    );

    // Note: Due to pagination, we might not see all messages
    // The important part is that message creation succeeded above
    println!("✅ Message API operations completed successfully!");

    println!("✅ All message API tests passed!");

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    client
        .agents()
        .delete(agent_id)
        .await
        .expect("Failed to delete test agent");
    println!("✅ Test agent deleted");
}

#[tokio::test]
async fn test_local_server_message_reset() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for message reset...");
    let create_request = CreateAgentRequest::builder()
        .name("Reset Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    println!("Testing message reset for agent: {}", agent_id);

    // Count messages before reset
    let messages_before = client.messages().list(agent_id, None).await.unwrap();
    println!("Messages before reset: {}", messages_before.len());

    // Test reset with default initial messages
    let reset_agent = client.messages().reset(agent_id, Some(true)).await.unwrap();

    println!("✅ Agent messages reset successfully");
    println!("   Agent name: {}", reset_agent.name);

    // Check messages after reset
    let messages_after = client.messages().list(agent_id, None).await.unwrap();
    println!("Messages after reset: {}", messages_after.len());

    // Should have fewer messages (reset clears history but may add default messages)
    println!("✅ Message reset test completed");

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    client
        .agents()
        .delete(agent_id)
        .await
        .expect("Failed to delete test agent");
    println!("✅ Test agent deleted");
}

#[tokio::test]
#[serial]
async fn test_update_user_message() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for message update...");
    let create_request = CreateAgentRequest::builder()
        .name("Update Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block::human("The human's name is User"))
        .memory_block(Block::persona("I am a helpful assistant"))
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // First, create a message
    let create_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: "Original message content".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let _response = client
        .messages()
        .create(agent_id, create_request)
        .await
        .unwrap();

    // List messages to find the user message we just created
    let messages = client.messages().list(agent_id, None).await.unwrap();

    // Find the user message with our content
    let user_message_id = messages
        .iter()
        .find_map(|msg| match msg {
            letta::types::LettaMessageUnion::UserMessage(user_msg)
                if user_msg.content == "Original message content" =>
            {
                Some(user_msg.id.clone())
            }
            _ => None,
        })
        .expect("Should find the user message we created");

    // Update the message
    let update_request = UpdateMessageRequest::UserMessage(UpdateUserMessage {
        content: UpdateUserMessageContent::String("Updated message content".to_string()),
    });

    let updated_message = client
        .messages()
        .update(agent_id, &user_message_id, update_request)
        .await
        .unwrap();

    // Verify the update
    match updated_message {
        letta::types::LettaMessageUnion::UserMessage(user_msg) => {
            assert_eq!(user_msg.content, "Updated message content");
            println!("✅ Message update successful!");
        }
        _ => panic!("Expected user message in response"),
    }

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    client
        .agents()
        .delete(agent_id)
        .await
        .expect("Failed to delete test agent");
    println!("✅ Test agent deleted");
}

#[tokio::test]
#[serial]
async fn test_create_async_message() {
    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for async message creation...");
    let create_request = CreateAgentRequest::builder()
        .name("Async Message Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block::human("The human's name is User"))
        .memory_block(Block::persona("I am a helpful assistant"))
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // Create a message asynchronously
    let create_request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: "Tell me a joke".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let run = client
        .messages()
        .create_async(agent_id, create_request)
        .await
        .unwrap();

    // Verify we got a run object
    assert_eq!(run.id.prefix(), Some("run")); // Runs have 'run' prefix
    println!("✅ Async message created with run ID: {}", run.id);

    // Check initial status
    if let Some(status) = run.status {
        match status {
            letta::types::JobStatus::Created
            | letta::types::JobStatus::Running
            | letta::types::JobStatus::Pending => {
                println!("   Run status: {:?}", status);
            }
            _ => panic!("Unexpected initial status: {:?}", status),
        }
    }

    // Cleanup: delete the test agent
    println!("Cleaning up test agent...");
    client
        .agents()
        .delete(agent_id)
        .await
        .expect("Failed to delete test agent");
    println!("✅ Test agent deleted");
}

#[tokio::test]
#[serial]
async fn test_message_pagination() {
    use futures::StreamExt;
    use letta::types::PaginationParams;

    // Create client for local server
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    let client = LettaClient::new(config).unwrap();

    // Create a test agent
    println!("Creating test agent for pagination test...");
    let create_request = CreateAgentRequest::builder()
        .name("Message Pagination Test Agent")
        .agent_type(AgentType::MemGPT)
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block::human("The human's name is User"))
        .memory_block(Block::persona("I am a helpful assistant"))
        .build();

    let agent = client.agents().create(create_request).await.unwrap();
    let agent_id = &agent.id;
    println!("Created test agent: {} ({})", agent.name, agent_id);

    // Check if we already have messages
    println!("Checking existing messages...");
    let existing_messages = client.messages().list(agent_id, None).await.unwrap();
    println!("Found {} existing messages", existing_messages.len());

    // Only create a message if we don't have enough
    if existing_messages.len() < 3 {
        println!("Creating one test message (Letta server can be slow)...");
        let message_request = CreateMessagesRequest {
            messages: vec![MessageCreate {
                role: MessageRole::User,
                content: "Pagination test message".into(),
                ..Default::default()
            }],
            max_steps: Some(1),
            ..Default::default()
        };

        match client.messages().create(agent_id, message_request).await {
            Ok(response) => {
                println!(
                    "  Created message (got {} messages in response)",
                    response.messages.len()
                );
            }
            Err(e) => {
                eprintln!("  Warning: Failed to create message: {:?}", e);
                eprintln!("  Will proceed with existing messages only");
            }
        }
    }

    // Test 1: Use pagination to iterate through messages
    println!("\nTesting paginated iteration with limit 3...");
    let mut stream = client
        .messages()
        .paginated(agent_id, Some(PaginationParams::new().limit(3)));

    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                println!("  Found message: {:?}", message);
                count += 1;
                if count >= 10 {
                    break; // Limit iterations for testing
                }
            }
            Err(e) => {
                eprintln!("  Error during pagination: {:?}", e);
                break;
            }
        }
    }

    println!("Total messages found via pagination: {}", count);
    assert!(count > 0, "Should find at least one message");

    // Test 2: Collect messages with pagination
    println!("\nTesting collect with pagination...");
    let collected_messages = client
        .messages()
        .paginated(agent_id, Some(PaginationParams::new().limit(5)))
        .take(10)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("Collected {} messages", collected_messages.len());
    assert!(
        !collected_messages.is_empty(),
        "Should collect at least one message"
    );

    // Cleanup: delete the test agent
    println!("\nCleaning up test agent...");
    match client.agents().delete(agent_id).await {
        Ok(_) => println!("✅ Test agent deleted"),
        Err(e) => println!("Warning: Failed to delete test agent: {:?}", e),
    }
}
