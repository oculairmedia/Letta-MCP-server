//! Integration tests for the Models API.

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
async fn test_list_models() -> LettaResult<()> {
    let client = get_test_client()?;

    // List all models
    let models = client.models().list(None).await?;

    println!("Found {} LLM models", models.len());
    for model in models.iter().take(5) {
        println!(
            "Model: {} - Type: {:?} - Context: {}",
            model.model, model.model_endpoint_type, model.context_window
        );
    }
    assert!(
        !models.is_empty(),
        "Should have at least one model available"
    );

    Ok(())
}

#[tokio::test]
async fn test_list_models_with_filter() -> LettaResult<()> {
    let client = get_test_client()?;

    // List models with filter
    let params = ListModelsParams {
        provider_category: Some(vec![ProviderCategory::Base]),
        ..Default::default()
    };

    let models = client.models().list(Some(params)).await?;
    println!(
        "Found {} models with provider_category filter",
        models.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_list_embedding_models() -> LettaResult<()> {
    let client = get_test_client()?;

    // List all embedding models
    let models = client.models().list_embedding_models(None).await?;

    println!("Found {} embedding models", models.len());
    for model in models.iter().take(5) {
        println!(
            "Embedding Model: {} - Dim: {} - Chunk: {}",
            model.embedding_model, model.embedding_dim, model.embedding_chunk_size
        );
    }
    assert!(
        !models.is_empty(),
        "Should have at least one embedding model available"
    );

    Ok(())
}
