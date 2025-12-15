//! Shared types for Letta MCP Server
//!
//! This crate contains all the shared types, enums, and structures
//! used across the Letta MCP server implementation.

use serde::{Deserialize, Serialize};

/// Common pagination parameters
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: Some(50),
            offset: Some(0),
        }
    }
}

/// Message structure for agent communication
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Standard response structure
#[derive(Debug, Serialize)]
pub struct StandardResponse {
    pub success: bool,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub message: String,
}

impl StandardResponse {
    pub fn success(operation: impl Into<String>, data: serde_json::Value, message: impl Into<String>) -> Self {
        Self {
            success: true,
            operation: operation.into(),
            data: Some(data),
            message: message.into(),
        }
    }

    pub fn success_no_data(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: true,
            operation: operation.into(),
            data: None,
            message: message.into(),
        }
    }

    pub fn error(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            operation: operation.into(),
            data: None,
            message: message.into(),
        }
    }
}
