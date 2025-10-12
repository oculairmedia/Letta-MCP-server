/**
 * Tool handler for letta_mcp_ops - MCP Server Operations Hub
 * Provides unified interface for complete MCP server lifecycle management
 */
import { createLogger } from '../../core/logger.js';
// eslint-disable-next-line no-unused-vars
import { mcpOpsInputSchema, mcpOpsOutputSchema } from '../schemas/mcp-ops-schemas.js';

const logger = createLogger('letta_mcp_ops');

/**
 * Handle letta_mcp_ops tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following mcpOpsInputSchema
 * @returns {Promise<Object>} Tool response following mcpOpsOutputSchema
 */
export async function handleLettaMcpOps(server, args) {
    const { operation } = args;

    logger.info(`Executing MCP operation: ${operation}`, { args });

    try {
        switch (operation) {
            case 'add':
                return await handleAddServer(server, args);
            case 'update':
                return await handleUpdateServer(server, args);
            case 'delete':
                return await handleDeleteServer(server, args);
            case 'test':
                return await handleTestServer(server, args);
            case 'connect':
                return await handleConnectServer(server, args);
            case 'resync':
                return await handleResyncServer(server, args);
            case 'execute':
                return await handleExecuteTool(server, args);
            case 'list_servers':
                return await handleListServers(server, args);
            case 'list_tools':
                return await handleListTools(server, args);
            case 'register_tool':
                return await handleRegisterTool(server, args);
            default:
                throw new Error(`Unknown operation: ${operation}`);
        }
    } catch (error) {
        logger.error(`MCP operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * Add a new MCP server
 */
async function handleAddServer(server, args) {
    const { server_config } = args;

    if (!server_config) {
        throw new Error('server_config is required for add operation');
    }

    // Use direct API call as SDK may not have this endpoint yet
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.put('/tools/mcp/servers', server_config, { headers });
            return response.data;
        },
        'Adding MCP server'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'add',
                    server_name: server_config.name || 'unnamed',
                    server_config: result,
                    message: 'MCP server added successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing MCP server
 */
async function handleUpdateServer(server, args) {
    const { server_name, server_config } = args;

    if (!server_name) {
        throw new Error('server_name is required for update operation');
    }
    if (!server_config) {
        throw new Error('server_config is required for update operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.patch(
                `/tools/mcp/servers/${encodeURIComponent(server_name)}`,
                server_config,
                { headers }
            );
            return response.data;
        },
        'Updating MCP server'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update',
                    server_name,
                    server_config: result,
                    message: 'MCP server updated successfully',
                }),
            },
        ],
    };
}

/**
 * Delete an MCP server
 */
async function handleDeleteServer(server, args) {
    const { server_name } = args;

    if (!server_name) {
        throw new Error('server_name is required for delete operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(
                `/tools/mcp/servers/${encodeURIComponent(server_name)}`,
                { headers }
            );
            return response.data;
        },
        'Deleting MCP server'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete',
                    server_name,
                    message: 'MCP server deleted successfully',
                }),
            },
        ],
    };
}

/**
 * Test MCP server connection
 */
async function handleTestServer(server, args) {
    const { server_config } = args;

    if (!server_config) {
        throw new Error('server_config is required for test operation');
    }

    const startTime = Date.now();
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post('/tools/mcp/servers/test', server_config, {
                headers,
            });
            return response.data;
        },
        'Testing MCP server connection'
    );
    const latency = Date.now() - startTime;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'test',
                    test_result: {
                        connected: true,
                        latency_ms: latency,
                        ...result,
                    },
                    message: 'MCP server connection successful',
                }),
            },
        ],
    };
}

/**
 * Connect to MCP server with OAuth
 */
async function handleConnectServer(server, args) {
    const { server_name, oauth_config } = args;

    if (!server_name) {
        throw new Error('server_name is required for connect operation');
    }
    if (!oauth_config) {
        throw new Error('oauth_config is required for connect operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                '/tools/mcp/servers/connect',
                {
                    server_name,
                    ...oauth_config,
                },
                { headers }
            );
            return response.data;
        },
        'Connecting to MCP server with OAuth'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'connect',
                    server_name,
                    oauth_url: result.authorization_url || result.url,
                    message: 'OAuth flow initiated. Use the provided URL to authorize.',
                }),
            },
        ],
    };
}

/**
 * Resync MCP server tools
 */
async function handleResyncServer(server, args) {
    const { server_name } = args;

    if (!server_name) {
        throw new Error('server_name is required for resync operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/tools/mcp/servers/${encodeURIComponent(server_name)}/resync`,
                {},
                { headers }
            );
            return response.data;
        },
        'Resyncing MCP server'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'resync',
                    server_name,
                    servers: result.tools || result,
                    message: 'MCP server resynced successfully',
                }),
            },
        ],
    };
}

