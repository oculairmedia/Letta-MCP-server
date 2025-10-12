/**
 * Tool handler for deleting a passage from an agent's archival memory
 */
export async function handleDeletePassage(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }
    if (!args?.memory_id) {
        server.createErrorResponse('Missing required argument: memory_id');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);
        const memoryId = encodeURIComponent(args.memory_id);

        // Use the specific endpoint from the OpenAPI spec
        await server.api.delete(`/agents/${agentId}/archival-memory/${memoryId}`, { headers });

        // Successful deletion usually returns 200 or 204 with no body
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        memory_id: args.memory_id,
                        agent_id: args.agent_id,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle potential 404 if agent or passage not found, or other API errors
        if (error.response && error.response.status === 404) {
            server.createErrorResponse(
                `Agent or Passage not found: agent_id=${args.agent_id}, memory_id=${args.memory_id}`,
            );
        }
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for delete_passage
 */
export const deletePassageDefinition = {
    name: 'delete_passage',
    description:
        "Delete a memory from an agent's archival memory store. Use list_passages to find memory IDs. WARNING: This action is permanent.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose passage to delete',
            },
            memory_id: {
                type: 'string',
                description: 'ID of the passage (memory) to delete',
            },
        },
        required: ['agent_id', 'memory_id'],
        additionalProperties: false,
    },
};
