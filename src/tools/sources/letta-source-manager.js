/**
 * Tool handler for letta_source_manager - Data Source Management Hub
 * Provides unified interface for complete data source lifecycle
 */
import { createLogger } from '../../core/logger.js';
import { sourceManagerInputSchema } from '../schemas/source-manager-schemas.js';

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

async function handleList(server, args) {
    const { options = {} } = args;
    const queryParams = new URLSearchParams();
    if (options.limit) queryParams.append('limit', options.limit);
    if (options.offset) queryParams.append('offset', options.offset);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/sources/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Listing sources'
    );

    const sources = Array.isArray(result) ? result : result.sources || [];
    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list',
                sources: sources.map(s => ({
                    id: s.id,
                    name: s.name,
                    description: s.description,
                    created_at: s.created_at,
                    num_passages: s.num_passages || 0,
                    num_files: s.num_files || 0,
                })),
                message: `Found ${sources.length} sources`,
            }),
        }],
    };
}

async function handleCreate(server, args) {
    const { source_data } = args;
    if (!source_data) throw new Error('source_data is required for create operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post('/sources/', source_data, { headers });
            return response.data;
        },
        'Creating source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'create',
                source_id: result.id,
                source: result,
                message: 'Source created successfully',
            }),
        }],
    };
}

async function handleGet(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for get operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/sources/${encodeURIComponent(source_id)}`, { headers });
            return response.data;
        },
        'Getting source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'get',
                source_id,
                source: result,
                message: 'Source retrieved successfully',
            }),
        }],
    };
}

async function handleUpdate(server, args) {
    const { source_id, source_data } = args;
    if (!source_id) throw new Error('source_id is required for update operation');
    if (!source_data) throw new Error('source_data is required for update operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.put(`/sources/${encodeURIComponent(source_id)}`, source_data, { headers });
            return response.data;
        },
        'Updating source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'update',
                source_id,
                source: result,
                message: 'Source updated successfully',
            }),
        }],
    };
}

async function handleDelete(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for delete operation');

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(`/sources/${encodeURIComponent(source_id)}`, { headers });
            return response.data;
        },
        'Deleting source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'delete',
                source_id,
                message: 'Source deleted successfully',
            }),
        }],
    };
}

async function handleCount(server, _args) {
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get('/sources/count', { headers });
            return response.data;
        },
        'Counting sources'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'count',
                count: result.count || result.total || 0,
                message: 'Sources counted successfully',
            }),
        }],
    };
}

async function handleGetByName(server, args) {
    const { source_name } = args;
    if (!source_name) throw new Error('source_name is required for get_by_name operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/sources/name/${encodeURIComponent(source_name)}`, { headers });
            return response.data;
        },
        'Getting source by name'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'get_by_name',
                source_name,
                source: result,
                message: 'Source retrieved successfully',
            }),
        }],
    };
}

async function handleUploadFile(server, args) {
    const { source_id, file_data } = args;
    if (!source_id) throw new Error('source_id is required for upload_file operation');
    if (!file_data) throw new Error('file_data is required for upload_file operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(`/sources/${encodeURIComponent(source_id)}/files`, file_data, { headers });
            return response.data;
        },
        'Uploading file to source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'upload_file',
                source_id,
                file_id: result.id || result.file_id,
                message: 'File uploaded successfully',
            }),
        }],
    };
}

async function handleListFiles(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for list_files operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/sources/${encodeURIComponent(source_id)}/files`, { headers });
            return response.data;
        },
        'Listing source files'
    );

    const files = Array.isArray(result) ? result : result.files || [];
    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_files',
                source_id,
                files: files.map(f => ({
                    id: f.id,
                    filename: f.filename,
                    size: f.size,
                    mime_type: f.mime_type,
                    uploaded_at: f.uploaded_at,
                })),
                message: `Found ${files.length} files`,
            }),
        }],
    };
}

async function handleDeleteFile(server, args) {
    const { source_id, file_id } = args;
    if (!source_id) throw new Error('source_id is required for delete_file operation');
    if (!file_id) throw new Error('file_id is required for delete_file operation');

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(`/sources/${encodeURIComponent(source_id)}/files/${encodeURIComponent(file_id)}`, { headers });
            return response.data;
        },
        'Deleting file from source'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'delete_file',
                source_id,
                file_id,
                message: 'File deleted successfully',
            }),
        }],
    };
}

async function handleListPassages(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for list_passages operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/sources/${encodeURIComponent(source_id)}/passages`, { headers });
            return response.data;
        },
        'Listing source passages'
    );

    const passages = Array.isArray(result) ? result : result.passages || [];
    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_passages',
                source_id,
                passages: passages.map(p => ({
                    id: p.id,
                    text: p.text,
                    doc_id: p.doc_id,
                    metadata: p.metadata,
                })),
                message: `Found ${passages.length} passages`,
            }),
        }],
    };
}

async function handleGetMetadata(server, args) {
    const { source_id } = args;
    if (!source_id) throw new Error('source_id is required for get_metadata operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/sources/${encodeURIComponent(source_id)}/metadata`, { headers });
            return response.data;
        },
        'Getting source metadata'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'get_metadata',
                source_id,
                metadata: result,
                message: 'Metadata retrieved successfully',
            }),
        }],
    };
}

async function handleAttachToAgent(server, args) {
    const { agent_id, source_id } = args;
    if (!agent_id) throw new Error('agent_id is required for attach_to_agent operation');
    if (!source_id) throw new Error('source_id is required for attach_to_agent operation');

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(`/agents/${encodeURIComponent(agent_id)}/sources/${encodeURIComponent(source_id)}`, {}, { headers });
            return response.data;
        },
        'Attaching source to agent'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'attach_to_agent',
                agent_id,
                source_id,
                attached: true,
                message: 'Source attached to agent successfully',
            }),
        }],
    };
}

async function handleDetachFromAgent(server, args) {
    const { agent_id, source_id } = args;
    if (!agent_id) throw new Error('agent_id is required for detach_from_agent operation');
    if (!source_id) throw new Error('source_id is required for detach_from_agent operation');

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(`/agents/${encodeURIComponent(agent_id)}/sources/${encodeURIComponent(source_id)}`, { headers });
            return response.data;
        },
        'Detaching source from agent'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'detach_from_agent',
                agent_id,
                source_id,
                detached: true,
                message: 'Source detached from agent successfully',
            }),
        }],
    };
}

async function handleListAgentSources(server, args) {
    const { agent_id } = args;
    if (!agent_id) throw new Error('agent_id is required for list_agent_sources operation');

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}/sources`, { headers });
            return response.data;
        },
        'Listing agent sources'
    );

    const sources = Array.isArray(result) ? result : result.sources || [];
    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_agent_sources',
                agent_id,
                sources: sources.map(s => ({
                    id: s.id,
                    name: s.name,
                    description: s.description,
                    created_at: s.created_at,
                    num_passages: s.num_passages || 0,
                    num_files: s.num_files || 0,
                })),
                message: `Found ${sources.length} sources for agent`,
            }),
        }],
    };
}

export const lettaSourceManagerDefinition = {
    name: 'letta_source_manager',
    description: 'Data Source Management Hub - Comprehensive tool for managing data sources with 15 operations: list, create, get, update, delete, count, get_by_name, upload_file, list_files, delete_file, list_passages, get_metadata, attach_to_agent, detach_from_agent, and list_agent_sources. Provides complete source lifecycle and file management with discriminator-based operation routing.',
    inputSchema: sourceManagerInputSchema,
};
