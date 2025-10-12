import { createLogger } from '../../core/logger.js';

const logger = createLogger('attach_tool');

/**
 * Tool handler for attaching tools (by ID or name) to an agent in the Letta system.
 * Handles finding existing Letta tools by name, finding MCP tools by name,
 * registering MCP tools if needed, and attaching them.
 */
export async function handleAttachTool(server, args) {
    const processingResults = []; // Track success/failure for each requested item
    const resolvedToolInfos = []; // Store { id, name } for tools to be attached
    const toolIdsToAttach = new Set(); // Avoid duplicate attachments

    try {
        // --- 1. Validate Arguments ---
        if (!args.agent_id) {
            throw new Error('Missing required argument: agent_id');
        }
        const agent_id = args.agent_id;

        const toolIdsInput = args.tool_ids || (args.tool_id ? [args.tool_id] : []);
        const toolNamesInput = args.tool_names || [];

        if (!Array.isArray(toolIdsInput)) {
            throw new Error('Invalid argument: tool_ids must be an array.');
        }
        if (!Array.isArray(toolNamesInput)) {
            throw new Error('Invalid argument: tool_names must be an array.');
        }
        if (toolIdsInput.length === 0 && toolNamesInput.length === 0) {
            throw new Error(
                'Missing required argument: either tool_id(s) or tool_names must be provided.',
            );
        }

        // --- 2. Prepare Headers and Get Agent Info ---
        const headers = server.getApiHeaders();
        // Add agent_id to headers if needed by API for authorization context
        // headers['user_id'] = agent_id;

        let agentName = 'Unknown';
        try {
            logger.info(`Fetching info for agent ${agent_id}...`);
            const agentInfoResponse = await server.api.get(`/agents/${agent_id}`, { headers });
            agentName = agentInfoResponse.data?.name || agent_id;
        } catch (agentError) {
            // Proceed even if agent info fetch fails, but log warning
            logger.info(`Could not fetch agent info for ${agent_id}: ${agentError.message}`);
            agentName = agent_id; // Use ID as name fallback
        }

        // --- 3. Process Provided Tool IDs ---
        logger.info(`Processing provided tool IDs: ${toolIdsInput.join(', ')}`);
        for (const toolId of toolIdsInput) {
            try {
                const toolResponse = await server.api.get(`/tools/${toolId}`, { headers });
                const toolInfo = {
                    id: toolId,
                    name: toolResponse.data?.name || `Unknown (${toolId})`,
                };
                if (!toolIdsToAttach.has(toolId)) {
                    resolvedToolInfos.push(toolInfo);
                    toolIdsToAttach.add(toolId);
                }
                processingResults.push({
                    input: toolId,
                    type: 'id',
                    success: true,
                    status: 'found',
                    details: toolInfo,
                });
            } catch (error) {
                const message = `Provided tool ID ${toolId} not found or error fetching: ${error.message}`;
                logger.error(message);
                processingResults.push({
                    input: toolId,
                    type: 'id',
                    success: false,
                    status: 'error',
                    error: message,
                });
            }
        }

        // --- 4. Process Provided Tool Names ---
        if (toolNamesInput.length > 0) {
            logger.info(`Processing provided tool names: ${toolNamesInput.join(', ')}`);

            // 4a. Fetch all existing Letta tools for efficient lookup
            let lettaTools = [];
            try {
                const listToolsResponse = await server.api.get('/tools/', { headers });
                lettaTools = listToolsResponse.data || []; // Assuming API returns list directly
                if (!Array.isArray(lettaTools)) {
                    logger.info('Unexpected format for /tools/ response, expected array.');
                    lettaTools = [];
                }
            } catch (listError) {
                logger.info(
                    `Could not list existing Letta tools: ${listError.message}. Proceeding without Letta tool check.`,
                );
            }

            // 4b. Fetch all MCP servers and their tools for efficient lookup
            let mcpServersData = {};
            const mcpToolMap = new Map(); // Map<tool_name, { server: string, tool: object }>
            try {
                const serversResponse = await server.api.get('/tools/mcp/servers', { headers });
                mcpServersData = serversResponse.data || {};
                if (typeof mcpServersData !== 'object') {
                    logger.info(
                        'Unexpected format for /tools/mcp/servers response, expected object.',
                    );
                    mcpServersData = {};
                }
                const serverNames = Object.keys(mcpServersData);
                for (const serverName of serverNames) {
                    try {
                        const mcpToolsResponse = await server.api.get(
                            `/tools/mcp/servers/${serverName}/tools`,
                            { headers },
                        );
                        if (mcpToolsResponse.data && Array.isArray(mcpToolsResponse.data)) {
                            mcpToolsResponse.data.forEach((tool) => {
                                if (!mcpToolMap.has(tool.name)) {
                                    // Avoid overwriting if names clash, take first found
                                    mcpToolMap.set(tool.name, { server: serverName, tool: tool });
                                } else {
                                    logger.info(
                                        `Duplicate MCP tool name found: '${tool.name}' exists on multiple servers. Using first found on server '${mcpToolMap.get(tool.name).server}'.`,
                                    );
                                }
                            });
                        }
                    } catch (mcpListError) {
                        logger.info(
                            `Could not list tools for MCP server ${serverName}: ${mcpListError.message}`,
                        );
                    }
                }
            } catch (mcpServersError) {
                logger.info(
                    `Could not list MCP servers: ${mcpServersError.message}. Proceeding without MCP tool check.`,
                );
            }

            // 4c. Iterate through names and resolve them
            for (const toolName of toolNamesInput) {
                let found = false;

                // Check if already resolved by ID earlier (if ID and name were both provided)
                if (resolvedToolInfos.some((info) => info.name === toolName)) {
                    processingResults.push({
                        input: toolName,
                        type: 'name',
                        success: true,
                        status: 'found_by_id_earlier',
                        details: `Tool '${toolName}' was already resolved by ID.`,
                    });
                    found = true;
                    continue; // Skip further processing for this name
                }

                // Try finding as existing Letta tool
                const existingLettaTool = lettaTools.find((t) => t.name === toolName);
                if (existingLettaTool) {
                    logger.info(
                        `Found existing Letta tool: ${toolName} (ID: ${existingLettaTool.id})`,
                    );
                    const toolInfo = { id: existingLettaTool.id, name: toolName };
                    if (!toolIdsToAttach.has(toolInfo.id)) {
                        resolvedToolInfos.push(toolInfo);
                        toolIdsToAttach.add(toolInfo.id);
                    }
                    processingResults.push({
                        input: toolName,
                        type: 'name',
                        success: true,
                        status: 'found_letta',
                        details: toolInfo,
                    });
                    found = true;
                    continue;
                }

                // Try finding as MCP tool and register if found
                const mcpToolInfo = mcpToolMap.get(toolName);
                if (mcpToolInfo) {
                    // eslint-disable-next-line no-unused-vars
                    const { server: mcp_server_name, tool } = mcpToolInfo;
                    logger.info(
                        `Found MCP tool '${toolName}' on server '${mcp_server_name}'. Attempting registration...`,
                    );
                    const registerUrl = `/tools/mcp/servers/${mcp_server_name}/${toolName}`;
                    try {
                        const registerResponse = await server.api.post(
                            registerUrl,
                            {},
                            { headers },
                        );
                        if (registerResponse.data && registerResponse.data.id) {
                            const lettaToolId = registerResponse.data.id;
                            const lettaToolName = registerResponse.data.name || toolName;
                            logger.info(
                                `Successfully registered MCP tool. New Letta ID: ${lettaToolId}`,
                            );
                            const toolInfo = { id: lettaToolId, name: lettaToolName };
                            if (!toolIdsToAttach.has(toolInfo.id)) {
                                resolvedToolInfos.push(toolInfo);
                                toolIdsToAttach.add(toolInfo.id);
                            }
                            processingResults.push({
                                input: toolName,
                                type: 'name',
                                success: true,
                                status: 'registered_mcp',
                                details: toolInfo,
                            });
                            found = true;
                        } else {
                            throw new Error(
                                `Registration API call succeeded but did not return expected ID. Response: ${JSON.stringify(registerResponse.data)}`,
                            );
                        }
                    } catch (registerError) {
                        const message = `Failed to register MCP tool ${mcp_server_name}/${toolName}: ${registerError.message}`;
                        logger.error(message);
                        processingResults.push({
                            input: toolName,
                            type: 'name',
                            success: false,
                            status: 'error_registration',
                            error: message,
                        });
                        found = true; // Mark as found to avoid "not found" error below, even though registration failed
                    }
                    continue;
                }

                // If not found anywhere
                if (!found) {
                    const message = `Tool name '${toolName}' not found as an existing Letta tool or a registerable MCP tool.`;
                    logger.error(message);
                    processingResults.push({
                        input: toolName,
                        type: 'name',
                        success: false,
                        status: 'not_found',
                        error: message,
                    });
                }
            }
        }

        // --- 5. Attach Resolved Tools ---
        const attachmentResults = [];
        if (resolvedToolInfos.length === 0) {
            logger.info('No tools resolved successfully for attachment.');
        } else {
            logger.info(
                `Attempting to attach ${resolvedToolInfos.length} resolved tool(s) to agent ${agent_id} (${agentName})...`,
            );
            for (const tool of resolvedToolInfos) {
                logger.info(`Attaching tool ${tool.name} (${tool.id})...`);
                const attachUrl = `/agents/${agent_id}/tools/attach/${tool.id}`;
                try {
                    const response = await server.api.patch(attachUrl, {}, { headers });
                    // Check if tool is now in agent's tools (optional but good)
                    const attachedToolIds = response.data?.tools?.map((t) => t.id) || [];
                    if (attachedToolIds.includes(tool.id)) {
                        attachmentResults.push({
                            tool_id: tool.id,
                            tool_name: tool.name,
                            success: true,
                            message: 'Successfully attached.',
                        });
                    } else {
                        attachmentResults.push({
                            tool_id: tool.id,
                            tool_name: tool.name,
                            success: false,
                            error: "Attachment API call succeeded, but tool not found in agent's list afterwards.",
                        });
                    }
                } catch (error) {
                    const message = `Failed to attach tool ${tool.name} (${tool.id}): ${error.message}`;
                    logger.error(message, error.response?.data || '');
                    attachmentResults.push({
                        tool_id: tool.id,
                        tool_name: tool.name,
                        success: false,
                        error: message,
                        details: error.response?.data || error,
                    });
                }
            }
        }

        // --- 6. Final Response ---
        const overallSuccess =
            processingResults.every((r) => r.success) && attachmentResults.every((r) => r.success);

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent_id: agent_id,
                        agent_name: agentName,
                        processing_summary: processingResults,
                        attachment_summary: attachmentResults,
                    }),
                },
            ],
            isError: !overallSuccess,
        };
    } catch (error) {
        logger.error(`Unhandled error in handleAttachTool: ${error.message}`);
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for attach_tool (now handles IDs and names)
 */
export const attachToolToolDefinition = {
    name: 'attach_tool',
    description:
        'Attach one or more tools (by ID or name) to an agent. If a name corresponds to an MCP tool not yet in Letta, it will be registered first. Find tools with list_mcp_tools_by_server or create custom ones with upload_tool. Use list_agent_tools to verify attachment.',
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to attach the tool(s) to.',
            },
            tool_id: {
                // Kept for backward compatibility
                type: 'string',
                description:
                    'The ID of a single tool to attach (deprecated, use tool_ids or tool_names instead).',
            },
            tool_ids: {
                type: 'array',
                items: { type: 'string' },
                description: 'Optional array of existing Letta tool IDs to attach.',
            },
            tool_names: {
                type: 'array',
                items: { type: 'string' },
                description:
                    'Optional array of tool names to attach. These can be existing Letta tools or MCP tools (which will be registered if found).',
            },
        },
        required: ['agent_id'],
        additionalProperties: false,
        // Custom validation could be added here if needed to ensure at least one tool input is present
    },
};
