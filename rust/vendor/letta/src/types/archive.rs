//! Archive management types for the Letta API.

use crate::types::agent::AgentState;
use crate::types::common::{LettaId, Timestamp};
use crate::types::memory::Passage;
use serde::{Deserialize, Serialize};

/// Archive resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archive {
    /// Archive ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Archive name.
    pub name: String,
    /// Archive description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Vector DB provider payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_db_provider: Option<serde_json::Value>,
    /// Embedding configuration payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<serde_json::Value>,
    /// Archive metadata payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to create an archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveCreateRequest {
    /// Archive name.
    pub name: String,
    /// Optional embedding handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<String>,
    /// Optional embedding configuration payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<serde_json::Value>,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update an archive.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveUpdateRequest {
    /// Optional archive name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional archive description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to create a passage within an archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassageCreateRequest {
    /// Passage text.
    pub text: String,
    /// Optional metadata payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Optional tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Optional creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// Request to batch-create passages within an archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassageBatchCreateRequest {
    /// Passages to create.
    pub passages: Vec<PassageCreateRequest>,
}

/// Response alias for listing agents attached to an archive.
pub type ArchiveAgentsResponse = Vec<AgentState>;

/// Response alias for listing passages in an archive batch insert.
pub type ArchivePassagesResponse = Vec<Passage>;
