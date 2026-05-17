use letta_server::LettaServer;
use std::env;
use turbomcp::prelude::*; // v3: McpHandlerExt for run_stdio/run_http

fn read_letta_auth() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(password) = env::var("LETTA_PASSWORD") {
        return Ok(password);
    }

    if let Ok(token) = env::var("LETTA_API_TOKEN") {
        return Ok(token);
    }

    Err("LETTA_PASSWORD or LETTA_API_TOKEN environment variable is required".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        // MCP stdio reserves stdout exclusively for JSON-RPC frames.
        // Keep diagnostics on stderr and disable ANSI escapes so spec-compliant
        // stdio clients never see non-JSON bytes on stdout.
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    let base_url =
        env::var("LETTA_BASE_URL").expect("LETTA_BASE_URL environment variable is required");
    let password = read_letta_auth()?;
    let transport = env::var("TRANSPORT").unwrap_or_else(|_| "stdio".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "6507".to_string())
        .parse()
        .expect("PORT must be a valid number");

    tracing::info!("╔══════════════════════════════════════╗");
    tracing::info!("║   Letta MCP Server (Rust/TurboMCP)  ║");
    tracing::info!("╚══════════════════════════════════════╝");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Transport: {}", transport);
    tracing::info!("Letta API: {}", base_url);

    let server = LettaServer::new(base_url, password)?;

    // LMS-116: Startup health check — validate Letta API connectivity before accepting connections
    tracing::info!("Running startup health check...");
    match server.health_check().await {
        Ok(agent_count) => {
            tracing::info!(
                "✅ Health check passed — Letta API reachable ({} agents)",
                agent_count
            );
        }
        Err(e) => {
            tracing::warn!(
                "⚠️  Health check failed: {}. Server will start but API operations may fail.",
                e
            );
        }
    }

    match transport.to_lowercase().as_str() {
        "http" => {
            let addr = format!("0.0.0.0:{}", port);
            tracing::info!("🚀 Starting HTTP transport");
            tracing::info!("📡 Listening on: http://{}", addr);
            tracing::info!("🔗 Endpoint: http://{}/mcp", addr);
            tracing::info!("Ready for MCP client connections");

            server.run_http_custom(&addr).await?;
        }
        _ => {
            tracing::info!("🚀 Starting stdio transport (default)");
            tracing::info!("Ready for MCP client connections");

            server.run_stdio().await?;
        }
    }

    tracing::info!("Server shutdown complete");
    Ok(())
}
