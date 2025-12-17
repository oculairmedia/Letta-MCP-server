/**
 * Tool handler for creating a conversation entry in archival memory
 *
 * Since Letta doesn't have a direct API to add messages to history without
 * triggering agent processing, this stores conversation entries as archival
 * passages with consistent formatting and tags for searchability.
 */
export async function handleCreateConversationEntry(server, args) {
    if (!args?.agent_id) {
        throw new Error('Missing required argument: agent_id');
    }
    if (!args?.role) {
        throw new Error('Missing required argument: role');
    }
    if (!args?.content) {
        throw new Error('Missing required argument: content');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);

        // Format the conversation entry to match Letta's message structure
        const timestamp = args.timestamp || new Date().toISOString();
        const source = args.source || 'unknown';
        const sessionId = args.session_id || 'no-session';

        // Map role to Letta message_type
        const messageTypeMap = {
            user: 'user_message',
            assistant: 'assistant_message',
            system: 'system_message',
        };

        // Create JSON structure matching Letta message format
        const messageEntry = {
            message_type: messageTypeMap[args.role] || 'user_message',
            date: timestamp,
            role: args.role,
            text: args.content,
            source: source,
            session_id: sessionId,
        };

        // Store as JSON for consistent parsing and search
        const entryText = JSON.stringify(messageEntry);

        // Create the passage
        const response = await server.api.post(
            `/agents/${agentId}/archival-memory`,
            { text: entryText },
            { headers },
        );

        // Strip embeddings from response
        const passage = response.data;
        if (passage.embedding) {
            delete passage.embedding;
        }

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        success: true,
                        passage_id: passage.id,
                        timestamp: timestamp,
                        role: args.role,
                        source: source,
                    }),
                },
            ],
        };
    } catch (error) {
        return server.createErrorResponse(error);
    }
}

/**
 * Tool definition for create_conversation_entry
 */
export const createConversationEntryDefinition = {
    name: 'create_conversation_entry',
    description:
        "Store a conversation entry in an agent's archival memory. Use this to record conversations from external sources (like Claude Code sessions) that should be searchable via search_archival_memory. Entries are formatted with metadata for easy filtering.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent to store the conversation entry for',
            },
            role: {
                type: 'string',
                enum: ['user', 'assistant', 'system'],
                description: 'Role of the message sender',
            },
            content: {
                type: 'string',
                description: 'The message content',
            },
            timestamp: {
                type: 'string',
                format: 'date-time',
                description: 'ISO 8601 timestamp of the message (defaults to now)',
            },
            source: {
                type: 'string',
                description: 'Source of the conversation (e.g., "claude_code", "letta_ade")',
            },
            session_id: {
                type: 'string',
                description: 'Session identifier for grouping related messages',
            },
        },
        required: ['agent_id', 'role', 'content'],
    },
};
