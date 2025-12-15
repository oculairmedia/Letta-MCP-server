//! Error types and handling for the Letta client.
//!
//! This module provides comprehensive error types with rich diagnostics
//! via [`miette`] for excellent error reporting and debugging experience.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

/// Result type alias for Letta operations.
pub type LettaResult<T> = Result<T, LettaError>;

/// Unauthorized error response structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnauthorizedError {
    /// Main error message.
    pub message: String,
    /// Detailed explanation.
    pub details: String,
    /// Server ownership information.
    pub ownership: String,
}

/// Simple detail error response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetailError {
    /// Error detail message.
    pub detail: String,
}

/// Simple message error response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageError {
    /// Error message.
    pub message: String,
}

/// Structured error response body from the Letta API.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorBody {
    /// Plain text error response.
    Text(String),
    /// Unauthorized error (401) with specific fields.
    Unauthorized(UnauthorizedError),
    /// Error with detail field (validation errors, simple errors).
    Detail(DetailError),
    /// Error with message field.
    Message(MessageError),
    /// Unstructured JSON error response (fallback).
    Json(serde_json::Value),
}

impl ErrorBody {
    /// Parse an error body from a response string.
    pub fn from_response(body: &str) -> Self {
        // Try to parse as JSON first
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
            // Try to deserialize into known error types

            // Try unauthorized error first (most specific)
            if let Ok(unauthorized) = serde_json::from_value::<UnauthorizedError>(json.clone()) {
                return Self::Unauthorized(unauthorized);
            }

            // Try detail error
            if let Ok(detail) = serde_json::from_value::<DetailError>(json.clone()) {
                return Self::Detail(detail);
            }

            // Try message error
            if let Ok(message) = serde_json::from_value::<MessageError>(json.clone()) {
                return Self::Message(message);
            }

            // Fall back to unstructured JSON
            Self::Json(json)
        } else {
            // Not JSON, try to extract message from HTML if possible
            let text = if body.contains("<pre>") && body.contains("</pre>") {
                // Extract text from <pre> tags (common in Letta HTML errors)
                if let Some(start) = body.find("<pre>") {
                    if let Some(end) = body.find("</pre>") {
                        let content = &body[start + 5..end];
                        content.to_string()
                    } else {
                        body.to_string()
                    }
                } else {
                    body.to_string()
                }
            } else {
                body.to_string()
            };

            Self::Text(text)
        }
    }

    /// Extract a human-readable message from the error body.
    pub fn message(&self) -> Option<String> {
        match self {
            Self::Text(text) => {
                if text.trim().is_empty() {
                    None
                } else {
                    Some(text.clone())
                }
            }
            Self::Unauthorized(err) => Some(err.message.clone()),
            Self::Detail(err) => Some(err.detail.clone()),
            Self::Message(err) => Some(err.message.clone()),
            Self::Json(json) => {
                // Try common error message fields for unstructured JSON
                json.get("message")
                    .or_else(|| json.get("error"))
                    .or_else(|| json.get("detail"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            }
        }
    }

    /// Extract an error code if available.
    pub fn code(&self) -> Option<String> {
        match self {
            Self::Json(json) => json
                .get("code")
                .or_else(|| json.get("error_code"))
                .or_else(|| json.get("type"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            _ => None,
        }
    }

    /// Check if this is a validation error.
    pub fn is_validation_error(&self) -> bool {
        match self {
            Self::Detail(detail) => detail.detail.contains("validation error"),
            _ => false,
        }
    }

    /// Get the raw string representation of the error body.
    pub fn as_str(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Json(json) => serde_json::to_string(json).unwrap_or_else(|_| json.to_string()),
            Self::Unauthorized(err) => {
                serde_json::to_string(err).unwrap_or_else(|_| format!("{:?}", err))
            }
            Self::Detail(err) => {
                serde_json::to_string(err).unwrap_or_else(|_| format!("{:?}", err))
            }
            Self::Message(err) => {
                serde_json::to_string(err).unwrap_or_else(|_| format!("{:?}", err))
            }
        }
    }
}

impl Serialize for ErrorBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Text(text) => serializer.serialize_str(text),
            Self::Json(json) => json.serialize(serializer),
            Self::Unauthorized(err) => err.serialize(serializer),
            Self::Detail(err) => err.serialize(serializer),
            Self::Message(err) => err.serialize(serializer),
        }
    }
}

