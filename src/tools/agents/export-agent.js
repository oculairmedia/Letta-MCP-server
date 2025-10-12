import fs from 'fs';
import path from 'path';
import axios from 'axios'; // Assuming axios is available
import FormData from 'form-data'; // Assuming form-data is available
import { createLogger } from '../../core/logger.js';
// McpError and ErrorCode imported by framework

const logger = createLogger('export_agent');

/**
 * Tool handler for exporting an agent's configuration
 */
export async function handleExportAgent(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }

    const agentId = args.agent_id;
    const outputPath = args.output_path || `agent_${agentId}.json`;
    const returnBase64 = args.return_base64 ?? false;
    const uploadToXBackbone = args.upload_to_xbackbone ?? false; // Default to false for security
    const xbackboneUrl = args.xbackbone_url || process.env.XBACKBONE_URL; // No hardcoded default
    const xbackboneToken = args.xbackbone_token || process.env.XBACKBONE_TOKEN; // No hardcoded default

    try {
        const headers = server.getApiHeaders();
        const encodedAgentId = encodeURIComponent(agentId);

        // Step 1: Fetch agent export data
        const response = await server.api.get(`/agents/${encodedAgentId}/export`, { headers });
        const agentData = response.data; // Assuming response.data is the AgentSchema JSON

        if (!agentData) {
            throw new Error('Received empty data from agent export endpoint.');
        }

        const agentJsonString = JSON.stringify(agentData, null, 2);

        // Step 2: Save locally
        const absoluteOutputPath = path.resolve(outputPath);
        try {
            fs.writeFileSync(absoluteOutputPath, agentJsonString);
        } catch (writeError) {
            logger.error(`Error writing agent export to ${absoluteOutputPath}:`, writeError);
            server.createErrorResponse(
                `Failed to save agent export to ${absoluteOutputPath}: ${writeError.message}`,
            );
        }

        // Step 3: Upload to XBackbone if requested
        let xbackboneResult = null;
        if (uploadToXBackbone) {
            if (!xbackboneUrl || !xbackboneToken) {
                logger.warn('XBackbone URL or Token not configured, skipping upload.');
            } else {
                try {
                    const form = new FormData();
                    form.append(
                        'upload',
                        fs.createReadStream(absoluteOutputPath),
                        path.basename(absoluteOutputPath),
                    );
                    form.append('token', xbackboneToken);

                    const uploadResponse = await axios.post(`${xbackboneUrl}/upload`, form, {
                        headers: {
                            ...form.getHeaders(),
                            // Add any other necessary headers for XBackbone if needed
                        },
                        // If XBackbone uses self-signed certs, might need:
                        // httpsAgent: new https.Agent({ rejectUnauthorized: false })
                    });

                    if (
                        uploadResponse.status >= 200 &&
                        uploadResponse.status < 300 &&
                        uploadResponse.data?.url
                    ) {
                        xbackboneResult = {
                            url: uploadResponse.data.url,
                            raw_url: `${uploadResponse.data.url}/raw`,
                            // Assuming XBackbone provides a delete URL structure like this
                            delete_url: `${uploadResponse.data.url}/delete/${xbackboneToken}`,
                        };
                        logger.info(
                            `Successfully uploaded ${absoluteOutputPath} to XBackbone: ${xbackboneResult.url}`,
                        );
                    } else {
                        logger.error(
                            `XBackbone upload failed with status ${uploadResponse.status}:`,
                            uploadResponse.data,
                        );
                        // Don't fail the whole tool, just report the upload issue
                        xbackboneResult = {
                            error: `Upload failed with status ${uploadResponse.status}`,
                        };
                    }
                } catch (uploadError) {
                    logger.error('Error uploading to XBackbone:', uploadError);
                    xbackboneResult = { error: `Upload failed: ${uploadError.message}` };
                }
            }
        }

        // Step 4: Prepare and return result
        const resultPayload = {
            agent_id: agentId,
            file_path: absoluteOutputPath,
        };

        if (xbackboneResult && !xbackboneResult.error) {
            resultPayload.xbackbone_url = xbackboneResult.url;
        }

        if (returnBase64) {
            resultPayload.base64_data = Buffer.from(agentJsonString).toString('base64');
        }

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(resultPayload),
                },
            ],
        };
    } catch (error) {
        // Handle potential 404 if agent not found, or other API errors
        if (error.response && error.response.status === 404) {
            server.createErrorResponse(`Agent not found: ${agentId}`);
        }
        logger.error('Error:', error.response?.data || error.message);
        server.createErrorResponse(`Failed to export agent ${agentId}: ${error.message}`);
    }
}

/**
 * Tool definition for export_agent
 */
export const exportAgentDefinition = {
    name: 'export_agent',
    description:
        "Export an agent's configuration to a JSON file and optionally upload it. Use import_agent to recreate the agent later, or clone_agent for a quick copy. Use list_agents to find agent IDs.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to export.',
            },
            output_path: {
                type: 'string',
                description:
                    'Optional: Path to save the exported JSON file (e.g., my_agent.json). Defaults to agent_{agent_id}.json.',
            },
            return_base64: {
                type: 'boolean',
                description:
                    'Optional: If true, return the JSON content as base64 string in the response. Defaults to false.',
                default: false,
            },
            upload_to_xbackbone: {
                type: 'boolean',
                description:
                    'Optional: If true, upload the exported file to XBackbone. Defaults to false.',
                default: false,
            },
            xbackbone_url: {
                type: 'string',
                description:
                    'Optional: URL of the XBackbone instance. Uses XBACKBONE_URL environment variable if not provided.',
            },
            xbackbone_token: {
                type: 'string',
                description:
                    'Optional: Token for XBackbone authentication. Uses XBACKBONE_TOKEN environment variable if not provided.',
            },
        },
        required: ['agent_id'],
        additionalProperties: false,
    },
};
