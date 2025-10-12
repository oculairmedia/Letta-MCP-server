import { createLogger } from '../../core/logger.js';
// We might need the list_agents handler logic if it's complex, or re-implement the API call.
// For simplicity, let's assume we can call the API directly here.

const logger = createLogger('bulk_attach_tool_to_agents');

/**
 * Tool handler for attaching a tool to multiple agents based on a filter
 */
export async function handleBulkAttachToolToAgents(server, args) {
    if (!args?.tool_id) {
        server.createErrorResponse('Missing required argument: tool_id');
    }
    // Require at least one filter criteria
    if (!args?.agent_name_filter && !args?.agent_tag_filter) {
        server.createErrorResponse(
            'Missing required argument: Provide either agent_name_filter or agent_tag_filter.',
        );
    }

    const toolId = args.tool_id;
    const nameFilter = args.agent_name_filter;
    const tagFilter = args.agent_tag_filter; // Assuming API supports tag filtering for list_agents

    const results = [];
    let agentsToProcess = [];

    try {
        const headers = server.getApiHeaders();

        // Step 1: List agents based on filter criteria
        logger.info(
            `[bulk_attach_tool] Listing agents with filter: name='${nameFilter}', tags='${tagFilter}'...`,
        );
        const listParams = {};
        if (nameFilter) listParams.name = nameFilter; // Assuming API uses 'name' for filtering
        if (tagFilter) listParams.tags = tagFilter; // Assuming API uses 'tags' (might need adjustment based on actual API)

        const listResponse = await server.api.get('/agents/', { headers, params: listParams });
        agentsToProcess = listResponse.data; // Assuming response.data is an array of AgentState objects

        if (!Array.isArray(agentsToProcess) || agentsToProcess.length === 0) {
            return {
                content: [
                    {
                        type: 'text',
                        text: JSON.stringify({
                            message: 'No agents found matching the specified filter.',
                            results: [],
                        }),
                    },
                ],
            };
        }
        logger.info(`[bulk_attach_tool] Found ${agentsToProcess.length} agents to process.`);

        // Step 2: Iterate and attach tool to each agent
        const encodedToolId = encodeURIComponent(toolId);
        for (const agent of agentsToProcess) {
            const agentId = agent.id;
            const encodedAgentId = encodeURIComponent(agentId);
            try {
                logger.info(`[bulk_attach_tool] Attaching tool ${toolId} to agent ${agentId}...`);
                // Use the specific endpoint from the OpenAPI spec
                await server.api.patch(
                    `/agents/${encodedAgentId}/tools/attach/${encodedToolId}`,
                    {},
                    { headers },
                ); // PATCH often has empty body for attach/detach
                results.push({ agent_id: agentId, name: agent.name, status: 'success' });
                logger.info(
                    `[bulk_attach_tool] Successfully attached tool ${toolId} to agent ${agentId}.`,
                );
            } catch (attachError) {
                let errorMessage = `Failed to attach tool ${toolId} to agent ${agentId}: ${attachError.message}`;
                if (attachError.response) {
                    errorMessage += ` (Status: ${attachError.response.status}, Data: ${JSON.stringify(attachError.response.data)})`;
                }
                logger.error(`[bulk_attach_tool] ${errorMessage}`);
                results.push({
                    agent_id: agentId,
                    name: agent.name,
                    status: 'error',
                    error: errorMessage,
                });
            }
        }

        // Step 3: Return summary of results
        const successCount = results.filter((r) => r.status === 'success').length;
        const errorCount = results.filter((r) => r.status === 'error').length;

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        summary: {
                            total_agents: agentsToProcess.length,
                            success_count: successCount,
                            error_count: errorCount,
                        },
                        results: results,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle errors during the list_agents call or unexpected issues
        logger.error('[bulk_attach_tool] Error:', error.response?.data || error.message);
        server.createErrorResponse(`Failed during bulk attach operation: ${error.message}`);
    }
}

/**
 * Tool definition for bulk_attach_tool_to_agents
 */
export const bulkAttachToolDefinition = {
    name: 'bulk_attach_tool_to_agents',
    description:
        'Attaches a specified tool to multiple agents based on filter criteria (name or tags). Use list_agents to find agents and list_mcp_tools_by_server or upload_tool to get tool IDs.',
    inputSchema: {
        type: 'object',
        properties: {
            tool_id: {
                type: 'string',
                description: 'The ID of the tool to attach.',
            },
            agent_name_filter: {
                type: 'string',
                description:
                    'Optional: Filter agents by name (exact match or substring, depending on API).',
            },
            agent_tag_filter: {
                // Assuming API accepts comma-separated string or array for tags
                type: 'string', // or 'array' with items: { type: 'string' }
                description:
                    'Optional: Filter agents by tag(s). Provide a single tag or comma-separated list.',
            },
            // Could add more filters like project_id if needed and supported by list_agents API
        },
        required: ['tool_id'],
        additionalProperties: false,
        // Custom validation could ensure at least one filter is present, but basic schema doesn't enforce this easily.
        // The handler function checks for this.
    },
};
