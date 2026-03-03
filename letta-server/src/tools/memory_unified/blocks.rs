use crate::tools::memory_utils::{BlockSummary, truncate_block_value};
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use turbomcp::McpError;

use super::MemoryUnifiedRequest;
use crate::tools::response_utils::ToolResponse;

const BLOCK_VALUE_TRUNCATE_LEN: usize = 500;

pub(crate) async fn handle_get_block_by_label(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for get_block_by_label",
    )?;
    let block_label = require_field(
        request.block_label,
        "block_label is required for get_block_by_label",
    )?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let block = client
        .memory()
        .get_core_memory_block(&letta_id, &block_label)
        .await
        .map_err(|e| sdk_err("get block by label", e))?;

    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(ToolResponse::success(
        "get_block_by_label",
        format!("Block '{}' retrieved successfully", block_label),
    )
    .with_json_data(block_value)
    .with_extra(serde_json::json!({ "agent_id": agent_id })))
}

pub(crate) async fn handle_list_blocks(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for list_blocks")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

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

    Ok(ToolResponse::success(
        "list_blocks",
        format!(
            "Found {} blocks{}",
            count,
            if verbose {
                ""
            } else {
                " (compact, use verbose=true for full values)"
            }
        ),
    )
    .with_count(count)
    .with_extra(serde_json::json!({
        "agent_id": agent_id,
        "blocks": blocks_data,
    })))
}

pub(crate) async fn handle_create_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let label = require_field(request.label, "label is required for create_block")?;
    let value = require_field(request.value, "value is required for create_block")?;
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

    Ok(
        ToolResponse::success("create_block", "Block created successfully")
            .with_json_data(block_value)
            .with_extra(serde_json::json!({ "block_id": block_id })),
    )
}

pub(crate) async fn handle_get_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let block_id = require_field(request.block_id, "block_id is required for get_block")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(block_id.clone()), "block_id")?;

    let block = client
        .blocks()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get block", e))?;

    let mut block_value = serde_json::to_value(block)?;
    if !verbose {
        truncate_block_value(&mut block_value, BLOCK_VALUE_TRUNCATE_LEN);
    }

    Ok(
        ToolResponse::success("get_block", "Block retrieved successfully")
            .with_json_data(block_value)
            .with_extra(serde_json::json!({ "block_id": block_id })),
    )
}

pub(crate) async fn handle_update_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let block_id = require_field(request.block_id, "block_id is required for update_block")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(block_id.clone()), "block_id")?;

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

    Ok(
        ToolResponse::success("update_block", "Block updated successfully")
            .with_json_data(block_value)
            .with_extra(serde_json::json!({ "block_id": block_id })),
    )
}

pub(crate) async fn handle_attach_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for attach_block")?;
    let block_id = require_field(request.block_id, "block_id is required for attach_block")?;
    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_block_id = require_id(Some(block_id.clone()), "block_id")?;

    let _agent_state = client
        .memory()
        .attach_memory_block(&letta_agent_id, &letta_block_id)
        .await
        .map_err(|e| sdk_err("attach block", e))?;

    Ok(
        ToolResponse::success("attach_block", "Block attached to agent successfully")
            .with_json_data(serde_json::json!({
                "attached": true,
                "hint": "Use get_core_memory to see updated blocks"
            }))
            .with_extra(serde_json::json!({ "agent_id": agent_id, "block_id": block_id })),
    )
}

pub(crate) async fn handle_detach_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for detach_block")?;
    let block_id = require_field(request.block_id, "block_id is required for detach_block")?;
    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_block_id = require_id(Some(block_id.clone()), "block_id")?;

    let _agent_state = client
        .memory()
        .detach_memory_block(&letta_agent_id, &letta_block_id)
        .await
        .map_err(|e| sdk_err("detach block", e))?;

    Ok(
        ToolResponse::success("detach_block", "Block detached from agent successfully")
            .with_json_data(serde_json::json!({
                "detached": true,
                "hint": "Use get_core_memory to see updated blocks"
            }))
            .with_extra(serde_json::json!({ "agent_id": agent_id, "block_id": block_id })),
    )
}

pub(crate) async fn handle_list_agents_using_block(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let block_id = require_field(
        request.block_id,
        "block_id is required for list_agents_using_block",
    )?;
    let letta_block_id = require_id(Some(block_id.clone()), "block_id")?;

    let limit = request.limit.and_then(|l| u32::try_from(l).ok());

    let agents = client
        .blocks()
        .list_agents(&letta_block_id, limit)
        .await
        .map_err(|e| sdk_err("list agents using block", e))?;

    let count = agents.len();
    let verbose = request.verbose.unwrap_or(false);

    let agents_data = if verbose {
        serde_json::to_value(&agents)?
    } else {
        let summaries: Vec<serde_json::Value> = agents
            .iter()
            .map(|agent| {
                serde_json::json!({
                    "id": agent.id.to_string(),
                    "name": agent.name,
                    "description": agent.description,
                    "model": agent.llm_config.as_ref().map(|c| &c.model),
                })
            })
            .collect();
        serde_json::to_value(&summaries)?
    };

    Ok(ToolResponse::success(
        "list_agents_using_block",
        format!("Found {} agents using block {}", count, block_id),
    )
    .with_json_data(agents_data)
    .with_count(count)
    .with_extra(serde_json::json!({ "block_id": block_id })))
}
