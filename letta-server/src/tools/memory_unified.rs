//! Memory Unified Operations Tool
//!
//! Consolidated tool for all memory operations using discriminator pattern.
//! Supports core memory, memory blocks, and archival/passage operations.

use crate::tools::validation_utils::sdk_err;
use chrono::{DateTime, Utc};
use letta::types::{LettaMessageUnion, ListMessagesRequest};
use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::info;
use turbomcp::McpError;

/// Memory operation discriminator
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryOperation {
    // Core memory
    GetCoreMemory,
    UpdateCoreMemory,
    // Memory blocks
    GetBlockByLabel,
    ListBlocks,
    CreateBlock,
    GetBlock,
    UpdateBlock,
    AttachBlock,
    DetachBlock,
    ListAgentsUsingBlock,
    // Archival/passages
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
    // Unified search
    SearchMemory,
}

/// Source for unified memory search
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchSource {
    /// Search only archival memory (passages)
    Archival,
    /// Search only conversation messages
    Messages,
    /// Search both archival and messages (default)
    Both,
}

impl Default for SearchSource {
    fn default() -> Self {
        SearchSource::Both
    }
}

/// Memory unified request
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MemoryUnifiedRequest {
    pub operation: MemoryOperation,

    // Common parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,

    // Create/Update data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    // Pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    // Block-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,

    // Search memory specific
    /// Which source to search: archival, messages, or both
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SearchSource>,
    /// Filter results after this datetime (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    /// Filter results before this datetime (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,

    #[serde(default)]
    pub verbose: Option<bool>,
}

/// Memory unified response
#[derive(Debug, Serialize)]
pub struct MemoryUnifiedResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passages: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_memory: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,

    // Search memory specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archival: Option<ArchivalSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<MessageSearchResult>,
}

/// Result from archival memory search
#[derive(Debug, Serialize)]
pub struct ArchivalSearchResult {
    pub passages: Vec<Value>,
    pub count: usize,
}

/// Result from message search
#[derive(Debug, Serialize)]
pub struct MessageSearchResult {
    pub messages: Vec<MessageMatch>,
    pub count: usize,
}

/// A matching message from search
#[derive(Debug, Serialize)]
pub struct MessageMatch {
    pub id: String,
    pub date: String,
    pub message_type: String,
    pub content: String,
}

/// Main handler for memory unified operations
pub async fn handle_memory_unified(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing memory operation");

    match request.operation {
        MemoryOperation::GetCoreMemory => handle_get_core_memory(client, request).await,
        MemoryOperation::UpdateCoreMemory => handle_update_core_memory(client, request).await,
        MemoryOperation::GetBlockByLabel => handle_get_block_by_label(client, request).await,
        MemoryOperation::ListBlocks => handle_list_blocks(client, request).await,
        MemoryOperation::CreateBlock => handle_create_block(client, request).await,
        MemoryOperation::GetBlock => handle_get_block(client, request).await,
        MemoryOperation::UpdateBlock => handle_update_block(client, request).await,
        MemoryOperation::AttachBlock => handle_attach_block(client, request).await,
        MemoryOperation::DetachBlock => handle_detach_block(client, request).await,
        MemoryOperation::ListAgentsUsingBlock => {
            handle_list_agents_using_block(client, request).await
        }
        MemoryOperation::SearchArchival => handle_search_archival(client, request).await,
        MemoryOperation::ListPassages => handle_list_passages(client, request).await,
        MemoryOperation::CreatePassage => handle_create_passage(client, request).await,
        MemoryOperation::UpdatePassage => handle_update_passage(client, request).await,
        MemoryOperation::DeletePassage => handle_delete_passage(client, request).await,
        MemoryOperation::ListArchives => handle_list_archives(client, request).await,
        MemoryOperation::GetArchive => handle_get_archive(client, request).await,
        MemoryOperation::CreateArchive => handle_create_archive(client, request).await,
        MemoryOperation::UpdateArchive => handle_update_archive(client, request).await,
        MemoryOperation::DeleteArchive => handle_delete_archive(client, request).await,
        MemoryOperation::AttachArchive => handle_attach_archive(client, request).await,
        MemoryOperation::DetachArchive => handle_detach_archive(client, request).await,
        MemoryOperation::ListAgentsUsingArchive => {
            handle_list_agents_using_archive(client, request).await
        }
        MemoryOperation::SearchMemory => handle_search_memory(client, request).await,
    }
}

