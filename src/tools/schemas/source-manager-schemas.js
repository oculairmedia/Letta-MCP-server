/**
 * Schemas for letta_source_manager tool
 * Provides discriminator-based schemas for data source management
 */

/**
 * Source data schema for create/update operations
 */
export const SourceDataSchema = {
    type: 'object',
    properties: {
        name: {
            type: 'string',
            description: 'Source name',
        },
        description: {
            type: 'string',
            description: 'Source description',
        },
        embedding_config: {
            type: 'object',
            properties: {
                embedding_model: { type: 'string' },
                embedding_dim: { type: 'integer' },
            },
            additionalProperties: false,
        },
    },
    additionalProperties: true,
};

/**
 * File data schema for file upload
 */
export const FileDataSchema = {
    type: 'object',
    properties: {
        file: {
            type: 'string',
            description: 'File content (base64 encoded or file path)',
        },
        filename: {
            type: 'string',
            description: 'Filename',
        },
        mime_type: {
            type: 'string',
            description: 'MIME type of the file',
        },
    },
    required: ['file', 'filename'],
    additionalProperties: false,
};

/**
 * Pagination options
 */
export const SourcePaginationSchema = {
    type: 'object',
    properties: {
        limit: {
            type: 'integer',
            minimum: 1,
            maximum: 100,
        },
        offset: {
            type: 'integer',
            minimum: 0,
        },
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_source_manager tool
 */
export const sourceManagerInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                'list',
                'create',
                'get',
                'update',
                'delete',
                'count',
                'get_by_name',
                'upload_file',
                'list_files',
                'delete_file',
                'list_passages',
                'get_metadata',
                'attach_to_agent',
                'detach_from_agent',
                'list_agent_sources',
            ],
            description: 'Source management operation to perform',
        },
        source_id: {
            type: 'string',
            description: 'Source ID (required for get, update, delete, upload_file, list_files, list_passages)',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for attach/detach/list operations)',
        },
        file_id: {
            type: 'string',
            description: 'File ID (required for delete_file)',
        },
        source_name: {
            type: 'string',
            description: 'Source name (required for get_by_name)',
        },
        source_data: {
            ...SourceDataSchema,
            description: 'Source data (for create, update)',
        },
        file_data: {
            ...FileDataSchema,
            description: 'File data (for upload_file)',
        },
        options: {
            ...SourcePaginationSchema,
            description: 'Pagination options for list operations',
        },
    },
    required: ['operation'],
    additionalProperties: false,
};

/**
 * Source summary schema
 */
export const SourceSummarySchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
        description: { type: 'string' },
        created_at: { type: 'string' },
        num_passages: { type: 'integer' },
        num_files: { type: 'integer' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};

/**
 * File metadata schema
 */
export const FileMetadataSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        filename: { type: 'string' },
        size: { type: 'integer' },
        mime_type: { type: 'string' },
        uploaded_at: { type: 'string' },
    },
    required: ['id', 'filename'],
    additionalProperties: false,
};

/**
 * Passage schema
 */
export const PassageSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        text: { type: 'string' },
        doc_id: { type: 'string' },
        metadata: { type: 'object', additionalProperties: true },
    },
    required: ['id', 'text'],
    additionalProperties: false,
};

/**
 * Output schema for letta_source_manager tool
 */
export const sourceManagerOutputSchema = {
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
        source_id: {
            type: 'string',
            description: 'Source ID',
        },
        source: {
            type: 'object',
            properties: {
                id: { type: 'string' },
                name: { type: 'string' },
                description: { type: 'string' },
                created_at: { type: 'string' },
                embedding_config: { type: 'object', additionalProperties: true },
            },
            additionalProperties: false,
            description: 'Source details',
        },
        sources: {
            type: 'array',
            items: SourceSummarySchema,
            description: 'List of sources',
        },
        files: {
            type: 'array',
            items: FileMetadataSchema,
            description: 'List of files',
        },
        passages: {
            type: 'array',
            items: PassageSchema,
            description: 'List of passages',
        },
        file_id: {
            type: 'string',
            description: 'Uploaded file ID',
        },
        count: {
            type: 'integer',
            description: 'Count result',
        },
        metadata: {
            type: 'object',
            additionalProperties: true,
            description: 'Source metadata',
        },
        attached: {
            type: 'boolean',
            description: 'Whether source was attached to agent',
        },
        detached: {
            type: 'boolean',
            description: 'Whether source was detached from agent',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
