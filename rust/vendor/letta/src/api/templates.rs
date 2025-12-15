//! Template management API endpoints (Cloud only).

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    AgentsCreateResponse, CreateAgentsFromTemplateRequest, CreateTemplateRequest, LettaId,
    MemoryVariablesListResponse, MigrateAgentRequest, TemplatesCreateResponse,
    TemplatesListResponse, TemplatesMigrateResponse, VersionTemplateRequest,
};

/// Template API operations (Cloud only).
#[derive(Debug)]
pub struct TemplateApi<'a> {
    client: &'a LettaClient,
}

impl<'a> TemplateApi<'a> {
    /// Create a new template API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all templates.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `offset` - Pagination offset
    /// * `limit` - Maximum number of templates to return
    /// * `name` - Filter by template name
    /// * `project_id` - Filter by project ID
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        offset: Option<String>,
        limit: Option<String>,
        name: Option<String>,
        project_id: Option<String>,
    ) -> LettaResult<TemplatesListResponse> {
        let mut params = Vec::new();
        if let Some(o) = offset {
            params.push(("offset", o));
        }
        if let Some(l) = limit {
            params.push(("limit", l));
        }
        if let Some(n) = name {
            params.push(("name", n));
        }
        if let Some(p) = project_id {
            params.push(("project_id", p));
        }

        if params.is_empty() {
            self.client.get("v1/templates").await
        } else {
            self.client.get_with_query("v1/templates", &params).await
        }
    }

    /// Create a template from an agent.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to create a template from
    /// * `request` - Template creation parameters
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_from_agent(
        &self,
        agent_id: &LettaId,
        request: &CreateTemplateRequest,
    ) -> LettaResult<TemplatesCreateResponse> {
        self.client
            .post(&format!("v1/agents/{}/template", agent_id), request)
            .await
    }

    /// Create a new version of a template.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to version
    /// * `request` - Version creation parameters
    /// * `return_agent_state` - Whether to return the agent state
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn version_template(
        &self,
        agent_id: &LettaId,
        request: &VersionTemplateRequest,
        return_agent_state: Option<bool>,
    ) -> LettaResult<String> {
        let mut path = format!("v1/agents/{}/version-template", agent_id);
        if let Some(true) = return_agent_state {
            path.push_str("?return_agent_state=true");
        }
        self.client.post(&path, request).await
    }

    /// Migrate an agent to a new template version.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to migrate
    /// * `request` - Migration parameters
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn migrate_agent(
        &self,
        agent_id: &LettaId,
        request: &MigrateAgentRequest,
    ) -> LettaResult<TemplatesMigrateResponse> {
        self.client
            .post(&format!("v1/agents/{}/migrate", agent_id), request)
            .await
    }

    /// Create agents from a template.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `project` - Project slug
    /// * `template_version` - Template version (format: "template-name:version" or "template-name:latest")
    /// * `request` - Agent creation parameters
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_agents_from_template(
        &self,
        project: &str,
        template_version: &str,
        request: &CreateAgentsFromTemplateRequest,
    ) -> LettaResult<AgentsCreateResponse> {
        self.client
            .post(
                &format!("v1/templates/{}/{}/agents", project, template_version),
                request,
            )
            .await
    }
}

/// Template-related agent sub-API operations.
impl LettaClient {
    /// Get the template API for this client.
    pub fn templates(&self) -> TemplateApi {
        TemplateApi::new(self)
    }

    /// Get memory variables for an agent.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get_agent_memory_variables(
        &self,
        agent_id: &LettaId,
    ) -> LettaResult<MemoryVariablesListResponse> {
        self.get(&format!("v1/agents/{}/core-memory/variables", agent_id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_template_api_creation() {
        let config = ClientConfig::new("https://api.letta.com").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = TemplateApi::new(&client);
    }
}
