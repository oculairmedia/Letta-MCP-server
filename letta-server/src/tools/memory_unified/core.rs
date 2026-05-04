use crate::tools::memory_utils::BlockSummary;
use crate::tools::response_utils::limits::core_memory_preview_len;
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use turbomcp::McpError;

use super::MemoryUnifiedRequest;
use crate::tools::response_utils::ToolResponse;

pub(crate) async fn handle_get_core_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for get_core_memory")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let memory = client
        .memory()
        .get_core_memory(&letta_id)
        .await
        .map_err(|e| sdk_err("get core memory", e))?;

    if verbose {
        return Ok(ToolResponse::success(
            "get_core_memory",
            "Core memory retrieved successfully (verbose)",
        )
        .with_json_data(serde_json::to_value(memory)?)
        .with_extra(serde_json::json!({ "agent_id": agent_id })));
    }

    // Compact mode: return block summaries with value previews
    let memory_value = serde_json::to_value(&memory)?;
    let block_summaries: Vec<BlockSummary> = memory_value
        .get("blocks")
        .and_then(|b| b.as_array())
        .map(|blocks| blocks.iter().map(BlockSummary::from_block_value).collect())
        .unwrap_or_default();

    let count = block_summaries.len();

    Ok(ToolResponse::success(
        "get_core_memory",
        format!(
            "Core memory: {} blocks (compact mode, use verbose=true for full values)",
            count
        ),
    )
    .with_extra(serde_json::json!({
        "agent_id": agent_id,
        "core_memory": serde_json::to_value(&block_summaries)?,
    }))
    .with_count(count))
}

pub(crate) async fn handle_update_core_memory(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for update_core_memory",
    )?;
    let block_label = require_field(
        request.block_label,
        "block_label is required for update_core_memory",
    )?;
    let value = require_field(request.value, "value is required for update_core_memory")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

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

    let mut block_value = serde_json::to_value(updated_block)?;
    if !verbose {
        crate::tools::memory_utils::truncate_block_value(&mut block_value, core_memory_preview_len());
    }

    Ok(ToolResponse::success(
        "update_core_memory",
        format!("Core memory block '{}' updated successfully", block_label),
    )
    .with_json_data(block_value)
    .with_extra(serde_json::json!({ "agent_id": agent_id })))
}
