//! Agent Advanced Operations Tool
//!
//! Consolidated tool for all advanced agent operations using discriminator pattern.
//! Split into domain sub-modules: crud, messaging, conversations, management.

mod conversations;
mod crud;
mod management;
mod messaging;

use letta::LettaClient;
use letta_types::{Message, Pagination};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use turbomcp::McpError;

// ===================================================
// Shared Types
// ===================================================

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
    ListConversations,
    GetConversation,
    SendConversationMessage,
    CancelConversation,
    CompactConversation,
}

pub(crate) fn truncate_text(text: &str, max_chars: usize) -> String {
    crate::tools::response_utils::truncate_with_indicator(text, max_chars)
}

/// Agent summary for list operations (consistent with ToolSummary, JobSummary, SourceSummary)
#[derive(Debug, Serialize)]
pub(crate) struct AgentSummary {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub tool_count: usize,
}

impl AgentSummary {
    pub fn from_agent(agent: &letta::types::AgentState) -> Self {
        Self {
            id: agent.id.to_string(),
            name: agent.name.clone(),
            description: agent.description.as_ref().map(|d| truncate_text(d, 100)),
            model: agent.llm_config.as_ref().map(|c| c.model.clone()),
            created_at: agent.created_at.map(|ts| ts.to_string()),
            tool_count: agent.tools.len(),
        }
    }
}

/// Bulk delete filters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
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
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AgentAdvancedRequest {
    /// The operation to perform (list, create, get, update, delete, send_message, etc.)
    #[schemars(schema_with = "operation_schema")]
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "value_object_schema")]
    pub llm_config: Option<Value>,

    /// Embedding model configuration (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "value_object_schema")]
    pub embedding_config: Option<Value>,

    /// Tool IDs to attach to agent (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "tool_ids_schema")]
    pub tool_ids: Option<Value>,

    /// Pagination settings (for list operations)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "pagination_schema")]
    pub pagination: Option<Pagination>,

    /// Messages to send to agent (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "messages_schema")]
    pub messages: Option<Vec<Message>>,

    /// Enable streaming response (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Filters for bulk delete operation (agent_name_filter, agent_tag_filter, agent_ids)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "bulk_delete_filters_schema")]
    pub filters: Option<BulkDeleteFilters>,

    /// Search query text (for search_messages and search operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Tags to filter by (for search operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Search filters (for search_messages operation: start_date, end_date, role)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "search_filters_schema")]
    pub search_filters: Option<SearchFilters>,

    /// Agent export data (for import operation)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "value_object_schema")]
    pub export_data: Option<Value>,

    /// Update data object (for update operation)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[schemars(schema_with = "value_object_schema")]
    pub update_data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(default)]
    pub verbose: Option<bool>,
}

// ===================================================
// Schema Helpers
// ===================================================

/// Schema helper for Value fields - generates object type
fn value_object_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({ "type": "object" })
}

fn tool_ids_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "array",
        "items": { "type": "string" }
    })
}

/// Schema helper for AgentOperation - fully inlined enum to avoid $ref
fn operation_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Agent operation discriminator",
        "type": "string",
        "enum": [
            "list", "create", "get", "update", "delete", "search",
            "list_tools", "send_message", "export", "import", "clone",
            "get_config", "bulk_delete", "context", "reset_messages",
            "summarize", "stream", "async_message", "cancel_message",
            "preview_payload", "search_messages", "get_message", "count",
            "list_conversations", "get_conversation", "send_conversation_message",
            "cancel_conversation", "compact_conversation"
        ]
    })
}

/// Schema helper for Pagination - fully inlined to avoid $ref
fn pagination_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Common pagination parameters",
        "type": "object",
        "properties": {
            "limit": {
                "type": ["integer", "null"],
                "format": "uint",
                "minimum": 0
            },
            "offset": {
                "type": ["integer", "null"],
                "format": "uint",
                "minimum": 0
            }
        }
    })
}