// ===================================================
// Core Memory Operations
// ===================================================

async fn handle_get_core_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_core_memory".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let memory = client
        .memory()
        .get_core_memory(&letta_id)
        .await
        .map_err(|e| sdk_err("get core memory", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_core_memory".to_string(),
        message: "Core memory retrieved successfully".to_string(),
        agent_id: Some(agent_id),
        core_memory: Some(serde_json::to_value(memory)?),
        block_id: None,
        passage_id: None,
        archive_id: None,
        data: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_update_core_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for update_core_memory".to_string())
    })?;
    let block_label = request.block_label.ok_or_else(|| {
        McpError::invalid_request("block_label is required for update_core_memory".to_string())
    })?;
    let value = request.value.ok_or_else(|| {
        McpError::invalid_request("value is required for update_core_memory".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let update_request = letta::types::memory::UpdateMemoryBlockRequest {
        label: None,
        value: Some(value),
        limit: None,
        name: None,
        preserve_on_migration: None,
        read_only: None,
        description: None,
        metadata: None,
    };

    let updated_block = client
        .memory()
        .update_core_memory_block(&letta_id, &block_label, update_request)
        .await
        .map_err(|e| sdk_err("update core memory", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_core_memory".to_string(),
        message: format!("Core memory block '{}' updated successfully", block_label),
        agent_id: Some(agent_id),
        data: Some(serde_json::to_value(updated_block)?),
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

// ===================================================
// Memory Block Operations
// ===================================================

async fn handle_get_block_by_label(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_block_by_label".to_string())
    })?;
    let block_label = request.block_label.ok_or_else(|| {
        McpError::invalid_request("block_label is required for get_block_by_label".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let block = client
        .memory()
        .get_core_memory_block(&letta_id, &block_label)
        .await
        .map_err(|e| sdk_err("get block by label", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block_by_label".to_string(),
        message: format!("Block '{}' retrieved successfully", block_label),
        agent_id: Some(agent_id),
        data: Some(serde_json::to_value(block)?),
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_list_blocks(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for list_blocks".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let blocks = client
        .memory()
        .list_core_memory_blocks(&letta_id)
        .await
        .map_err(|e| sdk_err("list blocks", e))?;

    let count = blocks.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_blocks".to_string(),
        message: format!("Found {} blocks", count),
        agent_id: Some(agent_id),
        blocks: Some(serde_json::to_value(&blocks)?),
        count: Some(count),
        archival: None,
        messages: None,
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        data: None,
        passages: None,
    })
}

async fn handle_create_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let label = request.label.ok_or_else(|| {
        McpError::invalid_request("label is required for create_block".to_string())
    })?;
    let value = request.value.ok_or_else(|| {
        McpError::invalid_request("value is required for create_block".to_string())
    })?;

    // Create block using blocks API
    let create_request = letta::types::memory::CreateBlockRequest {
        value,
        label,
        limit: None,
        name: None,
        is_template: request.is_template,
        preserve_on_migration: None,
        read_only: None,
        description: None,
        metadata: None,
    };

    let block = client
        .blocks()
        .create(create_request)
        .await
        .map_err(|e| sdk_err("create block", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_block".to_string(),
        message: "Block created successfully".to_string(),
        agent_id: None,
        block_id: block.id.as_ref().map(|id| id.to_string()),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_get_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for get_block".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&block_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid block_id: {}", e)))?;

    let block = client
        .blocks()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get block", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block".to_string(),
        message: "Block retrieved successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_update_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for update_block".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&block_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid block_id: {}", e)))?;

    let update_request = letta::types::memory::UpdateBlockRequest {
        value: request.value,
        label: request.label,
        limit: None,
        name: None,
        is_template: request.is_template,
        preserve_on_migration: None,
        read_only: None,
        description: None,
        metadata: None,
    };

    let block = client
        .blocks()
        .update(&letta_id, update_request)
        .await
        .map_err(|e| sdk_err("update block", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_block".to_string(),
        message: "Block updated successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_attach_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for attach_block".to_string())
    })?;
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for attach_block".to_string())
    })?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_block_id = letta::types::LettaId::from_str(&block_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid block_id: {}", e)))?;

    let agent_state = client
        .memory()
        .attach_memory_block(&letta_agent_id, &letta_block_id)
        .await
        .map_err(|e| sdk_err("attach block", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "attach_block".to_string(),
        message: "Block attached to agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: Some(block_id),
        data: Some(serde_json::to_value(agent_state)?),
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_detach_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for detach_block".to_string())
    })?;
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for detach_block".to_string())
    })?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_block_id = letta::types::LettaId::from_str(&block_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid block_id: {}", e)))?;

    let agent_state = client
        .memory()
        .detach_memory_block(&letta_agent_id, &letta_block_id)
        .await
        .map_err(|e| sdk_err("detach block", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "detach_block".to_string(),
        message: "Block detached from agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: Some(block_id),
        data: Some(serde_json::to_value(agent_state)?),
        passage_id: None,
        archive_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_list_agents_using_block(
    _client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    // This operation is not directly supported by the SDK
    // Would need to list all agents and check which use the block
    Err(McpError::internal(
        "list_agents_using_block not yet implemented - requires custom query".to_string(),
    ))
}

// ===================================================
// Archival/Passage Operations
// ===================================================

async fn handle_search_archival(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for search_archival".to_string())
    })?;
    let query = request.query.ok_or_else(|| {
        McpError::invalid_request("query is required for search_archival".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: Some(query),
        limit: request.limit.map(|l| l as u32),
        before: None,
        after: None,
        ascending: None,
    };

    let passages = client
        .memory()
        .list_archival_memory(&letta_id, Some(params))
        .await
        .map_err(|e| sdk_err("search archival", e))?;

    let count = passages.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "search_archival".to_string(),
        message: format!("Found {} passages", count),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        count: Some(count),
        archival: None,
        messages: None,
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        data: None,
        blocks: None,
    })
}

async fn handle_list_passages(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for list_passages".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: None,
        limit: request.limit.map(|l| l as u32),
        before: None,
        after: None,
        ascending: None,
    };

    let passages = client
        .memory()
        .list_archival_memory(&letta_id, Some(params))
        .await
        .map_err(|e| sdk_err("list passages", e))?;

    let count = passages.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_passages".to_string(),
        message: format!("Found {} passages", count),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        count: Some(count),
        archival: None,
        messages: None,
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        data: None,
        blocks: None,
    })
}

