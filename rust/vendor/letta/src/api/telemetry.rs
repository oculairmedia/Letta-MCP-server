//! Telemetry API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::TelemetryTrace;

/// Telemetry API operations.
#[derive(Debug)]
pub struct TelemetryApi<'a> {
    client: &'a LettaClient,
}

impl<'a> TelemetryApi<'a> {
    /// Create a new telemetry API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Retrieve provider trace by step ID.
    ///
    /// # Arguments
    ///
    /// * `step_id` - The ID of the step to retrieve the trace for
    ///
    /// # Errors
    ///
    /// Returns a [crate::error::LettaError] if the request fails or if the response cannot be parsed.
    pub async fn retrieve_provider_trace(&self, step_id: &str) -> LettaResult<TelemetryTrace> {
        self.client.get(&format!("v1/telemetry/{}", step_id)).await
    }
}

/// Convenience method for telemetry operations.
impl LettaClient {
    /// Get the telemetry API for this client.
    pub fn telemetry(&self) -> TelemetryApi {
        TelemetryApi::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_telemetry_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = TelemetryApi::new(&client);
    }
}
