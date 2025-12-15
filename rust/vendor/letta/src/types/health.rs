//! Health types.

use serde::{Deserialize, Serialize};

/// Health check response from the Letta server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Health {
    /// Server version (e.g., "0.8.8").
    pub version: String,

    /// Health status (e.g., "ok").
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_serialization() {
        let health = Health {
            version: "0.8.8".to_string(),
            status: "ok".to_string(),
        };

        let json = serde_json::to_string(&health).unwrap();
        let deserialized: Health = serde_json::from_str(&json).unwrap();

        assert_eq!(health.version, deserialized.version);
        assert_eq!(health.status, deserialized.status);
    }

    #[test]
    fn test_health_from_json() {
        let json = r#"{"version":"0.8.8","status":"ok"}"#;
        let health: Health = serde_json::from_str(json).unwrap();

        assert_eq!(health.version, "0.8.8");
        assert_eq!(health.status, "ok");
    }
}
