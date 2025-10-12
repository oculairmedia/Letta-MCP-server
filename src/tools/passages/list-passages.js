/**
 * Tool handler for listing passages in an agent's archival memory
 */
export async function handleListPassages(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);

        // Construct query parameters based on optional args
        const params = {};
        if (args.after) params.after = args.after;
        if (args.before) params.before = args.before;
        if (args.limit) params.limit = args.limit;
        if (args.search) params.search = args.search;
        if (args.ascending !== undefined) params.ascending = args.ascending; // Handle boolean false

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.get(`/agents/${agentId}/archival-memory`, {
            headers,
            params,
        });
        let passages = response.data; // Assuming response.data is an array of Passage objects

        // Optionally remove embeddings from the response
        const includeEmbeddings = args?.include_embeddings ?? false;
        if (!includeEmbeddings) {
            passages = passages.map((passage) => {
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
                        passages: passages,
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
 * Tool definition for list_passages
 */
export const listPassagesDefinition = {
    name: 'list_passages',
    description:
        "Retrieve the memories in an agent's archival memory store (paginated query). Use create_passage to add new memories, modify_passage to edit, or delete_passage to remove them.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose passages to list',
            },
            after: {
                type: 'string',
                description:
                    'Unique ID of the memory to start the query range at (for pagination).',
            },
            before: {
                type: 'string',
                description: 'Unique ID of the memory to end the query range at (for pagination).',
            },
            limit: {
                type: 'integer',
                description: 'How many results to include in the response.',
            },
            search: {
                type: 'string',
                description: 'Search passages by text content.',
            },
            ascending: {
                type: 'boolean',
                description:
                    'Whether to sort passages oldest to newest (True, default) or newest to oldest (False).',
                default: true,
            },
            include_embeddings: {
                type: 'boolean',
                description:
                    'Whether to include the full embedding vectors in the response (default: false).',
                default: false,
            },
        },
        required: ['agent_id'],
        additionalProperties: false,
    },
};
