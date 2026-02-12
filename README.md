# Letta MCP Server

[![npm version](https://img.shields.io/npm/v/letta-mcp-server.svg)](https://www.npmjs.com/package/letta-mcp-server)
[![Rust Tests](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-test.yml/badge.svg)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-test.yml)
[![Docker Build](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-docker-build.yml/badge.svg)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-docker-build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server for [Letta AI](https://github.com/letta-ai/letta), built with Rust and the [TurboMCP](https://github.com/oculairmedia/turbomcp) framework.

## Features

- **7 Consolidated Tools** covering 87 operations using the discriminator pattern
- **High Performance** - Rust implementation (~10-30MB memory, <500ms startup)
- **Dual Transport** - stdio (for Claude Desktop, Cursor, etc.) and HTTP (for production)
- **Response Size Optimization** - 68-96% reduction in response sizes for LLM context efficiency
- **Multi-Platform** - macOS, Linux, Windows (x64 and arm64)
- **Letta 0.15.1+ Compatible** - Full support for current Letta API
- **MCP 2025-11-25 Compliant** - Streamable HTTP transport with SSE support
- **Type-Safe** - Compile-time validation with Rust's type system

## Quick Start

### npm (Recommended)

```bash
npm install -g letta-mcp-server
```

The correct binary for your platform is installed automatically.

| Platform | Package |
|----------|---------|
| macOS Intel | `letta-mcp-darwin-x64` |
| macOS Apple Silicon | `letta-mcp-darwin-arm64` |
| Linux x64 | `letta-mcp-linux-x64` |
| Linux arm64 | `letta-mcp-linux-arm64` |
| Windows x64 | `letta-mcp-windows-x64` |

### Docker

```bash
docker pull ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest

docker run -d \
  -p 6507:6507 \
  -e LETTA_BASE_URL=http://your-letta-instance:8283 \
  -e LETTA_PASSWORD=your-password \
  -e TRANSPORT=http \
  --name letta-mcp \
  ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
```

### Docker Compose

```yaml
services:
  letta-mcp:
    image: ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
    container_name: letta-mcp
    restart: unless-stopped
    ports:
      - '6507:6507'
    environment:
      LETTA_BASE_URL: ${LETTA_BASE_URL}
      LETTA_PASSWORD: ${LETTA_PASSWORD}
      TRANSPORT: http
      PORT: 6507
      RUST_LOG: info
    env_file:
      - .env
    healthcheck:
      test: ['CMD-SHELL', "timeout 1 bash -c '</dev/tcp/localhost/6507' || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `LETTA_BASE_URL` | Yes | — | Letta API URL (e.g. `http://localhost:8283`) |
| `LETTA_PASSWORD` | Yes | — | Letta API password |
| `TRANSPORT` | No | `stdio` | Transport mode: `stdio` or `http` |
| `PORT` | No | `6507` | HTTP port (when `TRANSPORT=http`) |
| `RUST_LOG` | No | `info` | Log level: `debug`, `info`, `warn`, `error` |
| `RUST_BACKTRACE` | No | `0` | Enable backtraces (`0` or `1`) |

## Available Tools

The server provides **7 consolidated tools** with **87 operations**:

| Tool | Operations | Description |
|------|------------|-------------|
| `letta_agent_advanced` | 22 | Agent lifecycle, messaging, context, export/import |
| `letta_memory_unified` | 15 | Core memory, blocks, archival passages, search |
| `letta_tool_manager` | 13 | Tool CRUD, attach/detach, bulk operations |
| `letta_source_manager` | 15 | Data sources, files, passages, attachments |
| `letta_job_monitor` | 4 | Job tracking, cancellation, active monitoring |
| `letta_file_folder_ops` | 8 | File sessions, folder management |
| `letta_mcp_ops` | 10 | MCP server management, tool discovery |

### Tool Operations

#### letta_agent_advanced (22 operations)

```
list, create, get, update, delete, search, list_tools, send_message,
export, import, clone, get_config, bulk_delete, context, reset_messages,
summarize, stream, async_message, cancel_message, preview_payload,
search_messages, get_message, count
```

#### letta_memory_unified (15 operations)

```
get_core_memory, update_core_memory, get_block_by_label, list_blocks,
create_block, get_block, update_block, attach_block, detach_block,
list_agents_using_block, search_archival, list_passages, create_passage,
update_passage, delete_passage
```

#### letta_tool_manager (13 operations)

```
list, get, create, update, delete, upsert, attach, detach, bulk_attach,
generate_from_prompt, generate_schema, run_from_source, add_base_tools
```

#### letta_source_manager (15 operations)

```
list, get, create, update, delete, count, attach, detach, list_attached,
upload, delete_files, list_files, list_folders, get_folder_contents,
list_agents_using
```

#### letta_job_monitor (4 operations)

```
list, get, cancel, list_active
```

#### letta_file_folder_ops (8 operations)

```
list_files, open_file, close_file, close_all_files, list_folders,
attach_folder, detach_folder, list_agents_in_folder
```

#### letta_mcp_ops (10 operations)

```
add, update, delete, test, connect, resync, list_servers, list_tools,
register_tool, execute
```

## Response Size Optimizations

The Rust implementation includes significant response size optimizations for LLM context efficiency:

| Operation | Optimization | Size Reduction |
|-----------|-------------|----------------|
| Agent List | Default pagination (15 items), summary mode | 68-85% |
| Tool List | Default pagination (25 items), truncated descriptions | 70-90% |
| Memory Blocks | Excludes heavy fields in list mode | 60-80% |
| Source List | Summary mode, pagination | 75-95% |

### Pagination

All list operations support pagination:

```json
{
  "operation": "list",
  "pagination": {
    "limit": 25,
    "offset": 0
  }
}
```

### Summary vs Full Mode

List operations return summary data by default. Use `get` operation with specific ID for full details:

```json
// Summary (default for list)
{
  "id": "agent-123",
  "name": "My Agent",
  "model": "gpt-4",
  "tool_count": 5
}

// Full (with get operation)
{
  "id": "agent-123",
  "name": "My Agent",
  "model": "gpt-4",
  "system_prompt": "...",
  "tools": [...],
  "memory_blocks": [...]
}
```

## MCP Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "letta": {
      "command": "letta-mcp",
      "env": {
        "LETTA_BASE_URL": "http://localhost:8283",
        "LETTA_PASSWORD": "your-password"
      }
    }
  }
}
```

### Cursor / Windsurf

```json
{
  "mcpServers": {
    "letta": {
      "command": "letta-mcp",
      "env": {
        "LETTA_BASE_URL": "http://localhost:8283",
        "LETTA_PASSWORD": "your-password"
      }
    }
  }
}
```

### OpenCode (HTTP)

```json
{
  "mcp": {
    "letta-mcp": {
      "type": "remote",
      "url": "http://localhost:6507/mcp",
      "enabled": true
    }
  }
}
```

## Building from Source

### Prerequisites

- Rust 1.75+ (nightly recommended)
- Docker (for containerized builds)

### Local Build

```bash
git clone https://github.com/oculairmedia/Letta-MCP-server.git
cd Letta-MCP-server

cargo build --release

LETTA_BASE_URL=http://your-letta:8283 \
LETTA_PASSWORD=your-password \
./target/release/letta-server
```

### Docker Build

```bash
docker build -f Dockerfile.rust -t letta-mcp .

docker run -d \
  -p 6507:6507 \
  -e LETTA_BASE_URL=http://your-letta:8283 \
  -e LETTA_PASSWORD=your-password \
  -e TRANSPORT=http \
  letta-mcp
```

## Architecture

```
letta-server/
├── src/
│   ├── main.rs              # Entry point, transport selection
│   ├── lib.rs               # Library exports, server initialization
│   └── tools/
│       ├── mod.rs           # Tool registration
│       ├── agent_advanced.rs    # Agent operations (22 ops)
│       ├── memory_unified.rs    # Memory operations (15 ops)
│       ├── tool_manager.rs      # Tool operations (13 ops)
│       ├── source_manager.rs    # Source operations (15 ops)
│       ├── job_monitor.rs       # Job operations (4 ops)
│       ├── file_folder_ops.rs   # File operations (8 ops)
│       └── mcp_ops.rs           # MCP operations (10 ops)
├── tests/                   # Integration tests
└── Cargo.toml              # Dependencies
```

### Key Dependencies

- [TurboMCP](https://github.com/oculairmedia/turbomcp) - MCP protocol framework with streamable HTTP
- [letta-rs](https://github.com/oculairmedia/letta-rs) - Rust Letta API client
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **Reqwest** - HTTP client

## Troubleshooting

### Connection Refused

1. Ensure the server is running: `docker ps | grep letta-mcp`
2. Check logs: `docker logs letta-mcp`
3. Verify port is accessible: `curl http://localhost:6507/mcp`

### Authentication Errors

1. Verify `LETTA_BASE_URL` points to your Letta instance
2. Check `LETTA_PASSWORD` is correct
3. Ensure Letta server is accessible from the container

### Tool Not Found

1. List available tools via MCP: `tools/list`
2. Verify you're using correct operation names (e.g., `list` not `list_agents`)

### Logs

```bash
# View server logs
docker logs -f letta-mcp

# Enable debug logging
RUST_LOG=debug letta-mcp
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make changes and add tests
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Related Projects

- [Letta](https://github.com/letta-ai/letta) - The Letta AI framework
- [TurboMCP](https://github.com/oculairmedia/turbomcp) - MCP framework for Rust
- [Model Context Protocol](https://modelcontextprotocol.io) - MCP specification
