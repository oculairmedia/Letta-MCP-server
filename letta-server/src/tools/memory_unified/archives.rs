use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use std::str::FromStr;
use turbomcp::McpError;

use super::{ArchivalSearchResult, MemoryUnifiedRequest, MemoryUnifiedResponse};

pub(crate) async fn handle_list_archives(
    client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    let archives = client
        .archives()
        .list()
        .await
        .map_err(|e| sdk_err("list archives", e))?;

    let count = archives.len();

    let archive_values: Vec<serde_json::Value> = archives
        .iter()
        .map(|a| serde_json::to_value(a))
        .collect::<Result<Vec<_>, _>>()?;

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
            passages: archive_values,
            count,
        }),
        messages: None,
    })
}

pub(crate) async fn handle_get_archive(
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

pub(crate) async fn handle_create_archive(
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

pub(crate) async fn handle_update_archive(
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

pub(crate) async fn handle_delete_archive(
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

pub(crate) async fn handle_attach_archive(
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

pub(crate) async fn handle_detach_archive(
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

pub(crate) async fn handle_list_agents_using_archive(
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
