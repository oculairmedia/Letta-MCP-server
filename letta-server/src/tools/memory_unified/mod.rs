mod archives;
mod blocks;
mod core;
mod passages;
mod search;

use chrono::{DateTime, Utc};
use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use turbomcp::McpError;
use crate::tools::response_utils::ToolResponse;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryOperation {
    GetCoreMemory,
    UpdateCoreMemory,
    GetBlockByLabel,
    ListBlocks,
    CreateBlock,
    GetBlock,
    UpdateBlock,
    AttachBlock,
    DetachBlock,
    ListAgentsUsingBlock,
    SearchArchival,
    ListPassages,
    CreatePassage,
    UpdatePassage,
    DeletePassage,
    ListArchives,
    GetArchive,
    CreateArchive,
    UpdateArchive,
    DeleteArchive,
    AttachArchive,
    DetachArchive,
    ListAgentsUsingArchive,
    SearchMemory,
}

#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchSource {
    Archival,
    Messages,
    Both,
}

impl Default for SearchSource {
    fn default() -> Self {
        SearchSource::Both
    }
}

/// Memory unified request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MemoryUnifiedRequest {
    /// The operation to perform (get_core_memory, update_core_memory, get_block_by_label, list_blocks, create_block, get_block, update_block, attach_block, detach_block, list_agents_using_block, search_archival, list_passages, create_passage, update_passage, delete_passage, search_memory, list_archives, get_archive, create_archive, update_archive, delete_archive, attach_archive, detach_archive, list_agents_using_archive)
    pub operation: MemoryOperation,

    /// Agent ID (required for core memory, archival, passage, search, attach/detach operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Block ID (required for get_block, update_block, attach_block, detach_block, list_agents_using_block)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    /// Block label (required for get_block_by_label, update_core_memory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_label: Option<String>,
    /// Passage ID (required for update_passage, delete_passage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passage_id: Option<String>,
    /// Archive ID (required for get_archive, update_archive, delete_archive, attach_archive, detach_archive, list_agents_using_archive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,

    /// Label for block or archive (required for create_block, create_archive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Value for block or archive (required for create_block, update_core_memory, update_block)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Text content (required for create_passage, update_passage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Search query (required for search_archival, search_memory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of results to skip (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Whether block is a template (for create_block, list_blocks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,

    /// Search source filter (archival, recall, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SearchSource>,
    /// Filter results after this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    /// Filter results before this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
    /// When false (default), returns minimal confirmation; when true, returns full state
    #[serde(default)]
    pub verbose: Option<bool>,
}


#[derive(Debug, Serialize)]
pub struct ArchivalSearchResult {
    pub passages: Vec<Value>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct MessageSearchResult {
    pub messages: Vec<MessageMatch>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct MessageMatch {
    pub id: String,
    pub date: String,
    pub message_type: String,
    pub content: String,
}

pub async fn handle_memory_unified(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing memory operation");

    match request.operation {
        // Core
        MemoryOperation::GetCoreMemory => core::handle_get_core_memory(client, request).await,
        MemoryOperation::UpdateCoreMemory => {
            core::handle_update_core_memory(client, request).await
        }
        // Blocks
        MemoryOperation::GetBlockByLabel => {
            blocks::handle_get_block_by_label(client, request).await
        }
        MemoryOperation::ListBlocks => blocks::handle_list_blocks(client, request).await,
        MemoryOperation::CreateBlock => blocks::handle_create_block(client, request).await,
        MemoryOperation::GetBlock => blocks::handle_get_block(client, request).await,
        MemoryOperation::UpdateBlock => blocks::handle_update_block(client, request).await,
        MemoryOperation::AttachBlock => blocks::handle_attach_block(client, request).await,
        MemoryOperation::DetachBlock => blocks::handle_detach_block(client, request).await,
        MemoryOperation::ListAgentsUsingBlock => {
            blocks::handle_list_agents_using_block(client, request).await
        }
        // Passages
        MemoryOperation::SearchArchival => {
            passages::handle_search_archival(client, request).await
        }
        MemoryOperation::ListPassages => passages::handle_list_passages(client, request).await,
        MemoryOperation::CreatePassage => passages::handle_create_passage(client, request).await,
        MemoryOperation::UpdatePassage => passages::handle_update_passage(client, request).await,
        MemoryOperation::DeletePassage => passages::handle_delete_passage(client, request).await,
        // Archives
        MemoryOperation::ListArchives => archives::handle_list_archives(client, request).await,
        MemoryOperation::GetArchive => archives::handle_get_archive(client, request).await,
        MemoryOperation::CreateArchive => archives::handle_create_archive(client, request).await,
        MemoryOperation::UpdateArchive => archives::handle_update_archive(client, request).await,
        MemoryOperation::DeleteArchive => archives::handle_delete_archive(client, request).await,
        MemoryOperation::AttachArchive => archives::handle_attach_archive(client, request).await,
        MemoryOperation::DetachArchive => archives::handle_detach_archive(client, request).await,
        MemoryOperation::ListAgentsUsingArchive => {
            archives::handle_list_agents_using_archive(client, request).await
        }
        // Search
        MemoryOperation::SearchMemory => search::handle_search_memory(client, request).await,
    }
}
