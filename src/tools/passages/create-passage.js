/**
 * Tool handler for creating a passage in an agent's archival memory
 */
export async function handleCreatePassage(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }
    if (args?.text === undefined || args?.text === null) {
        server.createErrorResponse('Missing required argument: text');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);
        const payload = { text: args.text }; // Body requires 'text'

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.post(`/agents/${agentId}/archival-memory`, payload, {
            headers,
        });
        let createdPassages = response.data; // Assuming response.data is an array of created Passage objects

        // Optionally remove embeddings from the response
        const includeEmbeddings = args?.include_embeddings ?? false;
        if (!includeEmbeddings) {
            createdPassages = createdPassages.map((passage) => {
                // eslint-disable-next-line no-unused-vars
                const { embedding, ...rest } = passage; // Destructure to remove embedding
                return rest;
            });
        }

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        passages: createdPassages,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle potential 404 if agent not found, 422 for validation, or other API errors
        if (error.response) {
            if (error.response.status === 404) {
                server.createErrorResponse(`Agent not found: ${args.agent_id}`);
            }
            if (error.response.status === 422) {
                server.createErrorResponse(
                    `Validation error creating passage for agent ${args.agent_id}: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for create_passage
 */
export const createPassageDefinition = {
    name: 'create_passage',
    description:
        "Insert a memory into an agent's archival memory store. Use list_passages to view existing memories, modify_passage to edit, or delete_passage to remove.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent to add the passage to',
            },
            text: {
                type: 'string',
                description: 'Text content to write to archival memory.',
            },
            include_embeddings: {
                type: 'boolean',
                description:
                    'Whether to include the full embedding vectors in the response (default: false).',
                default: false,
            },
        },
        required: ['agent_id', 'text'],
        additionalProperties: false,
    },
};