async fn handle_create_passage(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for create_passage".to_string())
    })?;
    let text = request.text.ok_or_else(|| {
        McpError::invalid_request("text is required for create_passage".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let create_request = letta::types::memory::CreateArchivalMemoryRequest { text };

    let passages = client
        .memory()
        .create_archival_memory(&letta_id, create_request)
        .await
        .map_err(|e| sdk_err("create passage", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_passage".to_string(),
        message: "Passage created successfully".to_string(),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        data: None,
        blocks: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_update_passage(
    _client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    // UpdateArchivalMemoryRequest requires embedding and embedding_config fields
    // that are not available from the client request. This needs to be refactored
    // to work with the SDK's requirements or use a different approach.
    Err(McpError::internal(
        "update_passage not yet implemented - SDK requires embedding data not available from client".to_string(),
    ))
}

async fn handle_delete_passage(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for delete_passage".to_string())
    })?;
    let passage_id = request.passage_id.ok_or_else(|| {
        McpError::invalid_request("passage_id is required for delete_passage".to_string())
    })?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_passage_id = letta::types::LettaId::from_str(&passage_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid passage_id: {}", e)))?;

    client
        .memory()
        .delete_archival_memory(&letta_agent_id, &letta_passage_id)
        .await
        .map_err(|e| sdk_err("delete passage", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "delete_passage".to_string(),
        message: "Passage deleted successfully".to_string(),
        agent_id: Some(agent_id),
        passage_id: Some(passage_id),
        archive_id: None,
        block_id: None,
        core_memory: None,
        data: None,
        blocks: None,
        passages: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_list_archives(
    client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archives = client
        .archives()
        .list()
        .await
        .map_err(|e| sdk_err("list archives", e))?;

    let count = archives.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_archives".to_string(),
        message: format!("Found {} archives", count),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: None,
        data: None,
        blocks: None,
        passages: None,
        core_memory: None,
        count: Some(count),
        archival: Some(ArchivalSearchResult {
            passages: vec![serde_json::to_value(&archives)?],
            count,
        }),
        messages: None,
    })
}

async fn handle_get_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request("archive_id is required for get_archive".to_string())
    })?;

    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let archive = client
        .archives()
        .get(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("get archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_archive".to_string(),
        message: "Archive retrieved successfully".to_string(),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(archive)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_create_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let name = request.label.ok_or_else(|| {
        McpError::invalid_request("label is required for create_archive".to_string())
    })?;

    let create_request = letta::types::ArchiveCreateRequest {
        name,
        embedding: None,
        embedding_config: None,
        description: request.text,
    };

    let archive = client
        .archives()
        .create(create_request)
        .await
        .map_err(|e| sdk_err("create archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_archive".to_string(),
        message: "Archive created successfully".to_string(),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: archive.id.as_ref().map(|id| id.to_string()),
        data: Some(serde_json::to_value(archive)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_update_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request("archive_id is required for update_archive".to_string())
    })?;

    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let update_request = letta::types::ArchiveUpdateRequest {
        name: request.label,
        description: request.text,
    };

    let archive = client
        .archives()
        .update(&letta_archive_id, update_request)
        .await
        .map_err(|e| sdk_err("update archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_archive".to_string(),
        message: "Archive updated successfully".to_string(),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(archive)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_delete_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request("archive_id is required for delete_archive".to_string())
    })?;

    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let response = client
        .archives()
        .delete(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("delete archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "delete_archive".to_string(),
        message: "Archive deleted successfully".to_string(),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(response)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_attach_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for attach_archive".to_string())
    })?;
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request("archive_id is required for attach_archive".to_string())
    })?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let agent_state = client
        .archives()
        .agent_archives(letta_agent_id)
        .attach(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("attach archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "attach_archive".to_string(),
        message: "Archive attached to agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(agent_state)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_detach_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for detach_archive".to_string())
    })?;
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request("archive_id is required for detach_archive".to_string())
    })?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let agent_state = client
        .archives()
        .agent_archives(letta_agent_id)
        .detach(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("detach archive", e))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "detach_archive".to_string(),
        message: "Archive detached from agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(agent_state)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: None,
        archival: None,
        messages: None,
    })
}

