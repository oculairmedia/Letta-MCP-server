use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use letta_types::StandardResponse;
use turbomcp::McpError;

use super::{AgentAdvancedRequest, AgentSummary, truncate_text};
use crate::tools::response_utils::paginate;

pub(crate) async fn handle_list_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let pagination = request.pagination.unwrap_or_default();
    let (limit, offset) = paginate(
        pagination.limit.map(|l| l as usize),
        pagination.offset.map(|o| o as usize),
        15,  // agent lists default smaller
        50,
    );

    let params = letta::types::ListAgentsParams {
        limit: Some(limit as u32),
        ..Default::default()
    };

    // Fan-out: list + count in parallel
    let agents_api = client.agents();
    let count_api = client.agents();
    let (agents_result, count_result) = tokio::join!(
        agents_api.list(Some(params)),
        count_api.count()
    );

    let agents = agents_result.map_err(|e| sdk_err("list agents", e))?;
    let total = count_result.unwrap_or(agents.len() as u32);

    let agent_summaries: Vec<AgentSummary> = agents
        .iter()
        .map(AgentSummary::from_agent)
        .collect();

    let returned = agent_summaries.len() as u32;
    let has_more = total > (offset as u32 + returned);

    let mut hints = vec!["Use 'get' with agent_id for full details".to_string()];
    if has_more {
        let next_offset = offset + (returned as usize);
        hints.push(format!("Use offset={} for next page", next_offset));
    }

    let response_data = serde_json::json!({
        "total": total,
        "returned": returned,
        "offset": offset,
        "has_more": has_more,
        "agents": agent_summaries,
        "hints": hints,
    });

    Ok(StandardResponse::success(
        "list",
        response_data,
        format!("Retrieved {} of {} agents (summary mode)", returned, total),
    ))
}

pub(crate) async fn handle_search_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    if request.name.is_none() && request.tags.is_none() && request.query.is_none() {
        return Err(McpError::invalid_request(
            "At least one search parameter required: name, tags, or query".to_string(),
        ));
    }

    let params = letta::types::ListAgentsParams {
        name: request.name.clone(),
        tags: request.tags.clone(),
        query_text: request.query.clone(),
        limit: Some(50),
        ..Default::default()
    };

    let agents = client
        .agents()
        .list(Some(params))
        .await
        .map_err(|e| sdk_err("search agents", e))?;

    let mut criteria = Vec::new();
    if let Some(ref name) = request.name {
        criteria.push(format!("name='{}'", name));
    }
    if let Some(ref tags) = request.tags {
        criteria.push(format!("tags={:?}", tags));
    }
    if let Some(ref query) = request.query {
        criteria.push(format!("query='{}'", query));
    }
    let criteria_str = criteria.join(", ");

    let agent_summaries: Vec<AgentSummary> = agents
        .iter()
        .map(AgentSummary::from_agent)
        .collect();

    let count = agent_summaries.len();

    let response_data = serde_json::json!({
        "count": count,
        "agents": agent_summaries,
        "search_criteria": {
            "name": request.name,
            "tags": request.tags,
            "query": request.query,
        },
        "hint": "Use 'get' with agent_id for full details",
    });

    Ok(StandardResponse::success(
        "search",
        response_data,
        format!("Found {} agents matching: {}", count, criteria_str),
    ))
}

pub(crate) async fn handle_create_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let name = require_field(request.name, "name is required for create operation")?;

    let mut agent_request = letta::types::CreateAgentRequest {
        name: Some(name),
        ..Default::default()
    };

    if let Some(system) = request.system {
        agent_request.system = Some(system);
    }

    if let Some(llm_config_value) = request.llm_config {
        let llm_config: letta::types::LLMConfig = serde_json::from_value(llm_config_value)
            .map_err(|e| McpError::invalid_request(format!("Invalid llm_config: {}", e)))?;
        agent_request.llm_config = Some(llm_config);
    }

    if let Some(embedding_config_value) = request.embedding_config {
        let embedding_config: letta::types::EmbeddingConfig =
            serde_json::from_value(embedding_config_value).map_err(|e| {
                McpError::invalid_request(format!("Invalid embedding_config: {}", e))
            })?;
        agent_request.embedding_config = Some(embedding_config);
    }

    if let Some(tool_ids_value) = request.tool_ids {
        let tool_ids: Vec<letta::types::LettaId> = serde_json::from_value(tool_ids_value)
            .map_err(|e| McpError::invalid_request(format!("Invalid tool_ids: {}", e)))?;
        agent_request.tool_ids = Some(tool_ids);
    }

    let verbose = request.verbose.unwrap_or(false);

    let agent = client
        .agents()
        .create(agent_request)
        .await
        .map_err(|e| sdk_err("create agent", e))?;

    let data = if verbose {
        serde_json::to_value(&agent)?
    } else {
        serde_json::json!({
            "id": agent.id,
            "name": agent.name,
            "agent_type": agent.agent_type,
            "tool_count": agent.tools.len(),
        })
    };

    Ok(StandardResponse::success(
        "create",
        data,
        "Agent created successfully",
    ))
}

