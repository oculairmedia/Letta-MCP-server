use crate::tools::memory_utils::{truncate_block_value, BlockSummary};
use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use std::str::FromStr;
use turbomcp::McpError;

use super::{MemoryUnifiedRequest, MemoryUnifiedResponse};

const BLOCK_VALUE_TRUNCATE_LEN: usize = 500;

pub(crate) async fn handle_get_block_by_label(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_block_by_label".to_string())
    })?;
    let block_label = request.block_label.ok_or_else(|| {
        McpError::invalid_request("block_label is required for get_block_by_label".to_string())
    })?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let block = client
        .memory()
        .get_core_memory_block(&letta_id, &block_label)
        .await
        .map_err(|e| sdk_err("get block by label", e))?;

    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block_by_label".to_string(),
        message: format!("Block '{}' retrieved successfully", block_label),
        agent_id: Some(agent_id),
        data: Some(block_value),
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

pub(crate) async fn handle_list_blocks(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for list_blocks".to_string())
    })?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let blocks = client
        .memory()
        .list_core_memory_blocks(&letta_id)
        .await
        .map_err(|e| sdk_err("list blocks", e))?;

    let count = blocks.len();

    let blocks_data = if verbose {
        serde_json::to_value(&blocks)?
    } else {
        let blocks_value = serde_json::to_value(&blocks)?;
        let summaries: Vec<BlockSummary> = blocks_value
            .as_array()
            .map(|arr| arr.iter().map(BlockSummary::from_block_value).collect())
            .unwrap_or_default();
        serde_json::to_value(&summaries)?
    };

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "list_blocks".to_string(),
        message: format!(
            "Found {} blocks{}",
            count,
            if verbose { "" } else { " (compact, use verbose=true for full values)" }
        ),
        agent_id: Some(agent_id),
        blocks: Some(blocks_data),
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

pub(crate) async fn handle_create_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let label = request.label.ok_or_else(|| {
        McpError::invalid_request("label is required for create_block".to_string())
    })?;
    let value = request.value.ok_or_else(|| {
        McpError::invalid_request("value is required for create_block".to_string())
    })?;
    let verbose = request.verbose.unwrap_or(false);

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

    let block_id = block.id.as_ref().map(|id| id.to_string());
    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "create_block".to_string(),
        message: "Block created successfully".to_string(),
        agent_id: None,
        block_id,
        data: Some(block_value),
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

pub(crate) async fn handle_get_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for get_block".to_string())
    })?;
    let verbose = request.verbose.unwrap_or(false);

    let letta_id = letta::types::LettaId::from_str(&block_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid block_id: {}", e)))?;

    let block = client
        .blocks()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get block", e))?;

    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "get_block".to_string(),
        message: "Block retrieved successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(block_value),
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

pub(crate) async fn handle_update_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let block_id = request.block_id.ok_or_else(|| {
        McpError::invalid_request("block_id is required for update_block".to_string())
    })?;
    let verbose = request.verbose.unwrap_or(false);

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

    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(MemoryUnifiedResponse {
        success: true,
        operation: "update_block".to_string(),
        message: "Block updated successfully".to_string(),
        agent_id: None,
        block_id: Some(block_id),
        data: Some(block_value),
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

pub(crate) async fn handle_attach_block(
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

    let _agent_state = client
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
        data: Some(serde_json::json!({
            "attached": true,
            "hint": "Use get_core_memory to see updated blocks"
        })),
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

pub(crate) async fn handle_detach_block(
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

    let _agent_state = client
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
        data: Some(serde_json::json!({
            "detached": true,
            "hint": "Use get_core_memory to see updated blocks"
        })),
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

pub(crate) async fn handle_list_agents_using_block(
    _client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    Err(McpError::internal(
        "list_agents_using_block not yet implemented - requires custom query".to_string(),
    ))
}
