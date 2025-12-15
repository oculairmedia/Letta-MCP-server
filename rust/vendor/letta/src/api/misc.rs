//! Miscellaneous API endpoints (health, models, etc.).

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::HealthResponse;

/// Miscellaneous API operations.
#[derive(Debug)]
pub struct MiscApi<'a> {
    #[allow(dead_code)]
    client: &'a LettaClient,
}

impl<'a> MiscApi<'a> {
    /// Create a new misc API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Health check.
    pub async fn health(&self) -> LettaResult<HealthResponse> {
        todo!("Implement health check")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_misc_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = MiscApi::new(&client);
    }
}
