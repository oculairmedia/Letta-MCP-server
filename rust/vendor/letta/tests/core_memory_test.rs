//! Integration tests for core memory API endpoints.

use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::agent::CreateAgentRequest;
use letta::types::memory::{Block, UpdateMemoryBlockRequest};
use letta::{LettaClient, LettaId};
use serial_test::serial;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Create a test agent for memory operations.
async fn create_test_agent(client: &LettaClient) -> LettaResult<LettaId> {
    let request = CreateAgentRequest::builder()
        .name("Test Memory Agent")
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block {
            id: None,
            label: "human".to_string(),
            value: "The human's name is Test User.".to_string(),
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
        .memory_block(Block {
            id: None,
            label: "persona".to_string(),
            value: "I am a helpful test assistant.".to_string(),
            limit: Some(500),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Agent persona".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        })
        .build();

    let agent = client.agents().create(request).await?;
    Ok(agent.id)
}

#[tokio::test]
#[serial]
async fn test_get_core_memory() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Get core memory
    let memory = client.memory().get_core_memory(&agent_id).await?;

    // Verify memory structure
    assert!(!memory.blocks.is_empty());
    assert!(memory.blocks.len() >= 2);

    // Check for human and persona blocks
    let has_human = memory.blocks.iter().any(|b| b.label == "human");
    let has_persona = memory.blocks.iter().any(|b| b.label == "persona");
    assert!(has_human);
    assert!(has_persona);

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_core_memory_blocks() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // List all memory blocks
    let blocks = client.memory().list_core_memory_blocks(&agent_id).await?;

    // Verify blocks
    assert!(!blocks.is_empty());
    assert!(blocks.len() >= 2);

    // Check block labels
    let labels: Vec<String> = blocks.iter().map(|b| b.label.clone()).collect();
    assert!(labels.contains(&"human".to_string()));
    assert!(labels.contains(&"persona".to_string()));

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_core_memory_block() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Get specific block by label
    let human_block = client
        .memory()
        .get_core_memory_block(&agent_id, "human")
        .await?;

    // Verify block
    assert_eq!(human_block.label, "human");
    assert!(human_block.value.contains("Test User"));
    assert_eq!(human_block.limit, Some(1000));

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_update_core_memory_block() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Update human block
    let update_request = UpdateMemoryBlockRequest {
        value: Some("The human's name is Updated User.".to_string()),
        limit: Some(1500),
        description: Some("Updated human information".to_string()),
        label: None,
        name: None,
        preserve_on_migration: None,
        read_only: None,
        metadata: None,
    };

    let updated_block = client
        .memory()
        .update_core_memory_block(&agent_id, "human", update_request)
        .await?;

    // Verify update
    assert_eq!(updated_block.label, "human");
    assert!(updated_block.value.contains("Updated User"));
    assert_eq!(updated_block.limit, Some(1500));
    assert_eq!(
        updated_block.description,
        Some("Updated human information".to_string())
    );

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_attach_detach_memory_block() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Get a block to work with
    let blocks = client.memory().list_core_memory_blocks(&agent_id).await?;
    if let Some(block) = blocks.first() {
        if let Some(block_id) = &block.id {
            // Detach block
            let agent_after_detach = client
                .memory()
                .detach_memory_block(&agent_id, block_id)
                .await?;
            assert_eq!(agent_after_detach.id, agent_id);

            // Re-attach block
            let agent_after_attach = client
                .memory()
                .attach_memory_block(&agent_id, block_id)
                .await?;
            assert_eq!(agent_after_attach.id, agent_id);
        }
    }

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}
