//! Health API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::health::Health;

/// Health API operations.
#[derive(Debug)]
pub struct HealthApi<'a> {
    client: &'a LettaClient,
}

impl<'a> HealthApi<'a> {
    /// Create a new health API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Check the health of the Letta server.
    ///
    /// Returns the server version and health status.
    /// This endpoint does not require authentication.
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn check(&self) -> LettaResult<Health> {
        // Note: trailing slash is required on this endpoint
        self.client.get("v1/health/").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_health_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = HealthApi::new(&client);
    }
}
