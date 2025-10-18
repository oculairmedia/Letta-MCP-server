/**
 * Schemas for letta_mcp_ops tool
 * Provides discriminator-based schemas for MCP server lifecycle operations
 */

/**
 * Server configuration schema for stdio transport
 */
export const StdioServerConfigSchema = {
    type: 'object',
    properties: {
        type: {
            type: 'string',
            enum: ['stdio'],
            description: 'Transport type - stdio',
        },
        command: {
            type: 'string',
            description: 'Command to execute (e.g., "node", "python")',
        },
        args: {
            type: 'array',
            items: { type: 'string' },
            description: 'Command arguments',
        },
        env: {
            type: 'object',
            additionalProperties: { type: 'string' },
            description: 'Environment variables for the process',
        },
    },
    required: ['type', 'command'],
    additionalProperties: false,
};

/**
 * Server configuration schema for SSE transport
 */
export const SSEServerConfigSchema = {
    type: 'object',
    properties: {
        type: {
            type: 'string',
            enum: ['sse'],
            description: 'Transport type - SSE (Server-Sent Events)',
        },
        url: {
            type: 'string',
            format: 'uri',
            description: 'SSE endpoint URL',
        },
        headers: {
            type: 'object',
            additionalProperties: { type: 'string' },
            description: 'HTTP headers for SSE connection',
        },
    },
    required: ['type', 'url'],
    additionalProperties: false,
};

/**
 * OAuth configuration schema for server connection
 */
export const OAuthConfigSchema = {
    type: 'object',
    properties: {
        callback_url: {
            type: 'string',
            format: 'uri',
            description: 'OAuth callback URL',
        },
        session_id: {
            type: 'string',
            description: 'Session ID for OAuth flow',
        },
        scopes: {
            type: 'array',
            items: { type: 'string' },
            description: 'OAuth scopes to request',
        },
    },
    required: ['callback_url', 'session_id'],
    additionalProperties: false,
};

/**
 * Pagination options for list operations
 */
export const PaginationSchema = {
    type: 'object',
    properties: {
        limit: {
            type: 'integer',
            minimum: 1,
            maximum: 100,
            description: 'Maximum number of results',
        },
        offset: {
            type: 'integer',
            minimum: 0,
            description: 'Number of results to skip',
        },
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_mcp_ops tool
 */
export const mcpOpsInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                'add',
                'update',
                'delete',
                'test',
                'connect',
                'resync',
                'execute',
                'list_servers',
                'list_tools',
                'register_tool',
            ],
            description: 'Operation to perform on MCP servers',
        },
        server_name: {
            type: 'string',
            description:
                'MCP server name (required for update, delete, resync, execute, list_tools, register_tool operations)',
        },
        server_config: {
            oneOf: [StdioServerConfigSchema, SSEServerConfigSchema],
            description: 'Server configuration (required for add/update operations)',
        },
        tool_name: {
            type: 'string',
            description: 'Tool name (required for execute and register_tool operations)',
        },
        tool_args: {
            type: 'object',
            additionalProperties: true,
            description: 'Arguments for tool execution (for execute operation)',
        },
        oauth_config: {
            ...OAuthConfigSchema,
            description: 'OAuth configuration (required for connect operation)',
        },
        pagination: {
            ...PaginationSchema,
            description: 'Pagination options (for list operations)',
        },
        request_heartbeat: {
            type: 'boolean',
            description: 'Ignored parameter (for MCP client compatibility)',
        },
    },
    required: ['operation'],
    additionalProperties: true,
};

/**
 * Output schema for letta_mcp_ops tool
 */
export const mcpOpsOutputSchema = {
    type: 'object',
    properties: {
        success: {
            type: 'boolean',
            description: 'Whether the operation succeeded',
        },
        operation: {
            type: 'string',
            description: 'Operation that was performed',
        },
        server_name: {
            type: 'string',
            description: 'Name of the MCP server',
        },
        server_config: {
            oneOf: [StdioServerConfigSchema, SSEServerConfigSchema],
            description: 'Server configuration (for add/update/test operations)',
        },
        servers: {
            type: 'array',
            items: {
                type: 'object',
                properties: {
                    name: { type: 'string' },
                    type: { type: 'string', enum: ['stdio', 'sse'] },
                    status: { type: 'string' },
                },
                additionalProperties: false,
            },
            description: 'List of servers (for list operations)',
        },
        test_result: {
            type: 'object',
            properties: {
                connected: { type: 'boolean' },
                latency_ms: { type: 'number' },
                error: { type: 'string' },
            },
            additionalProperties: false,
            description: 'Test connection result',
        },
        execution_result: {
            type: 'object',
            additionalProperties: true,
            description: 'Result from tool execution',
        },
        oauth_url: {
            type: 'string',
            format: 'uri',
            description: 'OAuth authorization URL (for connect operation)',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
