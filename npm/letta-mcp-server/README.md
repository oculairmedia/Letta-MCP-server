# letta-mcp-server

High-performance [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server for [Letta AI](https://github.com/letta-ai/letta), built with Rust.

Provides **7 tools** with **87 operations** for managing agents, memory, tools, sources, jobs, files, and MCP servers.

## Install

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

## Usage

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `LETTA_BASE_URL` | Yes | — | Letta API URL (e.g. `http://localhost:8283`) |
| `LETTA_PASSWORD` | Yes | — | Letta API password |
| `TRANSPORT` | No | `stdio` | Transport mode: `stdio` or `http` |
| `PORT` | No | `6507` | HTTP port (when `TRANSPORT=http`) |

### stdio transport (default — for Claude Desktop, Cursor, etc.)

```bash
letta-mcp
```

### HTTP transport (for production / remote access)

```bash
TRANSPORT=http PORT=6507 letta-mcp
```

## Claude Desktop Configuration

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

## OpenCode Configuration (HTTP)

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

## Available Tools

| Tool | Ops | Description |
|------|-----|-------------|
| `letta_agent_advanced` | 22 | Agent lifecycle, messaging, context, export/import |
| `letta_memory_unified` | 15 | Core memory, blocks, archival passages, search |
| `letta_tool_manager` | 13 | Tool CRUD, attach/detach, bulk operations |
| `letta_source_manager` | 15 | Data sources, files, passages, attachments |
| `letta_mcp_ops` | 10 | MCP server management, tool discovery |
| `letta_file_folder_ops` | 8 | File sessions, folder management |
| `letta_job_monitor` | 4 | Job tracking, cancellation, active monitoring |

## Docker Alternative

```bash
docker pull ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
docker run -d -p 6507:6507 \
  -e LETTA_BASE_URL=http://your-letta:8283 \
  -e LETTA_PASSWORD=your-password \
  -e TRANSPORT=http \
  ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
```

Full documentation: [GitHub](https://github.com/oculairmedia/Letta-MCP-server)

## License

MIT
