//! Integration tests for the Batch API.

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;
use std::str::FromStr;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
async fn test_list_batch_runs() -> LettaResult<()> {
    let client = get_test_client()?;

    // List all batch runs
    let batches = client.batch().list().await?;

    println!("Found {} batch runs", batches.len());
    for batch in batches.iter().take(3) {
        println!(
            "Batch: {} - Status: {} - Type: {:?}",
            batch.id, batch.status, batch.job_type
        );
    }

    Ok(())
}

#[tokio::test]
#[ignore = "Batch API may not be implemented on all servers - requires LETTA_ENABLE_BATCH_JOB_POLLING=true"]
async fn test_create_batch() -> LettaResult<()> {
    let client = get_test_client()?;

    // First, we need to have an agent to send messages to
    // For this test, we'll assume we have an agent ID or create one
    let agents = client.agents().list(None).await?;

    if agents.is_empty() {
        println!("No agents available for batch test, skipping");
        return Ok(());
    }

    let agent_id = agents[0].id.clone();
    println!("Using agent {} for batch test", agent_id);

    // Create a batch request
    let batch_request = CreateBatchRequest {
        requests: vec![
            BatchMessageRequest {
                messages: vec![BatchMessage::user("Hello from batch test 1")],
                agent_id: agent_id.clone(),
            },
            BatchMessageRequest {
                messages: vec![BatchMessage::user("Hello from batch test 2")],
                agent_id: agent_id.clone(),
            },
        ],
        callback_url: None,
    };

    let batch = client.batch().create(batch_request).await?;

    println!("Created batch: {} with status: {}", batch.id, batch.status);
    assert_eq!(batch.job_type, BatchJobType::Batch);

    // Retrieve the batch to verify it exists
    let retrieved = client.batch().get(&batch.id).await?;
    assert_eq!(retrieved.id, batch.id);

    Ok(())
}

#[tokio::test]
async fn test_batch_messages() -> LettaResult<()> {
    let client = get_test_client()?;

    // Get existing batches
    let batches = client.batch().list().await?;

    if batches.is_empty() {
        println!("No batches available to test messages, skipping");
        return Ok(());
    }

    let batch_id = &batches[0].id;
    println!("Getting messages for batch: {}", batch_id);

    // List messages with pagination
    let params = ListBatchMessagesParams {
        limit: Some(10),
        sort_descending: Some(true),
        ..Default::default()
    };

    let response = client.batch().list_messages(batch_id, Some(params)).await?;

    println!("Found {} messages in batch", response.messages.len());
    for msg in response.messages.iter().take(3) {
        println!("Message: {:?} - Role: {:?}", msg.id, msg.role);
    }

    Ok(())
}

#[tokio::test]
async fn test_cancel_batch() -> LettaResult<()> {
    let client = get_test_client()?;

    // We can't test cancel without an active batch
    // This test will verify the endpoint works with a fake ID
    let fake_batch_id = LettaId::from_str("batch-00000000-0000-0000-0000-000000000000").unwrap();

    let result = client.batch().cancel(&fake_batch_id).await;

    // Expect this to fail with 404
    match result {
        Err(letta::error::LettaError::NotFound { resource_type, .. }) => {
            println!("Expected NotFound error for non-existent batch");
            println!("Resource type: {}", resource_type);
            // The server returns "Run" for this endpoint
            assert!(
                resource_type == "BatchRun"
                    || resource_type == "Batch"
                    || resource_type == "Job"
                    || resource_type == "BATCH"
                    || resource_type == "Run"
            );
            Ok(())
        }
        Err(letta::error::LettaError::Api { status: 404, .. }) => {
            println!("Got 404 API error for non-existent batch");
            Ok(())
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
        Ok(batch) => {
            panic!("Expected error but got batch: {:?}", batch);
        }
    }
}
