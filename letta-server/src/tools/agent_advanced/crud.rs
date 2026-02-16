use crate::tools::validation_utils::sdk_err;
use letta::LettaClient;
use letta_types::StandardResponse;
use turbomcp::McpError;

use super::{truncate_text, AgentAdvancedRequest};

pub(crate) async fn handle_list_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let mut pagination = request.pagination.unwrap_or_default();

    if pagination.limit.is_none() || pagination.limit == Some(50) {
        pagination.limit = Some(15);
    }

    if let Some(limit) = pagination.limit {
        if limit > 50 {
            pagination.limit = Some(50);
        }
    }

    let offset = pagination.offset.unwrap_or(0);

    let params = letta::types::ListAgentsParams {
        limit: pagination.limit.map(|l| l as u32),
        ..Default::default()
    };

    let agents = client
        .agents()
        .list(Some(params))
        .await
        .map_err(|e| sdk_err("list agents", e))?;

    let total = client.agents().count().await.unwrap_or(agents.len() as u32);

    let agent_summaries: Vec<serde_json::Value> = agents
        .iter()
        .map(|agent| {
            let model = agent.llm_config.as_ref().map(|config| config.model.clone());
            let description = agent.description.as_ref().map(|d| truncate_text(d, 100));

            serde_json::json!({
                "id": agent.id.to_string(),
                "name": agent.name,
                "description": description,
                "model": model,
                "created_at": agent.created_at.map(|ts| ts.to_string()),
                "tool_count": agent.tools.len(),
            })
        })
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

    let agent_summaries: Vec<serde_json::Value> = agents
        .iter()
        .map(|agent| {
            let model = agent.llm_config.as_ref().map(|config| config.model.clone());
            let description = agent.description.as_ref().map(|d| truncate_text(d, 100));

            serde_json::json!({
                "id": agent.id.to_string(),
                "name": agent.name,
                "description": description,
                "model": model,
                "created_at": agent.created_at.map(|ts| ts.to_string()),
                "tool_count": agent.tools.len(),
            })
        })
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
    let name = request.name.ok_or_else(|| {
        McpError::invalid_request("name is required for create operation".to_string())
    })?;

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
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

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

    // Compact mode (default): truncate large fields, replace tools with IDs
    let mut agent_value = serde_json::to_value(&agent)?;

    if let Some(system) = agent_value.get("system").and_then(|s| s.as_str()) {
        agent_value["system"] = serde_json::json!(truncate_text(system, 500));
    }
    if let Some(description) = agent_value.get("description").and_then(|d| d.as_str()) {
        agent_value["description"] = serde_json::json!(truncate_text(description, 200));
    }

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

    agent_value["tool_ids"] = serde_json::json!(tool_ids);
    agent_value["tool_count"] = serde_json::json!(tool_count);
    agent_value.as_object_mut().unwrap().remove("tools");

    Ok(StandardResponse::success(
        "get",
        agent_value,
        "Agent retrieved successfully (compact mode)",
    ))
}

pub(crate) async fn handle_update_agent(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    Err(McpError::internal(
        "Agent update operation not yet implemented in SDK v0.1.2. \
         Please use specific update operations (memory, tools, etc.)"
            .to_string(),
    ))
}

pub(crate) async fn handle_delete_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for delete operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

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
