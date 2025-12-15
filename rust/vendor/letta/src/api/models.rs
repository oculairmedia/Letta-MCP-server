//! Models API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{EmbeddingModel, ListEmbeddingModelsParams, ListModelsParams, LlmConfig};

/// Models API operations.
#[derive(Debug)]
pub struct ModelsApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ModelsApi<'a> {
    /// Create a new models API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List available LLM models.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering the list
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListModelsParams>) -> LettaResult<Vec<LlmConfig>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(provider_categories) = params.provider_category {
                for category in provider_categories {
                    query_params.push(("provider_category", category.to_string()));
                }
            }
            if let Some(provider_name) = params.provider_name {
                query_params.push(("provider_name", provider_name));
            }
            if let Some(provider_type) = params.provider_type {
                query_params.push(("provider_type", provider_type));
            }
        }

        if query_params.is_empty() {
            self.client.get("v1/models").await
        } else {
            self.client.get_with_query("v1/models", &query_params).await
        }
    }

    /// List available embedding models.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering the list
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_embedding_models(
        &self,
        params: Option<ListEmbeddingModelsParams>,
    ) -> LettaResult<Vec<EmbeddingModel>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            if let Some(provider_categories) = params.provider_category {
                for category in provider_categories {
                    query_params.push(("provider_category", category));
                }
            }
        }

        if query_params.is_empty() {
            self.client.get("v1/models/embedding").await
        } else {
            self.client
                .get_with_query("v1/models/embedding", &query_params)
                .await
        }
    }
}

/// Convenience method for models operations.
impl LettaClient {
    /// Get the models API for this client.
    pub fn models(&self) -> ModelsApi {
        ModelsApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_models_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ModelsApi::new(&client);
    }
}