/// Schema helper for BulkDeleteFilters - fully inlined to avoid $ref
fn bulk_delete_filters_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Bulk delete filters",
        "type": "object",
        "properties": {
            "agent_name_filter": {
                "description": "Filter agents by name pattern",
                "type": ["string", "null"]
            },
            "agent_tag_filter": {
                "description": "Filter agents by tag",
                "type": ["string", "null"]
            },
            "agent_ids": {
                "description": "Specific agent IDs to delete",
                "type": ["array", "null"],
                "items": { "type": "string" }
            }
        }
    })
}

/// Schema helper for SearchFilters - fully inlined to avoid $ref
fn search_filters_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Search filters for messages",
        "type": "object",
        "properties": {
            "start_date": {
                "description": "Filter messages after this date (ISO 8601 format)",
                "type": ["string", "null"]
            },
            "end_date": {
                "description": "Filter messages before this date (ISO 8601 format)",
                "type": ["string", "null"]
            },
            "role": {
                "description": "Filter messages by role (user, assistant, system)",
                "type": ["string", "null"]
            }
        }
    })
}

/// Schema helper for Messages array - fully inlined to avoid $ref
fn messages_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "description": "Messages to send to agent (for send_message operation)",
        "type": ["array", "null"],
        "items": {
            "description": "Message structure for agent communication",
            "type": "object",
            "properties": {
                "role": { "type": "string" },
                "content": { "type": "string" }
            },
            "required": ["role", "content"]
        }
    })
}

// ===================================================
// Dispatcher
// ===================================================

/// Main handler for agent advanced operations
pub async fn handle_agent_advanced(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();

    tracing::info!("Executing agent operation: {}", operation_str);

    let response = match request.operation {
        // CRUD
        AgentOperation::List => crud::handle_list_agents(client, request).await?,
        AgentOperation::Create => crud::handle_create_agent(client, request).await?,
        AgentOperation::Get => crud::handle_get_agent(client, request).await?,
        AgentOperation::Update => crud::handle_update_agent(client, request).await?,
        AgentOperation::Delete => crud::handle_delete_agent(client, request).await?,
        AgentOperation::Search => crud::handle_search_agents(client, request).await?,
        // Messaging
        AgentOperation::SendMessage => messaging::handle_send_message(client, request).await?,
        AgentOperation::Stream => messaging::handle_stream(client, request).await?,
        AgentOperation::AsyncMessage => messaging::handle_async_message(client, request).await?,
        AgentOperation::CancelMessage => messaging::handle_cancel_message(client, request).await?,
        AgentOperation::SearchMessages => {
            messaging::handle_search_messages(client, request).await?
        }
        AgentOperation::GetMessage => messaging::handle_get_message(client, request).await?,
        AgentOperation::Count => messaging::handle_count(client, request).await?,
        AgentOperation::PreviewPayload => {
            messaging::handle_preview_payload(client, request).await?
        }
        // Conversations
        AgentOperation::ListConversations => {
            conversations::handle_list_conversations(client, request).await?
        }
        AgentOperation::GetConversation => {
            conversations::handle_get_conversation(client, request).await?
        }
        AgentOperation::SendConversationMessage => {
            conversations::handle_send_conversation_message(client, request).await?
        }
        AgentOperation::CancelConversation => {
            conversations::handle_cancel_conversation(client, request).await?
        }
        AgentOperation::CompactConversation => {
            conversations::handle_compact_conversation(client, request).await?
        }
        // Management
        AgentOperation::ListTools => management::handle_list_tools(client, request).await?,
        AgentOperation::Export => management::handle_export_agent(client, request).await?,
        AgentOperation::Import => management::handle_import_agent(client, request).await?,
        AgentOperation::Clone => management::handle_clone_agent(client, request).await?,
        AgentOperation::GetConfig => management::handle_get_config(client, request).await?,
        AgentOperation::BulkDelete => management::handle_bulk_delete(client, request).await?,
        AgentOperation::Context => management::handle_get_context(client, request).await?,
        AgentOperation::ResetMessages => management::handle_reset_messages(client, request).await?,
        AgentOperation::Summarize => management::handle_summarize(client, request).await?,
    };

    Ok(serde_json::to_string_pretty(&response)?)
}
