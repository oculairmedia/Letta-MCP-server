//! Test example for the FlattenTool derive macro

use turbomcp::prelude::*;
use turbomcp_macros::FlattenTool;
use schemars::JsonSchema;

/// Agent request with flattened parameters
#[derive(Clone, serde::Serialize, serde::Deserialize, JsonSchema, FlattenTool)]
struct AgentRequest {
    /// Operation to perform: list, create, get, update, delete
    operation: String,
    /// Agent ID (required for get, update, delete operations)
    agent_id: Option<String>,
    /// Agent name (for create operation)
    name: Option<String>,
}

#[derive(Clone)]
struct TestServer;

#[turbomcp::server(
    name = "flatten-test",
    version = "1.0.0"
)]
impl TestServer {
    #[tool("Agent operations with flattened parameters")]
    async fn agent_ops(
        &self,
        operation: String,
        agent_id: Option<String>,
        name: Option<String>,
    ) -> McpResult<String> {
        // Construct the struct from flat parameters using the generated method
        let request = AgentRequest::agentrequest_from_flat(operation, agent_id, name);

        Ok(format!(
            "Operation: {}, agent_id: {:?}, name: {:?}",
            request.operation, request.agent_id, request.name
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test schema generation
    #[cfg(feature = "schemars")]
    {
        println!("=== Flat Schema with Descriptions ===");
        let schema = AgentRequest::agentrequest_flat_schema();
        println!("{}", serde_json::to_string_pretty(&schema)?);
        println!();
    }

    // Run the MCP server
    TestServer.run_stdio().await?;
    Ok(())
}
