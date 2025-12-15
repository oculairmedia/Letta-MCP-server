//! Telemetry-related types.

use serde::{Deserialize, Serialize};

/// Telemetry trace information for a specific step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryTrace {
    /// Request JSON data.
    pub request_json: serde_json::Value,
    /// Response JSON data.
    pub response_json: serde_json::Value,
    /// ID of the user who created this trace.
    pub created_by_id: Option<String>,
    /// ID of the user who last updated this trace.
    pub last_updated_by_id: Option<String>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Last update timestamp.
    pub updated_at: Option<String>,
    /// Trace ID.
    pub id: Option<String>,
    /// Step ID this trace belongs to.
    pub step_id: Option<String>,
}
