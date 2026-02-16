use turbomcp::McpError;

pub fn require_field<T>(value: Option<T>, message: impl Into<String>) -> Result<T, McpError> {
    value.ok_or_else(|| McpError::invalid_request(message.into()))
}

pub fn sdk_err(action: &str, e: impl std::fmt::Display) -> McpError {
    McpError::internal(format!("Failed to {}: {}", action, e))
}
