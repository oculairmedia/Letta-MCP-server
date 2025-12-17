# Letta MCP Server - Rust Implementation

[![Rust Tests](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-test.yml/badge.svg?branch=rust-implementation)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-test.yml)
[![Docker Build](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-docker-build.yml/badge.svg?branch=rust-implementation)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/rust-docker-build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Model Context Protocol (MCP) server for Letta AI, built with Rust and the TurboMCP framework. This implementation provides the same comprehensive toolset as the Node.js version with significant performance improvements and response size optimizations.

## Features

- **7 Consolidated Tools** covering 87 operations using the discriminator pattern
- **High Performance** - Rust implementation with TurboMCP framework
- **Response Size Optimization** - 68-96% reduction in response sizes for LLM context efficiency
- **Multi-Architecture Docker** - Supports both amd64 and arm64
- **Letta 0.15.1+ Compatible** - Full support for new ToolRule types
- **MCP 2025-06-18 Compliant** - Streamable HTTP transport with SSE support
- **Type-Safe** - Compile-time validation with Rust's type system

## Quick Start

### Docker (Recommended)

```bash
# Pull the latest image
docker pull ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest

# Run with environment variables
docker run -d \
  -p 6507:6507 \
  -e LETTA_BASE_URL=http://your-letta-instance:8289 \
  -e LETTA_PASSWORD=your-password \
  --name letta-mcp-rust \
  ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
```

### Docker Compose

Create a `compose.yaml`:

```yaml
services:
  letta-mcp-rust:
    image: ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
    container_name: letta-mcp-rust
    restart: unless-stopped
    ports:
      - '6507:6507'
    environment:
      LETTA_BASE_URL: ${LETTA_BASE_URL}
      LETTA_PASSWORD: ${LETTA_PASSWORD}
      PORT: 6507
      RUST_LOG: info
    env_file:
      - .env
    healthcheck:
      test: ['CMD-SHELL', "timeout 1 bash -c '</dev/tcp/localhost/6507' || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    container_name: letta-mcp-nginx
    restart: unless-stopped
    ports:
      - '3001:3001'
    volumes:
      - ./rust/nginx.conf:/etc/nginx/conf.d/default.conf:ro
    depends_on:
      letta-mcp-rust:
        condition: service_healthy
```

Then run:

```bash
docker compose up -d
```

### Environment Configuration

Create a `.env` file:

```bash
# Required
LETTA_BASE_URL=http://your-letta-instance:8289
LETTA_PASSWORD=your-password

# Optional
PORT=6507
RUST_LOG=info
RUST_BACKTRACE=1
```

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

## API Endpoints

### HTTP Transport (Default)

- **Endpoint**: `http://localhost:6507/mcp`
- **Methods**: GET, POST, DELETE
- **Content-Type**: `application/json`
- **Accept**: `application/json` or `text/event-stream` (for SSE)

### Health Check

Through nginx proxy:
```bash
curl http://localhost:3001/health
# {"status":"ok","service":"letta-mcp-nginx"}
```

### Example: List Agents

```bash
curl -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_agent_advanced",
      "arguments": {
        "operation": "list"
      }
    },
    "id": 1
  }'
```

### Example: Get Agent Details

```bash
curl -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_agent_advanced",
      "arguments": {
        "operation": "get",
        "agent_id": "agent-uuid-here"
      }
    },
    "id": 2
  }'
```

### Example: Send Message to Agent

```bash
curl -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_agent_advanced",
      "arguments": {
        "operation": "send_message",
        "agent_id": "agent-uuid-here",
        "messages": [
          {"role": "user", "content": "Hello!"}
        ]
      }
    },
    "id": 3
  }'
```

## MCP Client Configuration

### Claude Desktop

Add to your Claude Desktop MCP settings:

```json
{
  "mcpServers": {
    "letta-rust": {
      "url": "http://localhost:3001/mcp",
      "transport": "http"
    }
  }
}
```

### OpenCode

The server works directly with OpenCode's MCP integration. Configure in your MCP settings to point to `http://localhost:3001/mcp`.

## Building from Source

### Prerequisites

- Rust 1.75+ (nightly recommended for latest features)
- Docker (for containerized builds)

### Local Build

```bash
# Clone the repository
git clone https://github.com/oculairmedia/Letta-MCP-server.git
cd Letta-MCP-server
git checkout rust-implementation

# Build
cargo build --release

# Run
LETTA_BASE_URL=http://your-letta:8289 \
LETTA_PASSWORD=your-password \
./target/release/letta-server
```

### Docker Build

```bash
# Build image
docker build -f Dockerfile.rust -t letta-mcp-rust .

# Run
docker run -d \
  -p 6507:6507 \
  -e LETTA_BASE_URL=http://your-letta:8289 \
  -e LETTA_PASSWORD=your-password \
  letta-mcp-rust
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

- **TurboMCP** - MCP protocol implementation with streamable HTTP
- **Letta SDK** - Official Letta API client (vendored with patches)
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **Reqwest** - HTTP client

## Comparison with Node.js Implementation

| Feature | Node.js | Rust |
|---------|---------|------|
| **Performance** | Good | Excellent |
| **Memory Usage** | ~50-100MB | ~10-30MB |
| **Startup Time** | ~1-2s | ~100-500ms |
| **Response Optimization** | Standard | 68-96% reduction |
| **Type Safety** | Runtime (TypeScript) | Compile-time |
| **Package Distribution** | npm | Docker |
| **Letta Compatibility** | 0.14.x | 0.15.1+ |

## Troubleshooting

### Connection Refused

1. Ensure the server is running: `docker ps | grep letta-mcp`
2. Check logs: `docker logs letta-mcp-rust`
3. Verify port is accessible: `curl http://localhost:6507/mcp`

### Authentication Errors

1. Verify `LETTA_BASE_URL` points to your Letta instance
2. Check `LETTA_PASSWORD` is correct
3. Ensure Letta server is accessible from the container

### Tool Not Found

1. List available tools: `curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'`
2. Verify you're using correct operation names (e.g., `list` not `list_agents`)

### Logs

```bash
# View server logs
docker logs -f letta-mcp-rust

# Enable debug logging
docker run -e RUST_LOG=debug ...
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
