use crate::tools::memory_utils::{PassageSummary, truncate_passage_text};
use crate::tools::response_utils::limits::max_value_len;
use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use turbomcp::McpError;

use super::MemoryUnifiedRequest;
use crate::tools::response_utils::ToolResponse;

pub(crate) async fn handle_search_archival(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for search_archival")?;
    let query = require_field(request.query, "query is required for search_archival")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: Some(query),
        limit: request.limit.and_then(|l| u32::try_from(l).ok()),
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

    let passages_data = if verbose {
        serde_json::to_value(&passages)?
    } else {
        let passages_value = serde_json::to_value(&passages)?;
        let summaries: Vec<PassageSummary> = passages_value
            .as_array()
            .map(|arr| arr.iter().map(PassageSummary::from_passage_value).collect())
            .unwrap_or_default();
        serde_json::to_value(&summaries)?
    };

    Ok(
        ToolResponse::success("search_archival", format!("Found {} passages", count))
            .with_count(count)
            .with_extra(serde_json::json!({
                "agent_id": agent_id,
                "passages": passages_data,
            })),
    )
}

pub(crate) async fn handle_list_passages(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for list_passages")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let params = letta::types::memory::ArchivalMemoryQueryParams {
        search: None,
        limit: request.limit.and_then(|l| u32::try_from(l).ok()),
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

    let passages_data = if verbose {
        serde_json::to_value(&passages)?
    } else {
        let passages_value = serde_json::to_value(&passages)?;
        let summaries: Vec<PassageSummary> = passages_value
            .as_array()
            .map(|arr| arr.iter().map(PassageSummary::from_passage_value).collect())
            .unwrap_or_default();
        serde_json::to_value(&summaries)?
    };

    Ok(ToolResponse::success(
        "list_passages",
        format!(
            "Found {} passages{}",
            count,
            if verbose {
                ""
            } else {
                " (compact, use verbose=true for full text)"
            }
        ),
    )
    .with_count(count)
    .with_extra(serde_json::json!({
        "agent_id": agent_id,
        "passages": passages_data,
    })))
}

pub(crate) async fn handle_create_passage(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for create_passage")?;
    let text = require_field(request.text, "text is required for create_passage")?;
    let verbose = request.verbose.unwrap_or(false);
    let letta_id = require_id(Some(agent_id.clone()), "agent_id")?;

    let create_request = letta::types::memory::CreateArchivalMemoryRequest { text };

    let passages = client
        .memory()
        .create_archival_memory(&letta_id, create_request)
        .await
        .map_err(|e| sdk_err("create passage", e))?;

    let mut passages_value = serde_json::to_value(&passages)?;
    if !verbose && let Some(arr) = passages_value.as_array_mut() {
        for p in arr.iter_mut() {
            truncate_passage_text(p, max_value_len());
        }
    }

    Ok(
        ToolResponse::success("create_passage", "Passage created successfully").with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "passages": passages_value,
            }),
        ),
    )
}

pub(crate) async fn handle_update_passage(
    _client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    Err(McpError::invalid_request(
        "The Letta API does not provide a PATCH endpoint for passages. \
         To modify a passage, delete it with delete_passage and recreate it with create_passage."
            .to_string(),
    ))
}

pub(crate) async fn handle_delete_passage(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for delete_passage")?;
    let passage_id = require_field(
        request.passage_id,
        "passage_id is required for delete_passage",
    )?;
    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_passage_id = require_id(Some(passage_id.clone()), "passage_id")?;

    client
        .memory()
        .delete_archival_memory(&letta_agent_id, &letta_passage_id)
        .await
        .map_err(|e| sdk_err("delete passage", e))?;

    Ok(
        ToolResponse::success("delete_passage", "Passage deleted successfully").with_extra(
            serde_json::json!({
                "agent_id": agent_id,
                "passage_id": passage_id,
            }),
        ),
    )
}
