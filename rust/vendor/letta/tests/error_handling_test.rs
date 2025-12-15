//! Test that error handling properly causes test failures.

use letta::client::{ClientConfig, LettaClient};
use letta::types::*;

#[tokio::test]
#[should_panic(expected = "Failed to create agent")]
async fn test_error_handling_with_bad_server() {
    // Create a client pointing to a non-existent server
    let config = ClientConfig::new("http://localhost:9999").unwrap();
    let client = LettaClient::new(config).unwrap();

    // This should fail and panic
    let _agent = client
        .agents()
        .create(CreateAgentRequest {
            name: Some("Test Agent".to_string()),
            model: Some("letta/letta-free".to_string()),
            embedding: Some("letta/letta-free".to_string()),
            ..Default::default()
        })
        .await
        .expect("Failed to create agent");
}

#[tokio::test]
async fn test_list_jobs_with_connection_error() {
    // Create a client pointing to a non-existent server
    let config = ClientConfig::new("http://localhost:9999").unwrap();
    let client = LettaClient::new(config).unwrap();

    // This should fail
    let result = client.jobs().list(None, None, None).await;

    // Verify it returns an error
    assert!(
        result.is_err(),
        "Should fail to connect to non-existent server"
    );

    // Check that the error is related to connection
    let err = result.unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.contains("Connection refused")
            || err_str.contains("connect")
            || err_str.contains("failed")
            || err_str.contains("error"),
        "Error should indicate connection failure, got: {}",
        err_str
    );
}
