//! Memory Unified Operations Tool
//!
//! Consolidated tool for all memory operations using discriminator pattern.
//! Supports core memory, memory blocks, and archival/passage operations.

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

    // Ignored parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
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
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passages: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_memory: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
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
        MemoryOperation::ListAgentsUsingBlock => handle_list_agents_using_block(client, request).await,
        MemoryOperation::SearchArchival => handle_search_archival(client, request).await,
        MemoryOperation::ListPassages => handle_list_passages(client, request).await,
        MemoryOperation::CreatePassage => handle_create_passage(client, request).await,
        MemoryOperation::UpdatePassage => handle_update_passage(client, request).await,
        MemoryOperation::DeletePassage => handle_delete_passage(client, request).await,
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
        .map_err(|e| McpError::internal(format!("Failed to get core memory: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_core_memory".to_string(),
        message: "Core memory retrieved successfully".to_string(),
        agent_id: Some(agent_id),
        core_memory: Some(serde_json::to_value(memory)?),
        block_id: None,
        passage_id: None,
        data: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to update core memory: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_core_memory".to_string(),
        message: format!("Core memory block '{}' updated successfully", block_label),
        agent_id: Some(agent_id),
        data: Some(serde_json::to_value(updated_block)?),
        block_id: None,
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to get block by label: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block_by_label".to_string(),
        message: format!("Block '{}' retrieved successfully", block_label),
        agent_id: Some(agent_id),
        data: Some(serde_json::to_value(block)?),
        block_id: None,
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to list blocks: {}", e)))?;

    let count = blocks.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_blocks".to_string(),
        message: format!("Found {} blocks", count),
        agent_id: Some(agent_id),
        blocks: Some(serde_json::to_value(&blocks)?),
        count: Some(count),
        block_id: None,
        passage_id: None,
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
        .map_err(|e| McpError::internal(format!("Failed to create block: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_block".to_string(),
        message: "Block created successfully".to_string(),
        agent_id: None,
        block_id: block.id.as_ref().map(|id| id.to_string()),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to get block: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block".to_string(),
        message: "Block retrieved successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to update block: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_block".to_string(),
        message: "Block updated successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(serde_json::to_value(block)?),
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to attach block: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "attach_block".to_string(),
        message: "Block attached to agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: Some(block_id),
        data: Some(serde_json::to_value(agent_state)?),
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to detach block: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "detach_block".to_string(),
        message: "Block detached from agent successfully".to_string(),
        agent_id: Some(agent_id),
        block_id: Some(block_id),
        data: Some(serde_json::to_value(agent_state)?),
        passage_id: None,
        core_memory: None,
        blocks: None,
        passages: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to search archival: {}", e)))?;

    let count = passages.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "search_archival".to_string(),
        message: format!("Found {} passages", count),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        count: Some(count),
        block_id: None,
        passage_id: None,
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
        .map_err(|e| McpError::internal(format!("Failed to list passages: {}", e)))?;

    let count = passages.len();

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_passages".to_string(),
        message: format!("Found {} passages", count),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        count: Some(count),
        block_id: None,
        passage_id: None,
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
        .map_err(|e| McpError::internal(format!("Failed to create passage: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_passage".to_string(),
        message: "Passage created successfully".to_string(),
        agent_id: Some(agent_id),
        passages: Some(serde_json::to_value(&passages)?),
        block_id: None,
        passage_id: None,
        core_memory: None,
        data: None,
        blocks: None,
        count: None,
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
        .map_err(|e| McpError::internal(format!("Failed to delete passage: {}", e)))?;

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "delete_passage".to_string(),
        message: "Passage deleted successfully".to_string(),
        agent_id: Some(agent_id),
        passage_id: Some(passage_id),
        block_id: None,
        core_memory: None,
        data: None,
        blocks: None,
        passages: None,
        count: None,
    })
}
