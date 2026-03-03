//! Agent Advanced Operations Tool
//!
//! Consolidated tool for all advanced agent operations using discriminator pattern.
//! This maintains backward compatibility with the Node.js implementation.

use letta::LettaClient;
use letta_types::{Message, Pagination, StandardResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use turbomcp::McpError;
use turbomcp_macros::FlattenTool;

use super::response_utils::truncate_with_indicator;

/// Agent operation discriminator
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentOperation {
    // CRUD operations
    List,
    Create,
    Get,
    Update,
    Delete,
    Search,
    ListTools,
    SendMessage,
    Export,
    Import,
    Clone,
    GetConfig,
    BulkDelete,
    // Advanced operations
    Context,
    ResetMessages,
    Summarize,
    Stream,
    AsyncMessage,
    CancelMessage,
    PreviewPayload,
    SearchMessages,
    GetMessage,
    Count,
    // Conversation operations
    ListConversations,
    GetConversation,
    SendConversationMessage,
    CancelConversation,
    CompactConversation,
}

/// Bulk delete filters
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct BulkDeleteFilters {
    /// Filter agents by name pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name_filter: Option<String>,

    /// Filter agents by tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tag_filter: Option<String>,

    /// Specific agent IDs to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

/// Search filters for messages
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct SearchFilters {
    /// Filter messages after this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// Filter messages before this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// Filter messages by role (user, assistant, system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Agent advanced request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct AgentAdvancedRequest {
    /// The operation to perform (list, create, get, update, delete, send_message, etc.)
    pub operation: AgentOperation,

    /// Agent ID (required for get, update, delete, and message operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Agent name (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Agent description (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// System prompt for the agent (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// LLM configuration object (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "value_object_schema")]
    pub llm_config: Option<Value>,

    /// Embedding model configuration (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "value_object_schema")]
    pub embedding_config: Option<Value>,

    /// Tool IDs to attach to agent (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "value_array_schema")]
    pub tool_ids: Option<Value>,

    /// Pagination settings (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "pagination_schema")]
    pub pagination: Option<Pagination>,

    /// Messages to send to agent (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,

    /// Enable streaming response (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Filters for bulk delete operation (agent_name_filter, agent_tag_filter, agent_ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "bulk_delete_filters_schema")]
    pub filters: Option<BulkDeleteFilters>,

    /// Search query text (for search_messages and search operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Tags to filter by (for search operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Search filters (for search_messages operation: start_date, end_date, role)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "search_filters_schema")]
    pub search_filters: Option<SearchFilters>,

    /// Agent export data (for import operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "value_object_schema")]
    pub export_data: Option<Value>,

    /// Update data object (for update operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "value_object_schema")]
    pub update_data: Option<Value>,

    /// Conversation ID (required for conversation operations: get_conversation, send_conversation_message, cancel_conversation, compact_conversation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    /// Message ID (for get_message operation; if omitted, lists recent messages with limit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Simple text message (for send_conversation_message; alternative to messages array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Schema helper for Value fields - generates object type
fn value_object_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({ "type": "object" })
}

fn value_array_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({ "type": "array" })
}

/// Schema helper for Pagination - adds explicit type to $ref
fn pagination_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut base_schema = gen.subschema_for::<Pagination>();
    // Insert the type field into the schema
    base_schema.insert("type".to_string(), serde_json::json!("object"));
    base_schema
}

/// Schema helper for BulkDeleteFilters - adds explicit type to $ref
fn bulk_delete_filters_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut base_schema = gen.subschema_for::<BulkDeleteFilters>();
    // Insert the type field into the schema
    base_schema.insert("type".to_string(), serde_json::json!("object"));
    base_schema
}

/// Schema helper for SearchFilters - adds explicit type to $ref
fn search_filters_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let mut base_schema = gen.subschema_for::<SearchFilters>();
    // Insert the type field into the schema
    base_schema.insert("type".to_string(), serde_json::json!("object"));
    base_schema
}

