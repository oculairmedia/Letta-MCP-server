use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use std::str::FromStr;
use turbomcp::McpError;

use super::{MemoryUnifiedRequest, MemoryUnifiedResponse};

pub(crate) async fn handle_search_archival(
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

pub(crate) async fn handle_list_passages(
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

pub(crate) async fn handle_create_passage(
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

pub(crate) async fn handle_update_passage(
    _client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    Err(McpError::internal(
        "update_passage not yet implemented - SDK requires embedding data not available from client".to_string(),
    ))
}

pub(crate) async fn handle_delete_passage(
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
