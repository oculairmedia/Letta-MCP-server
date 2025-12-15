//! Integration tests for health API endpoint.

use letta::client::ClientBuilder;
use letta::error::LettaResult;
use letta::LettaClient;

/// Create a test client for the local server.
fn create_test_client() -> LettaResult<LettaClient> {
    ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()
}

#[tokio::test]
async fn test_health_check() -> LettaResult<()> {
    let client = create_test_client()?;

    // Check server health
    let health = client.health().check().await?;

    // Verify response
    assert!(!health.version.is_empty(), "Version should not be empty");
    assert_eq!(health.status, "ok", "Status should be 'ok'");

    println!("Server health check passed:");
    println!("  Version: {}", health.version);
    println!("  Status: {}", health.status);

    Ok(())
}

#[tokio::test]
async fn test_health_check_no_auth() -> LettaResult<()> {
    // Create client without any auth
    let client = ClientBuilder::new()
        .base_url("http://localhost:8283")
        .build()?;

    // Health check should work without authentication
    let health = client.health().check().await?;

    assert!(!health.version.is_empty());
    assert_eq!(health.status, "ok");

    Ok(())
}

#[tokio::test]
#[ignore = "Requires cloud API key"]
async fn test_health_check_cloud() -> LettaResult<()> {
    // This would test the cloud endpoint if we had an API key
    let api_key = std::env::var("LETTA_API_KEY").ok();
    if let Some(key) = api_key {
        let client = LettaClient::cloud(key)?;

        let health = client.health().check().await?;

        assert!(!health.version.is_empty());
        assert_eq!(health.status, "ok");

        println!("Cloud health check passed:");
        println!("  Version: {}", health.version);
        println!("  Status: {}", health.status);
    }

    Ok(())
}
