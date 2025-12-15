//! Retry logic for handling transient failures.

use crate::error::{LettaError, LettaResult};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_attempts: u32,
    /// Initial backoff duration.
    pub initial_backoff: Duration,
    /// Maximum backoff duration.
    pub max_backoff: Duration,
    /// Backoff multiplier (e.g., 2.0 for exponential backoff).
    pub backoff_multiplier: f64,
    /// Whether to add jitter to backoff delays.
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration with custom settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of retry attempts.
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set the initial backoff duration.
    pub fn with_initial_backoff(mut self, duration: Duration) -> Self {
        self.initial_backoff = duration;
        self
    }

    /// Set the maximum backoff duration.
    pub fn with_max_backoff(mut self, duration: Duration) -> Self {
        self.max_backoff = duration;
        self
    }

    /// Set the backoff multiplier.
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Enable or disable jitter.
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }

    /// Calculate the backoff duration for a given attempt.
    fn calculate_backoff(&self, attempt: u32) -> Duration {
        let base_backoff =
            self.initial_backoff.as_millis() as f64 * self.backoff_multiplier.powi(attempt as i32);

        let mut backoff_ms = base_backoff.min(self.max_backoff.as_millis() as f64) as u64;

        // Add jitter if enabled (±25% randomization)
        if self.jitter {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let jitter_factor = rng.gen_range(0.75..1.25);
            backoff_ms = (backoff_ms as f64 * jitter_factor) as u64;
        }

        Duration::from_millis(backoff_ms)
    }
}

/// Trait for determining if an error is retryable.
pub trait Retryable {
    /// Check if the error should trigger a retry.
    fn is_retryable(&self) -> bool;

    /// Get a custom retry delay if applicable (e.g., from Retry-After header).
    fn retry_after(&self) -> Option<Duration> {
        None
    }
}

impl Retryable for LettaError {
    fn is_retryable(&self) -> bool {
        match self {
            // Rate limit errors are retryable
            LettaError::RateLimit { .. } => true,

            // Request timeout errors are retryable
            LettaError::RequestTimeout { .. } => true,

            // HTTP errors might be retryable (network failures, etc)
            LettaError::Http(err) => {
                // Check if it's a network/connection error
                err.is_timeout() || err.is_connect() || err.is_request()
            }

            // API errors might be retryable depending on status code
            LettaError::Api { status, .. } => {
                matches!(
                    *status,
                    // Request timeout
                    408 |
                    // Too many requests
                    429 |
                    // Internal server error
                    500 |
                    // Bad gateway
                    502 |
                    // Service unavailable
                    503 |
                    // Gateway timeout
                    504
                )
            }

            // Other errors are not retryable
            _ => false,
        }
    }

    fn retry_after(&self) -> Option<Duration> {
        match self {
            LettaError::RateLimit { retry_after, .. } => retry_after.map(Duration::from_secs),
            _ => None,
        }
    }
}

/// Execute an async operation with retry logic.
pub async fn retry_with_config<T, F, Fut>(config: &RetryConfig, operation: F) -> LettaResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = LettaResult<T>>,
{
    let mut last_error = None;

    for attempt in 0..config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                // Check if the error is retryable
                if !error.is_retryable() || attempt == config.max_attempts - 1 {
                    return Err(error);
                }

                // Calculate backoff duration
                let backoff = if let Some(retry_after) = error.retry_after() {
                    // Use server-specified retry delay if available
                    retry_after
                } else {
                    // Otherwise use exponential backoff
                    config.calculate_backoff(attempt)
                };

                // Log retry attempt (in production, use proper logging)
                eprintln!(
                    "Retry attempt {} after {:?} due to: {:?}",
                    attempt + 1,
                    backoff,
                    error
                );

                last_error = Some(error);
                sleep(backoff).await;
            }
        }
    }

    // This should be unreachable, but just in case
    Err(last_error.unwrap_or_else(|| LettaError::Config {
        message: "Retry logic failed unexpectedly".to_string(),
    }))
}

/// Execute an async operation with default retry configuration.
pub async fn retry<T, F, Fut>(operation: F) -> LettaResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = LettaResult<T>>,
{
    retry_with_config(&RetryConfig::default(), operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use url::Url;

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::new()
            .with_max_attempts(5)
            .with_initial_backoff(Duration::from_secs(1))
            .with_max_backoff(Duration::from_secs(60))
            .with_backoff_multiplier(3.0)
            .with_jitter(false);

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.initial_backoff, Duration::from_secs(1));
        assert_eq!(config.max_backoff, Duration::from_secs(60));
        assert_eq!(config.backoff_multiplier, 3.0);
        assert!(!config.jitter);
    }

    #[test]
    fn test_backoff_calculation() {
        let config = RetryConfig::new()
            .with_initial_backoff(Duration::from_millis(100))
            .with_backoff_multiplier(2.0)
            .with_max_backoff(Duration::from_millis(1000))
            .with_jitter(false);

        // First retry: 100ms * 2^0 = 100ms
        assert_eq!(config.calculate_backoff(0), Duration::from_millis(100));

        // Second retry: 100ms * 2^1 = 200ms
        assert_eq!(config.calculate_backoff(1), Duration::from_millis(200));

        // Third retry: 100ms * 2^2 = 400ms
        assert_eq!(config.calculate_backoff(2), Duration::from_millis(400));

        // Fourth retry: 100ms * 2^3 = 800ms
        assert_eq!(config.calculate_backoff(3), Duration::from_millis(800));

        // Fifth retry: would be 1600ms but capped at 1000ms
        assert_eq!(config.calculate_backoff(4), Duration::from_millis(1000));
    }

    #[test]
    fn test_error_retryability() {
        // Rate limit errors are retryable
        let error = LettaError::RateLimit {
            retry_after: Some(60),
        };
        assert!(error.is_retryable());
        assert_eq!(error.retry_after(), Some(Duration::from_secs(60)));

        // 503 errors are retryable
        let error = LettaError::Api {
            status: 503,
            message: "Service unavailable".to_string(),
            code: None,
            body: crate::error::ErrorBody::Text(String::new()),
            url: Some(Url::parse("http://example.com/path").unwrap()),
            method: Some("GET".to_string()),
        };
        assert!(error.is_retryable());

        // 404 errors are not retryable
        let error = LettaError::NotFound {
            resource_type: "agent".to_string(),
            id: "123".to_string(),
        };
        assert!(!error.is_retryable());
    }

    #[tokio::test]
    async fn test_retry_success_on_second_attempt() {
        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry(|| {
            let count_clone = attempt_count_clone.clone();
            async move {
                let attempt_count = count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if attempt_count == 0 {
                    // Use RequestTimeout error which is retryable
                    Err(LettaError::RequestTimeout { seconds: 60 })
                } else {
                    Ok("Success".to_string())
                }
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry(|| {
            let count_clone = attempt_count_clone.clone();
            async move {
                count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err::<String, _>(LettaError::Auth {
                    message: "Invalid API key".to_string(),
                })
            }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 1); // Should not retry
    }

    #[tokio::test]
    async fn test_retry_exhausted_attempts() {
        let config = RetryConfig::new()
            .with_max_attempts(2)
            .with_initial_backoff(Duration::from_millis(10));

        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = retry_with_config(&config, || {
            let count_clone = attempt_count_clone.clone();
            async move {
                count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err::<String, _>(LettaError::RequestTimeout { seconds: 60 })
            }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 2); // Should try exactly max_attempts times
    }
}
