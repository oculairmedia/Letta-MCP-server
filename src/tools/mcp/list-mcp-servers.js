/**
 * Tool handler for listing all configured MCP servers on the Letta server
 */
export async function handleListMcpServers(server, _args) {
    try {
        const headers = server.getApiHeaders();

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.get('/tools/mcp/servers', { headers });
        const servers = response.data; // Assuming response.data is an object mapping server names to configs

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        server_count: Object.keys(servers).length,
                        servers: servers,
                    }),
                },
            ],
        };
    } catch (error) {
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for list_mcp_servers
 */
export const listMcpServersDefinition = {
    name: 'list_mcp_servers',
    description:
        'List all configured MCP servers on the Letta server. Use with list_mcp_tools_by_server to explore available tools from each server.',
    inputSchema: {
        type: 'object',
        properties: {}, // No input arguments needed
        required: [],
        additionalProperties: false,
    },
};
