use crate::tools::validation_utils::{require_field, require_id, sdk_err};
use letta::LettaClient;
use turbomcp::McpError;

use super::MemoryUnifiedRequest;
use crate::tools::response_utils::ToolResponse;

pub(crate) async fn handle_list_archives(
    client: &LettaClient,
    _request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
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

    Ok(
        ToolResponse::success("list_archives", format!("Found {} archives", count))
            .with_count(count)
            .with_extra(serde_json::json!({
                "archival": {
                    "passages": archive_values,
                    "count": count,
                },
            })),
    )
}

pub(crate) async fn handle_get_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let archive_id = require_field(request.archive_id, "archive_id is required for get_archive")?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let archive = client
        .archives()
        .get(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("get archive", e))?;

    Ok(
        ToolResponse::success("get_archive", "Archive retrieved successfully")
            .with_json_data(serde_json::to_value(archive)?)
            .with_extra(serde_json::json!({
                "archive_id": archive_id,
            })),
    )
}

pub(crate) async fn handle_create_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let name = require_field(request.label, "label is required for create_archive")?;

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

    Ok(
        ToolResponse::success("create_archive", "Archive created successfully")
            .with_json_data(serde_json::to_value(&archive)?)
            .with_extra(serde_json::json!({
                "archive_id": archive.id.as_ref().map(|id| id.to_string()),
            })),
    )
}

pub(crate) async fn handle_update_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let archive_id = require_field(
        request.archive_id,
        "archive_id is required for update_archive",
    )?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let update_request = letta::types::ArchiveUpdateRequest {
        name: request.label,
        description: request.text,
    };

    let archive = client
        .archives()
        .update(&letta_archive_id, update_request)
        .await
        .map_err(|e| sdk_err("update archive", e))?;

    Ok(
        ToolResponse::success("update_archive", "Archive updated successfully")
            .with_json_data(serde_json::to_value(archive)?)
            .with_extra(serde_json::json!({
                "archive_id": archive_id,
            })),
    )
}

pub(crate) async fn handle_delete_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let archive_id = require_field(
        request.archive_id,
        "archive_id is required for delete_archive",
    )?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let response = client
        .archives()
        .delete(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("delete archive", e))?;

    Ok(
        ToolResponse::success("delete_archive", "Archive deleted successfully")
            .with_json_data(serde_json::to_value(response)?)
            .with_extra(serde_json::json!({
                "archive_id": archive_id,
            })),
    )
}

pub(crate) async fn handle_attach_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for attach_archive")?;
    let archive_id = require_field(
        request.archive_id,
        "archive_id is required for attach_archive",
    )?;
    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let agent_state = client
        .archives()
        .agent_archives(letta_agent_id)
        .attach(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("attach archive", e))?;

    Ok(
        ToolResponse::success("attach_archive", "Archive attached to agent successfully")
            .with_json_data(serde_json::to_value(agent_state)?)
            .with_extra(serde_json::json!({
                "agent_id": agent_id,
                "archive_id": archive_id,
            })),
    )
}

pub(crate) async fn handle_detach_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let agent_id = require_field(request.agent_id, "agent_id is required for detach_archive")?;
    let archive_id = require_field(
        request.archive_id,
        "archive_id is required for detach_archive",
    )?;
    let letta_agent_id = require_id(Some(agent_id.clone()), "agent_id")?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let agent_state = client
        .archives()
        .agent_archives(letta_agent_id)
        .detach(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("detach archive", e))?;

    Ok(
        ToolResponse::success("detach_archive", "Archive detached from agent successfully")
            .with_json_data(serde_json::to_value(agent_state)?)
            .with_extra(serde_json::json!({
                "agent_id": agent_id,
                "archive_id": archive_id,
            })),
    )
}

pub(crate) async fn handle_list_agents_using_archive(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<ToolResponse, McpError> {
    let archive_id = require_field(
        request.archive_id,
        "archive_id is required for list_agents_using_archive",
    )?;
    let letta_archive_id = require_id(Some(archive_id.clone()), "archive_id")?;

    let agents = client
        .archives()
        .list_agents(&letta_archive_id)
        .await
        .map_err(|e| sdk_err("list agents using archive", e))?;

    let count = agents.len();

    Ok(ToolResponse::success(
        "list_agents_using_archive",
        format!("Found {} agents using archive", count),
    )
    .with_count(count)
    .with_json_data(serde_json::to_value(&agents)?)
    .with_extra(serde_json::json!({
        "archive_id": archive_id,
    })))
}
