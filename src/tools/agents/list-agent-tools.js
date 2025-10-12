/**
 * Tool handler for listing tools available for a specific agent
 */
export async function handleListAgentTools(server, args) {
    try {
        if (!args.agent_id) {
            throw new Error('Missing required argument: agent_id');
        }

        const headers = server.getApiHeaders();

        const agentInfoResponse = await server.api.get(`/agents/${args.agent_id}`, { headers });
        const agentName = agentInfoResponse.data.name;
        const tools = agentInfoResponse.data.tools || [];

        // Trim tool objects to essential fields only to reduce token usage
        // Full tool objects contain 15+ fields including source_code, json_schema, etc.
        // We only need 5 essential fields for tool listing
        const trimmedTools = tools.map((tool) => ({
            id: tool.id,
            name: tool.name,
            description: tool.description,
            source_type: tool.source_type || tool.sourceType, // Handle both formats
            tags: tool.tags || [],
        }));

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent_id: args.agent_id,
                        agent_name: agentName,
                        tool_count: trimmedTools.length,
                        tools: trimmedTools,
                    }),
                },
            ],
        };
    } catch (error) {
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for list_agent_tools
 */
export const listAgentToolsDefinition = {
    name: 'list_agent_tools',
    description:
        'List all tools available for a specific agent. Use attach_tool to add more tools or list_mcp_tools_by_server to discover available tools.',
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent to list tools for',
            },
        },
        required: ['agent_id'],
        additionalProperties: false,
    },
};
