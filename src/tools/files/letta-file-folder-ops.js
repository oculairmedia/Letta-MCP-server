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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListFiles(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_files operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.files.list() method
            return await server.client.agents.files.list(agent_id);
        },
        'Listing agent files'
    );

    // SDK returns PaginatedAgentFiles with files array
    const files = result.files || [];

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleOpenFile(server, args) {
    const { agent_id, file_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for open_file operation');
    }
    if (!file_id) {
        throw new Error('file_id is required for open_file operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.files.open() method - returns string[] of evicted files
            return await server.client.agents.files.open(agent_id, file_id);
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
                evicted_files: result, // SDK returns array of file names evicted due to LRU
                message: 'File opened successfully',
            }),
        }],
    };
}

/**
 * Close a specific file
 * MIGRATED: Now using Letta SDK instead of axios
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
            // Use SDK client.agents.files.close() method - returns void
            return await server.client.agents.files.close(agent_id, file_id);
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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCloseAllFiles(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for close_all_files operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.files.closeAll() method - returns string[] of closed files
            return await server.client.agents.files.closeAll(agent_id);
        },
        'Closing all files'
    );

    // SDK returns array of file names that were closed
    const closedFiles = Array.isArray(result) ? result : [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'close_all_files',
                agent_id,
                closed_count: closedFiles.length,
                closed_files: closedFiles,
                message: `Closed ${closedFiles.length} files`,
            }),
        }],
    };
}

/**
 * List all folders
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListFolders(server, _args) {
    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.folders.list() method
            return await server.client.folders.list();
        },
        'Listing folders'
    );

    // SDK returns Folder[] array
    const folders = Array.isArray(result) ? result : [];

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleAttachFolder(server, args) {
    const { agent_id, folder_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for attach_folder operation');
    }
    if (!folder_id) {
        throw new Error('folder_id is required for attach_folder operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.folders.attach() method - returns AgentState
            return await server.client.agents.folders.attach(agent_id, folder_id);
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
                agent_state: result, // SDK returns AgentState
                message: 'Folder attached to agent successfully',
            }),
        }],
    };
}

/**
 * Detach folder from agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDetachFolder(server, args) {
    const { agent_id, folder_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for detach_folder operation');
    }
    if (!folder_id) {
        throw new Error('folder_id is required for detach_folder operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.folders.detach() method - returns AgentState
            return await server.client.agents.folders.detach(agent_id, folder_id);
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
                agent_state: result, // SDK returns AgentState
                message: 'Folder detached from agent successfully',
            }),
        }],
    };
}

/**
 * List agents in a specific folder
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListAgentsInFolder(server, args) {
    const { folder_id } = args;

    if (!folder_id) {
        throw new Error('folder_id is required for list_agents_in_folder operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.folders.agents.list() method - returns string[] of agent IDs
            return await server.client.folders.agents.list(folder_id);
        },
        'Listing agents in folder'
    );

    // SDK returns array of agent ID strings
    const agentIds = Array.isArray(result) ? result : [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_agents_in_folder',
                folder_id,
                agent_ids: agentIds,
                agents: agentIds.map(id => ({ id })), // Convert IDs to objects for consistency
                message: `Found ${agentIds.length} agents in folder`,
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
