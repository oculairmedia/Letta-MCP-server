//! Integration tests for the Voice API (beta).

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;
use serde_json::json;
use std::str::FromStr;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
#[ignore = "Voice API requires OPENAI_API_KEY to be configured"]
async fn test_voice_chat_completions() -> LettaResult<()> {
    let client = get_test_client()?;

    // We need an agent ID to test with
    let agents = client.agents().list(None).await?;

    if agents.is_empty() {
        println!("No agents available for voice test, creating one");
        // Create a test agent
        let request = CreateAgentRequest::builder()
            .name("Voice Test Agent")
            .agent_type(AgentType::MemGPT)
            .model("letta/letta-free")
            .embedding("letta/letta-free")
            .build();

        let agent = client.agents().create(request).await?;
        println!("Created test agent: {}", agent.id);

        // Test voice endpoint with the created agent
        let result = test_voice_endpoint(&client, &agent.id).await;

        // Clean up
        client.agents().delete(&agent.id).await?;

        result
    } else {
        // Use existing agent
        test_voice_endpoint(&client, &agents[0].id).await
    }
}

async fn test_voice_endpoint(client: &LettaClient, agent_id: &LettaId) -> LettaResult<()> {
    println!("Testing voice endpoint with agent: {}", agent_id);

    // Create a test request - since the structure isn't documented,
    // we'll use a generic JSON object
    let request = VoiceChatCompletionRequest {
        data: json!({
            "message": "Hello from voice test",
            "voice_settings": {
                "language": "en",
                "speed": 1.0
            }
        }),
    };

    // Test without user_id first
    let response = client
        .voice()
        .create_voice_chat_completions(agent_id, request.clone(), None)
        .await?;
    println!(
        "Voice response received without user_id: {:?}",
        response.data
    );

    // Also test with user_id
    let response = client
        .voice()
        .create_voice_chat_completions(agent_id, request, Some("test-user-123"))
        .await?;
    println!("Voice response received: {:?}", response.data);
    // Since this is a beta endpoint, we can't make strong assertions
    // about the response structure
    Ok(())
}

#[tokio::test]
async fn test_voice_with_invalid_agent() -> LettaResult<()> {
    let client = get_test_client()?;

    let fake_agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap();

    let request = VoiceChatCompletionRequest {
        data: json!({
            "message": "Test message"
        }),
    };

    let result = client
        .voice()
        .create_voice_chat_completions(&fake_agent_id, request, None)
        .await;

    // We expect this to fail
    match result {
        Err(_) => {
            println!("Expected error for invalid agent ID");
            Ok(())
        }
        Ok(_) => {
            panic!("Expected error but got success for invalid agent ID");
        }
    }
}
