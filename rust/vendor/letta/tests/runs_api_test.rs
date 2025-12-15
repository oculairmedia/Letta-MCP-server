//! Integration tests for the Runs API.

use letta::client::{ClientConfig, LettaClient};
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaClient {
    let config = ClientConfig::new("http://localhost:8283").unwrap();
    LettaClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_runs() {
    let client = get_test_client();

    // List runs (might be empty)
    let runs = client
        .runs()
        .list(&[])
        .await
        .expect("this should return something");
    println!("Found {} runs", runs.len());
    for run in runs.iter().take(5) {
        println!("Run: {:?} - Status: {:?}", run.id, run.status);
    }
}

#[tokio::test]
async fn test_create_and_get_run() {
    let client = get_test_client();

    // First, we need an agent to create runs for
    let agent_result = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Test Agent for Runs".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are a test agent for runs testing.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await;

    let agent = agent_result.expect("Failed to create agent");

    // Create a run by sending messages asynchronously
    let run_result = client
        .messages()
        .create_async(
            &agent.id,
            CreateMessagesRequest {
                messages: vec![MessageCreate {
                    role: MessageRole::User,
                    content: "Hello from runs test!".into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await;

    let run = run_result.expect("Failed to create async run");

    println!("Created run: {:?}", run.id);
    println!("Run status: {:?}", run.status);

    // Try to get the run details
    let fetched_run = client.runs().get(&run.id).await.expect("Failed to get run");
    println!("Fetched run: {:?}", fetched_run.id);
    assert_eq!(run.id, fetched_run.id);

    // Try to get messages for the run
    let messages = client
        .runs()
        .get_messages(&run.id)
        .await
        .expect("Failed to get run messages");
    println!("Run has {} messages", messages.len());

    // Try to get steps for the run
    let steps = client
        .runs()
        .get_steps(&run.id)
        .await
        .expect("Failed to get run steps");
    println!("Run has {} steps", steps.len());

    // Clean up
    let _ = client.agents().delete(&agent.id).await;
}

#[tokio::test]
async fn test_list_active_runs() {
    let client = get_test_client();

    // Create a test agent
    let agent_result = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Test Agent for Run Listing".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            agent_type: Some(AgentType::MemGPT),
            system: Some("You are a test agent for run listing.".to_string()),
            tools: Some(vec!["send_message".to_string()]),
            ..Default::default()
        })
        .await;

    let agent = agent_result.expect("Failed to create agent");

    // Create a few runs
    for i in 0..3 {
        let _ = client
            .messages()
            .create_async(
                &agent.id,
                CreateMessagesRequest {
                    messages: vec![MessageCreate {
                        role: MessageRole::User,
                        content: format!("Test message {}", i).into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            )
            .await;
    }

    // List runs for this agent
    let runs = client
        .runs()
        .list_active(&[agent.id.clone()])
        .await
        .expect("Failed to list active runs");

    println!("Agent {} has {} runs", agent.id, runs.len());
    assert!(runs.len() >= 3, "Should have at least 3 runs");

    // Clean up
    let _ = client.agents().delete(&agent.id).await;
}
