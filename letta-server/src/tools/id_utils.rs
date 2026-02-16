use letta::types::LettaId;
use std::str::FromStr;
use turbomcp::McpError;

pub fn parse_letta_id(id: &str, field_name: &str) -> Result<LettaId, McpError> {
    LettaId::from_str(id)
        .map_err(|e| McpError::invalid_request(format!("Invalid {}: {}", field_name, e)))
}
