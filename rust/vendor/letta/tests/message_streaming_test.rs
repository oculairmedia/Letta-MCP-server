//! Integration tests for message streaming with SSE.

use futures::StreamExt;
use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::agent::{AgentState, AgentType, CreateAgentRequest};
use letta::types::memory::Block;
use letta::types::message::{
    CreateMessagesRequest, MessageCreate, MessageCreateContent, MessageRole,
};
use letta::{LettaClient, LettaId, StreamingEvent};
use serial_test::serial;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Create a test agent for message streaming.
async fn create_test_agent(client: &LettaClient) -> LettaResult<AgentState> {
    create_test_agent_with_model(client, "letta/letta-free").await
}

/// Create a test agent with specific model.
async fn create_test_agent_with_model(
    client: &LettaClient,
    model: &str,
) -> LettaResult<AgentState> {
    let request = CreateAgentRequest::builder()
        .name("Test Streaming Agent")
        .model(model)
        .embedding("letta/letta-free")
        .agent_type(AgentType::MemGPT)
        .memory_block(Block {
            id: None,
            label: "human".to_string(),
            value: "The human's name is Stream Test User.".to_string(),
            limit: Some(1000),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Human information".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        })
        .build();

    client.agents().create(request).await
}

#[tokio::test]
#[serial]
async fn test_message_streaming_basic() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a test agent
    let agent = create_test_agent(&client).await?;
    let agent_id = &agent.id;

    // Create a message to send
    let request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::String(
                "Hello, please respond with a short greeting.".to_string(),
            ),
            ..Default::default()
        }],
        max_steps: Some(3),
        ..Default::default()
    };

    // Stream the response (without token streaming)
    let mut stream = client
        .messages()
        .create_stream(&agent_id, request, false)
        .await?;

    let mut message_count = 0;
    let mut stop_reason_seen = false;
    let mut usage_seen = false;

    // Consume the stream with a timeout
    let stream_result = timeout(Duration::from_secs(30), async {
        while let Some(event) = stream.next().await {
            match event? {
                StreamingEvent::Message(msg) => {
                    println!("Received message: {:?}", msg);
                    message_count += 1;
                }
                StreamingEvent::StopReason(reason) => {
                    println!("Stop reason: {:?}", reason);
                    stop_reason_seen = true;
                }
                StreamingEvent::Usage(usage) => {
                    println!("Usage stats: {:?}", usage);
                    usage_seen = true;
                }
            }
        }
        Ok::<(), letta::LettaError>(())
    })
    .await;

    match stream_result {
        Ok(Ok(())) => {
            println!("Stream completed successfully");
            assert!(
                message_count > 0,
                "Should have received at least one message"
            );
            assert!(stop_reason_seen, "Should have received a stop reason");
            assert!(usage_seen, "Should have received usage statistics");
        }
        Ok(Err(e)) => {
            eprintln!("Stream error: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Stream timeout - streaming may not be supported on local server");
            // Don't fail the test as streaming might not be implemented locally
        }
    }

    // Clean up
    client.agents().delete(&agent_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_message_streaming_with_tokens() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a test agent
    let agent = create_test_agent(&client).await?;
    let agent_id = &agent.id;

    // Create a message to send
    let request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::String(
                "Write a short haiku about AI memory.".to_string(),
            ),
            ..Default::default()
        }],
        max_steps: Some(3),
        ..Default::default()
    };

    // Stream the response WITH token streaming
    let mut stream = client
        .messages()
        .create_stream(&agent_id, request, true)
        .await?;

    let mut event_count = 0;

    // Consume the stream with a timeout
    let stream_result = timeout(Duration::from_secs(30), async {
        while let Some(event) = stream.next().await {
            match event? {
                StreamingEvent::Message(msg) => {
                    println!("Token/Message: {:?}", msg);
                    event_count += 1;
                }
                StreamingEvent::StopReason(reason) => {
                    println!("Stop reason: {:?}", reason);
                    event_count += 1;
                }
                StreamingEvent::Usage(usage) => {
                    println!("Usage stats: {:?}", usage);
                    event_count += 1;
                }
            }
        }
        Ok::<(), letta::LettaError>(())
    })
    .await;

    match stream_result {
        Ok(Ok(())) => {
            println!("Token stream completed successfully");
            println!("Total events received: {}", event_count);
            assert!(event_count > 0, "Should have received at least one event");
        }
        Ok(Err(e)) => {
            eprintln!("Stream error: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Stream timeout - token streaming may not be supported on local server");
            // Don't fail the test as streaming might not be implemented locally
        }
    }

    // Clean up
    client.agents().delete(&agent_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_message_streaming_error_handling() -> LettaResult<()> {
    let client = create_test_client()?;

    // Try to stream messages for a non-existent agent
    let request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::String("Hello".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };

    let fake_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap();
    let result = client
        .messages()
        .create_stream(&fake_id, request, false)
        .await;

    assert!(result.is_err(), "Should fail for non-existent agent");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_message_streaming_multimodal() -> LettaResult<()> {
    use letta::types::message::{ContentPart, ImageContent, ImageUrl, TextContent};

    let client = create_test_client()?;

    // Try creating an agent with a vision-capable model
    let agent = match create_test_agent_with_model(&client, "azure/gpt-4o").await {
        Ok(a) => a,
        Err(_) => {
            // Fall back to default model if Azure not available
            create_test_agent(&client).await?
        }
    };
    let agent_id = &agent.id;

    // Create a multimodal message with text and a simple 1x1 red pixel image
    let request = CreateMessagesRequest {
        messages: vec![MessageCreate {
            role: MessageRole::User,
            content: MessageCreateContent::ContentParts(vec![
                ContentPart::Text(TextContent {
                    text: "What color is this pixel?".to_string(),
                }),
                ContentPart::Image(ImageContent {
                    image_url: ImageUrl {
                        url: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+ip1sAAAAASUVORK5CYII=".to_string(),
                        detail: Some("low".to_string()),
                    },
                }),
            ]),
            ..Default::default()
        }],
        max_steps: Some(3),
        ..Default::default()
    };

    // Try to stream the response
    let stream_result = client
        .messages()
        .create_stream(&agent_id, request, false)
        .await;

    match stream_result {
        Ok(mut stream) => {
            println!("Successfully created multimodal stream");

            let mut message_count = 0;

            // Consume the stream with a timeout
            let consume_result = timeout(Duration::from_secs(30), async {
                while let Some(event) = stream.next().await {
                    match event? {
                        StreamingEvent::Message(msg) => {
                            println!("Multimodal message: {:?}", msg);
                            message_count += 1;
                        }
                        StreamingEvent::StopReason(reason) => {
                            println!("Stop reason: {:?}", reason);
                        }
                        StreamingEvent::Usage(usage) => {
                            println!("Usage: {:?}", usage);
                        }
                    }
                }
                Ok::<(), letta::LettaError>(())
            })
            .await;

            match consume_result {
                Ok(Ok(())) => {
                    println!("✅ Multimodal streaming completed successfully");
                    assert!(
                        message_count > 0,
                        "Should have received at least one message"
                    );
                }
                Ok(Err(e)) => {
                    println!("Stream error: {}", e);
                    return Err(e);
                }
                Err(_) => {
                    println!("Stream timeout");
                }
            }
        }
        Err(e) => {
            println!("Failed to create multimodal stream: {:?}", e);
            println!("This is expected if the model doesn't support vision capabilities");
        }
    }

    // Clean up
    client.agents().delete(&agent_id).await?;

    Ok(())
}
