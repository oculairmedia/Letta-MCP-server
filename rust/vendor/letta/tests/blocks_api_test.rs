//! Integration tests for blocks API endpoints.

use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::{Block, CreateBlockRequest, ListBlocksParams, Metadata, UpdateBlockRequest};
use letta::{LettaClient, LettaId};
use serial_test::serial;
use std::collections::HashMap;
use std::str::FromStr;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Create a test block for testing.
async fn create_test_block(client: &LettaClient, label: &str) -> LettaResult<Block> {
    let request = CreateBlockRequest {
        value: format!("Test block content for {}", label),
        label: label.to_string(),
        limit: Some(1000),
        name: Some(format!("test_block_{}", chrono::Utc::now().timestamp())),
        is_template: Some(false),
        preserve_on_migration: Some(true),
        read_only: Some(false),
        description: Some("Test block for integration testing".to_string()),
        metadata: None,
    };

    client.blocks().create(request).await
}

#[tokio::test]
#[serial]
async fn test_block_lifecycle() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a block
    let block = match create_test_block(&client, "test_lifecycle").await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to create block: {:?}", e);
            return Err(e);
        }
    };
    assert!(block.id.is_some());
    assert_eq!(block.label, "test_lifecycle");
    assert!(block.value.contains("Test block content"));

    let block_id = block.id.as_ref().unwrap();

    // Get the block
    let retrieved = client.blocks().get(block_id).await?;
    assert_eq!(retrieved.id, block.id);
    assert_eq!(retrieved.label, block.label);
    assert_eq!(retrieved.value, block.value);

    // Update the block
    let update_request = UpdateBlockRequest {
        value: Some("Updated block content".to_string()),
        limit: Some(1500),
        description: Some("Updated test block".to_string()),
        ..Default::default()
    };

    let updated = client.blocks().update(block_id, update_request).await?;
    assert_eq!(updated.id, block.id);
    assert_eq!(updated.value, "Updated block content");
    assert_eq!(updated.limit, Some(1500));
    assert_eq!(updated.description, Some("Updated test block".to_string()));

    // List blocks with filter to find our specific block
    let params = ListBlocksParams {
        label: Some("test_lifecycle".to_string()),
        ..Default::default()
    };
    let filtered_blocks = client.blocks().list(Some(params)).await?;
    println!(
        "Found {} blocks with label 'test_lifecycle'",
        filtered_blocks.len()
    );
    let found = filtered_blocks.iter().any(|b| b.id == block.id);
    assert!(found, "Created block should be in the filtered list");

    // Count blocks
    let count = client.blocks().count().await?;
    assert!(count > 0, "Should have at least one block");

    // Delete the block
    client.blocks().delete(block_id).await?;

    // Verify deletion
    let result = client.blocks().get(block_id).await;
    assert!(result.is_err(), "Block should be deleted");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_blocks_with_filters() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create blocks with different labels
    let block1 = create_test_block(&client, "filter_test_1").await?;
    let block2 = create_test_block(&client, "filter_test_2").await?;

    // Create a template block
    let template_request = CreateBlockRequest {
        value: "Template block content".to_string(),
        label: "template_test".to_string(),
        is_template: Some(true),
        ..Default::default()
    };
    let template_block = client.blocks().create(template_request).await?;

    // List all blocks
    let all_blocks = client.blocks().list(None).await?;
    assert!(all_blocks.len() >= 3);

    // Filter by label
    let params = ListBlocksParams {
        label: Some("filter_test_1".to_string()),
        ..Default::default()
    };
    let filtered = client.blocks().list(Some(params)).await?;
    let has_block1 = filtered.iter().any(|b| b.id == block1.id);
    let has_block2 = filtered.iter().any(|b| b.id == block2.id);
    assert!(has_block1, "Should find block with matching label");
    assert!(!has_block2, "Should not find block with different label");

    // Filter by templates only
    let params = ListBlocksParams {
        templates_only: Some(true),
        ..Default::default()
    };
    let templates = client.blocks().list(Some(params)).await?;
    let has_template = templates.iter().any(|b| b.id == template_block.id);
    assert!(has_template, "Should find template block");

    // Test limit
    let params = ListBlocksParams {
        limit: Some(2),
        ..Default::default()
    };
    let limited = client.blocks().list(Some(params)).await?;
    assert!(limited.len() <= 2, "Should respect limit parameter");

    // Clean up
    if let Some(id) = block1.id.as_ref() {
        client.blocks().delete(id).await.ok();
    }
    if let Some(id) = block2.id.as_ref() {
        client.blocks().delete(id).await.ok();
    }
    if let Some(id) = template_block.id.as_ref() {
        client.blocks().delete(id).await.ok();
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_block_metadata() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create block with metadata
    let mut metadata_map = HashMap::new();
    metadata_map.insert("project".to_string(), serde_json::json!("test_project"));
    metadata_map.insert("version".to_string(), serde_json::json!(1));
    metadata_map.insert("tags".to_string(), serde_json::json!(["test", "block"]));

    let metadata = Metadata { data: metadata_map };

    let request = CreateBlockRequest {
        value: "Block with metadata".to_string(),
        label: "metadata_test".to_string(),
        metadata: Some(metadata.clone()),
        ..Default::default()
    };

    let block = client.blocks().create(request).await?;
    let block_id = block.id.as_ref().unwrap();

    // Verify metadata was saved
    assert!(block.metadata.is_some());
    let saved_metadata = block.metadata.as_ref().unwrap();
    assert_eq!(
        saved_metadata.data.get("project"),
        metadata.data.get("project")
    );
    assert_eq!(
        saved_metadata.data.get("version"),
        metadata.data.get("version")
    );

    // Update metadata
    let mut new_metadata_map = HashMap::new();
    new_metadata_map.insert("project".to_string(), serde_json::json!("updated_project"));
    new_metadata_map.insert("status".to_string(), serde_json::json!("active"));

    let new_metadata = Metadata {
        data: new_metadata_map,
    };

    let update = UpdateBlockRequest {
        metadata: Some(new_metadata),
        ..Default::default()
    };

    let updated = client.blocks().update(block_id, update).await?;
    assert!(updated.metadata.is_some());
    let updated_metadata = updated.metadata.as_ref().unwrap();
    assert_eq!(
        updated_metadata.data.get("project"),
        Some(&serde_json::json!("updated_project"))
    );
    assert_eq!(
        updated_metadata.data.get("status"),
        Some(&serde_json::json!("active"))
    );

    // Clean up
    client.blocks().delete(block_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_block_error_handling() -> LettaResult<()> {
    let client = create_test_client()?;

    let fake_id = LettaId::from_str("block-00000000-0000-0000-0000-000000000000").unwrap();

    // Test getting non-existent block
    let result = client.blocks().get(&fake_id).await;
    assert!(result.is_err(), "Should fail to get non-existent block");

    // Test updating non-existent block
    let update = UpdateBlockRequest {
        value: Some("Update".to_string()),
        ..Default::default()
    };
    let result = client.blocks().update(&fake_id, update).await;
    assert!(result.is_err(), "Should fail to update non-existent block");

    // Test deleting non-existent block
    let result = client.blocks().delete(&fake_id).await;
    assert!(result.is_err(), "Should fail to delete non-existent block");

    // Test creating block with missing required fields
    let invalid_request = CreateBlockRequest {
        value: "".to_string(), // Empty value might be invalid
        label: "".to_string(), // Empty label might be invalid
        ..Default::default()
    };
    let _result = client.blocks().create(invalid_request).await;
    // Server might accept empty strings, so we just check it doesn't panic

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_readonly_and_template_blocks() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a read-only block
    let readonly_request = CreateBlockRequest {
        value: "This is a read-only block".to_string(),
        label: "readonly_test".to_string(),
        read_only: Some(true),
        ..Default::default()
    };

    let readonly_block = client.blocks().create(readonly_request).await?;
    assert_eq!(readonly_block.read_only, true);

    // Create a template block
    let template_request = CreateBlockRequest {
        value: "This is a template block".to_string(),
        label: "template_block".to_string(),
        is_template: Some(true),
        preserve_on_migration: Some(true),
        ..Default::default()
    };

    let template_block = client.blocks().create(template_request).await?;
    assert_eq!(template_block.is_template, true);
    assert_eq!(template_block.preserve_on_migration, true);

    // Clean up
    if let Some(id) = readonly_block.id.as_ref() {
        client.blocks().delete(id).await.ok();
    }
    if let Some(id) = template_block.id.as_ref() {
        client.blocks().delete(id).await.ok();
    }

    Ok(())
}
