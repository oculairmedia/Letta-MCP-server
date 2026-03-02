use super::id_utils::parse_letta_id;
use letta::types::LettaId;
use turbomcp::McpError;

pub fn require_field<T>(value: Option<T>, message: impl Into<String>) -> Result<T, McpError> {
    value.ok_or_else(|| McpError::invalid_request(message.into()))
}

pub fn sdk_err(action: &str, e: impl std::fmt::Display) -> McpError {
    McpError::internal(format!("Failed to {}: {}", action, e))
}

pub fn require_id(value: Option<String>, field_name: &str) -> Result<LettaId, McpError> {
    let raw = require_field(value, format!("{} is required", field_name))?;
    parse_letta_id(&raw, field_name)
}
