//! Integration tests for retry logic.

use letta::client::ClientBuilder;
use letta::error::LettaError;
use letta::retry::RetryConfig;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test that retry logic works for transient failures.
#[tokio::test]
async fn test_retry_on_503_error() {
    let mock_server = MockServer::start().await;

    // First two calls return 503
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(503).set_body_json(serde_json::json!({
            "error": "Service temporarily unavailable"
        })))
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;

    // Third call and beyond succeed
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "version": "0.8.8",
            "status": "ok"
        })))
        .mount(&mock_server)
        .await;

    // Create client with fast retry for testing
    let mut client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    client.set_retry_config(
        RetryConfig::new()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_millis(10))
            .with_jitter(false),
    );

    // Should succeed after retries
    let health = client.health().check().await.unwrap();
    assert_eq!(health.status, "ok");
}

/// Test that retry logic works for rate limit errors.
#[tokio::test]
async fn test_retry_on_rate_limit() {
    let mock_server = MockServer::start().await;

    // First attempt returns 429
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "error": "Rate limit exceeded"
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Second attempt and beyond succeed
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "version": "0.8.8",
            "status": "ok"
        })))
        .mount(&mock_server)
        .await;

    let mut client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    client.set_retry_config(
        RetryConfig::new()
            .with_max_attempts(2)
            .with_initial_backoff(Duration::from_millis(100)),
    );

    // Should succeed after retry
    let health = client.health().check().await.unwrap();
    assert_eq!(health.status, "ok");
}

/// Test that non-retryable errors fail immediately.
#[tokio::test]
async fn test_no_retry_on_auth_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "Invalid API key"
        })))
        .expect(1) // Should only be called once
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.agents().list(None).await;
    assert!(result.is_err());

    if let Err(LettaError::Auth { message }) = result {
        assert!(message.contains("Invalid API key"));
    } else {
        panic!("Expected auth error");
    }
}

/// Test exponential backoff calculation.
#[tokio::test]
async fn test_exponential_backoff() {
    let mock_server = MockServer::start().await;

    // All attempts fail with 503
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_server)
        .await;

    let mut client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    client.set_retry_config(
        RetryConfig::new()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_millis(100))
            .with_backoff_multiplier(2.0)
            .with_jitter(false),
    );

    let start = std::time::Instant::now();
    let _ = client.health().check().await;
    let elapsed = start.elapsed();

    // With exponential backoff: 100ms + 200ms = 300ms minimum
    // Allow some tolerance
    assert!(elapsed >= Duration::from_millis(280));
}

/// Test jitter in backoff timing.
#[tokio::test]
async fn test_backoff_with_jitter() {
    let mock_server = MockServer::start().await;

    // All attempts fail with 503
    Mock::given(method("GET"))
        .and(path("/v1/health/"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_server)
        .await;

    let mut client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    // With jitter, timing should vary
    client.set_retry_config(
        RetryConfig::new()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_millis(100))
            .with_jitter(true),
    );

    let start = std::time::Instant::now();
    let _ = client.health().check().await;
    let elapsed = start.elapsed();

    // With jitter, timing should be at least 75% of expected (due to ±25% jitter)
    // Expected without jitter: 100ms + 200ms = 300ms
    // Minimum with jitter: 75ms + 150ms = 225ms
    assert!(elapsed >= Duration::from_millis(200));
}

/// Test rate limit error creation and handling.
#[tokio::test]
async fn test_rate_limit_error_details() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/agents"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "60")
                .set_body_json(serde_json::json!({
                    "error": "Rate limit exceeded",
                    "code": "rate_limit_exceeded"
                })),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = ClientBuilder::new()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    // Disable retry to test error details
    let mut client = client;
    client.set_retry_config(RetryConfig::new().with_max_attempts(1));

    let result = client.agents().list(None).await;
    assert!(result.is_err());

    if let Err(LettaError::RateLimit { retry_after }) = result {
        assert_eq!(retry_after, Some(60));
    } else {
        panic!("Expected rate limit error");
    }
}
