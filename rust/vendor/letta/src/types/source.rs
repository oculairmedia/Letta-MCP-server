//! Source-related types.

use crate::types::agent::EmbeddingConfig;
use crate::types::common::{LettaId, Metadata, Timestamp};
use bon::Builder;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// File processing status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "snake_case")]
pub enum FileProcessingStatus {
    /// File is pending processing.
    #[default]
    Pending,
    /// File is being parsed.
    Parsing,
    /// File is being embedded.
    Embedding,
    /// File processing completed.
    Completed,
    /// File processing failed.
    Error,
}

/// File metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Source ID this file belongs to.
    pub source_id: LettaId,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// File path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// File type (MIME type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// File creation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_creation_date: Option<String>,
    /// File last modified date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_last_modified_date: Option<String>,
    /// Processing status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<FileProcessingStatus>,
    /// Error message if processing failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// When the file was created in the database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the file was last updated in the database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Whether the file is deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Full file content (optional, large).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Source name.
    pub name: String,
    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions for how to use the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Embedding configuration.
    pub embedding_config: EmbeddingConfig,
    /// Source metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// When the source was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the source was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Source count response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceCounts {
    /// Number of sources.
    pub count: i32,
}

/// File upload response.
/// Can be either a job (local server) or file metadata (cloud).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileUploadResponse {
    /// Job response (local server).
    Job(FileUploadJob),
    /// Direct file metadata (cloud API).
    FileMetadata(FileMetadata),
}

/// File upload job response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadJob {
    /// Job ID.
    pub id: LettaId,
    /// Job status.
    pub status: String,
    /// Job metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FileUploadMetadata>,
    /// Created at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Updated at timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Metadata for file upload job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadMetadata {
    /// Upload type.
    #[serde(rename = "type")]
    pub upload_type: String,
    /// File name.
    pub filename: String,
    /// Source ID.
    pub source_id: LettaId,
}

/// Create source request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct CreateSourceRequest {
    /// Source name.
    pub name: String,
    /// Embedding handle (e.g., "openai/text-embedding-ada-002").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<String>,
    /// Embedding chunk size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_chunk_size: Option<i32>,
    /// Embedding configuration (legacy, prefer using `embedding`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions for how to use the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Source metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Update source request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSourceRequest {
    /// Source name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions for how to use the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Source metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
}

/// Query parameters for listing files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFilesParams {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Include file content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
}

/// Query parameters for getting file metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFileParams {
    /// Include file content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
}

/// List passages parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPassagesParams {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::agent::EmbeddingEndpointType;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_source_serialization() {
        let source = Source {
            id: Some(LettaId::from_str("source-550e8400-e29b-41d4-a716-446655440002").unwrap()),
            name: "documents".to_string(),
            description: Some("Document collection".to_string()),
            instructions: Some("Use for general knowledge".to_string()),
            embedding_config: EmbeddingConfig {
                embedding_endpoint_type: Some(EmbeddingEndpointType::Openai),
                embedding_endpoint: None,
                embedding_model: Some("text-embedding-ada-002".to_string()),
                embedding_dim: Some(1536),
                embedding_chunk_size: Some(300),
                handle: None,
                azure_config: None,
                extra: HashMap::new(),
            },
            metadata: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(source.name, deserialized.name);
    }

    #[test]
    fn test_file_metadata_serialization() {
        let file_metadata = FileMetadata {
            id: Some(LettaId::from_str("file-550e8400-e29b-41d4-a716-446655440000").unwrap()),
            organization_id: None,
            source_id: LettaId::from_str("source-550e8400-e29b-41d4-a716-446655440001").unwrap(),
            file_name: Some("test.txt".to_string()),
            file_path: Some("/path/to/test.txt".to_string()),
            file_type: Some("text/plain".to_string()),
            file_size: Some(1024),
            file_creation_date: None,
            file_last_modified_date: None,
            processing_status: Some(FileProcessingStatus::Completed),
            error_message: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
            is_deleted: Some(false),
            content: None,
        };

        let json = serde_json::to_string(&file_metadata).unwrap();
        let deserialized: FileMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(file_metadata.source_id, deserialized.source_id);
        assert_eq!(file_metadata.file_name, deserialized.file_name);
    }
}