/// Comprehensive error type for all Letta client operations.
///
/// This error type provides detailed context about failures and implements
/// [`miette::Diagnostic`] for rich error reporting with suggestions and
/// source code highlighting where applicable.
#[derive(thiserror::Error, Debug)]
pub enum LettaError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Authentication failed.
    #[error("Authentication failed: {message}")]
    Auth {
        /// Detailed authentication error message.
        message: String,
    },

    /// API returned an error response.
    #[error("API error {status}: {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
        /// Optional error code from the API.
        code: Option<String>,
        /// Structured error response body.
        body: ErrorBody,
        /// Request URL that failed.
        url: Option<url::Url>,
        /// Request method that failed.
        method: Option<String>,
    },

    /// JSON serialization/deserialization failed.
    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),

    /// Streaming operation failed.
    #[error("Streaming error: {message}")]
    Streaming {
        /// Detailed streaming error message.
        message: String,
        /// Source error if available.
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Client configuration error.
    #[error("Configuration error: {message}")]
    Config {
        /// Configuration error message.
        message: String,
    },

    /// URL parsing error.
    #[error("Invalid URL")]
    Url(#[from] url::ParseError),

    /// I/O operation failed.
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    /// URL encoding error.
    #[error("URL encoding error")]
    UrlEncoding(#[from] serde_urlencoded::ser::Error),

    /// Request timeout.
    #[error("Request timed out after {seconds} seconds")]
    RequestTimeout {
        /// Timeout duration in seconds.
        seconds: u64,
    },

    /// Rate limit exceeded.
    #[error("Rate limit exceeded. Retry after {retry_after:?} seconds")]
    RateLimit {
        /// Seconds to wait before retrying.
        retry_after: Option<u64>,
    },

    /// Resource not found.
    #[error("Resource not found: {resource_type} with ID {id}")]
    NotFound {
        /// Type of resource that was not found.
        resource_type: String,
        /// ID of the resource.
        id: String,
    },

    /// Validation error for request parameters.
    #[error("Validation error: {message}")]
    Validation {
        /// Validation error message.
        message: String,
        /// Field that failed validation, if applicable.
        field: Option<String>,
    },
}

impl miette::Diagnostic for LettaError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Http(_) => Some(Box::new("letta::http")),
            Self::Auth { .. } => Some(Box::new("letta::auth")),
            Self::Api {
                code: Some(code), ..
            } => Some(Box::new(format!("letta::api::{code}"))),
            Self::Api { .. } => Some(Box::new("letta::api")),
            Self::Serde(_) => Some(Box::new("letta::serde")),
            Self::Streaming { .. } => Some(Box::new("letta::streaming")),
            Self::Config { .. } => Some(Box::new("letta::config")),
            Self::Url(_) => Some(Box::new("letta::url")),
            Self::Io(_) => Some(Box::new("letta::io")),
            Self::UrlEncoding(_) => Some(Box::new("letta::url_encoding")),
            Self::RequestTimeout { .. } => Some(Box::new("letta::timeout")),
            Self::RateLimit { .. } => Some(Box::new("letta::rate_limit")),
            Self::NotFound { .. } => Some(Box::new("letta::not_found")),
            Self::Validation { .. } => Some(Box::new("letta::validation")),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Auth { .. } => Some(Box::new(
                "Check your API key or authentication configuration. \
                 For local servers, ensure the server is running and accessible.",
            )),
            Self::Api { status: 401, url, method, .. } => {
                let mut help = String::from("Your API key is invalid or expired. Please check your authentication credentials.");
                if let Some(u) = url {
                    help.push_str(&format!("\nFailed request: {} {}", method.as_deref().unwrap_or("?"), u));
                }
                Some(Box::new(help))
            }
            Self::Api { status: 403, .. } => Some(Box::new(
                "You don't have permission to access this resource. \
                 Check your API key permissions.",
            )),
            Self::Api { status: 404, url, .. } => {
                let mut help = String::from("The requested resource was not found. Verify the resource ID and that it exists.");
                if let Some(u) = url {
                    help.push_str(&format!("\nRequested URL: {}", u));
                }
                Some(Box::new(help))
            }
            Self::Api { status: 429, .. } => Some(Box::new(
                "You're being rate limited. Please wait before making more requests.",
            )),
            Self::Api { status: 500..=599, url, method, .. } => {
                let mut help = String::from("The server encountered an error. Please try again later or contact support.");
                if let (Some(u), Some(m)) = (url, method) {
                    help.push_str(&format!("\nFailed request: {} {}", m, u));
                }
                Some(Box::new(help))
            }
            Self::Config { .. } => Some(Box::new(
                "Check your client configuration, including base URL and authentication settings.",
            )),
            Self::RequestTimeout { .. } => Some(Box::new(
                "The request took too long. Try increasing the timeout or check your network connection.",
            )),
            Self::RateLimit { retry_after: Some(seconds) } => Some(Box::new(format!(
                "Wait {seconds} seconds before making another request."
            ))),
            Self::Validation { field: Some(field), .. } => Some(Box::new(format!(
                "Check the '{field}' field value and ensure it meets the API requirements."
            ))),
            _ => None,
        }
    }

    fn severity(&self) -> Option<miette::Severity> {
        match self {
            Self::Config { .. } | Self::Validation { .. } => Some(miette::Severity::Warning),
            Self::Auth { .. }
            | Self::Api {
                status: 401 | 403, ..
            } => Some(miette::Severity::Error),
            Self::Api {
                status: 500..=599, ..
            } => Some(miette::Severity::Error),
            _ => None,
        }
    }
}

