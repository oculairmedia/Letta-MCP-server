/**
 * Tool handler for searching agent archival memory using semantic similarity
 */
export async function handleSearchArchivalMemory(server, args) {
    if (!args?.agent_id) {
        throw new Error('Missing required argument: agent_id');
    }
    if (!args?.query) {
        throw new Error('Missing required argument: query');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);

        // Construct query parameters
        const params = {
            query: args.query,
        };
        if (args.tags) params.tags = args.tags;
        if (args.tag_match_mode) params.tag_match_mode = args.tag_match_mode;
        if (args.top_k) params.top_k = args.top_k;
        if (args.start_datetime) params.start_datetime = args.start_datetime;
        if (args.end_datetime) params.end_datetime = args.end_datetime;

        // Use the semantic search endpoint
        const response = await server.api.get(`/agents/${agentId}/archival-memory/search`, {
            headers,
            params,
        });
        // Search endpoint returns {results: [], count: N}
        let passages = response.data.results || [];

        // Optionally remove embeddings from the response
        const includeEmbeddings = args?.include_embeddings ?? false;
        if (!includeEmbeddings) {
            passages = passages.map((passage) => {
                // eslint-disable-next-line no-unused-vars
                const { embedding, ...rest } = passage;
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
        return server.createErrorResponse(error);
    }
}

/**
 * Tool definition for search_archival_memory
 */
export const searchArchivalMemoryDefinition = {
    name: 'search_archival_memory',
    description:
        "Search an agent's archival memory using semantic similarity. Returns passages most similar to the query. Use list_passages for text-based search or pagination, create_passage to add memories.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose archival memory to search',
            },
            query: {
                type: 'string',
                description: 'Search query for semantic similarity matching',
            },
            tags: {
                type: 'array',
                items: { type: 'string' },
                description: 'Optional list of tags to filter passages by',
            },
            tag_match_mode: {
                type: 'string',
                enum: ['any', 'all'],
                description:
                    'How to match tags: "any" returns passages with at least one matching tag, "all" requires all tags to match',
            },
            top_k: {
                type: 'integer',
                description: 'Maximum number of results to return (default varies by server)',
            },
            start_datetime: {
                type: 'string',
                format: 'date-time',
                description: 'Filter passages created after this datetime (ISO 8601 format)',
            },
            end_datetime: {
                type: 'string',
                format: 'date-time',
                description: 'Filter passages created before this datetime (ISO 8601 format)',
            },
            include_embeddings: {
                type: 'boolean',
                description:
                    'Whether to include the full embedding vectors in the response (default: false)',
                default: false,
            },
        },
        required: ['agent_id', 'query'],
    },
};
