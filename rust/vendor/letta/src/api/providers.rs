//! Providers API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::pagination::PaginatedStream;
use crate::types::{
    LettaId, ListProvidersParams, PaginationParams, Provider, ProviderCheckResponse,
    ProviderCreate, ProviderDeleteResponse, ProviderUpdate,
};

/// Providers API operations.
#[derive(Debug)]
pub struct ProvidersApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ProvidersApi<'a> {
    /// Create a new providers API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all custom providers.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListProvidersParams>) -> LettaResult<Vec<Provider>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(provider_category) = params.provider_category {
                query_params.push(("provider_category", provider_category.to_string()));
            }
            if let Some(provider_type) = params.provider_type {
                query_params.push(("provider_type", provider_type.to_string()));
            }
            if let Some(after) = params.after {
                query_params.push(("after", after));
            }
            if let Some(limit) = params.limit {
                query_params.push(("limit", limit.to_string()));
            }
        }

        if query_params.is_empty() {
            self.client.get("v1/providers").await
        } else {
            self.client
                .get_with_query("v1/providers", &query_params)
                .await
        }
    }

    /// Create a new custom provider.
    ///
    /// # Arguments
    ///
    /// * `provider` - The provider configuration to create
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, provider: ProviderCreate) -> LettaResult<Provider> {
        self.client.post("v1/providers", &provider).await
    }

    /// Delete a provider.
    ///
    /// # Arguments
    ///
    /// * `provider_id` - The ID of the provider to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, provider_id: &LettaId) -> LettaResult<ProviderDeleteResponse> {
        self.client
            .delete(&format!("v1/providers/{}", provider_id))
            .await
    }

    /// Update a provider.
    ///
    /// # Arguments
    ///
    /// * `provider_id` - The ID of the provider to update
    /// * `update` - The fields to update
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        provider_id: &LettaId,
        update: ProviderUpdate,
    ) -> LettaResult<Provider> {
        self.client
            .patch(&format!("v1/providers/{}", provider_id), &update)
            .await
    }

    /// Check provider status.
    ///
    /// This endpoint verifies that a provider is reachable and configured correctly.
    ///
    /// # Arguments
    ///
    /// * `provider_id` - The ID of the provider to check
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn check(&self, provider_id: &LettaId) -> LettaResult<ProviderCheckResponse> {
        self.client
            .get(&format!("v1/providers/{}/check", provider_id))
            .await
    }

    /// Get a paginated stream of providers.
    ///
    /// This method returns a [`PaginatedStream`] that automatically handles pagination
    /// and allows streaming through all providers using async iteration.
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
    /// let mut stream = client.providers().paginated(None);
    /// while let Some(provider) = stream.next().await {
    ///     let provider = provider?;
    ///     println!("Provider: {} ({})", provider.name, provider.provider_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn paginated(&self, params: Option<PaginationParams>) -> PaginatedStream<Provider> {
        let client = self.client.clone();
        let fetch_fn = move |params: Option<PaginationParams>| {
            let client = client.clone();
            async move {
                let mut query_params = Vec::new();

                if let Some(params) = params {
                    if let Some(after) = params.after {
                        query_params.push(("after", after));
                    }
                    if let Some(limit) = params.limit {
                        query_params.push(("limit", limit.to_string()));
                    }
                }

                if query_params.is_empty() {
                    client.get("v1/providers").await
                } else {
                    client.get_with_query("v1/providers", &query_params).await
                }
            }
        };

        PaginatedStream::new_with_string_cursor(params, fetch_fn, |provider: &Provider| {
            provider.id.to_string()
        })
    }
}

/// Convenience method for providers operations.
impl LettaClient {
    /// Get the providers API for this client.
    pub fn providers(&self) -> ProvidersApi {
        ProvidersApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_providers_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ProvidersApi::new(&client);
    }
}
