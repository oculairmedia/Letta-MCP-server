//! Integration tests for sources API endpoints.

use bytes::Bytes;
use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::types::agent::{AgentState, CreateAgentRequest};
use letta::types::common::Metadata;
use letta::types::memory::Block;
use letta::types::source::{
    CreateSourceRequest, FileProcessingStatus, FileUploadResponse, GetFileParams, ListFilesParams,
    ListPassagesParams, Source, UpdateSourceRequest,
};
use letta::{LettaClient, LettaId};
use serial_test::serial;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

/// Extract filename from upload response (handles both local and cloud responses).
fn get_filename_from_upload(response: FileUploadResponse) -> String {
    match response {
        FileUploadResponse::Job(job) => job.metadata.as_ref().unwrap().filename.clone(),
        FileUploadResponse::FileMetadata(file) => {
            file.file_name.unwrap_or_else(|| "unknown.txt".to_string())
        }
    }
}

/// Create a test agent for sources operations.
async fn create_test_agent(client: &LettaClient) -> LettaResult<AgentState> {
    let request = CreateAgentRequest::builder()
        .name("Test Sources Agent")
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
        .build();

    client.agents().create(request).await
}

/// Create a test source with unique name.
async fn create_test_source(client: &LettaClient, base_name: &str) -> LettaResult<Source> {
    let unique_name = format!(
        "{}_{}",
        base_name,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );

    let request = CreateSourceRequest {
        name: unique_name,
        embedding: Some("letta/letta-free".to_string()),
        embedding_chunk_size: Some(300),
        embedding_config: None,
        description: Some(format!("Test source: {}", base_name)),
        instructions: Some("Use this source for testing purposes".to_string()),
        metadata: None,
    };

    client.sources().create(request).await
}

