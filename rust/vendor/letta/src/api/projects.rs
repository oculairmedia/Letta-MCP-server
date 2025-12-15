//! Project management API endpoints (Cloud only).

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::ProjectsListResponse;

/// Project API operations (Cloud only).
#[derive(Debug)]
pub struct ProjectApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ProjectApi<'a> {
    /// Create a new project API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all projects.
    ///
    /// This endpoint is only available on Letta Cloud.
    ///
    /// # Arguments
    ///
    /// * `name` - Filter by project name
    /// * `offset` - Pagination offset
    /// * `limit` - Maximum number of projects to return
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        name: Option<String>,
        offset: Option<String>,
        limit: Option<String>,
    ) -> LettaResult<ProjectsListResponse> {
        let mut params = Vec::new();
        if let Some(n) = name {
            params.push(("name", n));
        }
        if let Some(o) = offset {
            params.push(("offset", o));
        }
        if let Some(l) = limit {
            params.push(("limit", l));
        }

        if params.is_empty() {
            self.client.get("v1/projects").await
        } else {
            self.client.get_with_query("v1/projects", &params).await
        }
    }
}

/// Convenience methods for project operations.
impl LettaClient {
    /// Get the project API for this client.
    pub fn projects(&self) -> ProjectApi {
        ProjectApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_project_api_creation() {
        let config = ClientConfig::new("https://api.letta.com").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ProjectApi::new(&client);
    }
}
