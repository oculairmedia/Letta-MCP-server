/**
 * Schemas for letta_agent_advanced tool
 * Provides discriminator-based schemas for advanced agent operations
 */

/**
 * Message data schema for sending messages
 */
export const MessageDataSchema = {
    type: 'object',
    properties: {
        messages: {
            type: 'array',
            items: {
                type: 'object',
                properties: {
                    role: {
                        type: 'string',
                        enum: ['user', 'assistant', 'system'],
                    },
                    content: {
                        type: 'string',
                    },
                },
                required: ['role', 'content'],
                additionalProperties: false,
            },
        },
        stream: {
            type: 'boolean',
            description: 'Whether to stream the response',
        },
    },
    additionalProperties: false,
};

/**
 * Search filters schema
 */
export const SearchFiltersSchema = {
    type: 'object',
    properties: {
        start_date: {
            type: 'string',
            format: 'date-time',
            description: 'Start date for search range',
        },
        end_date: {
            type: 'string',
            format: 'date-time',
            description: 'End date for search range',
        },
        role: {
            type: 'string',
            enum: ['user', 'assistant', 'system'],
            description: 'Filter by message role',
        },
        agent_id: {
            type: 'string',
            description: 'Filter by agent ID',
        },
    },
    additionalProperties: false,
};

/**
 * Agent data schema for create/update operations
 */
export const AgentDataSchema = {
    type: 'object',
    properties: {
        name: {
            type: 'string',
            description: 'Agent name',
        },
        description: {
            type: 'string',
            description: 'Agent description',
        },
        system: {
            type: 'string',
            description: 'System prompt',
        },
        llm_config: {
            type: 'object',
            additionalProperties: true,
            description: 'LLM configuration',
        },
        embedding_config: {
            type: 'object',
            additionalProperties: true,
            description: 'Embedding configuration',
        },
        tool_ids: {
            type: 'array',
            items: { type: 'string' },
            description: 'Tool IDs to attach',
        },
    },
    additionalProperties: true,
};

/**
 * Bulk delete filters schema
 */
export const BulkDeleteFiltersSchema = {
    type: 'object',
    properties: {
        agent_name_filter: {
            type: 'string',
            description: 'Filter agents by name',
        },
        agent_tag_filter: {
            type: 'string',
            description: 'Filter agents by tag(s)',
        },
        agent_ids: {
            type: 'array',
            items: { type: 'string' },
            description: 'Specific agent IDs to delete',
        },
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_agent_advanced tool
 */
export const agentAdvancedInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                // CRUD operations
                'list',
                'create',
                'get',
                'update',
                'delete',
                'list_tools',
                'send_message',
                'export',
                'import',
                'clone',
                'get_config',
                'bulk_delete',
                // Advanced operations
                'context',
                'reset_messages',
                'summarize',
                'stream',
                'async_message',
                'cancel_message',
                'preview_payload',
                'search_messages',
                'get_message',
                'count',
            ],
            description: 'Agent operation to perform',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for most operations)',
        },
        agent_data: {
            ...AgentDataSchema,
            description: 'Agent data (for create/update operations)',
        },
        message_id: {
            type: 'string',
            description: 'Message ID (required for get_message operation)',
        },
        message_data: {
            ...MessageDataSchema,
            description: 'Message data (for stream, async_message, send_message operations)',
        },
        search_query: {
            type: 'string',
            description: 'Search query text (for search_messages)',
        },
        filters: {
            ...SearchFiltersSchema,
            description: 'Search filters (for search_messages, count)',
        },
        bulk_delete_filters: {
            ...BulkDeleteFiltersSchema,
            description: 'Filters for bulk delete operation',
        },
        file_path: {
            type: 'string',
            description: 'File path for export/import operations',
        },
        new_agent_name: {
            type: 'string',
            description: 'New agent name for clone operation',
        },
        pagination: {
            type: 'object',
            properties: {
                limit: { type: 'integer', minimum: 1, maximum: 100 },
                offset: { type: 'integer', minimum: 0 },
            },
            additionalProperties: false,
            description: 'Pagination options for list operation',
        },
    },
    required: ['operation'],
    additionalProperties: false,
};

/**
 * Context window schema
 */
export const ContextSchema = {
    type: 'object',
    properties: {
        messages: {
            type: 'array',
            items: {
                type: 'object',
                properties: {
                    role: { type: 'string' },
                    content: { type: 'string' },
                    timestamp: { type: 'string' },
                },
                additionalProperties: false,
            },
        },
        token_count: {
            type: 'integer',
            description: 'Total tokens in context',
        },
        context_window_size: {
            type: 'integer',
            description: 'Maximum context window size',
        },
    },
    additionalProperties: false,
};

/**
 * Message summary schema
 */
export const MessageSummarySchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        role: { type: 'string' },
        content: { type: 'string' },
        timestamp: { type: 'string' },
        agent_id: { type: 'string' },
    },
    required: ['id', 'role'],
    additionalProperties: false,
};

/**
 * Output schema for letta_agent_advanced tool
 */
export const agentAdvancedOutputSchema = {
    type: 'object',
    properties: {
        success: {
            type: 'boolean',
            description: 'Whether the operation succeeded',
        },
        operation: {
            type: 'string',
            description: 'Operation that was performed',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID',
        },
        context: {
            ...ContextSchema,
            description: 'Agent context window (for context operation)',
        },
        summary: {
            type: 'string',
            description: 'Agent conversation summary (for summarize operation)',
        },
        messages: {
            type: 'array',
            items: MessageSummarySchema,
            description: 'Messages (for search_messages operation)',
        },
        message: {
            type: 'object',
            properties: {
                id: { type: 'string' },
                role: { type: 'string' },
                content: { type: 'string' },
                timestamp: { type: 'string' },
                tool_calls: {
                    type: 'array',
                    items: {
                        type: 'object',
                        properties: {
                            tool_name: { type: 'string' },
                            arguments: { type: 'object', additionalProperties: true },
                        },
                        additionalProperties: false,
                    },
                },
            },
            additionalProperties: false,
            description: 'Message details (for get_message operation)',
        },
        stream_url: {
            type: 'string',
            format: 'uri',
            description: 'SSE stream URL (for stream operation)',
        },
        async_job_id: {
            type: 'string',
            description: 'Job ID for async operation',
        },
        cancelled: {
            type: 'boolean',
            description: 'Whether message was cancelled',
        },
        raw_payload: {
            type: 'object',
            additionalProperties: true,
            description: 'Raw API payload preview',
        },
        count: {
            type: 'integer',
            description: 'Count result (for count operation)',
        },
        reset_count: {
            type: 'integer',
            description: 'Number of messages reset',
        },
        status_message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