/// Main handler for agent advanced operations
pub async fn handle_agent_advanced(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();

    tracing::info!("Executing agent operation: {}", operation_str);

    let response = match request.operation {
        AgentOperation::List => handle_list_agents(client, request).await?,
        AgentOperation::Create => handle_create_agent(client, request).await?,
        AgentOperation::Get => handle_get_agent(client, request).await?,
        AgentOperation::Update => handle_update_agent(client, request).await?,
        AgentOperation::Delete => handle_delete_agent(client, request).await?,
        AgentOperation::Search => handle_search_agents(client, request).await?,
        AgentOperation::SendMessage => handle_send_message(client, request).await?,
        AgentOperation::ListTools => handle_list_tools(client, request).await?,
        AgentOperation::Export => handle_export_agent(client, request).await?,
        AgentOperation::Import => handle_import_agent(client, request).await?,
        AgentOperation::Clone => handle_clone_agent(client, request).await?,
        AgentOperation::GetConfig => handle_get_config(client, request).await?,
        AgentOperation::BulkDelete => handle_bulk_delete(client, request).await?,
        AgentOperation::Context => handle_get_context(client, request).await?,
        AgentOperation::ResetMessages => handle_reset_messages(client, request).await?,
        AgentOperation::Summarize => handle_summarize(client, request).await?,
        AgentOperation::Stream => handle_stream(client, request).await?,
        AgentOperation::AsyncMessage => handle_async_message(client, request).await?,
        AgentOperation::CancelMessage => handle_cancel_message(client, request).await?,
        AgentOperation::PreviewPayload => handle_preview_payload(client, request).await?,
        AgentOperation::SearchMessages => handle_search_messages(client, request).await?,
        AgentOperation::GetMessage => handle_get_message(client, request).await?,
        AgentOperation::Count => handle_count(client, request).await?,
        // Conversation operations
        AgentOperation::ListConversations => handle_list_conversations(client, request).await?,
        AgentOperation::GetConversation => handle_get_conversation(client, request).await?,
        AgentOperation::SendConversationMessage => {
            handle_send_conversation_message(client, request).await?
        }
        AgentOperation::CancelConversation => handle_cancel_conversation(client, request).await?,
        AgentOperation::CompactConversation => handle_compact_conversation(client, request).await?,
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

// ===================================================
// Operation Handlers
// ===================================================

async fn handle_list_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // LMS-48: Apply optimized defaults: limit=15, max=50
    let mut pagination = request.pagination.unwrap_or_default();

    // Override default limit from 50 to 15
    if pagination.limit.is_none() || pagination.limit == Some(50) {
        pagination.limit = Some(15);
    }

    // Cap at max limit of 50
    if let Some(limit) = pagination.limit {
        if limit > 50 {
            pagination.limit = Some(50);
        }
    }

    let offset = pagination.offset.unwrap_or(0);
    let effective_limit = pagination.limit.unwrap_or(15);

    // LMS-165: Fetch offset + limit items so we can apply client-side offset
    // (SDK uses cursor-based pagination, not offset-based)
    let fetch_count = (offset + effective_limit) as u32;

    // LMS-169: Forward name/tag filters to SDK for server-side filtering
    let params = letta::types::ListAgentsParams {
        limit: Some(fetch_count),
        name: request.name.clone(),
        tags: request.tags.clone(),
        ..Default::default()
    };

    // Call SDK method
    let agents = client
        .agents()
        .list(Some(params))
        .await
        .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

    // Optimization: only call count() if we got a full page (more pages likely exist).
    // If returned < fetch_count, we already know the total = fetched count.
    let all_count = agents.len() as u32;
    let total = if all_count < fetch_count {
        all_count
    } else {
        client.agents().count().await.unwrap_or(all_count)
    };

    // LMS-165: Apply client-side offset (SDK uses cursor, not offset)
    let agent_summaries: Vec<serde_json::Value> = agents
        .iter()
        .skip(offset)
        .take(effective_limit)
        .map(|agent| {
            let model = agent.llm_config.as_ref().map(|config| config.model.clone());
            let description = agent
                .description
                .as_ref()
                .map(|d| truncate_with_indicator(d, 100));

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
    if request.name.is_some() || request.tags.is_some() {
        hints.push("Results filtered by name/tags".to_string());
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

/// Search agents by name, tags, or query text
async fn handle_search_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // At least one search parameter must be provided
    if request.name.is_none() && request.tags.is_none() && request.query.is_none() {
        return Err(McpError::invalid_request(
            "At least one search parameter required: name, tags, or query".to_string(),
        ));
    }

    // Build search parameters using SDK types
    let params = letta::types::ListAgentsParams {
        name: request.name.clone(),
        tags: request.tags.clone(),
        query_text: request.query.clone(),
        limit: Some(50), // Max results for search
        ..Default::default()
    };

    // Execute search
    let agents = client
        .agents()
        .list(Some(params))
        .await
        .map_err(|e| McpError::internal(format!("Failed to search agents: {}", e)))?;

    // Build search criteria for response message
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

    // Create optimized agent summaries (same as list)
    let agent_summaries: Vec<serde_json::Value> = agents
        .iter()
        .map(|agent| {
            let model = agent.llm_config.as_ref().map(|config| config.model.clone());

            let description = agent
                .description
                .as_ref()
                .map(|d| truncate_with_indicator(d, 100));

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

async fn handle_create_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let name = request.name.ok_or_else(|| {
        McpError::invalid_request("name is required for create operation".to_string())
    })?;

    // Build the agent request with SDK types
    let mut agent_request = letta::types::CreateAgentRequest {
        name: Some(name),
        ..Default::default()
    };

    // Add optional fields if provided
    if let Some(system) = request.system {
        agent_request.system = Some(system);
    }

    // For complex types, parse from JSON Value to SDK types
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

    let agent = client
        .agents()
        .create(agent_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create agent: {}", e)))?;

    Ok(StandardResponse::success(
        "create",
        serde_json::to_value(agent)?,
        "Agent created successfully",
    ))
}

async fn handle_get_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get operation".to_string())
    })?;

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get agent: {}", e)))?;

    // LMS-48: Optimize response - truncate system prompt, return tool IDs only
    let mut agent_value = serde_json::to_value(&agent)?;

    // Truncate system prompt to 500 chars
    if let Some(system) = agent_value.get("system").and_then(|s| s.as_str()) {
        agent_value["system"] = serde_json::json!(truncate_with_indicator(system, 500));
    }

    // Truncate description to 200 chars
    if let Some(description) = agent_value.get("description").and_then(|d| d.as_str()) {
        agent_value["description"] = serde_json::json!(truncate_with_indicator(description, 200));
    }

    // Replace full tool objects with tool_ids array and tool_count
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
    // Remove full tools array to save space
    agent_value.as_object_mut().unwrap().remove("tools");

    Ok(StandardResponse::success(
        "get",
        agent_value,
        "Agent retrieved successfully (compact mode)",
    ))
}

async fn handle_update_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for update operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // LMS-173: Build raw JSON body to avoid LettaId roundtrip transformation.
    // Previous approach deserialized tool_ids through Vec<LettaId> which could
    // alter IDs during the UUID parse/serialize cycle, silently dropping tools.
    let mut body = serde_json::Map::new();

    // If update_data is provided, use it as the base (passthrough to Letta API)
    if let Some(Value::Object(data)) = request.update_data {
        body = data;
    }

    // Top-level fields override update_data
    if let Some(name) = request.name {
        body.insert("name".into(), Value::String(name));
    }
    if let Some(description) = request.description {
        body.insert("description".into(), Value::String(description));
    }
    if let Some(system) = request.system {
        body.insert("system".into(), Value::String(system));
    }
    if let Some(tags) = request.tags {
        body.insert("tags".into(), serde_json::to_value(tags)?);
    }

    // tool_ids: pass through as-is, no LettaId deserialization
    if let Some(tool_ids) = request.tool_ids {
        body.insert("tool_ids".into(), tool_ids);
    }

    // Complex config objects: pass through as-is
    if let Some(llm_config) = request.llm_config {
        body.insert("llm_config".into(), llm_config);
    }
    if let Some(embedding_config) = request.embedding_config {
        body.insert("embedding_config".into(), embedding_config);
    }

    if body.is_empty() {
        return Err(McpError::invalid_request(
            "No update fields provided. Supply tool_ids, name, description, system, tags, llm_config, embedding_config, or update_data".to_string(),
        ));
    }

    // Raw PATCH to avoid any type transformation in the SDK layer
    let url = letta::api::endpoints::agents::update(&letta_id);
    let agent: serde_json::Value = client
        .patch(&url, &Value::Object(body))
        .await
        .map_err(|e| McpError::internal(format!("Failed to update agent: {}", e)))?;

    Ok(StandardResponse::success(
        "update",
        agent,
        "Agent updated successfully",
    ))
}

async fn handle_delete_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for delete operation".to_string())
    })?;

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    client
        .agents()
        .delete(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete agent: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "delete",
        format!("Agent {} deleted successfully", letta_id),
    ))
}

