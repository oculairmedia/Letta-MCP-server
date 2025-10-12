/**
 * Schemas for letta_file_folder_ops tool
 * Provides discriminator-based schemas for file and folder management
 */

/**
 * Input schema for letta_file_folder_ops tool
 */
export const fileOpsInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                'list_files',
                'open_file',
                'close_file',
                'close_all_files',
                'list_folders',
                'attach_folder',
                'detach_folder',
                'list_agents_in_folder',
            ],
            description: 'File/folder operation to perform',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for agent-specific operations)',
        },
        file_id: {
            type: 'string',
            description: 'File ID (required for open_file, close_file)',
        },
        folder_id: {
            type: 'string',
            description: 'Folder ID (required for attach/detach/list_agents_in_folder)',
        },
    },
    required: ['operation'],
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
        is_open: { type: 'boolean' },
        opened_at: { type: 'string' },
    },
    required: ['id', 'filename'],
    additionalProperties: false,
};

/**
 * Folder metadata schema
 */
export const FolderMetadataSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
        path: { type: 'string' },
        file_count: { type: 'integer' },
        agent_count: { type: 'integer' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};

/**
 * Agent reference schema
 */
export const AgentReferenceSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};

/**
 * Output schema for letta_file_folder_ops tool
 */
export const fileOpsOutputSchema = {
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
        file_id: {
            type: 'string',
            description: 'File ID',
        },
        folder_id: {
            type: 'string',
            description: 'Folder ID',
        },
        files: {
            type: 'array',
            items: FileMetadataSchema,
            description: 'List of files',
        },
        folders: {
            type: 'array',
            items: FolderMetadataSchema,
            description: 'List of folders',
        },
        agents: {
            type: 'array',
            items: AgentReferenceSchema,
            description: 'List of agents in folder',
        },
        opened: {
            type: 'boolean',
            description: 'Whether file was opened',
        },
        closed: {
            type: 'boolean',
            description: 'Whether file was closed',
        },
        closed_count: {
            type: 'integer',
            description: 'Number of files closed (for close_all_files)',
        },
        attached: {
            type: 'boolean',
            description: 'Whether folder was attached',
        },
        detached: {
            type: 'boolean',
            description: 'Whether folder was detached',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
