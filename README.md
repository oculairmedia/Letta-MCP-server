# Letta MCP Server

An MCP (Model Context Protocol) server implementation for interacting with the Letta API. This server provides tools for managing agents, memory blocks, and tools in the Letta system.

## Features

- Create and manage Letta agents
- List and filter available agents
- Create, read, update, and manage memory blocks
- List memory blocks with filtering and pagination
- Attach memory blocks to agents with custom labels
- List and manage agent tools
- Send messages to agents and receive responses

## Installation

```bash
# Clone the repository
git clone https://github.com/oculairmedia/Letta-MCP-server.git
cd letta-server

# Install dependencies
npm install
```

## Configuration

1. Create a `.env` file in the root directory with the following variables:
```
LETTA_BASE_URL=your_letta_api_url
LETTA_PASSWORD=your_letta_api_password
```

You can use the provided `.env.example` as a template.

## Available Scripts

- `npm run build`: Build the TypeScript code
- `npm run start`: Build and start the server
- `npm run dev`: Start the server in development mode with watch mode enabled

## Tools

### Agent Configuration

Agents can be configured with various options:
- Model selection (e.g., 'gpt-4', default: 'openai/gpt-4')
- Embedding model (default: 'openai/text-embedding-ada-002')
- Context window size (default: 16000)
- Temperature and token settings
- Custom function configurations

### Memory Block Types

Memory blocks serve different purposes based on their labels:
- `persona`: Define agent personality and behavior
- `human`: Store conversation history and user preferences
- `system`: Store system-level instructions and configurations
- `custom`: User-defined memory blocks for specific use cases

### Agent Management
- `create_agent`: Create a new Letta agent with specified configuration
- `list_agents`: List all available agents in the Letta system
- `prompt_agent`: Send a message to an agent and get a response
### Memory Block Management
- `create_memory_block`: Create a new memory block with name, label, and content
- `read_memory_block`: Get full details of a specific memory block
- `update_memory_block`: Update contents and metadata of a memory block
- `list_memory_blocks`: List memory blocks with filtering options:
  * Filter by name, label, or content
  * Filter by agent
  * Filter templates only
  * Pagination support
  * Include full content or previews
- `attach_memory_block`: Attach a memory block to an agent with custom labels

### Tool Management
- `list_tools`: List all available tools with filtering and pagination
- `list_agent_tools`: List tools available for a specific agent
- `attach_tool`: Attach a tool to an agent
- `upload_tool`: Upload a new Python tool with:
  * Custom name and description
  * Source code implementation
  * Category/tag support
  * Optional automatic agent attachment

## API Version

This server interacts with version 1 of the Letta API (endpoint: `/v1`). The API version is automatically handled by the server based on the configured `LETTA_BASE_URL`.

## Example Usage

When integrated with Cline, you can use the MCP tools as follows:

### Memory Block Operations

```typescript
// Create a memory block
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>create_memory_block</tool_name>
<arguments>
{
  "name": "example_block",
  "label": "custom",
  "value": "This is an example memory block.",
  "metadata": {
    "version": "1.0",
    "type": "documentation"
  }
}
</arguments>
</use_mcp_tool>

// List memory blocks with filtering
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>list_memory_blocks</tool_name>
<arguments>
{
  "label": "custom",
  "page": 1,
  "pageSize": 10,
  "include_full_content": true
}
</arguments>
</use_mcp_tool>

// Update a memory block
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>update_memory_block</tool_name>
<arguments>
{
  "block_id": "block-123",
  "value": "Updated content",
  "metadata": {
    "version": "1.1"
  }
}
</arguments>
</use_mcp_tool>

// Attach block to agent with label
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>attach_memory_block</tool_name>
<arguments>
{
  "block_id": "block-123",
  "agent_id": "agent-456",
  "label": "persona"
}
</arguments>
</use_mcp_tool>
```

### Tool Management

```typescript
// Upload a new tool
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>upload_tool</tool_name>
<arguments>
{
  "name": "weather_tool",
  "description": "Get weather information for a location",
  "source_code": "def get_weather(location):\n    # Tool implementation\n    return {'temp': 72, 'condition': 'sunny'}",
  "category": "utilities",
  "agent_id": "agent-456"  // Optional: automatically attach to agent
}
</arguments>
</use_mcp_tool>

// List tools with filtering
<use_mcp_tool>
<server_name>letta</server_name>
<tool_name>list_tools</tool_name>
<arguments>
{
  "filter": "weather",
  "page": 1,
  "pageSize": 10
}
</arguments>
</use_mcp_tool>
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Response Format

All MCP tools return responses in a consistent format:

```typescript
{
  "success": boolean,
  "message": string,           // Success/error message
  "error"?: string,           // Present only on error
  "details"?: any,            // Additional error details if available
  // Tool-specific data...
}
```

## Error Handling

The server handles various error scenarios:
- Invalid arguments or missing required parameters
- API authentication failures
- Resource not found errors
- Rate limiting and quota errors
- Network connectivity issues

Each error response includes detailed information to help troubleshoot issues.

## Performance Considerations

- Memory blocks support pagination to handle large datasets efficiently
- Tool source code is validated before upload
- Streaming support for agent responses to handle long conversations
- Automatic cleanup of old/unused resources
- Request rate limiting to prevent API overload

## License

This project is licensed under the MIT License - see the LICENSE file for details.