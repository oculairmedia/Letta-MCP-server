//! Memory blocks API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{Block, CreateBlockRequest, LettaId, ListBlocksParams, UpdateBlockRequest};

/// Memory blocks API operations.
#[derive(Debug)]
pub struct BlocksApi<'a> {
    client: &'a LettaClient,
}

impl<'a> BlocksApi<'a> {
    /// Create a new blocks API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all memory blocks.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering the list
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListBlocksParams>) -> LettaResult<Vec<Block>> {
        self.client
            .get_with_query("v1/blocks/", &params.unwrap_or_default())
            .await
    }

    /// Create a new memory block.
    ///
    /// # Arguments
    ///
    /// * `request` - The block creation request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: CreateBlockRequest) -> LettaResult<Block> {
        self.client.post("v1/blocks/", &request).await
    }

    /// Get a specific memory block by ID.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The ID of the block to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, block_id: &LettaId) -> LettaResult<Block> {
        self.client.get(&format!("v1/blocks/{}", block_id)).await
    }

    /// Update a memory block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The ID of the block to update
    /// * `request` - The update request
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        block_id: &LettaId,
        request: UpdateBlockRequest,
    ) -> LettaResult<Block> {
        self.client
            .patch(&format!("v1/blocks/{}", block_id), &request)
            .await
    }

    /// Delete a memory block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The ID of the block to delete
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails.
    pub async fn delete(&self, block_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/blocks/{}", block_id))
            .await
    }

    /// Get the count of memory blocks.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn count(&self) -> LettaResult<u32> {
        self.client.get("v1/blocks/count").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_blocks_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = BlocksApi::new(&client);
    }
}
