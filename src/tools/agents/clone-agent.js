import fs from 'fs/promises'; // Use promises for async file operations
import path from 'path';
import os from 'os'; // To get temporary directory
import FormData from 'form-data'; // Assuming form-data is available
import { createLogger } from '../../core/logger.js';

const logger = createLogger('clone_agent');

/**
 * Tool handler for cloning an agent
 */
export async function handleCloneAgent(server, args) {
    if (!args?.source_agent_id) {
        server.createErrorResponse('Missing required argument: source_agent_id');
    }
    if (!args?.new_agent_name) {
        server.createErrorResponse('Missing required argument: new_agent_name');
    }

    const sourceAgentId = args.source_agent_id;
    const newAgentName = args.new_agent_name;
    const overrideTools = args.override_existing_tools ?? true; // Default override
    const projectId = args.project_id; // Optional project ID for the new agent

    let tempFilePath = '';

    try {
        const headers = server.getApiHeaders();
        const encodedSourceAgentId = encodeURIComponent(sourceAgentId);

        // --- Step 1: Export the source agent ---
        logger.info(`Exporting source agent ${sourceAgentId}...`);
        const exportResponse = await server.api.get(`/agents/${encodedSourceAgentId}/export`, {
            headers,
        });
        const agentConfig = exportResponse.data;

        if (!agentConfig || typeof agentConfig !== 'object') {
            throw new Error('Received invalid data from agent export endpoint.');
        }
        logger.info(`Source agent ${sourceAgentId} exported successfully.`);

        // --- Step 2: Modify the configuration for the new agent ---
        agentConfig.name = newAgentName; // Set the new name
        // Optionally clear fields that shouldn't be copied or might cause conflicts
        // delete agentConfig.id; // ID will be assigned on import
        // delete agentConfig.created_at;
        // delete agentConfig.updated_at;
        // Consider if message history should be copied or cleared
        // agentConfig.messages = [];
        // agentConfig.message_ids = [];

        const agentJsonString = JSON.stringify(agentConfig, null, 2);

        // --- Step 3: Save modified config to a temporary file ---
        // Use os.tmpdir() which should work inside Docker if /tmp is writable
        tempFilePath = path.join(os.tmpdir(), `agent_clone_temp_${Date.now()}.json`);
        logger.info(`Saving temporary config to ${tempFilePath}...`);
        await fs.writeFile(tempFilePath, agentJsonString);
        logger.info('Temporary config saved.');

        // --- Step 4: Import the modified configuration ---
        logger.info(`Importing new agent '${newAgentName}' from ${tempFilePath}...`);
        const importHeaders = server.getApiHeaders();
        delete importHeaders['Content-Type']; // Let FormData set the correct header

        const form = new FormData();
        form.append('file', await fs.readFile(tempFilePath), path.basename(tempFilePath)); // Read file content for FormData

        const importParams = {
            append_copy_suffix: false, // We explicitly set the name, don't append suffix
            override_existing_tools: overrideTools,
        };
        if (projectId) {
            importParams.project_id = projectId;
        }

        const importResponse = await server.api.post('/agents/import', form, {
            headers: {
                ...importHeaders,
                ...form.getHeaders(),
            },
            params: importParams,
        });

        const importedAgentState = importResponse.data;
        logger.info(
            `Agent '${newAgentName}' imported successfully with ID: ${importedAgentState.id}`,
        );

        // --- Step 5: Cleanup temporary file ---
        await fs.unlink(tempFilePath);
        logger.info(`Cleaned up temporary file ${tempFilePath}.`);

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        new_agent: importedAgentState,
                    }),
                },
            ],
        };
    } catch (error) {
        logger.error('Error:', error.response?.data || error.message);
        // Attempt cleanup even on error
        if (tempFilePath) {
            try {
                await fs.unlink(tempFilePath);
                logger.info(`Cleaned up temporary file ${tempFilePath} after error.`);
            } catch (cleanupError) {
                logger.error(`Error cleaning up temporary file ${tempFilePath}:`, cleanupError);
            }
        }

        // Handle specific API errors
        if (error.response) {
            if (error.response.status === 404 && error.config.url.includes('/export')) {
                server.createErrorResponse(`Source agent not found: ${sourceAgentId}`);
            }
            if (error.response.status === 422 && error.config.url.includes('/import')) {
                server.createErrorResponse(
                    `Validation error importing cloned agent: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        server.createErrorResponse(`Failed to clone agent ${sourceAgentId}: ${error.message}`);
    }
}

/**
 * Tool definition for clone_agent
 */
export const cloneAgentDefinition = {
    name: 'clone_agent',
    description:
        'Creates a new agent by cloning the configuration of an existing agent. Use list_agents to find source agent ID. Alternative to export_agent + import_agent workflow. Modify the clone with modify_agent afterwards.',
    inputSchema: {
        type: 'object',
        properties: {
            source_agent_id: {
                type: 'string',
                description: 'The ID of the agent to clone.',
            },
            new_agent_name: {
                type: 'string',
                description: 'The name for the new cloned agent.',
            },
            override_existing_tools: {
                type: 'boolean',
                description:
                    'Optional: If set to True, existing tools can get their source code overwritten by the tool definitions from the source agent. Defaults to true.',
                default: true,
            },
            project_id: {
                type: 'string',
                description: 'Optional: The project ID to associate the new cloned agent with.',
            },
        },
        required: ['source_agent_id', 'new_agent_name'],
        additionalProperties: false,
    },
};