pub(crate) async fn handle_get_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for get operation")?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

    let verbose = request.verbose.unwrap_or(false);

    let agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| sdk_err("get agent", e))?;

    if verbose {
        let agent_value = serde_json::to_value(&agent)?;
        return Ok(StandardResponse::success(
            "get",
            agent_value,
            "Agent retrieved successfully (verbose mode)",
        ));
    }

    // Compact mode: strip heavy fields, keep only what's useful
    let tool_ids: Vec<String> = agent
        .tools
        .iter()
        .filter_map(|tool_ref| match tool_ref {
            letta::types::agent::ToolReference::Id(id) => Some(id.clone()),
            letta::types::agent::ToolReference::Object(obj) => obj
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
        .collect();
    let tool_count = tool_ids.len();

    let model = agent.llm_config.as_ref().map(|c| c.model.clone());
    let embedding_model = agent.embedding_config.as_ref().and_then(|c| c.embedding_model.clone());

    let memory_block_labels: Vec<String> = agent
        .memory
        .as_ref()
        .map(|m| m.blocks.iter().map(|b| b.label.clone()).collect())
        .unwrap_or_default();
    let memory_block_count = memory_block_labels.len();

    let compact = serde_json::json!({
        "id": agent.id.to_string(),
        "name": agent.name,
        "agent_type": agent.agent_type,
        "description": agent.description.as_ref().map(|d| truncate_text(d, 200)),
        "system": agent.system.as_ref().map(|s| truncate_text(s, 300)),
        "model": model,
        "embedding_model": embedding_model,
        "tool_count": tool_count,
        "tool_ids": tool_ids,
        "memory_block_count": memory_block_count,
        "memory_block_labels": memory_block_labels,
        "created_at": agent.created_at.map(|ts| ts.to_string()),
        "updated_at": agent.updated_at.map(|ts| ts.to_string()),
        "tags": agent.tags,
        "hint": "Use verbose=true for full agent data, or get_core_memory for memory values",
    });

    Ok(StandardResponse::success(
        "get",
        compact,
        "Agent retrieved successfully (compact mode)",
    ))
}

pub(crate) async fn handle_update_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for update operation")?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

    let mut update_request = letta::types::UpdateAgentRequest::default();

    // Two modes: flat fields (name, system, etc.) or update_data object.
    // Flat fields override update_data when both are provided.
    if let Some(update_data) = request.update_data {
        let parsed: letta::types::UpdateAgentRequest =
            serde_json::from_value(update_data).map_err(|e| {
                McpError::invalid_request(format!("Invalid update_data: {}", e))
            })?;
        update_request = parsed;
    }
    if let Some(name) = request.name {
        update_request.name = Some(name);
    }
    if let Some(description) = request.description {
        update_request.description = Some(description);
    }
    if let Some(system) = request.system {
        update_request.system = Some(system);
    }
    if let Some(tags) = request.tags {
        update_request.tags = Some(tags);
    }
    if let Some(llm_config_value) = request.llm_config {
        let llm_config: letta::types::LLMConfig =
            serde_json::from_value(llm_config_value).map_err(|e| {
                McpError::invalid_request(format!("Invalid llm_config: {}", e))
            })?;
        update_request.llm_config = Some(llm_config);
    }
    if let Some(embedding_config_value) = request.embedding_config {
        let embedding_config: letta::types::EmbeddingConfig =
            serde_json::from_value(embedding_config_value).map_err(|e| {
                McpError::invalid_request(format!("Invalid embedding_config: {}", e))
            })?;
        update_request.embedding_config = Some(embedding_config);
    }

    let verbose = request.verbose.unwrap_or(false);

    let agent = client
        .agents()
        .update(&letta_id, update_request)
        .await
        .map_err(|e| sdk_err("update agent", e))?;

    let data = if verbose {
        serde_json::to_value(&agent)?
    } else {
        serde_json::json!({
            "id": agent.id.to_string(),
            "name": agent.name,
            "agent_type": agent.agent_type,
            "description": agent.description,
            "model": agent.llm_config.as_ref().map(|c| &c.model),
            "tags": agent.tags,
            "updated_at": agent.updated_at.map(|ts| ts.to_string()),
        })
    };

    Ok(StandardResponse::success(
        "update",
        data,
        format!("Agent {} updated successfully", letta_id),
    ))
}

pub(crate) async fn handle_delete_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for delete operation")?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

    client
        .agents()
        .delete(&letta_id)
        .await
        .map_err(|e| sdk_err("delete agent", e))?;

    Ok(StandardResponse::success_no_data(
        "delete",
        format!("Agent {} deleted successfully", letta_id),
    ))
}
