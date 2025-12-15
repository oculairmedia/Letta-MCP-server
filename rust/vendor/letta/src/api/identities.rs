//! Identities API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::{
    CreateIdentityRequest, Identity, LettaId, ListIdentitiesParams, PaginationParams,
    UpdateIdentityRequest,
};

/// Identities API operations.
#[derive(Debug)]
pub struct IdentitiesApi<'a> {
    client: &'a LettaClient,
}

impl<'a> IdentitiesApi<'a> {
    /// Create a new identities API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all identities.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListIdentitiesParams>) -> LettaResult<Vec<Identity>> {
        if let Some(params) = params {
            self.client.get_with_query("v1/identities/", &params).await
        } else {
            self.client.get("v1/identities/").await
        }
    }

    /// Create a new identity.
    ///
    /// # Arguments
    ///
    /// * `request` - The identity creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: CreateIdentityRequest) -> LettaResult<Identity> {
        self.client.post("v1/identities/", &request).await
    }

    /// Create a new identity with optional project context.
    ///
    /// # Arguments
    ///
    /// * `request` - The identity creation request
    /// * `project_id` - Optional project ID to associate the identity with (sent as X-Project header)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create_with_project(
        &self,
        request: CreateIdentityRequest,
        project_id: &str,
    ) -> LettaResult<Identity> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Project",
            project_id.parse().map_err(|_| {
                crate::error::LettaError::validation("Invalid X-Project header value")
            })?,
        );

        self.client
            .post_with_headers("v1/identities/", &request, headers)
            .await
    }

    /// Get an identity by ID.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The ID of the identity to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, identity_id: &LettaId) -> LettaResult<Identity> {
        self.client
            .get(&format!("v1/identities/{}", identity_id))
            .await
    }

    /// Update an identity.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The ID of the identity to update
    /// * `update` - The fields to update
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        identity_id: &LettaId,
        update: UpdateIdentityRequest,
    ) -> LettaResult<Identity> {
        self.client
            .patch(&format!("v1/identities/{}", identity_id), &update)
            .await
    }

    /// Delete an identity.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The ID of the identity to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, identity_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/identities/{}", identity_id))
            .await
    }

    /// Get count of identities.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn count(&self) -> LettaResult<u32> {
        let response: serde_json::Value = self.client.get("v1/identities/count").await?;
        // The response is just a bare number
        response.as_u64().map(|v| v as u32).ok_or_else(|| {
            crate::error::LettaError::validation("Invalid count response - expected number")
        })
    }

    /// Upsert an identity (update existing).
    ///
    /// **Note**: Despite the typical upsert behavior of "create or update", the Letta server
    /// currently only supports updating existing identities with this endpoint. Attempting to
    /// upsert a non-existent identity will return a 404 error. To create a new identity, use
    /// the `create()` method instead.
    ///
    /// # Arguments
    ///
    /// * `request` - The identity to update (must have an existing identifier_key)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails, if the identity doesn't exist,
    /// or if the response cannot be parsed.
    pub async fn upsert(&self, request: CreateIdentityRequest) -> LettaResult<Identity> {
        self.client.put("v1/identities/", &request).await
    }

    /// Upsert an identity (update existing) with optional project context.
    ///
    /// **Note**: Despite the typical upsert behavior of "create or update", the Letta server
    /// currently only supports updating existing identities with this endpoint. Attempting to
    /// upsert a non-existent identity will return a 404 error. To create a new identity, use
    /// the `create()` method instead.
    ///
    /// # Arguments
    ///
    /// * `request` - The identity to update (must have an existing identifier_key)
    /// * `project_id` - Optional project ID to associate the identity with (sent as X-Project header)
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails, if the identity doesn't exist,
    /// or if the response cannot be parsed.
    pub async fn upsert_with_project(
        &self,
        request: CreateIdentityRequest,
        project_id: &str,
    ) -> LettaResult<Identity> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Project",
            project_id.parse().map_err(|_| {
                crate::error::LettaError::validation("Invalid X-Project header value")
            })?,
        );

        self.client
            .put_with_headers("v1/identities/", &request, headers)
            .await
    }

    /// Get a paginated stream of identities.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all identities using async iteration.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use letta::client::{ClientConfig, LettaClient};
    /// # use letta::{types::PaginationParams, pagination::PaginationExt};
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LettaClient::new(ClientConfig::new("http://localhost:8283")?)?;
    ///
    /// let mut stream = client.identities().paginated(None);
    /// while let Some(identity) = stream.next().await {
    ///     let identity = identity?;
    ///     println!("Identity: {} - {}", identity.name, identity.identity_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<Identity> {
        let client = self.client.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            async move {
                let list_params = params.map(|p| ListIdentitiesParams {
                    after: p.after,
                    before: p.before,
                    limit: p.limit.map(|l| l as i32),
                    ..Default::default()
                });

                client.identities().list(list_params).await
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |identity: &Identity| {
            identity.id.to_string()
        })
    }
}

/// Convenience method for identities operations.
impl LettaClient {
    /// Get the identities API for this client.
    pub fn identities(&self) -> IdentitiesApi {
        IdentitiesApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_identities_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = IdentitiesApi::new(&client);
    }
}