async fn handle_list_agents_using_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archive_id = request.archive_id.ok_or_else(|| {
        McpError::invalid_request(
            "archive_id is required for list_agents_using_archive".to_string(),
        )
    })?;

    let letta_archive_id = letta::types::LettaId::from_str(&archive_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid archive_id: {}", e)))?;

    let agents = client
        .archives()
        .list_agents(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("list agents using archive", e))?;

    let count = agents.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_agents_using_archive".to_string(),
        message: format!("Found {} agents using archive", count),
        agent_id: None,
        block_id: None,
        passage_id: None,
        archive_id: Some(archive_id),
        data: Some(serde_json::to_value(&agents)?),
        blocks: None,
        passages: None,
        core_memory: None,
        count: Some(count),
        archival: None,
        messages: None,
    })
}

// ===================================================
// Unified Search Operations
// ===================================================

async fn handle_search_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for search_memory".to_string())
    })?;
    let query = request.query.ok_or_else(|| {
        McpError::invalid_request("query is required for search_memory".to_string())
    })?;

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let source = request.source.unwrap_or_default();
    let limit = request.limit.unwrap_or(50) as usize;
    let start_date = request.start_date;
    let end_date = request.end_date;

    let mut archival_result: Option<ArchivalSearchResult> = None;
    let mut messages_result: Option<MessageSearchResult> = None;

    // Search archival memory if requested
    if matches!(source, SearchSource::Archival | SearchSource::Both) {
        archival_result = Some(
            search_archival_memory(client, &letta_id, &query, limit, start_date, end_date).await?,
        );
    }

    // Search messages if requested
    if matches!(source, SearchSource::Messages | SearchSource::Both) {
        messages_result =
            Some(search_messages(client, &letta_id, &query, limit, start_date, end_date).await?);
    }

    let archival_count = archival_result.as_ref().map(|r| r.count).unwrap_or(0);
    let messages_count = messages_result.as_ref().map(|r| r.count).unwrap_or(0);

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "search_memory".to_string(),
        message: format!(
            "Found {} archival passages and {} messages",
            archival_count, messages_count
        ),
        agent_id: Some(agent_id),
        archival: archival_result,
        messages: messages_result,
        count: Some(archival_count + messages_count),
        block_id: None,
        passage_id: None,
        archive_id: None,
        core_memory: None,
        data: None,
        blocks: None,
        passages: None,
    })
}

