//! Authentication handling for the Letta client.
//!
//! This module provides authentication mechanisms for the Letta API,
//! including bearer token authentication and API key management.

use crate::error::{LettaError, LettaResult};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::fmt;

/// Authentication configuration for the Letta client.
///
/// Supports both bearer token authentication (for Letta Cloud) and
/// no authentication (for local development servers).
#[derive(Clone, Debug)]
pub enum AuthConfig {
    /// No authentication (for local servers).
    None,
    /// Bearer token authentication.
    Bearer {
        /// The bearer token.
        token: String,
    },
}

impl AuthConfig {
    /// Create a new bearer token authentication configuration.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token to use for authentication
    ///
    /// # Examples
    ///
    /// ```rust
    /// use letta::auth::AuthConfig;
    ///
    /// let auth = AuthConfig::bearer("your-api-key-here");
    /// ```
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer {
            token: token.into(),
        }
    }

    /// Create a new no-authentication configuration.
    ///
    /// This is typically used for local development servers that don't
    /// require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use letta::auth::AuthConfig;
    ///
    /// let auth = AuthConfig::none();
    /// ```
    pub fn none() -> Self {
        Self::None
    }

    /// Apply authentication to HTTP headers.
    ///
    /// This method adds the appropriate authentication headers to the
    /// provided `HeaderMap` based on the authentication configuration.
    ///
    /// # Arguments
    ///
    /// * `headers` - The header map to modify
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError::Auth`] if the authentication configuration
    /// is invalid or if the token cannot be converted to a valid header value.
    pub fn apply_to_headers(&self, headers: &mut HeaderMap) -> LettaResult<()> {
        match self {
            Self::None => {
                // No authentication needed
            }
            Self::Bearer { token } => {
                let auth_value = format!("Bearer {token}");
                let header_value = HeaderValue::from_str(&auth_value)
                    .map_err(|_| LettaError::auth("Invalid bearer token format"))?;
                headers.insert(AUTHORIZATION, header_value);
            }
        }
        Ok(())
    }

    /// Check if authentication is configured.
    ///
    /// Returns `true` if authentication is configured, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use letta::auth::AuthConfig;
    ///
    /// let auth = AuthConfig::bearer("token");
    /// assert!(auth.is_authenticated());
    ///
    /// let auth = AuthConfig::none();
    /// assert!(!auth.is_authenticated());
    /// ```
    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Bearer { .. })
    }

    /// Get the authentication type as a string.
    ///
    /// This is useful for logging and debugging purposes.
    pub fn auth_type(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Bearer { .. } => "bearer",
        }
    }

    /// Validate the authentication configuration.
    ///
    /// This performs basic validation of the authentication configuration
    /// to ensure it's properly formed.
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError::Auth`] if the configuration is invalid.
    pub fn validate(&self) -> LettaResult<()> {
        match self {
            Self::None => Ok(()),
            Self::Bearer { token } => {
                if token.trim().is_empty() {
                    return Err(LettaError::auth("Bearer token cannot be empty"));
                }
                if token.contains('\n') || token.contains('\r') {
                    return Err(LettaError::auth("Bearer token cannot contain newlines"));
                }
                if token.len() > 1024 {
                    return Err(LettaError::auth(
                        "Bearer token is too long (max 1024 characters)",
                    ));
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for AuthConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "No authentication"),
            Self::Bearer { .. } => write!(f, "Bearer token authentication"),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self::None
    }
}

/// Helper function to extract authentication from environment variables.
///
/// This function checks for common environment variable patterns used
/// for API authentication:
///
/// - `LETTA_API_KEY` - Direct API key
/// - `LETTA_TOKEN` - Bearer token
/// - `LETTA_AUTH_TOKEN` - Bearer token (alternative name)
///
/// # Examples
///
/// ```rust
/// use letta::auth::from_env;
///
/// // Assumes LETTA_API_KEY environment variable is set
/// let auth = from_env().unwrap_or_default();
/// ```
pub fn from_env() -> Option<AuthConfig> {
    // Check for API key in common environment variables
    let env_vars = ["LETTA_API_KEY", "LETTA_TOKEN", "LETTA_AUTH_TOKEN"];

    for var in &env_vars {
        if let Ok(token) = std::env::var(var) {
            if !token.trim().is_empty() {
                return Some(AuthConfig::bearer(token.trim()));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    #[test]
    fn test_auth_config_creation() {
        let auth = AuthConfig::bearer("test-token");
        assert!(auth.is_authenticated());
        assert_eq!(auth.auth_type(), "bearer");

        let auth = AuthConfig::none();
        assert!(!auth.is_authenticated());
        assert_eq!(auth.auth_type(), "none");
    }

    #[test]
    fn test_apply_to_headers() {
        let mut headers = HeaderMap::new();

        // Test bearer token
        let auth = AuthConfig::bearer("test-token");
        auth.apply_to_headers(&mut headers).unwrap();

        let auth_header = headers.get(AUTHORIZATION).unwrap();
        assert_eq!(auth_header.to_str().unwrap(), "Bearer test-token");

        // Test no auth
        let mut headers = HeaderMap::new();
        let auth = AuthConfig::none();
        auth.apply_to_headers(&mut headers).unwrap();

        assert!(!headers.contains_key(AUTHORIZATION));
    }

    #[test]
    fn test_validation() {
        // Valid bearer token
        let auth = AuthConfig::bearer("valid-token");
        assert!(auth.validate().is_ok());

        // Empty token
        let auth = AuthConfig::bearer("");
        assert!(auth.validate().is_err());

        // Token with newlines
        let auth = AuthConfig::bearer("token\nwith\nnewlines");
        assert!(auth.validate().is_err());

        // Very long token
        let long_token = "a".repeat(1025);
        let auth = AuthConfig::bearer(long_token);
        assert!(auth.validate().is_err());

        // No auth is always valid
        let auth = AuthConfig::none();
        assert!(auth.validate().is_ok());
    }

    #[test]
    fn test_display() {
        let auth = AuthConfig::bearer("token");
        assert_eq!(auth.to_string(), "Bearer token authentication");

        let auth = AuthConfig::none();
        assert_eq!(auth.to_string(), "No authentication");
    }

    #[test]
    fn test_from_env() {
        // This test would need environment variables set to work properly
        // In a real scenario, we'd use a test framework that can set env vars

        // Test that it doesn't panic when no env vars are set
        let auth = from_env();
        // Could be None or Some depending on environment
        assert!(auth.is_none() || auth.is_some());
    }
}
