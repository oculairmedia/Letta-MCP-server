use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use std::str::FromStr;
use turbomcp::McpError;

use super::{MemoryUnifiedRequest, MemoryUnifiedResponse};

pub(crate) async fn handle_get_core_memory(
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

pub(crate) async fn handle_update_core_memory(
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
