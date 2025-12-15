//! Integration tests for the Telemetry API.

use letta::client::{ClientConfig, LettaClient};
use letta::error::LettaResult;

/// Get a test client for the local server.
fn get_test_client() -> LettaResult<LettaClient> {
    let config = ClientConfig::new("http://localhost:8283")?;
    LettaClient::new(config)
}

#[tokio::test]
async fn test_retrieve_provider_trace() -> LettaResult<()> {
    let client = get_test_client()?;

    // Since we don't have a real step_id, we'll test with a fake one
    // and expect a 404 error
    let fake_step_id = "step-00000000-0000-0000-0000-000000000000";

    let result = client
        .telemetry()
        .retrieve_provider_trace(fake_step_id)
        .await;

    // We expect this to fail with a 404 since the step doesn't exist
    match result {
        Err(letta::error::LettaError::NotFound { resource_type, id }) => {
            println!("Expected NotFound error for non-existent step");
            println!("Resource type: {}, ID: {}", resource_type, id);
            // The error extraction identifies this as "ProviderTrace"
            assert_eq!(resource_type, "ProviderTrace");
            Ok(())
        }
        Err(letta::error::LettaError::Api { status: 404, .. }) => {
            println!("Got 404 API error (alternative match)");
            Ok(())
        }
        Err(e) => {
            // Any other error should fail the test
            panic!("Unexpected error: {:?}", e);
        }
        Ok(trace) => {
            // If somehow we get a trace back, that's unexpected
            panic!("Expected 404 error but got trace: {:?}", trace);
        }
    }
}

#[tokio::test]
#[ignore = "requires a valid step_id from actual runs"]
async fn test_retrieve_provider_trace_with_valid_step() -> LettaResult<()> {
    let client = get_test_client()?;

    // This test would need a real step_id from an actual run
    let step_id = "REPLACE_WITH_ACTUAL_STEP_ID";

    let trace = client.telemetry().retrieve_provider_trace(step_id).await?;

    println!("Retrieved trace: {:?}", trace);

    // Verify the trace has expected fields
    assert!(trace.request_json.is_object() || trace.request_json.is_null());
    assert!(trace.response_json.is_object() || trace.response_json.is_null());

    // Step ID should match what we requested
    if let Some(returned_step_id) = &trace.step_id {
        assert_eq!(returned_step_id, step_id);
    }

    Ok(())
}
