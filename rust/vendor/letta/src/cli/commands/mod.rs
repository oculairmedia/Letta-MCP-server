//! CLI command implementations.

pub mod agent;
pub mod memory;
pub mod message;
pub mod sources;
pub mod tools;

use crate::LettaClient;
use miette::IntoDiagnostic;

/// Check the health of the Letta server.
pub async fn check_health(client: &LettaClient) -> miette::Result<()> {
    match client.health().check().await {
        Ok(status) => match serde_json::to_string_pretty(&status).into_diagnostic() {
            Ok(json) => {
                println!("Server is healthy!");
                println!("{}", json);
                Ok(())
            }
            Err(e) => {
                println!("Server is healthy but failed to format response: {}", e);
                Ok(())
            }
        },
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            std::process::exit(1);
        }
    }
}
