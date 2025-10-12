/**
 * Schemas for letta_tool_manager tool
 * Provides discriminator-based schemas for tool lifecycle management
 */

/**
 * Tool data schema for create/update operations
 */
export const ToolDataSchema = {
    type: 'object',
    properties: {
        name: {
            type: 'string',
            description: 'Tool name',
        },
        description: {
            type: 'string',
            description: 'Tool description',
        },
        source_code: {
            type: 'string',
            description: 'Python source code for the tool',
        },
        tags: {
            type: 'array',
            items: { type: 'string' },
            description: 'Tags for categorizing the tool',
        },
        source_type: {
            type: 'string',
            enum: ['python', 'javascript', 'mcp'],
            description: 'Source code type',
        },
    },
    additionalProperties: true,
};

/**
 * Pagination options schema
 */
export const PaginationSchema = {
    type: 'object',
    properties: {
        limit: {
            type: 'integer',
            minimum: 1,
            maximum: 100,
            description: 'Maximum number of results',
        },
        offset: {
            type: 'integer',
            minimum: 0,
            description: 'Number of results to skip',
        },
        after: {
            type: 'string',
            description: 'Cursor for pagination',
        },
    },
    additionalProperties: false,
};

/**
 * Filter options schema
 */
export const FilterSchema = {
    type: 'object',
    properties: {
        name: {
            type: 'string',
            description: 'Filter by tool name (partial match)',
        },
        tags: {
            type: 'array',
            items: { type: 'string' },
            description: 'Filter by tags',
        },
        source_type: {
            type: 'string',
            enum: ['python', 'javascript', 'mcp'],
            description: 'Filter by source type',
        },
    },
    additionalProperties: false,
};

/**
 * Bulk attach filters schema
 */
export const BulkAttachFiltersSchema = {
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
            description: 'Specific agent IDs to attach to',
        },
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_tool_manager tool
 */
export const toolManagerInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                'list',
                'get',
                'create',
                'update',
                'delete',
                'upsert',
                'attach',
                'detach',
                'bulk_attach',
                'generate_from_prompt',
                'generate_schema',
                'run_from_source',
                'add_base_tools',
            ],
            description: 'Tool management operation to perform',
        },
        tool_id: {
            type: 'string',
            description: 'Tool ID (required for get, update, delete, attach operations)',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for attach, detach operations)',
        },
        tool_data: {
            ...ToolDataSchema,
            description: 'Tool data (required for create, update, upsert operations)',
        },
        source_code: {
            type: 'string',
            description: 'Python source code (required for generate_schema, run_from_source)',
        },
        prompt: {
            type: 'string',
            description: 'Natural language prompt (required for generate_from_prompt)',
        },
        tool_args: {
            type: 'object',
            additionalProperties: true,
            description: 'Arguments for tool execution (for run_from_source)',
        },
        bulk_attach_filters: {
            ...BulkAttachFiltersSchema,
            description: 'Filters for bulk_attach operation',
        },
        options: {
            type: 'object',
            properties: {
                pagination: {
                    ...PaginationSchema,
                    description: 'Pagination options for list operation',
                },
                filters: {
                    ...FilterSchema,
                    description: 'Filter options for list operation',
                },
            },
            additionalProperties: false,
        },
    },
    required: ['operation'],
    additionalProperties: false,
};

/**
 * Tool summary schema
 */
export const ToolSummarySchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
        description: { type: 'string' },
        tags: {
            type: 'array',
            items: { type: 'string' },
        },
        source_type: { type: 'string' },
        created_at: { type: 'string' },
        updated_at: { type: 'string' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};

/**
 * Output schema for letta_tool_manager tool
 */
export const toolManagerOutputSchema = {
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
        tool_id: {
            type: 'string',
            description: 'Tool ID',
        },
        tool: {
            type: 'object',
            properties: {
                id: { type: 'string' },
                name: { type: 'string' },
                description: { type: 'string' },
                source_code: { type: 'string' },
                tags: {
                    type: 'array',
                    items: { type: 'string' },
                },
                json_schema: { type: 'object', additionalProperties: true },
            },
            additionalProperties: false,
            description: 'Tool details (for get operation)',
        },
        tools: {
            type: 'array',
            items: ToolSummarySchema,
            description: 'List of tools (for list operation)',
        },
        pagination: {
            type: 'object',
            properties: {
                total: { type: 'integer' },
                limit: { type: 'integer' },
                offset: { type: 'integer' },
                has_more: { type: 'boolean' },
            },
            additionalProperties: false,
            description: 'Pagination info for list operation',
        },
        generated_code: {
            type: 'string',
            description: 'Generated source code (for generate_from_prompt)',
        },
        generated_schema: {
            type: 'object',
            additionalProperties: true,
            description: 'Generated JSON schema (for generate_schema)',
        },
        execution_result: {
            type: 'object',
            additionalProperties: true,
            description: 'Result from tool execution (for run_from_source)',
        },
        detached_from_agent: {
            type: 'string',
            description: 'Agent ID tool was detached from',
        },
        added_tools_count: {
            type: 'integer',
            description: 'Number of base tools added',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
