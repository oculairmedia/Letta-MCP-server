/**
 * Tool handler for listing messages from an agent's conversation history
 */
export async function handleListMessages(server, args) {
    if (!args?.agent_id) {
        throw new Error('Missing required argument: agent_id');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);

        // Construct query parameters
        const params = {};
        if (args.limit) params.limit = args.limit;
        if (args.order) params.order = args.order;
        if (args.before) params.before = args.before;
        if (args.after) params.after = args.after;
        if (args.group_id) params.group_id = args.group_id;

        const response = await server.api.get(`/agents/${agentId}/messages`, {
            headers,
            params,
        });

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        messages: response.data,
                        count: response.data.length,
                    }),
                },
            ],
        };
    } catch (error) {
        return server.createErrorResponse(error);
    }
}

/**
 * Tool definition for list_messages
 */
export const listMessagesDefinition = {
    name: 'list_messages',
    description:
        "Retrieve messages from an agent's conversation history. Returns paginated message history including user messages, assistant responses, tool calls, and system messages. Use for reviewing past conversations or debugging agent behavior.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose messages to retrieve',
            },
            limit: {
                type: 'integer',
                description: 'Maximum number of messages to return',
            },
            order: {
                type: 'string',
                enum: ['asc', 'desc'],
                description: 'Sort order: "asc" for oldest first, "desc" for newest first',
            },
            before: {
                type: 'string',
                description: 'Pagination cursor - get messages before this message ID',
            },
            after: {
                type: 'string',
                description: 'Pagination cursor - get messages after this message ID',
            },
            group_id: {
                type: 'string',
                description: 'Filter messages by group ID',
            },
        },
        required: ['agent_id'],
    },
};
