//! File types and related structures for agent file sessions.

use crate::types::LettaId;
use serde::{Deserialize, Serialize};

/// Represents a file in an agent's file session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFile {
    /// The unique identifier for the file.
    pub id: LettaId,

    /// The name of the file.
    #[serde(alias = "name")]
    pub filename: String,

    /// The size of the file in bytes.
    pub size: u64,

    /// The MIME type of the file.
    #[serde(alias = "type")]
    pub mime_type: String,

    /// Whether the file is currently open in the agent's session.
    #[serde(default)]
    pub is_open: bool,

    /// Timestamp when the file was opened (if currently open).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
}

/// Response from listing agent files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedAgentFiles {
    /// List of files in the agent's session.
    pub files: Vec<AgentFile>,

    /// Total number of files.
    #[serde(default)]
    pub total: u32,
}

/// Parameters for listing agent files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAgentFilesParams {
    /// Maximum number of files to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Pagination cursor (returns files after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}
