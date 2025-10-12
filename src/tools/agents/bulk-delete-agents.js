import { createLogger } from '../../core/logger.js';

// McpError and ErrorCode imported for future use
const logger = createLogger('bulk_delete_agents');

/**
 * Tool handler for deleting multiple agents based on filter criteria
 */
export async function handleBulkDeleteAgents(server, args) {
    // Require at least one filter criteria to prevent accidental mass deletion
    if (!args?.agent_name_filter && !args?.agent_tag_filter && !args?.agent_ids) {
        server.createErrorResponse(
            'Missing required argument: Provide agent_ids, agent_name_filter, or agent_tag_filter.',
        );
    }

    const nameFilter = args.agent_name_filter;
    const tagFilter = args.agent_tag_filter;
    const specificAgentIds = args.agent_ids; // Allow deleting a specific list of IDs

    const results = [];
    let agentsToDelete = [];

    try {
        const headers = server.getApiHeaders();

        // Step 1: Identify agents to delete
        if (specificAgentIds && Array.isArray(specificAgentIds) && specificAgentIds.length > 0) {
            // If a list of IDs is provided, use that directly
            logger.info(`Received specific list of ${specificAgentIds.length} agents to delete.`);
            // We need agent objects (at least with 'id' and 'name') for consistent reporting
            // Fetch details for each ID or use list_agents with multiple ID filter if supported
            // For simplicity, we'll just use the IDs for deletion and report only IDs
            agentsToDelete = specificAgentIds.map((id) => ({ id: id, name: `ID: ${id}` })); // Create placeholder objects
        } else {
            // Otherwise, list agents based on filters
            logger.info(`Listing agents with filter: name='${nameFilter}', tags='${tagFilter}'...`);
            const listParams = {};
            if (nameFilter) listParams.name = nameFilter;
            if (tagFilter) listParams.tags = tagFilter; // Adjust if API uses different param name

            const listResponse = await server.api.get('/agents/', { headers, params: listParams });
            agentsToDelete = listResponse.data; // Assuming response.data is an array of AgentState objects

            if (!Array.isArray(agentsToDelete) || agentsToDelete.length === 0) {
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
            logger.info(`Found ${agentsToDelete.length} agents to delete.`);
        }

        // Step 2: Iterate and delete each agent
        for (const agent of agentsToDelete) {
            const agentId = agent.id;
            const encodedAgentId = encodeURIComponent(agentId);
            try {
                logger.info(`Deleting agent ${agentId} (${agent.name})...`);
                // Use the specific endpoint from the OpenAPI spec
                await server.api.delete(`/agents/${encodedAgentId}`, { headers });
                results.push({ agent_id: agentId, name: agent.name, status: 'success' });
                logger.info(`Successfully deleted agent ${agentId}.`);
            } catch (deleteError) {
                let errorMessage = `Failed to delete agent ${agentId} (${agent.name}): ${deleteError.message}`;
                if (deleteError.response) {
                    errorMessage += ` (Status: ${deleteError.response.status}, Data: ${JSON.stringify(deleteError.response.data)})`;
                }
                logger.error(errorMessage);
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
                            total_agents: agentsToDelete.length,
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
        logger.error('Error:', error.response?.data || error.message);
        server.createErrorResponse(`Failed during bulk delete operation: ${error.message}`);
    }
}

/**
 * Tool definition for bulk_delete_agents
 */
export const bulkDeleteAgentsDefinition = {
    name: 'bulk_delete_agents',
    description:
        'Deletes multiple agents based on filter criteria (name or tags) or a specific list of IDs. Use list_agents first to identify agents to delete. WARNING: This action is permanent.',
    inputSchema: {
        type: 'object',
        properties: {
            agent_ids: {
                type: 'array',
                items: { type: 'string' },
                description: 'Optional: A specific list of agent IDs to delete.',
            },
            agent_name_filter: {
                type: 'string',
                description:
                    'Optional: Filter agents to delete by name (exact match or substring, depending on API).',
            },
            agent_tag_filter: {
                type: 'string',
                description:
                    'Optional: Filter agents to delete by tag(s). Provide a single tag or comma-separated list.',
            },
            // Could add more filters like project_id if needed
        },
        // Custom validation in the handler ensures at least one argument is provided
        required: [],
        additionalProperties: false,
    },
};
