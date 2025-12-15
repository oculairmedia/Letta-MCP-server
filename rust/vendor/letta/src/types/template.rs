//! Template-related types for the Letta API.

use crate::types::common::LettaId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template item in list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateItem {
    /// Template name.
    pub name: String,
    /// Template ID.
    pub id: String,
}

/// Response from listing templates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesListResponse {
    /// List of templates.
    pub templates: Vec<TemplateItem>,
    /// Whether there are more pages.
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

/// Request to create a template from an agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTemplateRequest {
    /// Project to create the template in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

/// Response from creating a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesCreateResponse {
    /// Name of the created template.
    pub template_name: String,
    /// ID of the created template.
    pub template_id: String,
}

/// Request to create a new version of a template.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionTemplateRequest {
    /// Whether to migrate deployed agents to the new version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_deployed_agents: Option<bool>,
    /// Version message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Request to migrate an agent to a new template version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrateAgentRequest {
    /// Template to migrate to (format: "template-name:version" or "template-name:latest").
    pub to_template: String,
    /// Whether to preserve core memories.
    pub preserve_core_memories: bool,
    /// Variables to set if not preserving core memories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
}

/// Response from migrating an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesMigrateResponse {
    /// The migrated agent state.
    pub agent: crate::types::agent::AgentState,
}

/// Request to create agents from a template.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentsFromTemplateRequest {
    /// Tags to apply to created agents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Name for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Memory variables to set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_variables: Option<HashMap<String, String>>,
    /// Tool variables to set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_variables: Option<HashMap<String, String>>,
    /// Identity IDs to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ids: Option<Vec<LettaId>>,
}

/// Response from creating agents from a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsCreateResponse {
    /// Created agents.
    pub agents: Vec<crate::types::agent::AgentState>,
}

/// Response from getting agent memory variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryVariablesListResponse {
    /// Memory variables as key-value pairs.
    pub variables: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_item_serialization() {
        let item = TemplateItem {
            name: "test-template".to_string(),
            id: "template-123".to_string(),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("test-template"));
        assert!(json.contains("template-123"));
    }

    #[test]
    fn test_migrate_request_serialization() {
        let request = MigrateAgentRequest {
            to_template: "my-template:v2".to_string(),
            preserve_core_memories: true,
            variables: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("my-template:v2"));
        assert!(json.contains("true"));
        assert!(!json.contains("variables"));
    }
}
