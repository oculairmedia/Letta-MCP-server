//! Utility functions and helpers.

use crate::error::{LettaError, LettaResult};
use std::time::Duration;

/// Convert a duration to seconds as u64.
pub fn duration_to_seconds(duration: Duration) -> u64 {
    duration.as_secs()
}

/// Parse a retry-after header value.
pub fn parse_retry_after(value: &str) -> Option<u64> {
    value.parse().ok()
}

/// Validate that a string is a valid resource ID.
pub fn validate_resource_id(id: &str) -> LettaResult<()> {
    if id.trim().is_empty() {
        return Err(LettaError::validation("Resource ID cannot be empty"));
    }

    if id.len() > 255 {
        return Err(LettaError::validation("Resource ID is too long"));
    }

    Ok(())
}

/// Join URL path segments safely.
pub fn join_paths(base: &str, segments: &[&str]) -> String {
    let mut url = base.trim_end_matches('/').to_string();

    for segment in segments {
        if !segment.is_empty() {
            url.push('/');
            url.push_str(segment.trim_start_matches('/').trim_end_matches('/'));
        }
    }

    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_to_seconds() {
        let duration = Duration::from_secs(42);
        assert_eq!(duration_to_seconds(duration), 42);
    }

    #[test]
    fn test_parse_retry_after() {
        assert_eq!(parse_retry_after("60"), Some(60));
        assert_eq!(parse_retry_after("invalid"), None);
    }

    #[test]
    fn test_validate_resource_id() {
        assert!(validate_resource_id("valid-id").is_ok());
        assert!(validate_resource_id("").is_err());
        assert!(validate_resource_id("  ").is_err());

        let long_id = "a".repeat(256);
        assert!(validate_resource_id(&long_id).is_err());
    }

    #[test]
    fn test_join_paths() {
        assert_eq!(
            join_paths("http://example.com", &["api", "v1"]),
            "http://example.com/api/v1"
        );
        assert_eq!(
            join_paths("http://example.com/", &["/api/", "/v1/"]),
            "http://example.com/api/v1"
        );
        assert_eq!(join_paths("http://example.com", &[]), "http://example.com");
    }
}
