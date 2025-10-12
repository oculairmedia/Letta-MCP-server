[![npm version](https://img.shields.io/npm/v/letta-mcp-server.svg)](https://www.npmjs.com/package/letta-mcp-server)
[![npm downloads](https://img.shields.io/npm/dm/letta-mcp-server.svg)](https://www.npmjs.com/package/letta-mcp-server)
[![npm downloads total](https://img.shields.io/npm/dt/letta-mcp-server.svg)](https://www.npmjs.com/package/letta-mcp-server)
[![Docker Image](https://img.shields.io/badge/Docker-ghcr.io-blue)](https://github.com/oculairmedia/Letta-MCP-server/pkgs/container/letta-mcp-server)
[![MseeP.ai Security Assessment Badge](https://mseep.net/mseep-audited.png)](https://mseep.ai/app/oculairmedia-letta-mcp-server)
[![CI/CD](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/test.yml/badge.svg)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/test.yml)
[![Docker Build](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/docker-build.yml/badge.svg)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/docker-build.yml)
[![CodeQL](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/codeql.yml/badge.svg)](https://github.com/oculairmedia/letta-MCP-server/actions/workflows/codeql.yml)
[![Coverage Status](https://codecov.io/gh/oculairmedia/letta-MCP-server/branch/main/graph/badge.svg)](https://codecov.io/gh/oculairmedia/letta-MCP-server)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Letta MCP Server

A Model Context Protocol (MCP) server that provides comprehensive tools for agent management, memory operations, and integration with the Letta system. This server implements the full MCP specification including tools, prompts, and resources, with enhanced descriptions, output schemas, and behavioral annotations.

**[View on npm](https://www.npmjs.com/package/letta-mcp-server)** | **[View on GitHub](https://github.com/oculairmedia/Letta-MCP-server)**

## Features

- 🤖 **Agent Management** - Create, modify, clone, and manage Letta agents
- 🧠 **Memory Operations** - Handle memory blocks and passages
- 🔧 **Tool Integration** - Attach and manage tools for agents with full MCP support
- 💬 **Prompts** - Interactive wizards and assistants for common workflows
- 📚 **Resources** - Access system information, documentation, and agent data
- 🌐 **Multiple Transports** - HTTP, SSE, and stdio support
- 🔗 **MCP Server Integration** - Integrate with other MCP servers
- 📊 **Enhanced Metadata** - Output schemas and behavioral annotations for all tools
- 📦 **Docker Support** - Easy deployment with Docker
- 🔄 **Consolidated Tools** - 7 unified tools with 87 operations using discriminator pattern
- 🛡️ **SDK-Powered** - Built on official @letta-ai/letta-client v0.0.68664 (93% SDK coverage)
- ✅ **MCP Strict Mode** - Full compliance with `additionalProperties: false`
- ⚡ **Phase 3 Complete** - Full SDK migration with enhanced error handling

## Environment Configuration

Create a `.env` file with the following variables:

```bash
# Required
LETTA_BASE_URL=https://your-letta-instance.com/v1
LETTA_PASSWORD=your-secure-password

# Optional
PORT=3001
NODE_ENV=production
```

## Installation

### Install from npm

```bash
# Global installation (recommended for CLI usage)
npm install -g letta-mcp-server

# Or local installation
npm install letta-mcp-server
```

### Use with Claude Desktop

After installing globally, add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "letta": {
      "command": "letta-mcp",
      "args": [],
      "env": {
        "LETTA_BASE_URL": "https://your-letta-instance.com/v1",
        "LETTA_PASSWORD": "your-secure-password"
      }
    }
  }
}
```

### Quick Start with npm

```bash
# Install globally
npm install -g letta-mcp-server

# Set environment variables
export LETTA_BASE_URL=https://your-letta-instance.com/v1
export LETTA_PASSWORD=your-secure-password

# Run the server
letta-mcp              # stdio (for Claude Desktop)
letta-mcp --http       # HTTP transport
letta-mcp --sse        # SSE transport
```

## Quick Setup

### Option 1: Run from source

```bash
# Clone the repository
git clone https://github.com/oculairmedia/letta-MCP-server.git
cd letta-MCP-server

# Install dependencies
npm install

# Development
npm run dev         # Default (stdio) transport
npm run dev:sse     # SSE transport
npm run dev:http    # HTTP transport (recommended)

# Production
npm run start       # Default (stdio) transport
npm run start:sse   # SSE transport
npm run start:http  # HTTP transport (recommended)
```

### Option 2: Run with Docker

#### Using the prebuilt image from GitHub Container Registry

Available tags:
- `latest` - Latest stable release
- `2.0.1`, `2.0`, `2` - Specific version tags
- `master` - Latest master branch build

```bash
# Pull the latest image
docker pull ghcr.io/oculairmedia/letta-mcp-server:latest

# Run with environment variables
docker run -d \
  -p 3001:3001 \
  -e LETTA_BASE_URL=https://your-letta-instance.com/v1 \
  -e LETTA_PASSWORD=your-secure-password \
  -e PORT=3001 \
  -e NODE_ENV=production \
  --name letta-mcp \
  ghcr.io/oculairmedia/letta-mcp-server:latest

# Or use a specific version
docker run -d \
  -p 3001:3001 \
  -e LETTA_BASE_URL=https://your-letta-instance.com/v1 \
  -e LETTA_PASSWORD=your-secure-password \
  --name letta-mcp \
  ghcr.io/oculairmedia/letta-mcp-server:2.0.1
```

#### Using Docker Compose

```yaml
version: '3.8'
services:
  letta-mcp:
    image: ghcr.io/oculairmedia/letta-mcp-server:latest
    container_name: letta-mcp
    ports:
      - "3001:3001"
    environment:
      - LETTA_BASE_URL=https://your-letta-instance.com/v1
      - LETTA_PASSWORD=your-secure-password
      - PORT=3001
      - NODE_ENV=production
    restart: unless-stopped
```

#### Building from source

```bash
# Clone and build locally
git clone https://github.com/oculairmedia/letta-MCP-server.git
cd letta-MCP-server
docker build -t letta-mcp-server .
docker run -d -p 3001:3001 --env-file .env --name letta-mcp letta-mcp-server
```

### Option 3: Run with stdio for local MCP

```bash
# Create startup script
chmod +x /opt/stacks/letta-MCP-server/start-mcp.sh

# Add to Claude
claude mcp add --transport stdio letta-tools "/opt/stacks/letta-MCP-server/start-mcp.sh"
```

## Architecture

See the [Architecture Documentation](docs/ARCHITECTURE.md) for detailed system diagrams and component relationships.

## MCP Protocol Support

This server implements the full MCP specification with all three capabilities:

### 🔧 Tools
All tools include:
- **Enhanced Descriptions**: Detailed explanations with use cases and best practices
- **Output Schemas**: Structured response definitions for predictable outputs
- **Behavioral Annotations**: Hints about tool behavior (readOnly, costLevel, executionTime, etc.)

### 💬 Prompts
Interactive prompts for common workflows:
- `letta_agent_wizard` - Guided agent creation with memory and tool setup
- `letta_memory_optimizer` - Analyze and optimize agent memory usage
- `letta_debug_assistant` - Troubleshoot agent issues
- `letta_tool_config` - Discover, attach, create, or audit tools
- `letta_migration` - Export, import, upgrade, or clone agents

### 📚 Resources
Access system information and documentation:
- `letta://system/status` - System health and version info
- `letta://system/models` - Available LLM and embedding models
- `letta://agents/list` - Overview of all agents
- `letta://tools/all/docs` - Complete tool documentation with examples
- `letta://docs/mcp-integration` - Integration guide
- `letta://docs/api-reference` - API quick reference

Resource templates for dynamic content:
- `letta://agents/{agent_id}/config` - Agent configuration
- `letta://agents/{agent_id}/memory/{block_id}` - Memory block content
- `letta://tools/{tool_name}/docs` - Individual tool documentation

## Available Tools

### Consolidated Tools Architecture

The server provides **7 consolidated tools** covering **87 operations** using the discriminator pattern. Each tool uses an `operation` parameter to route to specific functionality, reducing tool count while maintaining comprehensive API coverage.

**🎉 Phase 3 Complete**: 81 out of 87 operations (93%) migrated from axios to the official Letta SDK, providing type-safe interactions, automatic error handling, and improved reliability.

#### Core Consolidated Tools

| Tool | Operations | Coverage | SDK Status |
|------|-----------|----------|------------|
| **letta_agent_advanced** | 22 | Complete agent lifecycle, messaging, context, export/import | ✅ 100% SDK |
| **letta_memory_unified** | 15 | Core memory, blocks, archival passages, search | ✅ 100% SDK |
| **letta_tool_manager** | 13 | Tool lifecycle, attach/detach, bulk operations | ✅ 85% SDK (2 awaiting) |
| **letta_mcp_ops** | 10 | MCP server management, tool discovery, execution | ✅ 80% SDK (2 awaiting) |
| **letta_source_manager** | 15 | Data sources, files, passages, attachments | ✅ 87% SDK (2 awaiting) |
| **letta_job_monitor** | 4 | Job tracking, cancellation, active monitoring | ✅ 100% SDK |
| **letta_file_folder_ops** | 8 | File sessions, folder management, agent files | ✅ 100% SDK |

**Total: 87 operations across 7 tools | 81 SDK-powered (93%)**

> **Migration Notice**: Individual endpoint tools (e.g., `create_agent`, `list_memory_blocks`) are deprecated. They now include automatic deprecation warnings pointing to their consolidated replacements. See the [Migration Guide](#migration-guide) below.

### SDK Integration & Migration Benefits

Built on the official **@letta-ai/letta-client** TypeScript SDK v0.0.68664, providing enterprise-grade reliability and developer experience.

#### SDK Features

**Type Safety & Validation**
- Full TypeScript definitions for all operations
- Compile-time parameter validation
- IDE auto-completion and inline documentation
- Reduced runtime errors from typos

**Automatic Error Handling**
- Built-in retry logic (maxRetries: 2)
- Intelligent timeout handling (30s default)
- Graceful fallback for transient failures
- Detailed error messages with context

**Performance & Reliability**
- Production-grade connection pooling (50 max connections per host)
- Keep-alive connection reuse (10 warm connections in pool)
- 30s request timeout aligned with SDK defaults
- Automatic request/response compression
- HTTP/2 support where available
- Reduced network overhead

**Code Quality**
- 70-80% code reduction per operation
- Single source of truth for API changes
- Consistent error handling across all operations
- Easier maintenance and updates

#### Migration Path

**Phase 3 (Complete ✅)**
- Migrated 81 out of 87 operations to SDK
- Enhanced error handling for both SDK and axios errors
- Comprehensive testing with 21 operation verifications
- Production-ready Docker builds

**Remaining Work**
- 6 operations awaiting SDK support (documented with TODO comments)
- Backward compatibility maintained for custom endpoints
- Deprecated individual tools to be removed in future release

For detailed migration progress, see [PHASE3_SDK_MIGRATION_PROGRESS.md](./PHASE3_SDK_MIGRATION_PROGRESS.md).

### Legacy Individual Tools (Deprecated)

#### Agent Management

| Tool | Description | Annotations |
|------|-------------|-------------|
| `create_agent` | Create a new Letta agent | 💰 Medium cost, ⚡ Fast |
| `list_agents` | List all available agents | 👁️ Read-only, 💰 Low cost |
| `prompt_agent` | Send a message to an agent | 💰 High cost, ⏱️ Variable time, 🔒 Rate limited |
| `retrieve_agent` | Get agent details by ID | 👁️ Read-only, ⚡ Fast |
| `get_agent_summary` | Get agent summary information | 👁️ Read-only, ⚡ Fast |
| `modify_agent` | Update an existing agent | ✏️ Modifies state, ⚡ Fast |
| `delete_agent` | Delete an agent | ⚠️ Dangerous, 🗑️ Permanent |
| `clone_agent` | Clone an existing agent | 💰 Medium cost, ⏱️ Medium time |
| `bulk_delete_agents` | Delete multiple agents | ⚠️ Dangerous, 📦 Bulk operation |
| `export_agent` | Export agent configuration and memory | 👁️ Read-only, ⚡ Fast, 📦 Full backup |
| `import_agent` | Import agent from backup | 💰 High cost, ⏱️ Slow, ✏️ Creates state |

### Memory Management

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_memory_blocks` | List all memory blocks | 👁️ Read-only, ⚡ Fast |
| `create_memory_block` | Create a new memory block | ✏️ Creates state, ⚡ Fast |
| `read_memory_block` | Read a memory block | 👁️ Read-only, ⚡ Fast |
| `update_memory_block` | Update a memory block | ✏️ Modifies state, ⚡ Fast |
| `attach_memory_block` | Attach memory to an agent | ✏️ Links resources, ⚡ Fast |

### Passage Management

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_passages` | Search archival memory | 👁️ Read-only, ⚡ Fast |
| `create_passage` | Create archival memory | 💰 Medium cost (embeddings), ⚡ Fast |
| `modify_passage` | Update archival memory | 💰 Medium cost (re-embedding), ⚡ Fast |
| `delete_passage` | Delete archival memory | 🗑️ Permanent, ⚡ Fast |

### Tool Management

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_agent_tools` | List tools for an agent | 👁️ Read-only, ⚡ Fast |
| `attach_tool` | Attach tools to an agent | ✏️ Modifies capabilities, ⚡ Fast |
| `upload_tool` | Upload a custom tool | 🔒 Security: Executes code, ⚡ Fast |
| `bulk_attach_tool_to_agents` | Attach tool to multiple agents | 📦 Bulk operation, ⏱️ Slow |

### Model Management

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_llm_models` | List available LLM models | 👁️ Read-only, ⚡ Fast |
| `list_embedding_models` | List available embedding models | 👁️ Read-only, ⚡ Fast |

### MCP Integration

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_mcp_servers` | List configured MCP servers | 👁️ Read-only, ⚡ Fast |
| `list_mcp_tools_by_server` | List tools from an MCP server | 👁️ Read-only, ⚡ Fast |
| `add_mcp_tool_to_letta` | Import MCP tool to Letta | ✏️ Creates tool, ⚡ Fast |

### Prompt Tools

| Tool | Description | Annotations |
|------|-------------|-------------|
| `list_prompts` | List available prompt templates | 👁️ Read-only, ⚡ Fast |
| `use_prompt` | Execute a prompt template | 💰 Variable cost, ⏱️ Variable time |

## Migration Guide

### Migrating from Individual Tools to Consolidated Tools

The consolidated tools provide the same functionality with a cleaner API. Here's how to migrate:

#### Example: Agent Operations

**Old (Deprecated):**
```javascript
// List agents
await callTool('list_agents', { filter: 'active' });

// Create agent
await callTool('create_agent', {
  name: 'my-agent',
  llm_model: 'gpt-4'
});

// Get agent
await callTool('retrieve_agent', { agent_id: 'agent-123' });
```

**New (Consolidated):**
```javascript
// List agents
await callTool('letta_agent_advanced', {
  operation: 'list',
  filters: { status: 'active' }
});

// Create agent
await callTool('letta_agent_advanced', {
  operation: 'create',
  name: 'my-agent',
  llm_model: 'gpt-4'
});

// Get agent
await callTool('letta_agent_advanced', {
  operation: 'get',
  agent_id: 'agent-123'
});
```

#### Example: Memory Operations

**Old (Deprecated):**
```javascript
// List memory blocks
await callTool('list_memory_blocks', { agent_id: 'agent-123' });

// Create memory block
await callTool('create_memory_block', {
  label: 'persona',
  value: 'You are a helpful assistant'
});

// List passages
await callTool('list_passages', {
  agent_id: 'agent-123',
  limit: 10
});
```

**New (Consolidated):**
```javascript
// List memory blocks
await callTool('letta_memory_unified', {
  operation: 'list_blocks',
  agent_id: 'agent-123'
});

// Create memory block
await callTool('letta_memory_unified', {
  operation: 'create_block',
  label: 'persona',
  value: 'You are a helpful assistant'
});

// List passages
await callTool('letta_memory_unified', {
  operation: 'list_passages',
  agent_id: 'agent-123',
  limit: 10
});
```

#### Complete Tool Mapping

| Old Tool | New Tool | Operation |
|----------|----------|-----------|
| `create_agent` | `letta_agent_advanced` | `create` |
| `list_agents` | `letta_agent_advanced` | `list` |
| `retrieve_agent` | `letta_agent_advanced` | `get` |
| `modify_agent` | `letta_agent_advanced` | `update` |
| `delete_agent` | `letta_agent_advanced` | `delete` |
| `prompt_agent` | `letta_agent_advanced` | `send_message` |
| `list_agent_tools` | `letta_agent_advanced` | `list_tools` |
| `export_agent` | `letta_agent_advanced` | `export` |
| `import_agent` | `letta_agent_advanced` | `import` |
| `clone_agent` | `letta_agent_advanced` | `clone` |
| `list_memory_blocks` | `letta_memory_unified` | `list_blocks` |
| `create_memory_block` | `letta_memory_unified` | `create_block` |
| `read_memory_block` | `letta_memory_unified` | `get_block` |
| `update_memory_block` | `letta_memory_unified` | `update_block` |
| `attach_memory_block` | `letta_memory_unified` | `attach_block` |
| `list_passages` | `letta_memory_unified` | `list_passages` |
| `create_passage` | `letta_memory_unified` | `create_passage` |
| `modify_passage` | `letta_memory_unified` | `update_passage` |
| `delete_passage` | `letta_memory_unified` | `delete_passage` |
| `attach_tool` | `letta_tool_manager` | `attach` |
| `upload_tool` | `letta_tool_manager` | `create` |
| `bulk_attach_tool_to_agents` | `letta_tool_manager` | `bulk_attach` |
| `list_mcp_servers` | `letta_mcp_ops` | `list_servers` |
| `list_mcp_tools_by_server` | `letta_mcp_ops` | `list_tools` |
| `add_mcp_tool_to_letta` | `letta_mcp_ops` | `register_tool` |

### Benefits of Consolidated Tools

1. **Reduced Tool Count**: 7 tools instead of 70+ individual endpoints
2. **Consistent Interface**: All tools follow the discriminator pattern
3. **Better Organization**: Operations grouped by domain
4. **Improved Documentation**: Comprehensive coverage in fewer tools
5. **MCP Compliance**: Full `additionalProperties: false` support
6. **SDK-Powered**: Type-safe operations with official Letta SDK

### Deprecation Timeline

- **Phase 1 (Current)**: Old tools remain functional with deprecation warnings
- **Phase 2 (Next Release)**: Old tools will log deprecation notices
- **Phase 3 (Future Release)**: Old tools will be removed from the codebase

We recommend migrating to consolidated tools now to prepare for future releases.

## Directory Structure

```
src/
├── index.js                      # Main entry point
├── core/
│   ├── server.js                 # LettaServer class with SDK integration
│   └── logger.js                 # Logging utilities
├── handlers/
│   ├── prompts.js                # Prompt handlers (wizards, assistants)
│   └── resources.js              # Resource handlers (system info, docs)
├── examples/
│   ├── prompts.js                # Example prompt templates
│   └── resources.js              # Example resource templates
├── tools/
│   ├── agents/
│   │   ├── letta-agent-advanced.js      # ✅ Consolidated (10 ops)
│   │   ├── create-agent.js              # ⚠️ Deprecated
│   │   ├── list-agents.js               # ⚠️ Deprecated
│   │   └── ... (other legacy tools)
│   ├── memory/
│   │   ├── letta-memory-unified.js      # ✅ Consolidated (7 ops)
│   │   ├── list-memory-blocks.js        # ⚠️ Deprecated
│   │   └── ... (other legacy tools)
│   ├── tools/
│   │   ├── letta-tool-manager.js        # ✅ Consolidated (10 ops)
│   │   ├── attach-tool.js               # ⚠️ Deprecated
│   │   └── ... (other legacy tools)
│   ├── mcp/
│   │   ├── letta-mcp-ops.js             # ✅ Consolidated (5 ops)
│   │   ├── list-mcp-servers.js          # ⚠️ Deprecated
│   │   └── ... (other legacy tools)
│   ├── sources/
│   │   └── letta-source-manager.js      # ✅ Consolidated (15 ops)
│   ├── jobs/
│   │   └── letta-job-monitor.js         # ✅ Consolidated (4 ops)
│   ├── files/
│   │   └── letta-file-folder-ops.js     # ✅ Consolidated (8 ops)
│   ├── models/
│   │   ├── list-llm-models.js           # Active (not yet consolidated)
│   │   └── list-embedding-models.js     # Active (not yet consolidated)
│   ├── prompts/
│   │   ├── list-prompts.js              # Active
│   │   └── use-prompt.js                # Active
│   ├── schemas/                          # Input schemas for tools
│   │   ├── agent-ops-schemas.js
│   │   ├── memory-ops-schemas.js
│   │   ├── tool-ops-schemas.js
│   │   ├── mcp-ops-schemas.js
│   │   ├── source-ops-schemas.js
│   │   ├── job-ops-schemas.js
│   │   └── file-ops-schemas.js
│   ├── index.js                          # Tool registration
│   ├── enhance-tools.js                  # Tool enhancement pipeline
│   ├── enhanced-descriptions.js          # Detailed tool descriptions
│   ├── output-schemas.js                 # Structured output definitions
│   ├── annotations.js                    # Behavioral hints
│   └── deprecated-tools.js               # Deprecation mapping
└── transports/
    ├── http-transport.js                 # Streamable HTTP (recommended)
    ├── sse-transport.js                  # Server-Sent Events
    └── stdio-transport.js                # Standard I/O
```

### Key Files

- **Consolidated Tools** (`letta-*-*.js`) - Main tool implementations using discriminator pattern
- **Schemas** (`schemas/*.js`) - Input validation schemas with Zod
- **Output Schemas** (`output-schemas.js`) - Response structure definitions
- **Deprecated Tools** (`deprecated-tools.js`) - Migration mapping for old tools
- **Enhanced Descriptions** (`enhanced-descriptions.js`) - Extended tool documentation
- **Annotations** (`annotations.js`) - Behavioral metadata (cost, speed, safety)

## Transport Protocols

The server supports three transport protocols:

1. **HTTP (Recommended)** - Streamable HTTP transport with full duplex communication
   - Endpoint: `http://your-server:3001/mcp`
   - Best for production use and remote connections
   - Supports health checks at `/health`

2. **SSE (Server-Sent Events)** - Real-time event streaming
   - Endpoint: `http://your-server:3001/sse`
   - Good for unidirectional server-to-client updates

3. **stdio** - Standard input/output
   - Direct process communication
   - Best for local development and Claude integration

## Configuration with MCP Settings

Add the server to your mcp_settings.json:

```json
"letta": {
  "command": "node",
  "args": [
    "--no-warnings",
    "--experimental-modules",
    "path/to/letta-server/src/index.js"
  ],
  "env": {
    "LETTA_BASE_URL": "https://your-letta-instance.com",
    "LETTA_PASSWORD": "yourPassword"
  },
  "disabled": false,
  "alwaysAllow": [
    "upload_tool",
    "attach_tool",
    "list_agents",
    "list_memory_blocks"
  ],
  "timeout": 300
}
```

For remote instances with HTTP transport (recommended):

```json
"remote_letta_tools": {
  "url": "http://your-server:3001/mcp",
  "transport": "http",
  "disabled": false,
  "alwaysAllow": [
    "attach_tool", 
    "list_agents",
    "list_tools",
    "get_agent"
  ],
  "timeout": 120
}
```

## Docker Operations

```bash
# View container logs
docker logs -f letta-mcp

# Stop the container
docker stop letta-mcp

# Update to latest version
docker pull ghcr.io/oculairmedia/letta-mcp-server:latest
docker stop letta-mcp
docker rm letta-mcp
docker run -d -p 3001:3001 -e PORT=3001 -e NODE_ENV=production --name letta-mcp ghcr.io/oculairmedia/letta-mcp-server:latest
```

## Troubleshooting

### Common Issues

1. **Connection refused errors**
   - Ensure the server is running and accessible
   - Check firewall settings for port 3001
   - Verify the correct transport protocol is being used

2. **Authentication failures**
   - Verify LETTA_BASE_URL includes `/v1` suffix
   - Check LETTA_PASSWORD is correct
   - Ensure environment variables are loaded
   - When self-hosting the Letta-Server, set environment variables accordingly:
     ```json
     "env": {
        "LETTA_BASE_URL": "http://localhost:8283",
        "LETTA_PASSWORD": "",
        "LOG_LEVEL": "info"
      }
     ```


3. **Tool execution timeouts**
   - Increase timeout values in MCP configuration
   - Check network latency for remote connections
   - Consider using HTTP transport for better reliability

### Health Check

The HTTP transport provides a health endpoint:

```bash
curl http://your-server:3001/health
```

Response:
```json
{
  "status": "healthy",
  "transport": "streamable_http",
  "protocol_version": "2025-06-18",
  "sessions": 0,
  "uptime": 12345.678
}
```

## Development

### Testing

```bash
# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Run linter
npm run lint
```

### Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details on:

- Development setup
- Code style and standards
- Adding new tools
- Testing requirements
- Pull request process

## Security

For security vulnerabilities, please see our [Security Policy](docs/SECURITY.md).

## License

MIT License - see LICENSE file for details
