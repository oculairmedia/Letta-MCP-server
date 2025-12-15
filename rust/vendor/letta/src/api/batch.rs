//! Batch API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    BatchMessagesResponse, BatchRun, CreateBatchRequest, LettaId, ListBatchMessagesParams,
};

/// Batch API operations.
#[derive(Debug)]
pub struct BatchApi<'a> {
    client: &'a LettaClient,
}

impl<'a> BatchApi<'a> {
    /// Create a new batch API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List batch runs.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self) -> LettaResult<Vec<BatchRun>> {
        self.client.get("v1/messages/batches").await
    }

    /// Create a batch of messages.
    ///
    /// **Note**: The batch API may not be fully implemented in all Letta server versions.
    /// The server must have `LETTA_ENABLE_BATCH_JOB_POLLING=true` configured to enable
    /// batch processing functionality. Some servers may return a `NotImplementedError`
    /// when attempting to create batches.
    ///
    /// # Arguments
    ///
    /// * `request` - The batch creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    /// May return a 500 Internal Server Error with `NotImplementedError` if the server
    /// does not support batch processing.
    pub async fn create(&self, request: CreateBatchRequest) -> LettaResult<BatchRun> {
        self.client.post("v1/messages/batches", &request).await
    }

    /// Retrieve a specific batch run.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, batch_id: &LettaId) -> LettaResult<BatchRun> {
        self.client
            .get(&format!("v1/messages/batches/{}", batch_id))
            .await
    }

    /// Cancel a batch run.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch to cancel
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn cancel(&self, batch_id: &LettaId) -> LettaResult<BatchRun> {
        self.client
            .patch(
                &format!("v1/messages/batches/{}/cancel", batch_id),
                &serde_json::Value::Null,
            )
            .await
    }

    /// List messages in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch_id` - The ID of the batch
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list_messages(
        &self,
        batch_id: &LettaId,
        params: Option<ListBatchMessagesParams>,
    ) -> LettaResult<BatchMessagesResponse> {
        let path = format!("v1/messages/batches/{}/messages", batch_id);
        if let Some(params) = params {
            self.client.get_with_query(&path, &params).await
        } else {
            self.client.get(&path).await
        }
    }
}

/// Convenience method for batch operations.
impl LettaClient {
    /// Get the batch API for this client.
    pub fn batch(&self) -> BatchApi {
        BatchApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_batch_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = BatchApi::new(&client);
    }
}
