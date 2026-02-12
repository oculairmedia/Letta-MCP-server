# Letta MCP Server — Rust Internals

This directory contains the Rust implementation of the Letta MCP Server.

> For usage, installation, and configuration, see the [main README](../README.md).

## Architecture

- **Rust MCP Server** on port `6507` (configurable via `PORT`)
- **[TurboMCP](https://github.com/oculairmedia/turbomcp)** framework for MCP protocol
- **[letta-rs](https://github.com/oculairmedia/letta-rs)** for Letta API client
- Dual transport: stdio (default) or HTTP (`TRANSPORT=http`)

## Project Structure

```
letta-server/
├── src/
│   ├── main.rs              # Entry point, transport selection
│   ├── lib.rs               # Server initialization, tool registration
│   └── tools/
│       ├── mod.rs            # Tool registration
│       ├── agent_advanced.rs # Agent operations (22 ops)
│       ├── memory_unified.rs # Memory operations (15 ops)
│       ├── tool_manager.rs   # Tool operations (13 ops)
│       ├── source_manager.rs # Source operations (15 ops)
│       ├── mcp_ops.rs        # MCP operations (10 ops)
│       ├── file_folder_ops.rs# File operations (8 ops)
│       └── job_monitor.rs    # Job operations (4 ops)
├── tests/                    # Integration tests
└── Cargo.toml

letta-types/                  # Shared type definitions
├── src/lib.rs
└── Cargo.toml

vendor/                       # Vendored SDK patches
```

## Development

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check without building
cargo check
```

### Build Optimizations (Release)

- LTO (Link-Time Optimization)
- Single codegen unit
- Symbol stripping
- opt-level 3

### TurboMCP Features

- `#[turbomcp(flatten)]` for auto-generated parameter schemas
- JSON Schema 2020-12 with `$defs` support
- Discriminator-based tool operations
- Type-safe parameter validation via schemars

## Dependencies

- **turbomcp** — MCP protocol framework
- **letta** — Rust Letta API client (vendored with patches)
- **tokio** — Async runtime
- **serde** / **serde_json** — Serialization
- **schemars** — JSON Schema generation
- **reqwest** — HTTP client

## License

MIT
