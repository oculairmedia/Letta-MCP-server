use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use futures::stream::{self, StreamExt};
use letta::LettaClient;
use letta_types::StandardResponse;
use std::collections::HashSet;
use turbomcp::McpError;

use super::{truncate_text, AgentAdvancedRequest};

fn build_bulk_delete_list_params(
    cursor: Option<String>,
    page_size: u32,
) -> letta::types::ListAgentsParams {
    letta::types::ListAgentsParams {
        limit: Some(page_size),
        after: cursor,
        ..Default::default()
    }
}

fn resolve_get_config_results<T, U, E1, E2>(
    agent_result: Result<T, E1>,
    tools_result: Result<U, E2>,
) -> Result<(T, U), McpError>
where
    E1: std::fmt::Display,
    E2: std::fmt::Display,
{
    let agent = agent_result.map_err(|e| sdk_err("get agent", e))?;
    let tools = tools_result.map_err(|e| sdk_err("list agent tools", e))?;
    Ok((agent, tools))
}

pub(crate) async fn handle_list_tools(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for list_tools operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for export operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let export_data = require_field(
        request.export_data,
        "export_data is required for import operation (JSON from agent export)",
    )?;

    let json_bytes = serde_json::to_vec(&export_data).map_err(|e| {
        McpError::invalid_request(format!("Failed to serialize export_data: {}", e))
    })?;

    let tmp_dir = tempfile::tempdir().map_err(|e| sdk_err("create temp directory", e))?;
    let tmp_path = tmp_dir.path().join("agent_import.json");

    tokio::fs::write(&tmp_path, &json_bytes)
        .await
        .map_err(|e| sdk_err("write temp file", e))?;

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
    let agent_id = require_field(request.agent_id, "agent_id is required for clone operation")?;
    let new_name = require_field(request.name, "name is required for clone operation")?;
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for get_config operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

    // Fan-out: get agent + list tools in parallel
    let agents_api = client.agents();
    let memory_api = client.memory();
    let (agent_result, tools_result) = tokio::join!(
        agents_api.get(&letta_id),
        memory_api.list_agent_tools(&letta_id)
    );

    let (agent, tools) = resolve_get_config_results(agent_result, tools_result)?;

    Ok(StandardResponse::success(
        "get_config",
        serde_json::json!({
            "name": agent.name,
            "description": agent.description,
            "system": agent.system,
            "llm_config": agent.llm_config,
            "embedding_config": agent.embedding_config,
            "tools": tools,
            "created_at": agent.created_at,
        }),
        "Agent configuration retrieved successfully",
    ))
}

pub(crate) async fn handle_bulk_delete(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let filters = require_field(
        request.filters,
        "filters are required for bulk_delete operation",
    )?;

    let has_name_filter = filters.agent_name_filter.is_some();
    let has_id_filter = filters.agent_ids.is_some();

    if !has_name_filter && !has_id_filter {
        return Err(McpError::invalid_request(
            "At least one filter (agent_name_filter or agent_ids) is required".to_string(),
        ));
    }

    let id_filter: Option<HashSet<String>> = filters
        .agent_ids
        .clone()
        .map(|ids| ids.into_iter().collect());

    let mut all_agents = Vec::new();
    let page_size = 50u32;
    let mut cursor: Option<String> = None;
    loop {
        let list_params = build_bulk_delete_list_params(cursor.clone(), page_size);
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
        cursor = all_agents.last().map(|agent| agent.id.to_string());
        // Safety cap to prevent infinite loops
        if all_agents.len() > 10_000 {
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

        let id_matches = match &id_filter {
            Some(ids) => ids.contains(&agent.id.to_string()),
            None => true,
        };

        if name_matches && id_matches {
            to_delete.push(agent.id);
        }
    }

    // Concurrent deletes with bounded concurrency
    let matched = to_delete.len();
    let results: Vec<bool> = stream::iter(to_delete)
        .map(|agent_id| async move { client.agents().delete(&agent_id).await.is_ok() })
        .buffer_unordered(10)
        .collect()
        .await;
    let deleted_count = results.iter().filter(|&&ok| ok).count();

    Ok(StandardResponse::success(
        "bulk_delete",
        serde_json::json!({
            "total_scanned": total_scanned,
            "matched": matched,
            "deleted_count": deleted_count,
            "failed_count": matched - deleted_count,
            "filter_logic": "AND (all provided filters must match)"
        }),
        format!("Deleted {} agents", deleted_count),
    ))
}

pub(crate) async fn handle_get_context(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for context operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for reset_messages operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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
    let agent_id = require_field(
        request.agent_id,
        "agent_id is required for summarize operation",
    )?;
    let letta_id = require_id(Some(agent_id), "agent_id")?;

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

#[cfg(test)]
mod tests {
    use super::{build_bulk_delete_list_params, resolve_get_config_results};

    #[test]
    fn bulk_delete_list_params_start_without_cursor() {
        let params = build_bulk_delete_list_params(None, 50);

        assert_eq!(params.limit, Some(50));
        assert_eq!(params.after, None);
        assert_eq!(params.before, None);
    }

    #[test]
    fn bulk_delete_list_params_advance_with_cursor() {
        let params = build_bulk_delete_list_params(
            Some("agent-12345678-1234-1234-1234-123456789012".to_string()),
            50,
        );

        assert_eq!(params.limit, Some(50));
        assert_eq!(
            params.after,
            Some("agent-12345678-1234-1234-1234-123456789012".to_string())
        );
        assert_eq!(params.before, None);
    }

    #[test]
    fn resolve_get_config_results_returns_both_values() {
        let result = resolve_get_config_results::<_, _, &str, &str>(Ok("agent"), Ok(vec![1, 2]));

        let (agent, tools) = result.expect("parallel results should succeed");
        assert_eq!(agent, "agent");
        assert_eq!(tools, vec![1, 2]);
    }

    #[test]
    fn resolve_get_config_results_propagates_tool_failures() {
        let result =
            resolve_get_config_results::<_, Vec<i32>, &str, &str>(Ok("agent"), Err("boom"));

        let err = result.expect_err("tool failures must be surfaced");
        assert!(err.to_string().contains("Failed to list agent tools: boom"));
    }
}
