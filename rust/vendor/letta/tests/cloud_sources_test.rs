//! Cloud API integration tests for sources endpoints.

use bytes::Bytes;
use letta::auth::AuthConfig;
use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::agent::{AgentState, CreateAgentRequest};
use letta::types::memory::Block;
use letta::types::source::{CreateSourceRequest, Source, UpdateSourceRequest};
use letta::LettaClient;
use serial_test::serial;
use std::time::Duration;
use tokio::time::sleep;

/// Create a test client for the cloud API.
/// Requires LETTA_API_KEY environment variable to be set.
fn create_cloud_client() -> LettaResult<LettaClient> {
    let api_key = std::env::var("LETTA_API_KEY")
        .expect("LETTA_API_KEY environment variable must be set for cloud tests");

    ClientBuilder::new()
        .base_url("https://api.letta.com")
        .auth(AuthConfig::bearer(api_key))
        .build()
}

/// Create a test agent for cloud sources operations.
async fn create_test_agent(client: &LettaClient) -> LettaResult<AgentState> {
    let request = CreateAgentRequest::builder()
        .name("Cloud Test Sources Agent")
        .model("letta/letta-free")
        .embedding("letta/letta-free")
        .memory_block(Block {
            id: None,
            label: "human".to_string(),
            value: "The human's name is Cloud Test User.".to_string(),
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

/// Create a test source with unique name for cloud.
async fn create_test_source(client: &LettaClient, base_name: &str) -> LettaResult<Source> {
    let unique_name = format!(
        "cloud_{}_{}",
        base_name,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );

    let request = CreateSourceRequest {
        name: unique_name,
        embedding: Some("letta/letta-free".to_string()),
        embedding_chunk_size: Some(300),
        embedding_config: None,
        description: Some(format!("Cloud test source: {}", base_name)),
        instructions: Some("Use this source for cloud testing purposes".to_string()),
        metadata: None,
    };

    client.sources().create(request).await
}

#[tokio::test]
#[serial]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_source_lifecycle() -> LettaResult<()> {
    let client = create_cloud_client()?;

    // Create a source
    let source = create_test_source(&client, "lifecycle_test").await?;
    println!("Created cloud source: {:?}", source.id);
    assert!(source.id.is_some());
    assert!(source.name.starts_with("cloud_lifecycle_test_"));

    let source_id = source.id.as_ref().unwrap();

    // Get the source
    let retrieved = client.sources().get(source_id).await?;
    assert_eq!(retrieved.id, source.id);
    assert_eq!(retrieved.name, source.name);

    // Update the source
    let update_request = UpdateSourceRequest {
        name: None,
        description: Some("Cloud updated description".to_string()),
        instructions: Some("Cloud updated instructions".to_string()),
        metadata: None,
        embedding_config: None,
    };

    let updated = client.sources().update(source_id, update_request).await?;
    assert_eq!(updated.id, source.id);
    assert_eq!(
        updated.description,
        Some("Cloud updated description".to_string())
    );

    // List sources
    let sources = client.sources().list().await?;
    let found = sources.iter().any(|s| s.id == source.id);
    assert!(found, "Created source should be in the list");

    // Count sources
    let count = client.sources().count().await?;
    println!("Cloud sources count: {}", count);
    assert!(count > 0, "Should have at least one source");

    // Get by name
    let id_by_name = client.sources().get_by_name(&source.name).await?;
    assert_eq!(id_by_name, source_id.to_string());

    // Delete the source
    let _ = client.sources().delete(source_id).await?;
    println!("Deleted cloud source: {}", source_id);

    // Verify deletion
    let sources_after = client.sources().list().await?;
    let still_exists = sources_after.iter().any(|s| s.id == source.id);
    assert!(!still_exists, "Source should be deleted");

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_file_upload_behavior() -> LettaResult<()> {
    let client = create_cloud_client()?;

    // Create a source
    let source = create_test_source(&client, "file_behavior_test").await?;
    let source_id = source.id.as_ref().unwrap();
    println!("Created cloud source for file test: {}", source_id);

    // Upload a text file
    let file_content =
        b"This is a cloud test document.\nTesting file upload behavior.\nOn the cloud API.";
    let file_name = format!("cloud_test_doc_{}.txt", chrono::Utc::now().timestamp());

    println!("Uploading file: {}", file_name);
    let upload_result = client
        .sources()
        .upload_file(
            source_id,
            file_name.clone(),
            Bytes::from(file_content.to_vec()),
            Some("text/plain".to_string()),
        )
        .await?;

    use letta::types::source::FileUploadResponse;

    match upload_result {
        FileUploadResponse::Job(job) => {
            println!("Cloud API returned a job response");
            println!("Job status: {}", job.status);
            assert!(job.metadata.is_some());
            let job_metadata = job.metadata.as_ref().unwrap();
            println!("Job metadata filename: {}", job_metadata.filename);
            println!("Original filename: {}", file_name);

            // Check if filename was modified
            if job_metadata.filename != file_name {
                println!("Cloud API modified the filename!");
            }

            // Wait for file to appear
            sleep(Duration::from_secs(2)).await;

            // List files to find the uploaded one
            let files = client.sources().list_files(source_id, None).await?;
            println!("Files found: {}", files.len());
            for f in &files {
                println!("  File: {:?} (name: {:?})", f.id, f.file_name);
            }

            // Find by the job filename
            let file = files
                .into_iter()
                .find(|f| f.file_name.as_ref() == Some(&job_metadata.filename));

            if let Some(file) = file {
                println!("Found uploaded file with ID: {:?}", file.id);

                // Test file deletion
                let file_id = file.id.as_ref().unwrap();
                client.sources().delete_file(source_id, file_id).await?;
                println!("Successfully deleted file");
            } else {
                println!("WARNING: Could not find uploaded file in list");
            }
        }
        FileUploadResponse::FileMetadata(file) => {
            println!("Cloud API returned file metadata directly!");
            println!("File ID: {:?}", file.id);
            println!("File name: {:?}", file.file_name);
            println!("Processing status: {:?}", file.processing_status);

            if let Some(file_id) = &file.id {
                // Test file deletion
                client.sources().delete_file(source_id, file_id).await?;
                println!("Successfully deleted file");
            }
        }
    }

    // Clean up
    client.sources().delete(source_id).await?;
    println!("Cleaned up source");

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_agent_sources() -> LettaResult<()> {
    let client = create_cloud_client()?;

    // Create an agent and sources
    let agent = create_test_agent(&client).await?;
    let agent_id = &agent.id;
    println!("Created cloud agent: {}", agent_id);

    let source1 = create_test_source(&client, "cloud_agent_source_1").await?;
    let source2 = create_test_source(&client, "cloud_agent_source_2").await?;

    let source1_id = source1.id.as_ref().unwrap();
    let source2_id = source2.id.as_ref().unwrap();
    println!("Created sources: {} and {}", source1_id, source2_id);

    // Get the sources API
    let sources_api = client.sources();

    // Initially, agent should have no sources
    let initial_sources = sources_api.agent_sources(agent_id.clone()).list().await?;
    println!("Initial agent sources: {}", initial_sources.len());

    // Attach first source
    let updated_agent = sources_api
        .agent_sources(agent_id.clone())
        .attach(source1_id)
        .await?;
    assert_eq!(updated_agent.id, agent.id);
    println!("Attached source 1");

    // Verify source is attached
    let sources_after_attach = sources_api.agent_sources(agent_id.clone()).list().await?;
    println!("Sources after attach: {}", sources_after_attach.len());
    assert!(sources_after_attach.iter().any(|s| s.id == source1.id));

    // Detach source
    let _ = sources_api
        .agent_sources(agent_id.clone())
        .detach(source1_id)
        .await?;
    println!("Detached source 1");

    // Verify detachment
    let final_sources = sources_api.agent_sources(agent_id.clone()).list().await?;
    println!("Final sources: {}", final_sources.len());
    assert!(!final_sources.iter().any(|s| s.id == source1.id));

    // Clean up
    client.agents().delete(agent_id).await?;
    client.sources().delete(source1_id).await?;
    client.sources().delete(source2_id).await?;
    println!("Cleaned up all resources");

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_passages_behavior() -> LettaResult<()> {
    let client = create_cloud_client()?;

    // Create a source
    let source = create_test_source(&client, "passages_test").await?;
    let source_id = source.id.as_ref().unwrap();
    println!("Created cloud source for passages test: {}", source_id);

    // Upload a file that will create passages
    let file_content = b"This is the first paragraph for cloud testing.\n\nThis is the second paragraph with different content.\n\nAnd this is the third paragraph.";
    let file_name = format!("cloud_passages_test_{}.txt", chrono::Utc::now().timestamp());

    let upload_job = client
        .sources()
        .upload_file(
            source_id,
            file_name.clone(),
            Bytes::from(file_content.to_vec()),
            Some("text/plain".to_string()),
        )
        .await?;

    use letta::types::source::FileUploadResponse;

    let actual_filename = match upload_job {
        FileUploadResponse::Job(job) => job.metadata.as_ref().unwrap().filename.clone(),
        FileUploadResponse::FileMetadata(file) => {
            file.file_name.unwrap_or_else(|| "unknown.txt".to_string())
        }
    };
    println!("Uploaded file with name: {}", actual_filename);

    // Wait longer for cloud processing
    println!("Waiting for cloud processing...");
    sleep(Duration::from_secs(5)).await;

    // Get file metadata
    let files = client.sources().list_files(source_id, None).await?;
    let file = files
        .into_iter()
        .find(|f| f.file_name.as_ref() == Some(&actual_filename));

    if let Some(file) = file {
        let file_id = file.id.as_ref().unwrap();

        // Check processing status
        let file_status = client.sources().get_file(source_id, file_id, None).await?;

        println!(
            "File processing status: {:?}",
            file_status.processing_status
        );

        // List passages
        let passages = client.sources().list_passages(source_id, None).await?;

        println!("Passages found: {}", passages.len());
        for (i, passage) in passages.iter().take(3).enumerate() {
            println!("  Passage {}: {} chars", i + 1, passage.text.len());
        }
    } else {
        println!("WARNING: Could not find uploaded file");
    }

    // Clean up
    client.sources().delete(source_id).await?;
    println!("Cleaned up source");

    Ok(())
}

/// Run this test with: cargo test test_cloud_sources_quick -- --ignored
#[tokio::test]
#[ignore = "Requires LETTA_API_KEY environment variable and makes real API calls"]
async fn test_cloud_sources_quick() -> LettaResult<()> {
    println!("\n=== Cloud Sources API Quick Test ===\n");

    let client = create_cloud_client()?;

    // Quick test: List sources and count
    let sources = client.sources().list().await?;
    println!("Found {} sources in cloud", sources.len());

    let count = client.sources().count().await?;
    println!("Count endpoint reports: {}", count);

    // Create and immediately delete a test source
    let source = create_test_source(&client, "quick_test").await?;
    println!("Created source: {}", source.name);

    if let Some(id) = source.id.as_ref() {
        client.sources().delete(id).await?;
        println!("Deleted source successfully");
    }

    println!("\n=== Cloud API is working correctly ===\n");

    Ok(())
}