#[tokio::test]
#[serial]
async fn test_source_lifecycle() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a source
    let source = create_test_source(&client, "lifecycle_test").await?;
    assert!(source.id.is_some());
    assert!(source.name.starts_with("lifecycle_test_"));
    assert_eq!(
        source.description,
        Some("Test source: lifecycle_test".to_string())
    );

    let source_id = source.id.as_ref().unwrap();

    // Get the source
    let retrieved = client.sources().get(source_id).await?;
    assert_eq!(retrieved.id, source.id);
    assert_eq!(retrieved.name, source.name);

    // Update the source
    let update_request = UpdateSourceRequest {
        name: None,
        description: Some("Updated description".to_string()),
        instructions: Some("Updated instructions".to_string()),
        metadata: None,
        embedding_config: None,
    };

    let updated = client.sources().update(source_id, update_request).await?;
    assert_eq!(updated.id, source.id);
    assert_eq!(updated.description, Some("Updated description".to_string()));
    assert_eq!(
        updated.instructions,
        Some("Updated instructions".to_string())
    );

    // List sources
    let sources = client.sources().list().await?;
    let found = sources.iter().any(|s| s.id == source.id);
    assert!(found, "Created source should be in the list");

    // Count sources
    let count = client.sources().count().await?;
    assert!(count > 0, "Should have at least one source");

    // Get by name
    let id_by_name = client.sources().get_by_name(&source.name).await?;
    assert_eq!(id_by_name, source_id.to_string());

    // Delete the source
    let _ = client.sources().delete(source_id).await?;

    // Verify deletion
    let sources_after = client.sources().list().await?;
    let still_exists = sources_after.iter().any(|s| s.id == source.id);
    assert!(!still_exists, "Source should be deleted");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_source_file_operations() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a source
    let source = create_test_source(&client, "file_test").await?;
    let source_id = source.id.as_ref().unwrap();

    // Upload a text file
    let file_content = b"This is a test document.\nIt has multiple lines.\nFor testing purposes.";
    let file_name = format!("test_doc_{}.txt", chrono::Utc::now().timestamp());

    let upload_response = client
        .sources()
        .upload_file(
            source_id,
            file_name.clone(),
            Bytes::from(file_content.to_vec()),
            Some("text/plain".to_string()),
        )
        .await?;

    // Handle both job response (local) and direct file metadata (cloud)
    let actual_filename = get_filename_from_upload(upload_response);

    // Wait a bit for the file to be processed
    sleep(Duration::from_secs(1)).await;

    // Get the actual file metadata from the list
    let files = client.sources().list_files(source_id, None).await?;
    let file_metadata = files
        .into_iter()
        .find(|f| f.file_name.as_ref() == Some(&actual_filename))
        .expect("Uploaded file should be in the list");

    let file_id = file_metadata.id.as_ref().unwrap();

    // Verify file is in the list
    let files_check = client.sources().list_files(source_id, None).await?;
    assert!(!files_check.is_empty(), "Should have at least one file");

    let found = files_check.iter().any(|f| f.id.as_ref() == Some(file_id));
    assert!(found, "Uploaded file should be in the list");

    // Get file metadata without content
    let file_without_content = client.sources().get_file(source_id, file_id, None).await?;
    assert_eq!(file_without_content.id, file_metadata.id);
    assert!(file_without_content.content.is_none());

    // Get file metadata with content
    let params = GetFileParams {
        include_content: Some(true),
    };
    let _file_with_content = client
        .sources()
        .get_file(source_id, file_id, Some(params))
        .await?;
    // Content may not be available immediately or at all on local server
    // Just verify the request succeeds

    // Test pagination
    let list_params = ListFilesParams {
        limit: Some(1),
        after: None,
        include_content: Some(false),
    };
    let paginated_files = client
        .sources()
        .list_files(source_id, Some(list_params))
        .await?;
    assert!(paginated_files.len() <= 1);

    // Delete the file
    println!("Deleting file {} from source {}", file_id, source_id);
    client.sources().delete_file(source_id, file_id).await?;

    // Verify deletion
    let files_after = client.sources().list_files(source_id, None).await?;
    let still_exists = files_after.iter().any(|f| f.id.as_ref() == Some(file_id));
    assert!(!still_exists, "File should be deleted");

    // Clean up
    client.sources().delete(source_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_source_passages() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a source
    let source = create_test_source(&client, "passages_test").await?;
    let source_id = source.id.as_ref().unwrap();

    // Upload a file that will create passages
    let file_content = b"This is the first paragraph of our test document. It contains important information about testing.\n\nThis is the second paragraph. It has different content to test passage creation.\n\nAnd this is the third paragraph with even more test content.";
    let file_name = format!("passages_test_{}.txt", chrono::Utc::now().timestamp());

    let upload_response = client
        .sources()
        .upload_file(
            source_id,
            file_name.clone(),
            Bytes::from(file_content.to_vec()),
            Some("text/plain".to_string()),
        )
        .await?;

    // Get the actual filename from the response
    let actual_filename = get_filename_from_upload(upload_response);

    // Wait for file to appear
    sleep(Duration::from_millis(500)).await;

    // Get file metadata
    let files = client.sources().list_files(source_id, None).await?;
    let file_metadata = files
        .into_iter()
        .find(|f| f.file_name.as_ref() == Some(&actual_filename))
        .expect("Uploaded file should be in the list");

    let file_id = file_metadata.id.as_ref().unwrap();

    // Wait a bit for processing (in real scenarios, you'd poll the processing status)
    sleep(Duration::from_secs(2)).await;

    // Check file processing status
    let file_status = client.sources().get_file(source_id, file_id, None).await?;

    // The file might still be processing, but let's check if we can get passages
    if file_status.processing_status == Some(FileProcessingStatus::Completed) {
        // List passages
        let passages = client.sources().list_passages(source_id, None).await?;

        assert!(
            !passages.is_empty(),
            "Should have passages after file processing"
        );

        // Test pagination
        let passage_params = ListPassagesParams {
            limit: Some(2),
            before: None,
            after: None,
        };
        let paginated_passages = client
            .sources()
            .list_passages(source_id, Some(passage_params))
            .await?;
        assert!(paginated_passages.len() <= 2);
    }

    // Clean up
    client.sources().delete(source_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_agent_sources() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create an agent and sources
    let agent = create_test_agent(&client).await?;
    let agent_id = &agent.id;

    let source1 = create_test_source(&client, "agent_source_1").await?;
    let source2 = create_test_source(&client, "agent_source_2").await?;

    let source1_id = source1.id.as_ref().unwrap();
    let source2_id = source2.id.as_ref().unwrap();

    // Get the sources API first (needs to live long enough)
    let sources_api = client.sources();

    // Initially, agent should have no sources
    let initial_sources = sources_api.agent_sources(agent_id.clone()).list().await?;
    assert_eq!(
        initial_sources.len(),
        0,
        "Agent should start with no sources"
    );

    // Attach first source
    let updated_agent = sources_api
        .agent_sources(agent_id.clone())
        .attach(source1_id)
        .await?;
    assert_eq!(updated_agent.id, agent.id);

    // Verify source is attached
    let sources_after_attach1 = sources_api.agent_sources(agent_id.clone()).list().await?;
    assert_eq!(sources_after_attach1.len(), 1);
    assert!(sources_after_attach1.iter().any(|s| s.id == source1.id));

    // Attach second source
    let _ = sources_api
        .agent_sources(agent_id.clone())
        .attach(source2_id)
        .await?;

    // Verify both sources are attached
    let sources_after_attach2 = sources_api.agent_sources(agent_id.clone()).list().await?;
    assert_eq!(sources_after_attach2.len(), 2);
    assert!(sources_after_attach2.iter().any(|s| s.id == source1.id));
    assert!(sources_after_attach2.iter().any(|s| s.id == source2.id));

    // Detach first source
    let _ = sources_api
        .agent_sources(agent_id.clone())
        .detach(source1_id)
        .await?;

    // Verify only second source remains
    let sources_after_detach = sources_api.agent_sources(agent_id.clone()).list().await?;
    assert_eq!(sources_after_detach.len(), 1);
    assert!(!sources_after_detach.iter().any(|s| s.id == source1.id));
    assert!(sources_after_detach.iter().any(|s| s.id == source2.id));

    // Detach second source
    let _ = sources_api
        .agent_sources(agent_id.clone())
        .detach(source2_id)
        .await?;

    // Verify no sources remain
    let final_sources = sources_api.agent_sources(agent_id.clone()).list().await?;
    assert_eq!(final_sources.len(), 0);

    // Clean up
    client.agents().delete(agent_id).await?;
    client.sources().delete(source1_id).await?;
    client.sources().delete(source2_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_source_with_multiple_files() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create a source
    let source = create_test_source(&client, "multi_file_test").await?;
    let source_id = source.id.as_ref().unwrap();

    // Upload multiple files
    let files = vec![
        ("doc1.txt", b"Content of document 1" as &[u8], "text/plain"),
        ("doc2.txt", b"Content of document 2", "text/plain"),
        (
            "doc3.md",
            b"# Markdown Content\n\nThis is markdown.",
            "text/x-markdown", // this is the correct mimetype for Markdown, according to Letta
        ),
    ];

    let mut uploaded_files = Vec::new();
    for (name, content, mime_type) in files {
        let file_name = format!("{}_{}", chrono::Utc::now().timestamp_micros(), name);
        println!("filename: {}", file_name);
        let upload_response = client
            .sources()
            .upload_file(
                source_id,
                file_name.clone(),
                Bytes::from(content.to_vec()),
                Some(mime_type.to_string()),
            )
            .await?;

        // Get the actual filename from the response
        let actual_filename = get_filename_from_upload(upload_response);

        // Wait for file to be processed
        sleep(Duration::from_millis(200)).await;

        // Get the actual file metadata
        let files = client.sources().list_files(source_id, None).await?;
        let file = files
            .into_iter()
            .find(|f| f.file_name.as_ref() == Some(&actual_filename))
            .expect("Uploaded file should be in the list");
        uploaded_files.push(file);
    }

    // List all files
    let all_files = client.sources().list_files(source_id, None).await?;
    assert_eq!(all_files.len(), 3, "Should have all 3 uploaded files");

    println!("all files: {:?}", all_files);

    // Test different file types
    let txt_files: Vec<_> = all_files
        .iter()
        .filter(|f| f.file_type.as_deref() == Some("text/plain"))
        .collect();
    assert_eq!(txt_files.len(), 2, "Should have 2 text files");

    let md_files: Vec<_> = all_files
        .iter()
        .filter(|f| f.file_type.as_deref() == Some("text/x-markdown")) // this is the correct mimetype
        .collect();
    assert_eq!(md_files.len(), 1, "Should have 1 markdown file");

    // Clean up
    client.sources().delete(source_id).await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_source_error_handling() -> LettaResult<()> {
    let client = create_test_client()?;

    let fake_id = LettaId::from_str("source-00000000-0000-0000-0000-000000000000").unwrap();

    // Test getting non-existent source
    let result = client.sources().get(&fake_id).await;
    assert!(result.is_err(), "Should fail to get non-existent source");

    // Test updating non-existent source
    let update = UpdateSourceRequest {
        description: Some("Update".to_string()),
        ..Default::default()
    };
    let result = client.sources().update(&fake_id, update).await;
    assert!(result.is_err(), "Should fail to update non-existent source");

    // Test deleting non-existent source
    let result = client.sources().delete(&fake_id).await;
    assert!(result.is_err(), "Should fail to delete non-existent source");

    // Test uploading to non-existent source
    let result = client
        .sources()
        .upload_file(
            &fake_id,
            "test.txt".to_string(),
            Bytes::from(b"test".to_vec()),
            None,
        )
        .await;
    assert!(
        result.is_err(),
        "Should fail to upload to non-existent source"
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_source_metadata() -> LettaResult<()> {
    let client = create_test_client()?;

    // Create source with metadata
    let mut metadata_map = HashMap::new();
    metadata_map.insert("project".to_string(), serde_json::json!("test_project"));
    metadata_map.insert("version".to_string(), serde_json::json!(1));
    metadata_map.insert(
        "tags".to_string(),
        serde_json::json!(["test", "integration"]),
    );

    let metadata = Metadata { data: metadata_map };

    let request = CreateSourceRequest {
        name: format!("metadata_test_{}", chrono::Utc::now().timestamp()),
        embedding: Some("letta/letta-free".to_string()),
        embedding_chunk_size: Some(300),
        embedding_config: None,
        description: Some("Testing metadata".to_string()),
        instructions: None,
        metadata: Some(metadata.clone()),
    };

    let source = client.sources().create(request).await?;
    let source_id = source.id.as_ref().unwrap();

    // Verify metadata was saved
    assert!(source.metadata.is_some());
    let saved_metadata = source.metadata.as_ref().unwrap();
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

    let update = UpdateSourceRequest {
        metadata: Some(new_metadata),
        ..Default::default()
    };

    let updated = client.sources().update(source_id, update).await?;
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
    client.sources().delete(source_id).await?;

    Ok(())
}
