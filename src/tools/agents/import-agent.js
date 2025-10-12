import fs from 'fs';
import path from 'path';
import FormData from 'form-data'; // Assuming form-data is available
import { createLogger } from '../../core/logger.js';

const logger = createLogger('import_agent');

/**
 * Tool handler for importing an agent from a JSON file
 */
export async function handleImportAgent(server, args) {
    if (!args?.file_path) {
        server.createErrorResponse('Missing required argument: file_path');
    }

    const filePath = path.resolve(args.file_path); // Resolve to absolute path

    // Check if file exists
    if (!fs.existsSync(filePath)) {
        server.createErrorResponse(`File not found at path: ${filePath}`);
    }

    try {
        const headers = server.getApiHeaders();
        // Remove content-type as axios will set it correctly for FormData
        delete headers['Content-Type'];

        const form = new FormData();
        form.append('file', fs.createReadStream(filePath), path.basename(filePath));

        // Construct query parameters for optional settings
        const params = {};
        if (args.append_copy_suffix !== undefined) {
            params.append_copy_suffix = args.append_copy_suffix;
        }
        if (args.override_existing_tools !== undefined) {
            params.override_existing_tools = args.override_existing_tools;
        }
        if (args.project_id) {
            params.project_id = args.project_id;
        }

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.post('/agents/import', form, {
            headers: {
                ...headers,
                ...form.getHeaders(), // Let FormData set the Content-Type with boundary
            },
            params: params, // Add optional query parameters
        });

        const importedAgentState = response.data; // Assuming response.data is the new AgentState object

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent_id: importedAgentState.id,
                        agent: importedAgentState,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle potential 422 for validation errors, or other API/file errors
        if (error.response) {
            if (error.response.status === 422) {
                server.createErrorResponse(
                    `Validation error importing agent from ${args.file_path}: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        logger.error('[import_agent] Error:', error.response?.data || error.message);
        server.createErrorResponse(
            `Failed to import agent from ${args.file_path}: ${error.message}`,
        );
    }
}

/**
 * Tool definition for import_agent
 */
export const importAgentDefinition = {
    name: 'import_agent',
    description:
        'Import a serialized agent JSON file and recreate the agent in the system. Use export_agent to create the JSON file, then modify_agent or attach_tool to customize the imported agent.',
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'Path to the agent JSON file to import.',
            },
            append_copy_suffix: {
                type: 'boolean',
                description:
                    'Optional: If set to True, appends "_copy" to the end of the agent name. Defaults to true.',
                default: true,
            },
            override_existing_tools: {
                type: 'boolean',
                description:
                    'Optional: If set to True, existing tools can get their source code overwritten by the uploaded tool definitions. Letta core tools cannot be updated. Defaults to true.',
                default: true,
            },
            project_id: {
                type: 'string',
                description: 'Optional: The project ID to associate the uploaded agent with.',
            },
        },
        required: ['file_path'],
        additionalProperties: false,
    },
};
