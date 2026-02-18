use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use letta_types::StandardResponse;
use turbomcp::McpError;

use super::{truncate_text, AgentAdvancedRequest};

pub(crate) async fn handle_list_tools(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for list_tools operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let tools = client
        .memory()
        .list_agent_tools(&letta_id)
        .await
        .map_err(|e| sdk_err("list agent tools", e))?;

    let default_limit: usize = 25;
    let limit = request
        .pagination
        .and_then(|p| p.limit)
        .unwrap_or(default_limit)
        .min(default_limit);

    let tool_summaries: Vec<serde_json::Value> = tools
        .iter()
        .take(limit)
        .map(|tool| {
            let description = tool.description.as_ref().map(|d| truncate_text(d, 80));
            let id = tool
                .id
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or_default();

            serde_json::json!({
                "id": id,
                "name": tool.name,
                "description": description,
                "source_type": tool.source_type,
            })
        })
        .collect();

    let total = tools.len();
    let returned = tool_summaries.len();
    let has_more = total > returned;

    let response_data = serde_json::json!({
        "total": total,
        "returned": returned,
        "has_more": has_more,
        "tools": tool_summaries,
        "hint": "Use tool manager for full tool details including source code",
    });

    Ok(StandardResponse::success(
        "list_tools",
        response_data,
        format!("Retrieved {} of {} tools (summary mode)", returned, total),
    ))
}

pub(crate) async fn handle_export_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for export operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let export_json = client
        .agents()
        .export_file(&letta_id)
        .await
        .map_err(|e| sdk_err("export agent", e))?;

    Ok(StandardResponse::success(
        "export",
        serde_json::json!({ "export_data": export_json }),
        "Agent exported successfully",
    ))
}

pub(crate) async fn handle_import_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let export_data = request.export_data.ok_or_else(|| {
        McpError::invalid_request(
            "export_data is required for import operation (JSON from agent export)".to_string(),
        )
    })?;

    let json_bytes = serde_json::to_vec(&export_data)
        .map_err(|e| McpError::invalid_request(format!("Failed to serialize export_data: {}", e)))?;

    let tmp_dir = tempfile::tempdir()
        .map_err(|e| McpError::internal(format!("Failed to create temp directory: {}", e)))?;
    let tmp_path = tmp_dir.path().join("agent_import.json");

    tokio::fs::write(&tmp_path, &json_bytes)
        .await
        .map_err(|e| McpError::internal(format!("Failed to write temp file: {}", e)))?;

    let import_request = letta::types::ImportAgentRequest::default();

    let agent = client
        .agents()
        .import_file(&tmp_path, import_request)
        .await
        .map_err(|e| sdk_err("import agent", e))?;

    Ok(StandardResponse::success(
        "import",
        serde_json::json!({
            "id": agent.id.to_string(),
            "name": agent.name,
            "agent_type": agent.agent_type,
            "description": agent.description,
        }),
        "Agent imported successfully",
    ))
}

pub(crate) async fn handle_clone_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for clone operation".to_string())
    })?;
    let new_name = request.name.ok_or_else(|| {
        McpError::invalid_request("name is required for clone operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let source_agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get source agent", e))?;

    let clone_request = letta::types::CreateAgentRequest {
        name: Some(new_name.clone()),
        description: source_agent.description.clone(),
        system: source_agent.system.clone(),
        llm_config: source_agent.llm_config.clone(),
        embedding_config: source_agent.embedding_config.clone(),
        ..Default::default()
    };

    let new_agent = client
        .agents()
        .create(clone_request)
        .await
        .map_err(|e| sdk_err("create cloned agent", e))?;

    Ok(StandardResponse::success(
        "clone",
        serde_json::json!({
            "source_agent_id": agent_id,
            "new_agent_id": new_agent.id.to_string(),
            "new_agent_name": new_name
        }),
        "Agent cloned successfully",
    ))
}

pub(crate) async fn handle_get_config(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_config operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get agent", e))?;

    let tools = client.memory().list_agent_tools(&letta_id).await.ok();

    Ok(StandardResponse::success(
        "get_config",
        serde_json::json!({
            "name": agent.name,
            "description": agent.description,
            "system": agent.system,
            "llm_config": agent.llm_config,
            "embedding_config": agent.embedding_config,
            "tools": tools.unwrap_or_default(),
            "created_at": agent.created_at,
        }),
        "Agent configuration retrieved successfully",
    ))
}

pub(crate) async fn handle_bulk_delete(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let filters = request.filters.ok_or_else(|| {
        McpError::invalid_request("filters are required for bulk_delete operation".to_string())
    })?;

    let has_name_filter = filters.agent_name_filter.is_some();
    let has_id_filter = filters.agent_ids.is_some();

    if !has_name_filter && !has_id_filter {
        return Err(McpError::invalid_request(
            "At least one filter (agent_name_filter or agent_ids) is required".to_string(),
        ));
    }

    let mut all_agents = Vec::new();
    let page_size = 50u32;
    let mut offset = 0u32;
    loop {
        let list_params = letta::types::ListAgentsParams {
            limit: Some(page_size),
            ..Default::default()
        };
        let page = client
            .agents()
            .list(Some(list_params))
            .await
            .map_err(|e| sdk_err("list agents", e))?;

        let page_len = page.len() as u32;
        all_agents.extend(page);

        if page_len < page_size {
            break;
        }
        offset += page_len;
        // Safety cap to prevent infinite loops
        if offset > 10_000 {
            break;
        }
    }

    let total_scanned = all_agents.len();
    let mut to_delete: Vec<letta::types::LettaId> = Vec::new();

    for agent in all_agents {
        // AND logic: all provided filters must match
        let name_matches = match &filters.agent_name_filter {
            Some(name_filter) => agent.name.contains(name_filter),
            None => true,
        };

        let id_matches = match &filters.agent_ids {
            Some(ids) => ids.contains(&agent.id.to_string()),
            None => true,
        };

        if name_matches && id_matches {
            to_delete.push(agent.id);
        }
    }

    let mut deleted_count = 0;
    for agent_id in &to_delete {
        if client.agents().delete(agent_id).await.is_ok() {
            deleted_count += 1;
        }
    }

    Ok(StandardResponse::success(
        "bulk_delete",
        serde_json::json!({
            "total_scanned": total_scanned,
            "matched": to_delete.len(),
            "deleted_count": deleted_count,
            "failed_count": to_delete.len() - deleted_count,
            "filter_logic": "AND (all provided filters must match)"
        }),
        format!("Deleted {} agents", deleted_count),
    ))
}

pub(crate) async fn handle_get_context(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for context operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let context = client
        .agents()
        .get_context(&letta_id)
        .await
        .map_err(|e| sdk_err("get context", e))?;

    Ok(StandardResponse::success(
        "context",
        context,
        "Context retrieved successfully",
    ))
}

pub(crate) async fn handle_reset_messages(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for reset_messages operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    client
        .agents()
        .reset_messages(&letta_id)
        .await
        .map_err(|e| sdk_err("reset messages", e))?;

    Ok(StandardResponse::success_no_data(
        "reset_messages",
        "Messages reset successfully",
    ))
}

pub(crate) async fn handle_summarize(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for summarize operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let max_message_length = 10u32;

    let agent_state = client
        .agents()
        .summarize_agent_conversation(&letta_id, max_message_length)
        .await
        .map_err(|e| sdk_err("summarize conversation", e))?;

    Ok(StandardResponse::success(
        "summarize",
        serde_json::to_value(agent_state)?,
        "Conversation summarized successfully",
    ))
}
