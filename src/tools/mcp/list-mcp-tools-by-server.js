import { createLogger } from '../../core/logger.js';

const logger = createLogger('list-mcp-tools-by-server');

/**
 * Tool handler for listing all available tools for a specific MCP server
 */
export async function handleListMcpToolsByServer(server, args) {
    if (!args?.mcp_server_name) {
        throw new Error('Missing required argument: mcp_server_name');
    }

    try {
        const serverName = encodeURIComponent(args.mcp_server_name);
        // Construct the relative API path
        const api_path = `/tools/mcp/servers/${serverName}/tools`;

        // Get headers using the server's built-in method
        const headers = server.getApiHeaders();

        // Use the server's configured api instance and get method
        const response = await server.api.get(api_path, {
            headers,
            timeout: 60000, // Keep the increased timeout
        });

        let tools = response.data; // Assuming response.data is an array of MCPTool objects

        // Apply filtering if provided
        if (args?.filter) {
            const filterLower = args.filter.toLowerCase();
            tools = tools.filter(
                (tool) =>
                    (tool.name && tool.name.toLowerCase().includes(filterLower)) ||
                    (tool.description && tool.description.toLowerCase().includes(filterLower)),
            );
        }

        // Apply pagination
        const page = args?.page || 1;
        const pageSize = args?.pageSize || 10;
        const startIndex = (page - 1) * pageSize;
        const endIndex = startIndex + pageSize;
        const totalTools = tools.length;
        const totalPages = Math.ceil(totalTools / pageSize);
        const paginatedTools = tools.slice(startIndex, endIndex);

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        mcp_server_name: args.mcp_server_name,
                        pagination: {
                            page,
                            pageSize,
                            totalTools,
                            totalPages,
                            hasNextPage: page < totalPages,
                            hasPreviousPage: page > 1,
                        },
                        tool_count: paginatedTools.length,
                        tools: paginatedTools,
                    }),
                },
            ],
        };
    } catch (error) {
        logger.error('Full error:', error.message); // Log message only to avoid circular refs
        // Handle potential 404 if server name not found, or other API errors
        if (error.response && error.response.status === 404) {
            throw new Error(`MCP Server not found: ${args.mcp_server_name}`);
        }
        // Safely extract error details without circular references
        let errorDetails = '';
        try {
            if (error.response?.data) {
                errorDetails = JSON.stringify(error.response.data);
            }
        } catch {
            errorDetails = String(error.response?.data || '');
        }
        throw new Error(
            `Error executing list_mcp_tools_by_server: ${error.message}${errorDetails ? `\nResponse: ${errorDetails}` : ''}`,
        );
    }
}

/**
 * Tool definition for list_mcp_tools_by_server
 */
export const listMcpToolsByServerDefinition = {
    name: 'list_mcp_tools_by_server',
    description:
        'List all available tools for a specific MCP server. Use list_mcp_servers first to see available servers, then add_mcp_tool_to_letta to import tools into Letta.',
    inputSchema: {
        type: 'object',
        properties: {
            mcp_server_name: {
                type: 'string',
                description: 'The name of the MCP server to list tools for',
            },
            filter: {
                type: 'string',
                description: 'Optional filter to search for specific tools by name or description',
            },
            page: {
                type: 'number',
                description: 'Page number for pagination (starts at 1)',
            },
            pageSize: {
                type: 'number',
                description: 'Number of tools per page (1-100, default: 10)',
            },
        },
        required: ['mcp_server_name'], // mcp_server_name is now required
    },
};
