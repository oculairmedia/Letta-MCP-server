//! Folder types and related structures.

use crate::types::{EmbeddingConfig, LettaId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A folder resource for organizing files and agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// The name of the folder.
    pub name: String,

    /// Optional description of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional instructions for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Metadata associated with the folder.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,

    /// The unique identifier for the folder.
    pub id: LettaId,

    /// Embedding configuration for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,

    /// The organization ID that owns this folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,

    /// ID of the user who created this folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,

    /// ID of the user who last updated this folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,

    /// Timestamp when the folder was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp when the folder was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new folder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFolderRequest {
    /// The name of the folder.
    pub name: String,

    /// Optional description of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional instructions for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Metadata associated with the folder.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,

    /// Embedding configuration for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
}

/// Request to update an existing folder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFolderRequest {
    /// The name of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional description of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional instructions for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Metadata associated with the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Embedding configuration for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
}

/// Parameters for listing folders.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFoldersParams {
    /// Folder ID cursor for pagination (returns folders before this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Folder ID cursor for pagination (returns folders after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Maximum number of folders to return (default: 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Sort order by creation time ('asc' or 'desc', default: 'asc').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Field to sort by (default: 'created_at').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Filter by folder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
