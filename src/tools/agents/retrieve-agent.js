/**
 * Tool handler for retrieving the state of a specific agent
 */
export async function handleRetrieveAgent(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.get(`/agents/${agentId}`, { headers });
        const agentState = response.data; // Assuming response.data is the AgentState object

        // Trim agent object to essential fields only to reduce token usage
        // Full AgentState objects contain 30+ fields including full tools array, memory blocks, etc.
        // We only need 10-12 essential fields for agent details
        const trimmedAgent = {
            id: agentState.id,
            name: agentState.name,
            description: agentState.description,
            system: agentState.system,
            llm_config: agentState.llm_config,
            embedding_config: agentState.embedding_config,
            tool_ids: agentState.tool_ids || [],           // Just IDs, not full tool objects
            source_ids: agentState.source_ids || [],       // Just IDs, not full source objects
            block_ids: agentState.block_ids || [],         // Just IDs, not full block objects
            message_count: agentState.message_count,
            created_at: agentState.created_at,
            updated_at: agentState.updated_at,
        };

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent: trimmedAgent,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle potential 404 if agent not found, or other API errors
        if (error.response && error.response.status === 404) {
            server.createErrorResponse(`Agent not found: ${args.agent_id}`);
        }
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for retrieve_agent
 */
export const retrieveAgentDefinition = {
    name: 'retrieve_agent',
    description:
        'Get the full state of a specific agent by ID. Similar to get_agent_summary but returns complete details. Use list_agents to find agent IDs.',
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to retrieve',
            },
        },
        required: ['agent_id'],
        additionalProperties: false,
    },
};