/**
 * Execute a tool from an MCP server
 */
async function handleExecuteTool(server, args) {
    const { server_name, tool_name, tool_args = {} } = args;

    if (!server_name) {
        throw new Error('server_name is required for execute operation');
    }
    if (!tool_name) {
        throw new Error('tool_name is required for execute operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/tools/mcp/servers/${encodeURIComponent(server_name)}/tools/${encodeURIComponent(tool_name)}/execute`,
                tool_args,
                { headers }
            );
            return response.data;
        },
        'Executing MCP tool'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'execute',
                    server_name,
                    tool_name,
                    execution_result: result,
                    message: 'Tool executed successfully',
                }),
            },
        ],
    };
}

/**
 * List all MCP servers
 */
async function handleListServers(server, args) {
    const { pagination = {} } = args;

    const queryParams = new URLSearchParams();
    if (pagination.limit) queryParams.append('limit', pagination.limit);
    if (pagination.offset) queryParams.append('offset', pagination.offset);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/tools/mcp/servers${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Listing MCP servers'
    );

    const servers = Array.isArray(result) ? result : result.servers || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_servers',
                    servers: servers.map(s => ({
                        name: s.name,
                        type: s.type || s.transport,
                        status: s.status || 'unknown',
                    })),
                    message: `Found ${servers.length} MCP servers`,
                }),
            },
        ],
    };
}

/**
 * List tools from an MCP server
 */
async function handleListTools(server, args) {
    const { server_name, pagination = {} } = args;

    if (!server_name) {
        throw new Error('server_name is required for list_tools operation');
    }

    const queryParams = new URLSearchParams();
    if (pagination.limit) queryParams.append('limit', pagination.limit);
    if (pagination.offset) queryParams.append('offset', pagination.offset);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/tools/mcp/servers/${encodeURIComponent(server_name)}/tools${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Listing MCP server tools'
    );

    const tools = Array.isArray(result) ? result : result.tools || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_tools',
                    server_name,
                    tools: tools.map(t => ({
                        name: t.name,
                        description: t.description,
                        schema: t.schema || t.inputSchema,
                    })),
                    message: `Found ${tools.length} tools on server ${server_name}`,
                }),
            },
        ],
    };
}

/**
 * Register an MCP tool in Letta
 */
async function handleRegisterTool(server, args) {
    const { server_name, tool_name } = args;

    if (!server_name) {
        throw new Error('server_name is required for register_tool operation');
    }
    if (!tool_name) {
        throw new Error('tool_name is required for register_tool operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/tools/mcp/servers/${encodeURIComponent(server_name)}/tools/${encodeURIComponent(tool_name)}/register`,
                {},
                { headers }
            );
            return response.data;
        },
        'Registering MCP tool in Letta'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'register_tool',
                    server_name,
                    tool_name,
                    tool_id: result.id || result.tool_id,
                    message: `Tool ${tool_name} from ${server_name} registered successfully in Letta`,
                }),
            },
        ],
    };
}

/**
 * Tool definition for letta_mcp_ops
 */
export const lettaMcpOpsDefinition = {
    name: 'letta_mcp_ops',
    description:
        'MCP Server Operations Hub - Unified tool for complete MCP server lifecycle management including server management (add, update, delete, test, connect, resync), tool operations (list_servers, list_tools, register_tool, execute). Provides 10 operations with discriminator-based operation routing. Replaces 10 individual MCP server endpoints.',
    inputSchema: mcpOpsInputSchema,
};
