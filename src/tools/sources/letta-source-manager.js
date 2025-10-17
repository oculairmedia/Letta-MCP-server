/**
 * Tool handler for letta_source_manager - Data Source Management Hub
 * Provides unified interface for complete data source lifecycle
 */
import fs from 'node:fs';
import { Blob } from 'node:buffer';
import { createLogger } from '../../core/logger.js';
import { sourceManagerInputSchema } from '../schemas/source-manager-schemas.js';

const FileCtor = typeof File === 'function' ? File : null;
const logger = createLogger('letta_source_manager');

export async function handleLettaSourceManager(server, args) {
    const { operation } = args;
    logger.info(`Executing source operation: ${operation}`, { args });

    try {
        const handlers = {
            list: handleList,
            create: handleCreate,
            get: handleGet,
            update: handleUpdate,
            delete: handleDelete,
            count: handleCount,
            get_by_name: handleGetByName,
            upload_file: handleUploadFile,
            list_files: handleListFiles,
            delete_file: handleDeleteFile,
            list_passages: handleListPassages,
            get_metadata: handleGetMetadata,
            attach_to_agent: handleAttachToAgent,
            detach_from_agent: handleDetachFromAgent,
            list_agent_sources: handleListAgentSources,
        };

        if (!handlers[operation]) {
            throw new Error(`Unknown operation: ${operation}`);
        }

        return await handlers[operation](server, args);
    } catch (error) {
        logger.error(`Source operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * List all sources
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleList(server, args) {
    const { options = {} } = args;

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.list() method
        return await server.client.sources.list();
    }, 'Listing sources');

    // SDK returns Source[] array
    const sources = Array.isArray(result) ? result : [];
    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list',
                    sources: sources.map((s) => ({
                        id: s.id,
                        name: s.name,
                        description: s.description,
                        created_at: s.created_at,
                        num_passages: s.num_passages || 0,
                        num_files: s.num_files || 0,
                    })),
                    message: `Found ${sources.length} sources`,
                }),
            },
        ],
    };
}

/**
 * Create a new source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCreate(server, args) {
    const { source_data } = args;
    if (!source_data) throw new Error('source_data is required for create operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.create() method
        return await server.client.sources.create(source_data);
    }, 'Creating source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create',
                    source_id: result.id,
                    source: result,
                    message: 'Source created successfully',
                }),
            },
        ],
    };
}

/**
 * Get source details by ID
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGet(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for get operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.retrieve() method
        return await server.client.sources.retrieve(source_id);
    }, 'Getting source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get',
                    source_id,
                    source: result,
                    message: 'Source retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleUpdate(server, args) {
    const { source_id, source_data } = args;
    if (!source_id) throw new Error('source_id is required for update operation');
    if (!source_data) throw new Error('source_data is required for update operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.modify() method
        return await server.client.sources.modify(source_id, source_data);
    }, 'Updating source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update',
                    source_id,
                    source: result,
                    message: 'Source updated successfully',
                }),
            },
        ],
    };
}

/**
 * Delete a source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDelete(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for delete operation');

    await server.handleSdkCall(async () => {
        // Use SDK client.sources.delete() method
        return await server.client.sources.delete(source_id);
    }, 'Deleting source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete',
                    source_id,
                    message: 'Source deleted successfully',
                }),
            },
        ],
    };
}

/**
 * Count all sources
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCount(server, _args) {
    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.count() method - returns number directly
        return await server.client.sources.count();
    }, 'Counting sources');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'count',
                    count: typeof result === 'number' ? result : result.count || 0,
                    message: 'Sources counted successfully',
                }),
            },
        ],
    };
}

/**
 * Get source by name
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetByName(server, args) {
    const { source_name } = args;
    if (!source_name) throw new Error('source_name is required for get_by_name operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.retrieveByName() method
        // SDK returns source ID as string, need to fetch full source
        const sourceId = await server.client.sources.retrieveByName(source_name);
        return await server.client.sources.retrieve(sourceId);
    }, 'Getting source by name');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_by_name',
                    source_name,
                    source: result,
                    message: 'Source retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Upload file to source
 * MIGRATED: Now using the Letta SDK with upload payload normalization
 */
async function handleUploadFile(server, args) {
    const { source_id, file_data } = args;
    if (!source_id) throw new Error('source_id is required for upload_file operation');
    if (!file_data) throw new Error('file_data is required for upload_file operation');

    const uploadPayload = normalizeUploadPayload(file_data);

    const result = await server.handleSdkCall(async () => {
        return await server.client.sources.files.upload(source_id, uploadPayload);
    }, 'Uploading file to source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'upload_file',
                    source_id,
                    file_id: result?.id || result?.file_id,
                    message: 'File uploaded successfully',
                }),
            },
        ],
    };
}

function normalizeUploadPayload(fileData) {
    if (fileData == null) {
        throw new Error('Unsupported file_data format for upload_file operation');
    }

    if (isBlobLike(fileData) || isReadableStream(fileData)) {
        return fileData;
    }

    if (typeof fileData === 'string') {
        if (fs.existsSync(fileData)) {
            return fs.createReadStream(fileData);
        }
        return new Blob([fileData], { type: 'text/plain' });
    }

    if (Buffer.isBuffer(fileData)) {
        return new Blob([fileData], { type: 'application/octet-stream' });
    }

    if (fileData instanceof ArrayBuffer) {
        return new Blob([Buffer.from(fileData)], { type: 'application/octet-stream' });
    }

    if (ArrayBuffer.isView(fileData)) {
        return new Blob([Buffer.from(fileData.buffer, fileData.byteOffset, fileData.byteLength)], {
            type: 'application/octet-stream',
        });
    }

    if (typeof fileData === 'object') {
        if (fileData.path && typeof fileData.path === 'string' && fs.existsSync(fileData.path)) {
            return fs.createReadStream(fileData.path);
        }

        if (fileData.file !== undefined) {
            const { file, filename = 'upload.bin', encoding, mime_type: mimeType } = fileData;

            const buffer = coerceToBuffer(file, encoding);
            if (buffer) {
                const type = mimeType || 'application/octet-stream';
                if (FileCtor) {
                    return new FileCtor([buffer], filename, { type });
                }
                return new Blob([buffer], { type });
            }
        }

        if (isBlobLike(fileData.blob)) {
            return fileData.blob;
        }
    }

    throw new Error('Unsupported file_data format for upload_file operation');
}

function isBlobLike(value) {
    return (
        Boolean(value) &&
        typeof value === 'object' &&
        typeof value.arrayBuffer === 'function' &&
        typeof value.stream === 'function'
    );
}

function isReadableStream(value) {
    return Boolean(value) && typeof value === 'object' && typeof value.pipe === 'function';
}

function coerceToBuffer(value, encoding) {
    if (Buffer.isBuffer(value)) {
        return value;
    }

    if (value instanceof ArrayBuffer) {
        return Buffer.from(value);
    }

    if (ArrayBuffer.isView(value)) {
        return Buffer.from(value.buffer, value.byteOffset, value.byteLength);
    }

    if (value && typeof value === 'object') {
        if (value.data !== undefined) {
            return coerceToBuffer(value.data, value.encoding || encoding);
        }
    }

    if (typeof value === 'string') {
        const normalizedEncoding = (encoding || 'base64').toLowerCase();

        if (normalizedEncoding === 'base64url') {
            const normalized = value.replace(/-/g, '+').replace(/_/g, '/');
            const paddingNeeded = normalized.length % 4;
            const padded = paddingNeeded
                ? normalized.padEnd(normalized.length + (4 - paddingNeeded), '=')
                : normalized;
            return Buffer.from(padded, 'base64');
        }

        if (normalizedEncoding === 'base64') {
            return Buffer.from(value, 'base64');
        }

        if (normalizedEncoding === 'hex' || normalizedEncoding === 'binary') {
            return Buffer.from(value, normalizedEncoding);
        }

        if (normalizedEncoding === 'utf8' || normalizedEncoding === 'utf-8') {
            return Buffer.from(value, 'utf8');
        }

        return Buffer.from(value);
    }

    return null;
}

/**
 * List files in a source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListFiles(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for list_files operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.files.list() method
        return await server.client.sources.files.list(source_id);
    }, 'Listing source files');

    // SDK returns FileMetadata[] array
    const files = Array.isArray(result) ? result : [];
    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_files',
                    source_id,
                    files: files.map((f) => ({
                        id: f.id,
                        filename: f.filename,
                        size: f.size,
                        mime_type: f.mime_type,
                        uploaded_at: f.uploaded_at,
                    })),
                    message: `Found ${files.length} files`,
                }),
            },
        ],
    };
}

/**
 * Delete file from source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDeleteFile(server, args) {
    const { source_id, file_id } = args;
    if (!source_id) throw new Error('source_id is required for delete_file operation');
    if (!file_id) throw new Error('file_id is required for delete_file operation');

    await server.handleSdkCall(async () => {
        // Use SDK client.sources.files.delete() method - returns void
        return await server.client.sources.files.delete(source_id, file_id);
    }, 'Deleting file from source');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete_file',
                    source_id,
                    file_id,
                    message: 'File deleted successfully',
                }),
            },
        ],
    };
}

/**
 * List passages in a source
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListPassages(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for list_passages operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.sources.passages.list() method
        return await server.client.sources.passages.list(source_id);
    }, 'Listing source passages');

    // SDK returns Passage[] array
    const passages = Array.isArray(result) ? result : [];
    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_passages',
                    source_id,
                    passages: passages.map((p) => ({
                        id: p.id,
                        text: p.text,
                        doc_id: p.doc_id,
                        metadata: p.metadata,
                    })),
                    message: `Found ${passages.length} passages`,
                }),
            },
        ],
    };
}

/**
 * Get source metadata
 * Note: SDK may not have direct /sources/{id}/metadata endpoint support
 * Keeping axios for now
 * TODO: Check if SDK adds getSourceMetadata() for single source
 */
async function handleGetMetadata(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for get_metadata operation');

    const result = await server.handleSdkCall(async () => {
        const headers = server.getApiHeaders();
        const response = await server.api.get(
            `/sources/${encodeURIComponent(source_id)}/metadata`,
            { headers },
        );
        return response.data;
    }, 'Getting source metadata');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_metadata',
                    source_id,
                    metadata: result,
                    message: 'Metadata retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Attach source to agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleAttachToAgent(server, args) {
    const { agent_id, source_id } = args;
    if (!agent_id) throw new Error('agent_id is required for attach_to_agent operation');
    if (!source_id) throw new Error('source_id is required for attach_to_agent operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.sources.attach() method - returns AgentState
        return await server.client.agents.sources.attach(agent_id, source_id);
    }, 'Attaching source to agent');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'attach_to_agent',
                    agent_id,
                    source_id,
                    attached: true,
                    agent_state: result, // SDK returns AgentState
                    message: 'Source attached to agent successfully',
                }),
            },
        ],
    };
}

/**
 * Detach source from agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDetachFromAgent(server, args) {
    const { agent_id, source_id } = args;
    if (!agent_id) throw new Error('agent_id is required for detach_from_agent operation');
    if (!source_id) throw new Error('source_id is required for detach_from_agent operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.sources.detach() method - returns AgentState
        return await server.client.agents.sources.detach(agent_id, source_id);
    }, 'Detaching source from agent');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'detach_from_agent',
                    agent_id,
                    source_id,
                    detached: true,
                    agent_state: result, // SDK returns AgentState
                    message: 'Source detached from agent successfully',
                }),
            },
        ],
    };
}

/**
 * List sources attached to an agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListAgentSources(server, args) {
    const { agent_id } = args;
    if (!agent_id) throw new Error('agent_id is required for list_agent_sources operation');

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.sources.list() method
        return await server.client.agents.sources.list(agent_id);
    }, 'Listing agent sources');

    // SDK returns Source[] array
    const sources = Array.isArray(result) ? result : [];
    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_agent_sources',
                    agent_id,
                    sources: sources.map((s) => ({
                        id: s.id,
                        name: s.name,
                        description: s.description,
                        created_at: s.created_at,
                        num_passages: s.num_passages || 0,
                        num_files: s.num_files || 0,
                    })),
                    message: `Found ${sources.length} sources for agent`,
                }),
            },
        ],
    };
}

export const lettaSourceManagerDefinition = {
    name: 'letta_source_manager',
    description:
        'Data Source Management Hub - Comprehensive tool for managing data sources with 15 operations: list, create, get, update, delete, count, get_by_name, upload_file, list_files, delete_file, list_passages, get_metadata, attach_to_agent, detach_from_agent, and list_agent_sources. Provides complete source lifecycle and file management with discriminator-based operation routing.',
    inputSchema: sourceManagerInputSchema,
};
