//! Project management API endpoints (Cloud only).

use crate::api::endpoints;
use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{ListProjectsParams, ProjectsListResponse};

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
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        params: Option<ListProjectsParams>,
    ) -> LettaResult<ProjectsListResponse> {
        self.client
            .get_with_query(endpoints::projects::LIST, &params.unwrap_or_default())
            .await
    }
}

/// Convenience methods for project operations.
impl LettaClient {
    /// Get the project API for this client.
    pub fn projects(&self) -> ProjectApi<'_> {
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
