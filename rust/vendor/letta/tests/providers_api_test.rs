//! Integration tests for the Providers API.

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;
use letta::types::*;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
async fn test_list_providers() -> LettaResult<()> {
    let client = get_test_client()?;

    // List all providers
    let providers = client.providers().list(None).await?;

    println!("Found {} providers", providers.len());
    for provider in providers.iter().take(5) {
        println!(
            "Provider: {} - Type: {} - Category: {}",
            provider.name, provider.provider_type, provider.provider_category
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_provider_crud() -> LettaResult<()> {
    let client = get_test_client()?;

    // Create a provider
    let create_req = ProviderCreate {
        name: format!("Test Provider {}", uuid::Uuid::new_v4()),
        provider_type: ProviderType::Openai,
        provider_category: None, // Let the API set the default
        api_key: "sk-test-12345".to_string(),
        base_url: None, // Let the API set the default
        access_key: None,
        secret_key: None,
        region: None,
        metadata: None,
    };

    let provider = client.providers().create(create_req).await?;
    println!("Created provider: {} ({})", provider.name, provider.id);

    // Update the provider (only api_key, access_key, and region can be updated)
    let update_req = ProviderUpdate {
        api_key: "sk-updated-test-key".to_string(),
        access_key: None,
        region: None,
    };

    let updated = client.providers().update(&provider.id, update_req).await?;
    println!("Updated provider: {} - API key changed", updated.name);
    // We can't verify the API key changed since it's encrypted/hidden

    // Delete the provider
    client.providers().delete(&provider.id).await?;
    println!("Successfully deleted provider");

    Ok(())
}

#[tokio::test]
async fn test_list_providers_with_filter() -> LettaResult<()> {
    let client = get_test_client()?;

    // List providers with filter (use Byok since that's what we create)
    let params = ListProvidersParams {
        provider_category: Some(ProviderCategory::Byok),
        ..Default::default()
    };

    let providers = client.providers().list(Some(params)).await?;

    println!("Found {} BYOK providers", providers.len());
    for provider in &providers {
        assert_eq!(provider.provider_category, ProviderCategory::Byok);
    }

    Ok(())
}

#[tokio::test]
#[ignore = "Provider check may fail depending on provider configuration"]
async fn test_provider_check() -> LettaResult<()> {
    let client = get_test_client()?;

    // First, list providers to find one to check
    let providers = client.providers().list(None).await.unwrap_or_default();

    if providers.is_empty() {
        println!("No providers available to check");
        return Ok(());
    }

    if let Some(provider) = providers.first() {
        let check_result = client.providers().check(&provider.id).await?;

        println!("Provider check status: {}", check_result.status);
        if let Some(error) = check_result.error {
            println!("Provider check error: {}", error);
        }
    }

    Ok(())
}
