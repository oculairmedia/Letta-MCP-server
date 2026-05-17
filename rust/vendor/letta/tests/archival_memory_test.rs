//! Integration tests for archival memory API endpoints.

use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::agent::CreateAgentRequest;
use letta::types::memory::{ArchivalMemoryQueryParams, Block, CreateArchivalMemoryRequest};
use letta::{LettaClient, LettaId};
use serial_test::serial;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Create a test agent for archival memory operations.
async fn create_test_agent(client: &LettaClient) -> LettaResult<LettaId> {
    let request = CreateAgentRequest::builder()
        .name("Test Archival Agent")
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
async fn test_create_archival_memory() -> LettaResult<()> {
    let client = create_test_client()?;

    let agent_id = create_test_agent(&client).await?;

    // Create archival memory
    let request = CreateArchivalMemoryRequest {
        text: "The user mentioned they love hiking in the mountains.".to_string(),
    };

    let passages = client
        .memory()
        .create_archival_memory(&agent_id, request)
        .await?;

    // Verify response (returns array)
    assert!(!passages.is_empty());
    let passage = &passages[0];
    assert_eq!(
        passage.text,
        "The user mentioned they love hiking in the mountains."
    );
    assert_eq!(passage.agent_id.as_ref(), Some(&agent_id));

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_archival_memory() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Create some archival memories
    let memories = vec![
        "User's favorite color is blue.",
        "User works as a software engineer.",
        "User has two cats named Whiskers and Shadow.",
    ];

    for text in &memories {
        let request = CreateArchivalMemoryRequest {
            text: text.to_string(),
        };
        client
            .memory()
            .create_archival_memory(&agent_id, request)
            .await?;
    }

    // List all archival memories
    let passages = client
        .memory()
        .list_archival_memory(&agent_id, None)
        .await?;

    // Verify we got all memories
    assert!(passages.len() >= memories.len());

    // Test with search
    let search_params = ArchivalMemoryQueryParams {
        search: Some("cats".to_string()),
        limit: Some(10),
        ..Default::default()
    };

    let search_results = client
        .memory()
        .list_archival_memory(&agent_id, Some(search_params))
        .await?;

    // Should find the memory about cats
    assert!(!search_results.is_empty());
    assert!(search_results
        .iter()
        .any(|p| p.text.contains("cats") || p.text.contains("Whiskers")));

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_delete_archival_memory() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Create archival memory
    let request = CreateArchivalMemoryRequest {
        text: "Temporary memory to be deleted.".to_string(),
    };

    let passages = client
        .memory()
        .create_archival_memory(&agent_id, request)
        .await?;

    let passage_id = &passages[0].id;

    // Delete the memory
    client
        .memory()
        .delete_archival_memory(&agent_id, passage_id)
        .await?;

    // Verify it's gone by listing all memories
    let remaining = client
        .memory()
        .list_archival_memory(&agent_id, None)
        .await?;

    // Should not find the deleted memory
    assert!(!remaining.iter().any(|p| &p.id == passage_id));

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_archival_memory_pagination() -> LettaResult<()> {
    let client = create_test_client()?;
    let agent_id = create_test_agent(&client).await?;

    // Create multiple archival memories
    for i in 0..5 {
        let request = CreateArchivalMemoryRequest {
            text: format!("Memory number {}", i),
        };
        client
            .memory()
            .create_archival_memory(&agent_id, request)
            .await?;
    }

    // Test pagination with limit
    let params = ArchivalMemoryQueryParams {
        limit: Some(2),
        ..Default::default()
    };

    let first_page = client
        .memory()
        .list_archival_memory(&agent_id, Some(params))
        .await?;

    assert_eq!(first_page.len(), 2);

    // Clean up
    client.agents().delete(&agent_id).await?;
    Ok(())
}
