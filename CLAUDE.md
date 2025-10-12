# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Management

This repository is tracked in the Huly project management system:
- **Project**: Letta MCP Server (LMS)
- **Description**: Model Context Protocol server for Letta AI - providing comprehensive API coverage through consolidated, SDK-based tools
- **Issues**: Track bugs, features, and improvements in Huly under the LMS project identifier

## Current Status

**Phase 3 Complete ✅** - SDK Migration and Error Handling
- 81 out of 87 operations (93%) migrated to official Letta SDK
- Enhanced error handling for both SDK and axios errors
- 7 consolidated tools with 87 operations
- Comprehensive testing and verification complete
- See [PHASE3_SDK_MIGRATION_PROGRESS.md](./PHASE3_SDK_MIGRATION_PROGRESS.md) for details

## Commands

### Development
```bash
# Install dependencies
npm install

# Run server with different transports
npm run dev         # stdio transport (for local MCP integration)
npm run dev:sse     # SSE transport
npm run dev:http    # HTTP transport (recommended for production)

# Production
npm run start       # stdio transport
npm run start:sse   # SSE transport  
npm run start:http  # HTTP transport
```

### Docker Operations
```bash
# Build image
docker build -t letta-mcp-server .

# Run container
docker run -d -p 3001:3001 \
  -e LETTA_BASE_URL=https://your-letta-instance.com/v1 \
  -e LETTA_PASSWORD=your-password \
  -e PORT=3001 \
  -e NODE_ENV=production \
  --name letta-mcp \
  letta-mcp-server

# Health check
curl http://localhost:3001/health
```

### Testing Individual Tools
```bash
# Run server on alternate port for testing
PORT=3002 npm run dev:http

# Test tool functionality via MCP
curl -X POST http://localhost:3002/mcp \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

## Architecture

### Core Structure
The server implements the Model Context Protocol (MCP) specification with a modular architecture:

1. **Entry Point** (`src/index.js`)
   - Initializes LettaServer instance
   - Registers all tool handlers via `registerToolHandlers()`
   - Determines transport mode from CLI args (`--http`, `--sse`, or default stdio)
   - Launches appropriate transport handler

2. **Server Core** (`src/core/server.js`)
   - `LettaServer` class manages MCP server instance and Letta API communication
   - Validates environment variables (`LETTA_BASE_URL`, `LETTA_PASSWORD`)
   - Initializes official Letta SDK client (`@letta-ai/letta-client` v0.0.68664)
   - Provides backward-compatible axios instance for custom endpoints
   - Implements unified error handling via `handleSdkCall()` for both SDK and axios errors
   - SDK features: automatic retries (maxRetries: 2), timeout handling (30s)

3. **Transport Layer** (`src/transports/`)
   - **http-transport.js**: Streamable HTTP with SSE fallback, session management, CORS, health endpoint
   - **sse-transport.js**: Server-Sent Events for unidirectional streaming
   - **stdio-transport.js**: Standard I/O for local process communication
   - All transports handle MCP protocol compliance and message routing

4. **Tool System** (`src/tools/`)
   - **Consolidated Tools** (7 files): Modern SDK-based tools with discriminator pattern
     - `agents/letta-agent-advanced.js` (22 operations)
     - `memory/letta-memory-unified.js` (15 operations)
     - `tools/letta-tool-manager.js` (13 operations)
     - `mcp/letta-mcp-ops.js` (10 operations)
     - `sources/letta-source-manager.js` (15 operations)
     - `jobs/letta-job-monitor.js` (4 operations)
     - `files/letta-file-folder-ops.js` (8 operations)
   - **Legacy Tools**: Individual endpoint tools (deprecated, to be removed)
   - Each tool exports:
     - Handler function: Async function that processes requests
     - Tool definition: JSON schema describing parameters
   - `index.js` aggregates all tools and registers handlers with MCP server
   - `schemas/` directory: Input validation schemas using Zod

### Key Patterns

**Modern Tool Implementation Pattern (SDK-Based)**:
```javascript
// Consolidated tool with discriminator pattern
export async function handleLettaToolName(server, args) {
    const { operation } = args;

    try {
        const handlers = {
            operation_one: handleOperationOne,
            operation_two: handleOperationTwo,
        };

        if (!handlers[operation]) {
            throw new Error(`Unknown operation: ${operation}`);
        }

        return await handlers[operation](server, args);
    } catch (error) {
        logger.error(`Operation failed: ${operation}`, { error, args });
        throw error;
    }
}

// Individual operation handler using SDK
async function handleOperationOne(server, args) {
    const { param1, param2 } = args;

    // Use handleSdkCall wrapper for automatic error handling
    const result = await server.handleSdkCall(
        async () => {
            // Use Letta SDK client methods
            return await server.client.agents.retrieve(param1);
        },
        'Getting agent details'  // Context for error messages
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'operation_one',
                data: result,
                message: 'Operation completed successfully'
            })
        }]
    };
}