/// Search archival memory using the SDK
async fn search_archival_memory(
    client: &LettaClient,
    agent_id: &letta::types::LettaId,
    query: &str,
    limit: usize,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<ArchivalSearchResult, McpError> {
    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: Some(query.to_string()),
        limit: Some(limit as u32),
        before: None,
        after: None,
        ascending: None,
    };

    let passages = client
        .memory()
        .list_archival_memory(agent_id, Some(params))
        .await
        .map_err(|e| sdk_err("search archival memory", e))?;

    // Filter by date if provided and convert to JSON values
    let mut filtered_passages: Vec<Value> = Vec::new();
    for passage in passages {
        let passage_value = serde_json::to_value(&passage)?;

        // Check date range if specified
        if let Some(created_at) = passage_value.get("created_at").and_then(|v| v.as_str()) {
            if let Ok(passage_date) = DateTime::parse_from_rfc3339(created_at) {
                let passage_date_utc = passage_date.with_timezone(&Utc);
                if let Some(ref start) = start_date {
                    if passage_date_utc < *start {
                        continue;
                    }
                }
                if let Some(ref end) = end_date {
                    if passage_date_utc > *end {
                        continue;
                    }
                }
            }
        }

        // Remove embedding to reduce response size
        let mut passage_obj = passage_value
            .as_object()
            .cloned()
            .unwrap_or_else(serde_json::Map::new);
        passage_obj.remove("embedding");

        filtered_passages.push(Value::Object(passage_obj));
    }

    let count = filtered_passages.len();
    Ok(ArchivalSearchResult {
        passages: filtered_passages,
        count,
    })
}