impl LettaError {
    /// Create a new authentication error.
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth {
            message: message.into(),
        }
    }

    /// Create a new API error.
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            code: None,
            body: ErrorBody::Text(String::new()),
            url: None,
            method: None,
        }
    }

    /// Create a new API error with error code.
    pub fn api_with_code(status: u16, message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            code: Some(code.into()),
            body: ErrorBody::Text(String::new()),
            url: None,
            method: None,
        }
    }

    /// Create a new API error from response body.
    /// Automatically parses and structures the error body and maps to specific error types.
    pub fn from_response(status: u16, body_str: String) -> Self {
        Self::from_response_with_context(status, body_str, None, None, None)
    }

    /// Create a new API error from response body with optional headers.
    /// Automatically parses and structures the error body and maps to specific error types.
    pub fn from_response_with_headers(
        status: u16,
        body_str: String,
        headers: Option<&reqwest::header::HeaderMap>,
    ) -> Self {
        Self::from_response_with_context(status, body_str, headers, None, None)
    }

    /// Create a new API error from response body with full context.
    /// Automatically parses and structures the error body and maps to specific error types.
    pub fn from_response_with_context(
        status: u16,
        body_str: String,
        headers: Option<&reqwest::header::HeaderMap>,
        url: Option<url::Url>,
        method: Option<String>,
    ) -> Self {
        let body = ErrorBody::from_response(&body_str);

        // Extract message from structured body or use default
        let message = body
            .message()
            .unwrap_or_else(|| Self::default_message_for_status(status));

        // Extract error code from structured body
        let code = body.code();

        // Map status codes to specific error types
        match status {
            401 => {
                // Check if we have an UnauthorizedError structure
                if let ErrorBody::Unauthorized(_) = &body {
                    Self::Api {
                        status,
                        message,
                        code,
                        body,
                        url,
                        method,
                    }
                } else {
                    Self::Auth { message }
                }
            }
            404 => {
                // Try to extract resource info from the message
                // Common patterns: "Agent not found", "Tool with ID xxx not found", etc.
                if let Some(resource_info) = Self::extract_resource_info(&message) {
                    Self::NotFound {
                        resource_type: resource_info.0,
                        id: resource_info.1,
                    }
                } else {
                    Self::Api {
                        status,
                        message,
                        code,
                        body,
                        url,
                        method,
                    }
                }
            }
            422 => {
                // Validation error
                if let Some(field) = Self::extract_validation_field(&message, &body) {
                    Self::Validation {
                        message,
                        field: Some(field),
                    }
                } else {
                    Self::Validation {
                        message,
                        field: None,
                    }
                }
            }
            429 => {
                // Rate limit - try to extract retry-after from headers or body
                let retry_after = if let Some(headers) = headers {
                    headers
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .or_else(|| Self::extract_retry_after(&body))
                } else {
                    Self::extract_retry_after(&body)
                };
                Self::RateLimit { retry_after }
            }
            408 | 504 => {
                // Timeout errors
                Self::RequestTimeout { seconds: 60 } // Default timeout
            }
            _ => {
                // Default to generic API error
                Self::Api {
                    status,
                    message,
                    code,
                    body,
                    url,
                    method,
                }
            }
        }
    }

    /// Create a new API error with specific error body.
    pub fn api_with_body(status: u16, message: impl Into<String>, body: ErrorBody) -> Self {
        Self::Api {
            status,
            message: message.into(),
            code: body.code(),
            body,
            url: None,
            method: None,
        }
    }

    /// Get default error message for HTTP status code.
    fn default_message_for_status(status: u16) -> String {
        match status {
            400 => "Bad Request".to_string(),
            401 => "Unauthorized".to_string(),
            403 => "Forbidden".to_string(),
            404 => "Not Found".to_string(),
            408 => "Request Timeout".to_string(),
            422 => "Unprocessable Entity".to_string(),
            429 => "Too Many Requests".to_string(),
            500 => "Internal Server Error".to_string(),
            502 => "Bad Gateway".to_string(),
            503 => "Service Unavailable".to_string(),
            504 => "Gateway Timeout".to_string(),
            _ => format!("HTTP {status}"),
        }
    }

    /// Create a new streaming error.
    pub fn streaming(message: impl Into<String>) -> Self {
        Self::Streaming {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new streaming error with source.
    pub fn streaming_with_source(
        message: impl Into<String>,
        source: impl Into<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::Streaming {
            message: message.into(),
            source: Some(source.into()),
        }
    }

    /// Create a new configuration error.
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new not found error.
    pub fn not_found(resource_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            id: id.into(),
        }
    }

    /// Create a new validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            field: None,
        }
    }

    /// Create a new validation error for a specific field.
    pub fn validation_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Create a new request timeout error.
    pub fn request_timeout(seconds: u64) -> Self {
        Self::RequestTimeout { seconds }
    }

    /// Create a new rate limit error.
    pub fn rate_limit(retry_after: Option<u64>) -> Self {
        Self::RateLimit { retry_after }
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RequestTimeout { .. }
                | Self::RateLimit { .. }
                | Self::Api {
                    status: 500..=599,
                    ..
                }
        )
    }

    /// Get the HTTP status code if this is an API error.
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Get the structured response body if this is an API error.
    pub fn response_body(&self) -> Option<&ErrorBody> {
        match self {
            Self::Api { body, .. } => Some(body),
            _ => None,
        }
    }

    /// Get the error code if this is an API error.
    pub fn error_code(&self) -> Option<&str> {
        match self {
            Self::Api { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// Check if this is a specific type of API error.
    pub fn is_unauthorized(&self) -> bool {
        matches!(
            self,
            Self::Api {
                body: ErrorBody::Unauthorized(_),
                ..
            }
        )
    }

    /// Check if this is a validation error.
    pub fn is_validation_error(&self) -> bool {
        match self {
            Self::Api { body, .. } => body.is_validation_error(),
            Self::Validation { .. } => true,
            _ => false,
        }
    }

    /// Get the unauthorized error details if this is an unauthorized error.
    pub fn unauthorized_details(&self) -> Option<(&str, &str, &str)> {
        match self {
            Self::Api {
                body: ErrorBody::Unauthorized(err),
                ..
            } => Some((&err.message, &err.details, &err.ownership)),
            _ => None,
        }
    }

    /// Extract resource type and ID from a 404 error message.
    /// Handles patterns like:
    /// - "Agent not found"
    /// - "Agent with ID xxx not found"
    /// - "Tool 'xxx' not found"
    /// - "No source found with ID: xxx"
    fn extract_resource_info(message: &str) -> Option<(String, String)> {
        let lower = message.to_lowercase();

        // Pattern: "X not found" or variants with IDs
        if let Some(not_found_pos) = lower.find(" not found") {
            let prefix = &message[..not_found_pos];

            // Try to extract ID from patterns like "with ID xxx" or "'xxx'"
            if let Some(id_start) = prefix.find(" with ID ") {
                let id_part = &prefix[id_start + 9..];
                let id = id_part
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                let resource = prefix[..id_start].trim().to_string();
                return Some((resource, id));
            } else if let Some(quote_start) = prefix.rfind('\'') {
                // Handle pattern like "Tool 'tool_name' not found"
                if let Some(prev_quote) = prefix[..quote_start].rfind('\'') {
                    let id = prefix[prev_quote + 1..quote_start].to_string();
                    let resource = prefix[..prev_quote].trim().to_string();
                    return Some((resource, id));
                }
            } else {
                // Simple pattern like "Agent not found" - no ID
                let resource = prefix.trim().to_string();
                return Some((resource, String::new()));
            }
        }

        // Handle patterns like "No source found with ID: xxx"
        // This pattern has "found" but not immediately followed by "not found"
        if lower.contains(" found ") && lower.contains(" with id:") {
            // Find the ID after the colon
            if let Some(colon_pos) = message.find(':') {
                let id = message[colon_pos + 1..].trim().to_string();

                // Find the resource type (word before "found")
                let before_colon = &message[..colon_pos];
                let words: Vec<&str> = before_colon.split_whitespace().collect();
                for (i, word) in words.iter().enumerate() {
                    if word.to_lowercase() == "found" && i > 0 {
                        return Some((words[i - 1].to_string(), id));
                    }
                }
            }
        }

        None
    }

    /// Extract field name from validation error message or body.
    fn extract_validation_field(message: &str, body: &ErrorBody) -> Option<String> {
        // Check if the body has field information in JSON
        if let ErrorBody::Json(json) = body {
            if let Some(field) = json.get("field").and_then(|v| v.as_str()) {
                return Some(field.to_string());
            }
            // Check for validation_errors structure
            if let Some(errors) = json.get("validation_errors").and_then(|v| v.as_object()) {
                // Return the first field with an error
                if let Some(field) = errors.keys().next() {
                    return Some(field.clone());
                }
            }
        }

        // Try to extract from message patterns like "Field 'xxx' is required"
        if message.contains("Field '") {
            if let Some(start) = message.find("Field '") {
                let after_field = &message[start + 7..];
                if let Some(end) = after_field.find('\'') {
                    return Some(after_field[..end].to_string());
                }
            }
        }

        None
    }

    /// Extract retry-after value from error body.
    fn extract_retry_after(body: &ErrorBody) -> Option<u64> {
        if let ErrorBody::Json(json) = body {
            // Common fields for retry-after
            json.get("retry_after")
                .or_else(|| json.get("retryAfter"))
                .or_else(|| json.get("retry-after"))
                .and_then(|v| v.as_u64())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use miette::Diagnostic;

    use super::*;

    #[test]
    fn test_error_creation() {
        let err = LettaError::auth("Invalid token");
        assert!(matches!(err, LettaError::Auth { .. }));
        assert_eq!(err.to_string(), "Authentication failed: Invalid token");
    }

    #[test]
    fn test_api_error() {
        let err = LettaError::api(404, "Not found");
        assert!(matches!(err, LettaError::Api { .. }));
        assert_eq!(err.status_code(), Some(404));
    }

    #[test]
    fn test_retryable_errors() {
        assert!(LettaError::request_timeout(30).is_retryable());
        assert!(LettaError::rate_limit(Some(60)).is_retryable());
        assert!(LettaError::api(500, "Server error").is_retryable());
        assert!(!LettaError::auth("Invalid token").is_retryable());
    }

    #[test]
    fn test_diagnostic_codes() {
        let err = LettaError::auth("test");
        assert!(err.code().is_some());

        let err = LettaError::api_with_code(400, "Bad request", "invalid_params");
        let code = err.code().unwrap();
        assert_eq!(code.to_string(), "letta::api::invalid_params");
    }

    #[test]
    fn test_error_body_parsing() {
        // Test unauthorized error pattern
        let unauthorized_json = r#"{"message": "Unauthorized", "details": "You are attempting to access a resource...", "ownership": "This is api.letta.com..."}"#;
        let body = ErrorBody::from_response(unauthorized_json);

        assert!(matches!(body, ErrorBody::Unauthorized(_)));
        assert_eq!(body.message(), Some("Unauthorized".to_string()));

        // Test simple detail pattern
        let detail_json = r#"{"detail": "Not Found"}"#;
        let body = ErrorBody::from_response(detail_json);

        assert!(matches!(body, ErrorBody::Detail(_)));
        assert_eq!(body.message(), Some("Not Found".to_string()));

        // Test validation error pattern
        let validation_json = r#"{"detail": "1 validation error for Tool..."}"#;
        let body = ErrorBody::from_response(validation_json);

        assert!(matches!(body, ErrorBody::Detail(_)));
        assert!(body.is_validation_error());

        // Test message pattern
        let message_json = r#"{"message": "Simple error"}"#;
        let body = ErrorBody::from_response(message_json);

        assert!(matches!(body, ErrorBody::Message(_)));
        assert_eq!(body.message(), Some("Simple error".to_string()));

        // Test plain text
        let body = ErrorBody::from_response("Server error");
        assert!(matches!(body, ErrorBody::Text(_)));
        assert_eq!(body.message(), Some("Server error".to_string()));
    }

    #[test]
    fn test_api_error_from_responses() {
        // Test unauthorized error
        let unauthorized_json = r#"{"message": "Unauthorized", "details": "You are attempting to access a resource...", "ownership": "This is api.letta.com..."}"#;
        let err = LettaError::from_response(401, unauthorized_json.to_string());

        assert_eq!(err.status_code(), Some(401));
        assert!(err.is_unauthorized());
        assert_eq!(err.to_string(), "API error 401: Unauthorized");

        let (message, details, ownership) = err.unauthorized_details().unwrap();
        assert_eq!(message, "Unauthorized");
        assert!(details.contains("You are attempting to access"));
        assert!(ownership.contains("api.letta.com"));

        // Test validation error
        let validation_json = r#"{"detail": "1 validation error for Tool"}"#;
        let err = LettaError::from_response(422, validation_json.to_string());

        assert!(err.is_validation_error());
        assert_eq!(
            err.to_string(),
            "Validation error: 1 validation error for Tool"
        );

        // Test plain text error
        let err = LettaError::from_response(500, "Internal Server Error".to_string());
        assert_eq!(err.to_string(), "API error 500: Internal Server Error");
    }

    #[test]
    fn test_error_body_round_trip() {
        let unauthorized = ErrorBody::Unauthorized(UnauthorizedError {
            message: "Unauthorized".to_string(),
            details: "Access denied".to_string(),
            ownership: "api.letta.com".to_string(),
        });

        let json = serde_json::to_string(&unauthorized).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["message"], "Unauthorized");
        assert_eq!(parsed["details"], "Access denied");
        assert_eq!(parsed["ownership"], "api.letta.com");
    }

    #[test]
    fn test_default_status_messages() {
        let err = LettaError::from_response(404, "".to_string());
        assert_eq!(err.to_string(), "API error 404: Not Found");

        let err = LettaError::from_response(999, "".to_string());
        assert_eq!(err.to_string(), "API error 999: HTTP 999");
    }

    #[test]
    fn test_real_letta_api_responses() {
        // Test actual 401 unauthorized response from Letta API
        let unauthorized_response = r#"{"message":"Unauthorized","details":"You are attempting to access a resource that you don't have permission to access, this could be because you have not provided an appropriate API key or are connecting to the wrong server","ownership":"This is api.letta.com, which is used to access Letta Cloud resources. If you are self hosting, you should connect to your own server, usually http://localhost:8283"}"#;

        let err = LettaError::from_response(401, unauthorized_response.to_string());
        assert!(err.is_unauthorized());
        assert_eq!(err.to_string(), "API error 401: Unauthorized");

        let (message, details, ownership) = err.unauthorized_details().unwrap();
        assert_eq!(message, "Unauthorized");
        assert!(details.contains("you don't have permission"));
        assert!(ownership.contains("api.letta.com"));

        // Test actual 404 HTML response
        let html_404 = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Error</title>
</head>
<body>
<pre>Not Found</pre>
</body>
</html>"#;

        let err = LettaError::from_response(404, html_404.to_string());
        assert_eq!(err.to_string(), "API error 404: Not Found");
        assert!(matches!(err.response_body(), Some(ErrorBody::Text(_))));

        // Test actual 404 JSON response
        let json_404 = r#"{"detail":"Not Found"}"#;
        let err = LettaError::from_response(404, json_404.to_string());
        assert_eq!(err.to_string(), "API error 404: Not Found");
        assert!(matches!(err.response_body(), Some(ErrorBody::Detail(_))));

        // Test actual 400 HTML response
        let html_400 = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Error</title>
</head>
<body>
<pre>Bad Request</pre>
</body>
</html>"#;

        let err = LettaError::from_response(400, html_400.to_string());
        assert_eq!(err.to_string(), "API error 400: Bad Request");

        // Test actual validation error response
        let validation_error = r#"{"detail":"1 validation error for Tool\n  Value error, 'close_file' is not a user-defined function in module 'letta.functions.function_sets.files' [type=value_error, input_value=<letta.orm.tool.Tool object at 0x7ac389d9c860>, input_type=Tool]\n    For further information visit https://errors.pydantic.dev/2.11/v/value_error"}"#;

        let err = LettaError::from_response(500, validation_error.to_string());
        // This is a 500 error that contains validation error details in the body
        assert!(matches!(err, LettaError::Api { .. }));
        assert!(err.is_validation_error());
        assert!(err.to_string().contains("validation error"));
    }

    #[test]
    fn test_status_specific_error_mapping() {
        // Test 401 -> Auth error
        let err = LettaError::from_response(401, "Invalid API key".to_string());
        assert!(matches!(err, LettaError::Auth { .. }));

        // Test 404 -> NotFound error with resource extraction
        let err = LettaError::from_response(404, "Agent with ID agent-123 not found".to_string());
        match err {
            LettaError::NotFound { resource_type, id } => {
                assert_eq!(resource_type, "Agent");
                assert_eq!(id, "agent-123");
            }
            _ => panic!("Expected NotFound error"),
        }

        // Test 404 with quotes
        let err = LettaError::from_response(404, "Tool 'calculator' not found".to_string());
        match err {
            LettaError::NotFound { resource_type, id } => {
                assert_eq!(resource_type, "Tool");
                assert_eq!(id, "calculator");
            }
            _ => panic!("Expected NotFound error"),
        }

        // Test 422 -> Validation error
        let err = LettaError::from_response(422, "Field 'name' is required".to_string());
        match err {
            LettaError::Validation { message, field } => {
                assert!(message.contains("Field 'name' is required"));
                assert_eq!(field, Some("name".to_string()));
            }
            _ => panic!("Expected Validation error"),
        }

        // Test 429 -> RateLimit error
        let err = LettaError::from_response(429, r#"{"retry_after": 60}"#.to_string());
        match err {
            LettaError::RateLimit { retry_after } => {
                assert_eq!(retry_after, Some(60));
            }
            _ => panic!("Expected RateLimit error"),
        }

        // Test 408/504 -> RequestTimeout
        let err = LettaError::from_response(408, "Request timeout".to_string());
        assert!(matches!(err, LettaError::RequestTimeout { .. }));

        let err = LettaError::from_response(504, "Gateway timeout".to_string());
        assert!(matches!(err, LettaError::RequestTimeout { .. }));
    }

    #[test]
    fn test_extract_resource_info() {
        // Test various patterns
        let cases = vec![
            ("Agent not found", Some(("Agent", ""))),
            (
                "Agent with ID agent-123 not found",
                Some(("Agent", "agent-123")),
            ),
            ("Tool 'my_tool' not found", Some(("Tool", "my_tool"))),
            (
                "No source found with ID: source-456",
                Some(("source", "source-456")),
            ),
            ("Something else entirely", None),
        ];

        for (message, expected) in cases {
            let result = LettaError::extract_resource_info(message);
            match (result.as_ref(), expected) {
                (Some((res_type, id)), Some((exp_type, exp_id))) => {
                    assert_eq!(
                        res_type, exp_type,
                        "Resource type mismatch for: {}",
                        message
                    );
                    assert_eq!(id, exp_id, "ID mismatch for: {}", message);
                }
                (None, None) => {}
                _ => {
                    eprintln!("Result: {:?}, Expected: {:?}", result, expected);
                    panic!("Mismatch for message: {}", message);
                }
            }
        }
    }

    #[test]
    fn test_extract_validation_field() {
        let body = ErrorBody::Text("".to_string());
        assert_eq!(
            LettaError::extract_validation_field("Field 'name' is required", &body),
            Some("name".to_string())
        );

        // Test JSON body with field
        let json_body = ErrorBody::Json(serde_json::json!({
            "field": "email",
            "message": "Invalid email format"
        }));
        assert_eq!(
            LettaError::extract_validation_field("Invalid email format", &json_body),
            Some("email".to_string())
        );

        // Test validation_errors structure
        let json_body = ErrorBody::Json(serde_json::json!({
            "validation_errors": {
                "password": ["Too short"],
                "username": ["Already taken"]
            }
        }));
        let field = LettaError::extract_validation_field("Validation failed", &json_body);
        assert!(field.is_some());
        assert!(field == Some("password".to_string()) || field == Some("username".to_string()));
    }

    #[test]
    fn test_error_context_trait() {
        use super::ErrorContext;

        // Test context_file
        let result: LettaResult<()> = Err(LettaError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        )));
        let err = result.context_file("/path/to/file.txt").unwrap_err();
        // IO errors get wrapped in a new IO error with the path
        match &err {
            LettaError::Io(io_err) => {
                assert!(io_err.to_string().contains("/path/to/file.txt"));
            }
            _ => panic!("Expected IO error"),
        }

        // Test context_resource
        let result: LettaResult<()> = Err(LettaError::Api {
            status: 404,
            message: "Not found".to_string(),
            code: None,
            body: ErrorBody::Text("Not found".to_string()),
            url: None,
            method: None,
        });
        let err = result.context_resource("agent", "agent-123").unwrap_err();
        assert!(matches!(err, LettaError::NotFound { .. }));

        // Test context_operation
        let result: LettaResult<()> = Err(LettaError::Api {
            status: 500,
            message: "Internal error".to_string(),
            code: None,
            body: ErrorBody::Text("Internal error".to_string()),
            url: None,
            method: None,
        });
        let err = result.context_operation("uploading file").unwrap_err();
        assert!(err.to_string().contains("while uploading file"));

        // Test context_msg
        let result: LettaResult<()> = Err(LettaError::Config {
            message: "Invalid config".to_string(),
        });
        let err = result.context_msg("during initialization").unwrap_err();
        assert!(err.to_string().contains("during initialization"));
    }
}

/// Extension trait for adding context to Letta errors.
///
/// This trait provides convenient methods for attaching additional context
/// to errors, making debugging easier by providing more information about
/// what operation failed and with what parameters.
pub trait ErrorContext<T> {
    /// Add file path context to an error.
    ///
    /// # Example
    /// ```no_run
    /// # use letta::error::{LettaResult, ErrorContext};
    /// # async fn example() -> LettaResult<()> {
    /// # let path = "test.txt";
    /// # let result: LettaResult<()> = Ok(());
    /// result.context_file(path)?;
    /// # Ok(())
    /// # }
    /// ```
    fn context_file<P: AsRef<Path>>(self, path: P) -> LettaResult<T>;

    /// Add resource context to an error.
    ///
    /// # Example
    /// ```no_run
    /// # use letta::error::{LettaResult, ErrorContext};
    /// # async fn example() -> LettaResult<()> {
    /// # let result: LettaResult<()> = Ok(());
    /// result.context_resource("agent", "agent-123")?;
    /// # Ok(())
    /// # }
    /// ```
    fn context_resource(self, resource_type: &str, id: &str) -> LettaResult<T>;

    /// Add operation context to an error.
    ///
    /// # Example
    /// ```no_run
    /// # use letta::error::{LettaResult, ErrorContext};
    /// # async fn example() -> LettaResult<()> {
    /// # let result: LettaResult<()> = Ok(());
    /// result.context_operation("uploading file")?;
    /// # Ok(())
    /// # }
    /// ```
    fn context_operation(self, operation: &str) -> LettaResult<T>;

    /// Add custom context message to an error.
    ///
    /// # Example
    /// ```no_run
    /// # use letta::error::{LettaResult, ErrorContext};
    /// # async fn example() -> LettaResult<()> {
    /// # let result: LettaResult<()> = Ok(());
    /// result.context_msg("while processing batch request")?;
    /// # Ok(())
    /// # }
    /// ```
    fn context_msg(self, msg: &str) -> LettaResult<T>;
}

impl<T> ErrorContext<T> for LettaResult<T> {
    fn context_file<P: AsRef<Path>>(self, path: P) -> LettaResult<T> {
        self.map_err(|e| {
            let path_str = path.as_ref().display().to_string();
            match e {
                LettaError::Io(io_err) => LettaError::Io(std::io::Error::new(
                    io_err.kind(),
                    format!("{}: {}", path_str, io_err),
                )),
                other => LettaError::Config {
                    message: format!("Error with file '{}': {}", path_str, other),
                },
            }
        })
    }

    fn context_resource(self, resource_type: &str, id: &str) -> LettaResult<T> {
        self.map_err(|e| match e {
            LettaError::Api { status, .. } if status == 404 => LettaError::NotFound {
                resource_type: resource_type.to_string(),
                id: id.to_string(),
            },
            other => LettaError::Config {
                message: format!("Error with {} '{}': {}", resource_type, id, other),
            },
        })
    }

    fn context_operation(self, operation: &str) -> LettaResult<T> {
        self.map_err(|e| match e {
            LettaError::Api {
                status,
                message,
                code,
                body,
                url,
                method,
            } => LettaError::Api {
                status,
                message: format!("{} (while {})", message, operation),
                code,
                body,
                url,
                method,
            },
            other => LettaError::Config {
                message: format!("Error while {}: {}", operation, other),
            },
        })
    }

    fn context_msg(self, msg: &str) -> LettaResult<T> {
        self.map_err(|e| LettaError::Config {
            message: format!("{}: {}", msg, e),
        })
    }
}
