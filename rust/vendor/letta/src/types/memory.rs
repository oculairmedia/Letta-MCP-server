//! Memory-related types.

use crate::types::common::{LettaId, Metadata, Timestamp};
use bon::Builder;
use serde::{Deserialize, Serialize};

use super::EmbeddingConfig;

/// Request to create a new memory block.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
pub struct CreateBlockRequest {
    /// Block value/content (required).
    pub value: String,
    /// Block label (required).
    pub label: String,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this is a template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    /// Whether to preserve on migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_on_migration: Option<bool>,
    /// Whether the block is read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Request to update a memory block.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBlockRequest {
    /// Updated block value/content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Updated block label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Updated character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Updated block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated template flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    /// Updated preserve on migration flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_on_migration: Option<bool>,
    /// Updated read-only flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Parameters for listing memory blocks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBlocksParams {
    /// Filter by label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Only return templates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_only: Option<bool>,
    /// Filter by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by identity ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<LettaId>,
    /// Filter by identifier keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_keys: Option<Vec<String>>,
    /// Maximum number of blocks to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Memory block for agent context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Block label (e.g., "human", "persona").
    pub label: String,
    /// Block value/content.
    pub value: String,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Whether this is a template.
    #[serde(default)]
    pub is_template: bool,
    /// Whether to preserve on migration.
    #[serde(default)]
    pub preserve_on_migration: bool,
    /// Whether the block is read-only.
    #[serde(default)]
    pub read_only: bool,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

impl Block {
    /// Create a human memory block with common defaults.
    ///
    /// # Example
    /// ```
    /// # use letta::types::memory::Block;
    /// let block = Block::human("The human's name is Alice");
    /// assert_eq!(block.label, "human");
    /// assert_eq!(block.value, "The human's name is Alice");
    /// ```
    pub fn human(value: impl Into<String>) -> Self {
        Self {
            id: None,
            label: "human".to_string(),
            value: value.into(),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: Some("Information about the human user".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Create a persona memory block with common defaults.
    ///
    /// # Example
    /// ```
    /// # use letta::types::memory::Block;
    /// let block = Block::persona("I am a helpful assistant");
    /// assert_eq!(block.label, "persona");
    /// assert_eq!(block.value, "I am a helpful assistant");
    /// ```
    pub fn persona(value: impl Into<String>) -> Self {
        Self {
            id: None,
            label: "persona".to_string(),
            value: value.into(),
            limit: Some(2000),
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: Some("The agent's persona".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Create a custom memory block.
    ///
    /// # Example
    /// ```
    /// # use letta::types::memory::Block;
    /// let block = Block::new("system", "System information goes here")
    ///     .with_limit(1000)
    ///     .with_description("System-level information");
    /// assert_eq!(block.label, "system");
    /// ```
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: None,
            label: label.into(),
            value: value.into(),
            limit: None,
            is_template: false,
            preserve_on_migration: false,
            read_only: false,
            description: None,
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Set the character limit for this block.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the description for this block.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the name for this block.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Mark this block as a template.
    pub fn as_template(mut self) -> Self {
        self.is_template = true;
        self
    }

    /// Mark this block as read-only.
    pub fn as_read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Mark this block to be preserved on migration.
    pub fn preserve_on_migration(mut self) -> Self {
        self.preserve_on_migration = true;
        self
    }
}

/// Archival memory passage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passage {
    /// Passage ID.
    pub id: LettaId,
    /// Passage text content.
    pub text: String,
    /// Agent ID this passage belongs to.
    pub agent_id: LettaId,
    /// Optional embedding vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    /// Optional embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Optional source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<LettaId>,
    /// Optional file ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<LettaId>,
    /// Optional file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the passage was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the passage was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Whether the passage is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

/// Request to create archival memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArchivalMemoryRequest {
    /// Memory text content.
    pub text: String,
}

/// Request to update archival memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArchivalMemoryRequest {
    /// Passage ID (required).
    pub id: LettaId,
    /// Created by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Created at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Is deleted flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Updated agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<LettaId>,
    /// Updated source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<LettaId>,
    /// Updated file ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<LettaId>,
    /// Updated file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Updated metadata.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Updated text content.
    pub text: String,
    /// Updated embedding.
    pub embedding: Vec<f32>,
    /// Updated embedding config.
    pub embedding_config: EmbeddingConfig,
}

/// Query parameters for listing archival memory.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArchivalMemoryQueryParams {
    /// Search text for semantic search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Sort order (true for ascending/oldest first).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

/// Memory response from GET /v1/agents/{id}/core-memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Memory blocks contained in the agent's in-context memory.
    pub blocks: Vec<Block>,
    /// Blocks representing the agent's in-context memory of an attached file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_blocks: Option<Vec<Block>>,
    /// Jinja2 template for compiling memory blocks into a prompt string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
}

/// Core memory update request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCoreMemoryRequest {
    /// New value for the memory block.
    pub value: String,
}

/// Request to update a memory block.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMemoryBlockRequest {
    /// Block label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Block value/content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether to preserve on migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_on_migration: Option<bool>,
    /// Whether the block is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Request to create a memory block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemoryBlockRequest {
    /// Block label.
    pub label: String,
    /// Block value/content.
    pub value: String,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this is a template.
    #[serde(default)]
    pub is_template: bool,
    /// Whether to preserve on migration.
    #[serde(default)]
    pub preserve_on_migration: bool,
    /// Whether the block is read-only.
    #[serde(default)]
    pub read_only: bool,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Memory query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryQueryParams {
    /// Query text for semantic search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Request to summarize agent messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeMessagesRequest {
    /// Force a summarization (by default will only trigger when context is near limit).
    #[serde(default)]
    pub force: bool,
}

/// Response from message summarization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeMessagesResponse {
    /// The generated summary.
    pub summary: String,
    /// Number of messages summarized.
    pub messages_summarized: u32,
}

/// Recall memory summary containing all memory types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMemorySummary {
    /// Core memory blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_memory: Option<Memory>,
    /// Archival memory entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archival_memory: Vec<Passage>,
    /// Recall memory entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recall_memory: Vec<RecallMemoryEntry>,
}

/// Recall memory entry (message history).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMemoryEntry {
    /// Message ID.
    pub id: LettaId,
    /// Agent ID.
    pub agent_id: LettaId,
    /// Message role.
    pub role: String,
    /// Message content.
    pub content: String,
    /// When the message was created.
    pub created_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_block_serialization() {
        let block = Block {
            id: Some(LettaId::from_str("block-550e8400-e29b-41d4-a716-446655440000").unwrap()),
            label: "human".to_string(),
            value: "The human's name is Alice.".to_string(),
            limit: Some(1000),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Human information".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: Block = serde_json::from_str(&json).unwrap();
        assert_eq!(block.label, deserialized.label);
        assert_eq!(block.value, deserialized.value);
    }

    #[test]
    fn test_memory_structure() {
        let memory = Memory {
            blocks: vec![
                Block {
                    id: Some(
                        LettaId::from_str("block-550e8400-e29b-41d4-a716-446655440001").unwrap(),
                    ),
                    label: "human".to_string(),
                    value: "Name: Alice".to_string(),
                    limit: Some(1000),
                    is_template: false,
                    preserve_on_migration: true,
                    read_only: false,
                    description: None,
                    metadata: None,
                    name: None,
                    organization_id: None,
                    created_by_id: None,
                    last_updated_by_id: None,
                    created_at: None,
                    updated_at: None,
                },
                Block {
                    id: Some(
                        LettaId::from_str("block-550e8400-e29b-41d4-a716-446655440002").unwrap(),
                    ),
                    label: "persona".to_string(),
                    value: "I am a helpful assistant".to_string(),
                    limit: Some(500),
                    is_template: false,
                    preserve_on_migration: true,
                    read_only: false,
                    description: None,
                    metadata: None,
                    name: None,
                    organization_id: None,
                    created_by_id: None,
                    last_updated_by_id: None,
                    created_at: None,
                    updated_at: None,
                },
            ],
            file_blocks: None,
            prompt_template: Some("{{human}}\n{{persona}}".to_string()),
        };

        let json = serde_json::to_string(&memory).unwrap();
        let parsed: Memory = serde_json::from_str(&json).unwrap();
        assert_eq!(memory.blocks.len(), parsed.blocks.len());
        assert_eq!(memory.prompt_template, parsed.prompt_template);
    }

    #[test]
    fn test_passage_serialization() {
        let passage = Passage {
            id: LettaId::from_str("passage-550e8400-e29b-41d4-a716-446655440003").unwrap(),
            text: "Important information to remember".to_string(),
            agent_id: LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            embedding_config: None,
            source_id: None,
            file_id: None,
            file_name: None,
            metadata: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
            is_deleted: None,
        };

        let json = serde_json::to_string(&passage).unwrap();
        let deserialized: Passage = serde_json::from_str(&json).unwrap();
        assert_eq!(passage.text, deserialized.text);
        assert_eq!(passage.id, deserialized.id);
    }

    #[test]
    fn test_memory_query_params() {
        let params = MemoryQueryParams {
            query: Some("search term".to_string()),
            limit: Some(10),
            before: None,
            after: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("query"));
        assert!(!json.contains("before")); // Should be skipped when None
    }
}
