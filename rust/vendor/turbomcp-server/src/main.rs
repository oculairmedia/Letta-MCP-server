use turbomcp_server::{ServerBuilder, default_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging if available
    let _ = tracing_subscriber::fmt::try_init();

    // Build server with default config
    let config = default_config();
    let server = ServerBuilder::new()
        .name(config.name.clone())
        .version(config.version.clone())
        .build();

    // Run via stdio transport
    server.run_stdio().await.map_err(|e| e.into())
}
