/**
 * Tool handler for deleting a memory block in the Letta system
 */
export async function handleDeleteMemoryBlock(server, args) {
    try {
        // Validate arguments
        if (!args?.block_id) {
            throw new Error('Missing required argument: block_id');
        }

        // Headers for API requests
        const headers = server.getApiHeaders();

        // If agent_id is provided, set the user_id header
        if (args.agent_id) {
            headers['user_id'] = args.agent_id;
        }

        // Delete the memory block
        await server.api.delete(`/blocks/${args.block_id}`, {
            headers,
        });

        // Return success response
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        success: true,
                        deleted_block_id: args.block_id,
                    }),
                },
            ],
        };
    } catch (error) {
        return server.createErrorResponse(error);
    }
}

/**
 * Tool definition for delete_memory_block
 */
export const deleteMemoryBlockToolDefinition = {
    name: 'delete_memory_block',
    description:
        'Delete a memory block by ID. Use list_memory_blocks to find block IDs. WARNING: This action is permanent and cannot be undone.',
    inputSchema: {
        type: 'object',
        properties: {
            block_id: {
                type: 'string',
                description: 'ID of the memory block to delete',
            },
            agent_id: {
                type: 'string',
                description: 'Optional agent ID for authorization',
            },
        },
        required: ['block_id'],
    },
};