// Tool definition
export const lettaToolNameDefinition = {
    name: 'letta_tool_name',
    description: 'Consolidated tool description with multiple operations',
    inputSchema: toolNameInputSchema  // Zod schema with discriminator
};
```

**Legacy Tool Pattern (Deprecated)**:
```javascript
// Old pattern - still functional but deprecated
export async function handleOldTool(server, args) {
    const result = await server.handleSdkCall(
        async () => {
            // For endpoints without SDK support, use axios
            const headers = server.getApiHeaders();
            const response = await server.api.post('/custom-endpoint', data, { headers });
            return response.data;
        },
        'Custom operation'
    );

    return { content: [{ type: 'text', text: JSON.stringify(result) }] };
}
```

**Error Handling**:
- Unified error handling via `handleSdkCall()` wrapper method
- Automatically detects and processes both Letta SDK errors and axios errors
- SDK errors: `LettaError`, `LettaTimeoutError` with statusCode and body properties
- Axios errors: Standard axios error format with response.status and response.data
- HTTP status codes mapped to appropriate MCP error codes:
  - 400 → InvalidParams
  - 401/403 → InvalidRequest
  - 404 → InvalidRequest
  - 422 → InvalidParams
  - 500+ → InternalError
- Detailed error context preserved for debugging
- Automatic retry logic (2 retries) for transient failures
- Timeout handling (30s default)

**Session Management** (HTTP Transport):
- Sessions identified by UUID, stored in memory
- Automatic cleanup of inactive sessions (5-minute timeout)
- Session required for all non-initialize requests

### Environment Variables
- `LETTA_BASE_URL`: Letta API base URL (must include /v1 suffix)
- `LETTA_PASSWORD`: Authentication password for Letta API
- `PORT`: Server port (default: 3001)
- `NODE_ENV`: Environment mode (development/production)

### API Communication

**Primary: Letta SDK** (93% of operations)
1. Use official `@letta-ai/letta-client` SDK via `server.client`
2. Type-safe method calls (e.g., `server.client.agents.retrieve()`)
3. Automatic authentication, retries, and timeout handling
4. Wrapped in `handleSdkCall()` for error transformation
5. Return MCP-formatted tool responses

**Fallback: Axios** (7% of operations - awaiting SDK support)
1. Use axios instance via `server.api` for custom endpoints
2. Include authentication headers via `getApiHeaders()`
3. Wrapped in `handleSdkCall()` for consistent error handling
4. Return MCP-formatted tool responses

**Connection Pooling** (Production-ready configuration)
1. HTTP/HTTPS agents with keepAlive enabled for connection reuse
2. MaxSockets: 50 concurrent connections per host (prevents exhaustion)
3. MaxFreeSockets: 10 warm connections in pool (faster subsequent requests)
4. Socket timeout: 60s for connection establishment
5. Request timeout: 30s (aligned with SDK timeout)

**SDK Client Structure:**
- `server.client.agents.*` - Agent operations
- `server.client.agents.tools.*` - Agent-tool relationships
- `server.client.agents.coreMemory.*` - Core memory operations
- `server.client.agents.blocks.*` - Memory blocks for agents
- `server.client.agents.passages.*` - Archival memory
- `server.client.agents.messages.*` - Message operations
- `server.client.agents.sources.*` - Agent-source relationships
- `server.client.agents.files.*` - Agent file operations
- `server.client.agents.folders.*` - Agent-folder relationships
- `server.client.blocks.*` - Standalone memory blocks
- `server.client.tools.*` - Tool operations
- `server.client.sources.*` - Source operations
- `server.client.jobs.*` - Job monitoring
- `server.client.folders.*` - Folder operations

### Adding New Tools

**For Consolidated Tools (Recommended):**
1. Identify the appropriate consolidated tool file (e.g., `letta-agent-advanced.js`)
2. Add new operation to the handlers object in main function
3. Create handler function using SDK methods via `server.client.*`
4. Wrap SDK calls in `handleSdkCall()` for error handling
5. Update input schema in `src/tools/schemas/` with new operation
6. Return MCP-formatted response with operation name

**For New Standalone Tools (If Needed):**
1. Create new file in appropriate subdirectory under `src/tools/`
2. Define Zod schema for input validation in `src/tools/schemas/`
3. Implement handler using SDK-first approach
4. Export tool definition and handler function
5. Import and register in `src/tools/index.js`
6. Follow SDK-based error handling patterns

**Best Practices:**
- **SDK First**: Always check if SDK has the method before using axios
- **Error Handling**: Always wrap API calls in `handleSdkCall()`
- **Context Messages**: Provide clear context strings for errors
- **Response Format**: Follow consistent MCP response structure
- **Logging**: Use `logger.info()` for operations, `logger.error()` for failures
- **Documentation**: Update README.md and add JSDoc comments