/// Search messages with client-side filtering
async fn search_messages(
    client: &LettaClient,
    agent_id: &letta::types::LettaId,
    query: &str,
    limit: usize,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<MessageSearchResult, McpError> {
    let query_lower = query.to_lowercase();
    let mut matching_messages: Vec<MessageMatch> = Vec::new();
    let page_size = 100i32;
    let mut cursor: Option<String> = None;
    let mut has_more = true;

    while has_more && matching_messages.len() < limit {
        let params = ListMessagesRequest {
            limit: Some(page_size),
            before: None,
            after: cursor.clone(),
            group_id: None,
            use_assistant_message: None,
            assistant_message_tool_name: None,
            assistant_message_tool_kwargs: None,
        };

        let messages = client
            .messages()
            .list(agent_id, Some(params))
            .await
            .map_err(|e| sdk_err("list messages", e))?;

        if messages.is_empty() {
            has_more = false;
            break;
        }

        for msg in &messages {
            let (id, date, message_type, content) = extract_message_info(msg);

            // Parse date for filtering
            if let Ok(msg_date) = DateTime::parse_from_rfc3339(&date) {
                let msg_date_utc = msg_date.with_timezone(&Utc);

                // Check date range
                if let Some(ref start) = start_date {
                    if msg_date_utc < *start {
                        continue;
                    }
                }
                if let Some(ref end) = end_date {
                    if msg_date_utc > *end {
                        // If we've passed the end date and we're paginating forward, stop
                        has_more = false;
                        break;
                    }
                }
            }

            // Check if content matches query (case-insensitive)
            if content.to_lowercase().contains(&query_lower) {
                let truncated_content = if content.len() > 500 {
                    format!("{}...", &content[..500])
                } else {
                    content
                };

                matching_messages.push(MessageMatch {
                    id,
                    date,
                    message_type,
                    content: truncated_content,
                });

                if matching_messages.len() >= limit {
                    break;
                }
            }
        }

        // Update cursor for next page
        if let Some(last_msg) = messages.last() {
            cursor = Some(extract_message_id(last_msg));
        }

        // If we got fewer messages than requested, we've reached the end
        if (messages.len() as i32) < page_size {
            has_more = false;
        }
    }

    // Sort by date (oldest first)
    matching_messages.sort_by(|a, b| a.date.cmp(&b.date));

    let count = matching_messages.len();
    Ok(MessageSearchResult {
        messages: matching_messages,
        count,
    })
}

/// Extract info from a LettaMessageUnion
fn extract_message_info(msg: &LettaMessageUnion) -> (String, String, String, String) {
    match msg {
        LettaMessageUnion::SystemMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "system_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::UserMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "user_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::AssistantMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "assistant_message".to_string(),
            m.content.clone(),
        ),
        LettaMessageUnion::ReasoningMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "reasoning_message".to_string(),
            m.reasoning.clone(),
        ),
        LettaMessageUnion::HiddenReasoningMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "hidden_reasoning_message".to_string(),
            "[hidden]".to_string(),
        ),
        LettaMessageUnion::ToolCallMessage(m) => {
            let tool_call_str = format!("{}({})", m.tool_call.name, m.tool_call.arguments);
            (
                m.id.to_string(),
                m.date.to_rfc3339(),
                "tool_call_message".to_string(),
                tool_call_str,
            )
        }
        LettaMessageUnion::ToolReturnMessage(m) => (
            m.id.to_string(),
            m.date.to_rfc3339(),
            "tool_return_message".to_string(),
            m.tool_return.clone(),
        ),
    }
}

/// Extract message ID from a LettaMessageUnion
fn extract_message_id(msg: &LettaMessageUnion) -> String {
    match msg {
        LettaMessageUnion::SystemMessage(m) => m.id.to_string(),
        LettaMessageUnion::UserMessage(m) => m.id.to_string(),
        LettaMessageUnion::AssistantMessage(m) => m.id.to_string(),
        LettaMessageUnion::ReasoningMessage(m) => m.id.to_string(),
        LettaMessageUnion::HiddenReasoningMessage(m) => m.id.to_string(),
        LettaMessageUnion::ToolCallMessage(m) => m.id.to_string(),
        LettaMessageUnion::ToolReturnMessage(m) => m.id.to_string(),
    }
}
