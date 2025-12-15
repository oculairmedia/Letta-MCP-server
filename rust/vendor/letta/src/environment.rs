//! Environment configuration for Letta API.

use serde::{Deserialize, Serialize};

/// Letta API environment configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LettaEnvironment {
    /// Letta Cloud API (<https://api.letta.com>).
    /// Requires API authentication.
    Cloud,
    /// Self-hosted or local Letta server (<http://localhost:8283>).
    /// Typically doesn't require authentication.
    SelfHosted,
}

impl Default for LettaEnvironment {
    fn default() -> Self {
        Self::Cloud
    }
}

impl LettaEnvironment {
    /// Get the base URL for this environment.
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Cloud => "https://api.letta.com",
            Self::SelfHosted => "http://localhost:8283",
        }
    }

    /// Check if this environment typically requires authentication.
    pub fn requires_auth(&self) -> bool {
        matches!(self, Self::Cloud)
    }

    /// Check if this is a cloud environment.
    pub fn is_cloud(&self) -> bool {
        matches!(self, Self::Cloud)
    }

    /// Check if this is a self-hosted/local environment.
    pub fn is_self_hosted(&self) -> bool {
        matches!(self, Self::SelfHosted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_defaults() {
        assert_eq!(LettaEnvironment::default(), LettaEnvironment::Cloud);
    }

    #[test]
    fn test_environment_urls() {
        assert_eq!(LettaEnvironment::Cloud.base_url(), "https://api.letta.com");
        assert_eq!(
            LettaEnvironment::SelfHosted.base_url(),
            "http://localhost:8283"
        );
    }

    #[test]
    fn test_environment_auth_requirements() {
        assert!(LettaEnvironment::Cloud.requires_auth());
        assert!(!LettaEnvironment::SelfHosted.requires_auth());
    }

    #[test]
    fn test_environment_serialization() {
        let json = serde_json::to_string(&LettaEnvironment::Cloud).unwrap();
        assert_eq!(json, "\"cloud\"");

        let json = serde_json::to_string(&LettaEnvironment::SelfHosted).unwrap();
        assert_eq!(json, "\"self_hosted\"");
    }
}
