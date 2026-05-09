//! Regression tests for spec-compliant MCP stdio clients.

use serde_json::json;
use turbomcp::InitializeRequest;

#[test]
fn initialize_accepts_empty_sampling_capabilities_object() {
    let request = json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "sampling": {}
        },
        "clientInfo": {
            "name": "stdio-compat-regression",
            "version": "0.1.0"
        }
    });

    let parsed: InitializeRequest =
        serde_json::from_value(request).expect("MCP clients advertise sampling as an empty object");

    assert!(parsed.capabilities.sampling.is_some());
}

#[test]
fn initialize_accepts_absent_sampling_capabilities() {
    let request = json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {
            "name": "stdio-compat-regression",
            "version": "0.1.0"
        }
    });

    let parsed: InitializeRequest = serde_json::from_value(request)
        .expect("sampling remains optional for clients without sampling support");

    assert!(parsed.capabilities.sampling.is_none());
}
