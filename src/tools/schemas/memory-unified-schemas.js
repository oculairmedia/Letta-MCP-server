/**
 * Schemas for letta_memory_unified tool
 * Provides discriminator-based schemas for unified memory operations
 */

/**
 * Core memory data schema
 */
export const CoreMemoryDataSchema = {
    type: 'object',
    properties: {
        persona: {
            type: 'string',
            description: 'Persona memory block content',
        },
        human: {
            type: 'string',
            description: 'Human memory block content',
        },
    },
    additionalProperties: true,
};

/**
 * Search options schema for archival memory
 */
export const SearchOptionsSchema = {
    type: 'object',
    properties: {
        limit: {
            type: 'integer',
            minimum: 1,
            maximum: 100,
            description: 'Maximum number of results',
        },
        threshold: {
            type: 'number',
            minimum: 0,
            maximum: 1,
            description: 'Similarity threshold for search',
        },
    },
    additionalProperties: false,
};

/**
 * Block data schema for create/update block operations
 */
export const BlockDataSchema = {
    type: 'object',
    properties: {
        label: {
            type: 'string',
            description: 'Block label (e.g., "persona", "human", "system")',
        },
        value: {
            type: 'string',
            description: 'Block content/value',
        },
        limit: {
            type: 'integer',
            description: 'Character limit for the block',
        },
    },
    additionalProperties: true,
};

/**
 * Passage data schema for create/update passage operations
 */
export const PassageDataSchema = {
    type: 'object',
    properties: {
        text: {
            type: 'string',
            description: 'Passage text content',
        },
    },
    required: ['text'],
    additionalProperties: true,
};

/**
 * Pagination options for list operations
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
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_memory_unified tool
 */
export const memoryUnifiedInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                // Core memory operations
                'get_core_memory',
                'update_core_memory',
                // Block operations
                'get_block_by_label',
                'list_blocks',
                'create_block',
                'get_block',
                'update_block',
                'attach_block',
                'detach_block',
                'list_agents_using_block',
                // Archival/passage operations
                'search_archival',
                'list_passages',
                'create_passage',
                'update_passage',
                'delete_passage',
            ],
            description: 'Memory operation to perform',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for agent-specific operations)',
        },
        block_id: {
            type: 'string',
            description: 'Memory block ID (required for block operations)',
        },
        block_label: {
            type: 'string',
            description: 'Memory block label (required for get_block_by_label)',
        },
        block_data: {
            ...BlockDataSchema,
            description: 'Block data (for create_block, update_block)',
        },
        memory_data: {
            ...CoreMemoryDataSchema,
            description: 'Memory data (for update_core_memory)',
        },
        passage_id: {
            type: 'string',
            description: 'Passage ID (required for update_passage, delete_passage)',
        },
        passage_data: {
            ...PassageDataSchema,
            description: 'Passage data (for create_passage, update_passage)',
        },
        search_query: {
            type: 'string',
            description: 'Search query text (for search_archival)',
        },
        search_options: {
            ...SearchOptionsSchema,
            description: 'Search options (for search_archival)',
        },
        pagination: {
            ...PaginationSchema,
            description: 'Pagination options (for list operations)',
        },
        request_heartbeat: {
            type: 'boolean',
            description: 'Ignored parameter (for MCP client compatibility)',
        },
    },
    required: ['operation'],
    additionalProperties: true,
};

/**
 * Memory block schema
 */
export const MemoryBlockSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        label: { type: 'string' },
        value: { type: 'string' },
        limit: { type: 'integer' },
    },
    required: ['id', 'label', 'value'],
    additionalProperties: false,
};

/**
 * Archival memory result schema
 */
export const ArchivalResultSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        text: { type: 'string' },
        timestamp: { type: 'string' },
        similarity_score: { type: 'number' },
    },
    required: ['id', 'text'],
    additionalProperties: false,
};

/**
 * Output schema for letta_memory_unified tool
 */
export const memoryUnifiedOutputSchema = {
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
        core_memory: {
            type: 'object',
            properties: {
                persona: { type: 'string' },
                human: { type: 'string' },
            },
            additionalProperties: true,
            description: 'Core memory content (for get/update operations)',
        },
        block: {
            ...MemoryBlockSchema,
            description: 'Memory block (for get_block_by_label)',
        },
        blocks: {
            type: 'array',
            items: MemoryBlockSchema,
            description: 'List of memory blocks',
        },
        agents: {
            type: 'array',
            items: {
                type: 'object',
                properties: {
                    id: { type: 'string' },
                    name: { type: 'string' },
                },
                additionalProperties: false,
            },
            description: 'List of agents using the block',
        },
        search_results: {
            type: 'array',
            items: ArchivalResultSchema,
            description: 'Archival memory search results',
        },
        detached: {
            type: 'boolean',
            description: 'Whether block was detached',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
