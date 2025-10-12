/**
 * Tool handler for letta_file_folder_ops - File and Folder Management Hub
 * Provides unified interface for file session management and folder operations
 */
import { createLogger } from '../../core/logger.js';
import { fileOpsInputSchema } from '../schemas/file-ops-schemas.js';

const logger = createLogger('letta_file_folder_ops');

/**
 * Handle letta_file_folder_ops tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following fileOpsInputSchema
 * @returns {Promise<Object>} Tool response
 */
export async function handleLettaFileFolderOps(server, args) {
    const { operation } = args;
    logger.info(`Executing file/folder operation: ${operation}`, { args });

    try {
        const handlers = {
            list_files: handleListFiles,
            open_file: handleOpenFile,
            close_file: handleCloseFile,
            close_all_files: handleCloseAllFiles,
            list_folders: handleListFolders,
            attach_folder: handleAttachFolder,
            detach_folder: handleDetachFolder,
            list_agents_in_folder: handleListAgentsInFolder,
        };

        if (!handlers[operation]) {
            throw new Error(`Unknown operation: ${operation}`);
        }

        return await handlers[operation](server, args);
    } catch (error) {
        logger.error(`File/folder operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * List files for an agent
 */
async function handleListFiles(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_files operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}/files`, { headers });
            return response.data;
        },
        'Listing agent files'
    );

    const files = Array.isArray(result) ? result : result.files || [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_files',
                agent_id,
                files: files.map(f => ({
                    id: f.id,
                    filename: f.filename || f.name,
                    size: f.size,
                    mime_type: f.mime_type || f.type,
                    is_open: f.is_open || false,
                    opened_at: f.opened_at,
                })),
                message: `Found ${files.length} files`,
            }),
        }],
    };
}

/**
 * Open a file for an agent
 */
async function handleOpenFile(server, args) {
    const { agent_id, file_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for open_file operation');
    }
    if (!file_id) {
        throw new Error('file_id is required for open_file operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/files/${encodeURIComponent(file_id)}/open`,
                {},
                { headers }
            );
            return response.data;
        },
        'Opening file'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'open_file',
                agent_id,
                file_id,
                opened: true,
                message: 'File opened successfully',
            }),
        }],
    };
}

/**
 * Close a specific file
 */
async function handleCloseFile(server, args) {
    const { agent_id, file_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for close_file operation');
    }
    if (!file_id) {
        throw new Error('file_id is required for close_file operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/files/${encodeURIComponent(file_id)}/close`,
                {},
                { headers }
            );
            return response.data;
        },
        'Closing file'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'close_file',
                agent_id,
                file_id,
                closed: true,
                message: 'File closed successfully',
            }),
        }],
    };
}

/**
 * Close all files for an agent
 */
async function handleCloseAllFiles(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for close_all_files operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/files/close-all`,
                {},
                { headers }
            );
            return response.data;
        },
        'Closing all files'
    );

    const closedCount = result.closed_count || result.count || 0;

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'close_all_files',
                agent_id,
                closed_count: closedCount,
                message: `Closed ${closedCount} files`,
            }),
        }],
    };
}

/**
 * List all folders
 */
async function handleListFolders(server, _args) {
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get('/folders', { headers });
            return response.data;
        },
        'Listing folders'
    );

    const folders = Array.isArray(result) ? result : result.folders || [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_folders',
                folders: folders.map(f => ({
                    id: f.id,
                    name: f.name,
                    path: f.path,
                    file_count: f.file_count || 0,
                    agent_count: f.agent_count || 0,
                })),
                message: `Found ${folders.length} folders`,
            }),
        }],
    };
}

/**
 * Attach folder to agent
 */
async function handleAttachFolder(server, args) {
    const { agent_id, folder_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for attach_folder operation');
    }
    if (!folder_id) {
        throw new Error('folder_id is required for attach_folder operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/folders/${encodeURIComponent(folder_id)}`,
                {},
                { headers }
            );
            return response.data;
        },
        'Attaching folder to agent'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'attach_folder',
                agent_id,
                folder_id,
                attached: true,
                message: 'Folder attached to agent successfully',
            }),
        }],
    };
}

/**
 * Detach folder from agent
 */
async function handleDetachFolder(server, args) {
    const { agent_id, folder_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for detach_folder operation');
    }
    if (!folder_id) {
        throw new Error('folder_id is required for detach_folder operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(
                `/agents/${encodeURIComponent(agent_id)}/folders/${encodeURIComponent(folder_id)}`,
                { headers }
            );
            return response.data;
        },
        'Detaching folder from agent'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'detach_folder',
                agent_id,
                folder_id,
                detached: true,
                message: 'Folder detached from agent successfully',
            }),
        }],
    };
}

/**
 * List agents in a specific folder
 */
async function handleListAgentsInFolder(server, args) {
    const { folder_id } = args;

    if (!folder_id) {
        throw new Error('folder_id is required for list_agents_in_folder operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/folders/${encodeURIComponent(folder_id)}/agents`,
                { headers }
            );
            return response.data;
        },
        'Listing agents in folder'
    );

    const agents = Array.isArray(result) ? result : result.agents || [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_agents_in_folder',
                folder_id,
                agents: agents.map(a => ({
                    id: a.id,
                    name: a.name || a.agent_name,
                })),
                message: `Found ${agents.length} agents in folder`,
            }),
        }],
    };
}

/**
 * Tool definition for letta_file_folder_ops
 */
export const lettaFileFolderOpsDefinition = {
    name: 'letta_file_folder_ops',
    description: 'File and Folder Management Hub - Tool for managing file sessions and folder operations with 8 operations: list_files, open_file, close_file, close_all_files (agent file management), list_folders, attach_folder, detach_folder, list_agents_in_folder (folder operations). Provides complete file session lifecycle and folder-agent relationships with discriminator-based operation routing.',
    inputSchema: fileOpsInputSchema,
};