async fn handle_send_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for send_message operation".to_string())
    })?;

    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages is required for send_message operation".to_string())
    })?;

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert Message structs to MessageCreate (SDK type)
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    // Build the request (no stream field in CreateMessagesRequest)
    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    // For streaming, we'd use client.messages().create_stream() instead
    // For now, use non-streaming create
    let response = client
        .messages()
        .create(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to send message: {}", e)))?;

    // LMS-48: Truncate assistant response to 1000 chars
    let mut response_value = serde_json::to_value(&response)?;

    // Try to find and truncate assistant message content
    if let Some(messages) = response_value
        .get_mut("messages")
        .and_then(|m| m.as_array_mut())
    {
        for msg in messages.iter_mut() {
            if let Some(content) = msg.get("text").and_then(|t| t.as_str()) {
                let original_length = content.len();
                if original_length > 1000 {
                    msg["text"] = serde_json::json!(truncate_with_indicator(content, 1000));
                    msg["full_response_length"] = serde_json::json!(original_length);
                }
            }
        }
    }

    // Add hint about full response
    response_value["hint"] = serde_json::json!("Full response visible in agent's message history");

    Ok(StandardResponse::success(
        "send_message",
        response_value,
        "Message sent successfully",
    ))
}

async fn handle_list_tools(
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
        .map_err(|e| McpError::internal(format!("Failed to list agent tools: {}", e)))?;

    // LMS-48: Default limit=25, return summary mode only
    let default_limit: usize = 25;
    let limit = request
        .pagination
        .and_then(|p| p.limit)
        .unwrap_or(default_limit)
        .min(default_limit);

    // Create tool summaries - exclude source_code, json_schema
    let tool_summaries: Vec<serde_json::Value> = tools
        .iter()
        .take(limit)
        .map(|tool| {
            let description = tool
                .description
                .as_ref()
                .map(|d| truncate_with_indicator(d, 80));
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
                // Exclude: source_code, json_schema, args_schema
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

async fn handle_export_agent(
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
        .map_err(|e| McpError::internal(format!("Failed to export agent: {}", e)))?;

    Ok(StandardResponse::success(
        "export",
        serde_json::json!({ "export_data": export_json }),
        "Agent exported successfully",
    ))
}

async fn handle_import_agent(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // Import requires file upload which is not directly supported in MCP tools
    // Would need special handling with multipart form data
    Err(McpError::internal(
        "Import operation not yet implemented - requires file upload support".to_string(),
    ))
}

async fn handle_clone_agent(
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

    // Get source agent
    let source_agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get source agent: {}", e)))?;

    // Create cloned agent with new name
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
        .map_err(|e| McpError::internal(format!("Failed to create cloned agent: {}", e)))?;

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

async fn handle_get_config(
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
        .map_err(|e| McpError::internal(format!("Failed to get agent: {}", e)))?;

    // Get agent tools (may fail if not accessible)
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

async fn handle_bulk_delete(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let filters = request.filters.ok_or_else(|| {
        McpError::invalid_request("filters are required for bulk_delete operation".to_string())
    })?;

    // Pre-compute agent_ids as HashSet for O(1) lookup instead of O(n) Vec::contains
    let id_filter: Option<HashSet<String>> = filters.agent_ids.map(|ids| ids.into_iter().collect());

    // Paginated fetch: collect matching agents without loading entire list at once
    let mut to_delete: Vec<letta::types::LettaId> = Vec::new();
    let mut cursor: Option<String> = None;
    let page_size = 50u32;

    loop {
        let params = letta::types::ListAgentsParams {
            limit: Some(page_size),
            after: cursor.clone(),
            ..Default::default()
        };

        let agents = client
            .agents()
            .list(Some(params))
            .await
            .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

        let page_len = agents.len();

        for agent in &agents {
            let mut should_delete = false;

            if let Some(ref name_filter) = filters.agent_name_filter {
                if agent.name.contains(name_filter) {
                    should_delete = true;
                }
            }

            if let Some(ref ids) = id_filter {
                if ids.contains(&agent.id.to_string()) {
                    should_delete = true;
                }
            }

            if should_delete {
                to_delete.push(agent.id.clone());
            }
        }

        // If we got fewer than page_size, we've reached the end
        if (page_len as u32) < page_size {
            break;
        }

        // Use last agent's ID as cursor for next page
        cursor = agents.last().map(|a| a.id.to_string());
    }

    if to_delete.is_empty() {
        return Ok(StandardResponse::success(
            "bulk_delete",
            serde_json::json!({
                "deleted_count": 0,
                "failed_count": 0,
                "errors": []
            }),
            "No agents matched the filter criteria",
        ));
    }

    // Delete concurrently using futures::join_all for O(1) wall-clock per batch
    let delete_futures: Vec<_> = to_delete
        .iter()
        .map(|agent_id| {
            let client = client.clone();
            let agent_id = agent_id.clone();
            async move {
                match client.agents().delete(&agent_id).await {
                    Ok(_) => Ok(agent_id),
                    Err(e) => Err((agent_id, e.to_string())),
                }
            }
        })
        .collect();

    let results = futures::future::join_all(delete_futures).await;

    let mut deleted_count = 0u32;
    let mut errors: Vec<serde_json::Value> = Vec::new();

    for result in results {
        match result {
            Ok(_) => deleted_count += 1,
            Err((agent_id, err_msg)) => {
                errors.push(serde_json::json!({
                    "agent_id": agent_id.to_string(),
                    "error": err_msg
                }));
            }
        }
    }

    let failed_count = errors.len() as u32;

    Ok(StandardResponse::success(
        "bulk_delete",
        serde_json::json!({
            "deleted_count": deleted_count,
            "failed_count": failed_count,
            "errors": errors
        }),
        format!("Deleted {} agents ({} failed)", deleted_count, failed_count),
    ))
}

async fn handle_get_context(
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
        .map_err(|e| McpError::internal(format!("Failed to get context: {}", e)))?;

    Ok(StandardResponse::success(
        "context",
        context,
        "Context retrieved successfully",
    ))
}

async fn handle_reset_messages(
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
        .map_err(|e| McpError::internal(format!("Failed to reset messages: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "reset_messages",
        "Messages reset successfully",
    ))
}

async fn handle_summarize(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for summarize operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Default to 10 max messages if not specified
    let max_message_length = 10u32;

    let agent_state = client
        .agents()
        .summarize_agent_conversation(&letta_id, max_message_length)
        .await
        .map_err(|e| McpError::internal(format!("Failed to summarize conversation: {}", e)))?;

    Ok(StandardResponse::success(
        "summarize",
        serde_json::to_value(agent_state)?,
        "Conversation summarized successfully",
    ))
}

async fn handle_stream(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // Streaming requires special handling and is not directly compatible with MCP tool responses
    Err(McpError::internal(
        "Stream operation not supported in MCP tool context - use async_message instead"
            .to_string(),
    ))
}

async fn handle_async_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for async_message operation".to_string())
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages are required for async_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert to MessageCreate types
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let run_id = client
        .messages()
        .create_async(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create async message: {}", e)))?;

    Ok(StandardResponse::success(
        "async_message",
        serde_json::json!({ "run_id": run_id }),
        "Async message created successfully",
    ))
}

async fn handle_cancel_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for cancel_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Note: SDK cancel takes Option<CancelAgentRunRequest>
    // For now, pass None to cancel the most recent run
    // TODO: Add run_id to request structure to cancel specific runs
    client
        .messages()
        .cancel(&letta_id, None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to cancel message: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "cancel_message",
        "Message cancelled successfully",
    ))
}

async fn handle_preview_payload(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for preview_payload operation".to_string())
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages are required for preview_payload operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert to MessageCreate types
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let preview = client
        .messages()
        .preview(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to preview payload: {}", e)))?;

    Ok(StandardResponse::success(
        "preview_payload",
        serde_json::to_value(preview)?,
        "Payload preview generated successfully",
    ))
}

async fn handle_search_messages(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let query = request.query.ok_or_else(|| {
        McpError::invalid_request("query is required for search_messages operation".to_string())
    })?;

    let search_request = letta::types::MessageSearchRequest {
        query: Some(query),
        ..Default::default()
    };

    let results = client
        .messages()
        .search(search_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to search messages: {}", e)))?;

    // LMS-48: Default limit=10, max=50, truncate message content to 200 chars
    let default_limit = 10;
    let max_limit = 50;
    let limit = request
        .pagination
        .and_then(|p| p.limit)
        .unwrap_or(default_limit)
        .min(max_limit);

    // Create message summaries
    let message_summaries: Vec<serde_json::Value> = results
        .iter()
        .take(limit)
        .map(|msg| {
            // Convert message to JSON to access fields
            let msg_value = serde_json::to_value(msg).unwrap_or(serde_json::json!({}));

            // Try to extract text content from different possible locations
            let content = msg_value
                .get("text")
                .or_else(|| msg_value.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or("");

            let content_length = content.len();
            let content_preview = truncate_with_indicator(content, 200);

            let role = msg_value
                .get("role")
                .and_then(|r| r.as_str())
                .unwrap_or("unknown");

            let created_at = msg_value
                .get("created_at")
                .and_then(|c| c.as_str())
                .unwrap_or("");

            let id = msg_value.get("id").and_then(|i| i.as_str()).unwrap_or("");

            serde_json::json!({
                "id": id,
                "role": role,
                "content_preview": content_preview,
                "content_length": content_length,
                "created_at": created_at,
            })
        })
        .collect();

    let total = results.len();
    let returned = message_summaries.len();
    let has_more = total > returned;

    let response_data = serde_json::json!({
        "total": total,
        "returned": returned,
        "has_more": has_more,
        "messages": message_summaries,
        "hint": "Use get_message with message_id for full content",
    });

    Ok(StandardResponse::success(
        "search_messages",
        response_data,
        format!("Found {} of {} messages (preview mode)", returned, total),
    ))
}

async fn handle_get_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // If message_id is provided, use SDK's list with targeted params then filter client-side
    if let Some(message_id) = request.message_id {
        let msg_id: letta::types::LettaId = message_id
            .parse()
            .map_err(|e| McpError::invalid_request(format!("Invalid message_id format: {}", e)))?;

        // Use the message_id as a cursor: fetch a small window around it
        let params = letta::types::ListMessagesRequest {
            before: Some(msg_id.to_string()),
            limit: Some(2), // Fetch the message and its predecessor
            ..Default::default()
        };

        let messages = client
            .messages()
            .list(&letta_id, Some(params))
            .await
            .map_err(|e| McpError::internal(format!("Failed to list messages: {}", e)))?;

        // Filter to the exact message
        let target = msg_id.to_string();
        let found: Vec<_> = messages
            .into_iter()
            .filter(|m| {
                let id_str = match m {
                    letta::types::LettaMessageUnion::SystemMessage(msg) => msg.id.to_string(),
                    letta::types::LettaMessageUnion::UserMessage(msg) => msg.id.to_string(),
                    letta::types::LettaMessageUnion::AssistantMessage(msg) => msg.id.to_string(),
                    letta::types::LettaMessageUnion::ReasoningMessage(msg) => msg.id.to_string(),
                    letta::types::LettaMessageUnion::HiddenReasoningMessage(msg) => {
                        msg.id.to_string()
                    }
                    letta::types::LettaMessageUnion::ToolCallMessage(msg) => msg.id.to_string(),
                    letta::types::LettaMessageUnion::ToolReturnMessage(msg) => msg.id.to_string(),
                };
                id_str == target
            })
            .collect();

        if found.is_empty() {
            return Err(McpError::invalid_request(format!(
                "Message {} not found for agent {}",
                target, letta_id
            )));
        }

        return Ok(StandardResponse::success(
            "get_message",
            serde_json::to_value(&found[0])?,
            "Message retrieved successfully",
        ));
    }

    // No message_id: list recent messages with a capped limit (default 20, max 100)
    let default_limit = 20i32;
    let max_limit = 100i32;
    let limit = request
        .pagination
        .and_then(|p| p.limit)
        .map(|l| (l as i32).min(max_limit))
        .unwrap_or(default_limit);

    let params = letta::types::ListMessagesRequest {
        limit: Some(limit),
        ..Default::default()
    };

    let messages = client
        .messages()
        .list(&letta_id, Some(params))
        .await
        .map_err(|e| McpError::internal(format!("Failed to list messages: {}", e)))?;

    let count = messages.len();

    Ok(StandardResponse::success(
        "get_message",
        serde_json::to_value(&messages)?,
        format!(
            "Retrieved {} messages (limit={}). Use message_id for a specific message.",
            count, limit
        ),
    ))
}

async fn handle_count(
    client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let count = client
        .agents()
        .count()
        .await
        .map_err(|e| McpError::internal(format!("Failed to count agents: {}", e)))?;

    Ok(StandardResponse::success(
        "count",
        serde_json::json!({ "count": count }),
        format!("Total agents: {}", count),
    ))
}

// ===================================================
// Conversation Operation Handlers
// ===================================================

async fn handle_list_conversations(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request(
            "agent_id is required for list_conversations operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let conversations = client
        .conversations()
        .list(&letta_id, None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list conversations: {}", e)))?;

    let count = conversations.len();

    Ok(StandardResponse::success(
        "list_conversations",
        serde_json::to_value(&conversations)?,
        format!("Found {} conversations", count),
    ))
}

async fn handle_get_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = request.conversation_id.ok_or_else(|| {
        McpError::invalid_request(
            "conversation_id is required for get_conversation operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = conversation_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid conversation_id format: {}", e)))?;

    let conversation = client
        .conversations()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get conversation: {}", e)))?;

    Ok(StandardResponse::success(
        "get_conversation",
        serde_json::to_value(&conversation)?,
        "Conversation retrieved successfully",
    ))
}

async fn handle_send_conversation_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = request.conversation_id.ok_or_else(|| {
        McpError::invalid_request(
            "conversation_id is required for send_conversation_message operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = conversation_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid conversation_id format: {}", e)))?;

    // Build ConversationMessageRequest from available fields
    let mut msg_request = letta::types::ConversationMessageRequest::default();

    // Simple text message → use input field
    if let Some(message) = request.message {
        msg_request.input = Some(serde_json::json!(message));
    }

    // Structured messages array → serialize to JSON Value
    if let Some(messages) = request.messages {
        msg_request.messages = Some(serde_json::to_value(messages)?);
    }

    if let Some(stream) = request.stream {
        msg_request.streaming = Some(stream);
    }

    let response = client
        .conversations()
        .send_message(&letta_id, msg_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to send conversation message: {}", e)))?;

    Ok(StandardResponse::success(
        "send_conversation_message",
        serde_json::to_value(&response)?,
        "Message sent to conversation successfully",
    ))
}

async fn handle_cancel_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = request.conversation_id.ok_or_else(|| {
        McpError::invalid_request(
            "conversation_id is required for cancel_conversation operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = conversation_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid conversation_id format: {}", e)))?;

    let result = client
        .conversations()
        .cancel(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to cancel conversation: {}", e)))?;

    Ok(StandardResponse::success(
        "cancel_conversation",
        result,
        "Conversation cancelled successfully",
    ))
}

async fn handle_compact_conversation(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let conversation_id = request.conversation_id.ok_or_else(|| {
        McpError::invalid_request(
            "conversation_id is required for compact_conversation operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = conversation_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid conversation_id format: {}", e)))?;

    let result = client
        .conversations()
        .compact(&letta_id, None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to compact conversation: {}", e)))?;

    Ok(StandardResponse::success(
        "compact_conversation",
        result,
        "Conversation compacted successfully",
    ))
}
