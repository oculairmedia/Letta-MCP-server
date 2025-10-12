import { createLogger } from '../../core/logger.js';

const logger = createLogger('add_mcp_tool_to_letta');

/**
 * Tool handler for registering an MCP tool as a native Letta tool.
 */
export async function handleAddMcpToolToLetta(server, args) {
    try {
        // Validate arguments (new arguments: tool_name, agent_id)
        if (!args.tool_name) {
            throw new Error('Missing required argument: tool_name (MCP tool name)');
        }
        if (!args.agent_id) {
            throw new Error('Missing required argument: agent_id');
        }

        const { tool_name, agent_id } = args;
        const mcp_tool_name = tool_name;

        // Headers for API requests
        const headers = server.getApiHeaders();

        // --- Find the MCP Server Name for the given tool_name ---
        logger.info(`Searching for MCP server providing tool: ${mcp_tool_name}...`);
        let mcp_server_name = null;
        const serversResponse = await server.api.get('/tools/mcp/servers', { headers });
        if (!serversResponse.data || typeof serversResponse.data !== 'object') {
            throw new Error('Failed to list MCP servers or invalid response format.');
        }
        const serverNames = Object.keys(serversResponse.data);

        for (const serverName of serverNames) {
            logger.info(`Checking server: ${serverName}`);
            try {
                const toolsResponse = await server.api.get(
                    `/tools/mcp/servers/${serverName}/tools`,
                    { headers },
                );
                if (toolsResponse.data && Array.isArray(toolsResponse.data)) {
                    const foundTool = toolsResponse.data.find(
                        (tool) => tool.name === mcp_tool_name,
                    );
                    if (foundTool) {
                        mcp_server_name = serverName;
                        logger.info(
                            `Found tool '${mcp_tool_name}' on server '${mcp_server_name}'.`,
                        );
                        break; // Stop searching once found
                    }
                }
            } catch (toolListError) {
                // Log error but continue searching other servers
                logger.info(
                    `Could not list tools for server ${serverName}: ${toolListError.message}`,
                );
            }
        }

        if (!mcp_server_name) {
            throw new Error(
                `Could not find any MCP server providing the tool named '${mcp_tool_name}'.`,
            );
        }
        // --- End of Find MCP Server Name ---

        // Headers for API requests
        // Note: The API spec didn't explicitly require user_id for this endpoint,
        // but adding it might be necessary depending on Letta's auth setup.

        logger.info(
            `Attempting to register MCP tool ${mcp_server_name}/${mcp_tool_name} with Letta...`,
        );
        const registerUrl = `/tools/mcp/servers/${mcp_server_name}/${mcp_tool_name}`;

        // Make the POST request to register the tool
        logger.info(`DEBUG: Calling registration URL: POST ${registerUrl}`);
        const registerResponse = await server.api.post(registerUrl, {}, { headers });

        // Check registration response data for success and the new tool ID
        if (!registerResponse.data || !registerResponse.data.id) {
            throw new Error(
                `Registration API call succeeded but did not return the expected tool ID. Response: ${JSON.stringify(registerResponse.data)}`,
            );
        }

        const lettaToolId = registerResponse.data.id;
        const lettaToolName = registerResponse.data.name || mcp_tool_name;
        logger.info(`Successfully registered tool. Letta Tool ID: ${lettaToolId}`);

        // Now, attempt to attach the newly registered tool to the agent
        logger.info(`Attempting to attach tool ${lettaToolId} to agent ${agent_id}...`);
        const attachUrl = `/agents/${agent_id}/tools/attach/${lettaToolId}`;
        let attachSuccess = false;
        let attachMessage = '';
        let attachError = null;

        try {
            // Use specific headers for the agent context if needed, otherwise reuse
            const attachHeaders = { ...headers };

            logger.info(`DEBUG: Calling attachment URL: PATCH ${attachUrl}`);
            const attachResponse = await server.api.patch(
                attachUrl,
                {},
                { headers: attachHeaders },
            );

            // Verify attachment (optional but good practice)
            const attachedToolIds = attachResponse.data.tools?.map((t) => t.id) || [];
            if (attachedToolIds.includes(lettaToolId)) {
                attachSuccess = true;
                attachMessage = `Successfully attached tool '${lettaToolName}' (ID: ${lettaToolId}) to agent ${agent_id}.`;
                logger.info(attachMessage);
            } else {
                attachMessage = `Attachment API call succeeded, but tool ${lettaToolId} was not found in agent's tools list afterwards.`;
                logger.info(attachMessage);
                // Consider this a partial failure
            }
        } catch (error) {
            attachSuccess = false;
            attachMessage = `Failed to attach tool ${lettaToolId} to agent ${agent_id}.`;
            attachError = error.response?.data || error.message || error;
            logger.error(`${attachMessage} Error:`, attachError);
        }

        // Return combined result
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        letta_tool_id: lettaToolId,
                        letta_tool_name: lettaToolName,
                        agent_id: agent_id,
                        attached: attachSuccess,
                        mcp_server_name: mcp_server_name,
                        mcp_tool_name: mcp_tool_name,
                        ...(attachError ? { error: attachError } : {}),
                    }),
                },
            ],
            isError: !attachSuccess, // Consider it an error if attachment failed
        };
    } catch (error) {
        logger.error(`Error during MCP tool registration or attachment: ${error.message}`);
        // Ensure the error response includes context about which step failed if possible
        server.createErrorResponse(
            error,
            `Failed during registration/attachment of ${args.mcp_server_name || 'unknown_server'}/${args.mcp_tool_name || 'unknown_tool'}`,
        );
    }
}

/**
 * Tool definition for add_mcp_tool_to_letta
 */
export const addMcpToolToLettaDefinition = {
    name: 'add_mcp_tool_to_letta',
    description:
        'Registers a tool from a connected MCP server as a native Letta tool AND attaches it to a specified agent. Use list_mcp_tools_by_server to find available tools, and list_agents to get agent IDs.',
    inputSchema: {
        type: 'object',
        properties: {
            tool_name: {
                type: 'string',
                description: 'The name of the MCP tool to find, register, and attach.',
            },
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to attach the newly registered tool to.',
            },
        },
        required: ['tool_name', 'agent_id'], // Updated required fields
        additionalProperties: false,
    },
};
