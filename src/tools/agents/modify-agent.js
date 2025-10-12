/**
 * Tool handler for modifying an existing agent
 */
export async function handleModifyAgent(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }
    if (!args?.update_data) {
        server.createErrorResponse('Missing required argument: update_data');
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);
        const updatePayload = args.update_data; // This should conform to the UpdateAgent schema

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.patch(`/agents/${agentId}`, updatePayload, { headers });
        const updatedAgentState = response.data; // Assuming response.data is the updated AgentState object

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent: updatedAgentState,
                    }),
                },
            ],
        };
    } catch (error) {
        // Handle potential 404 if agent not found, 422 for validation errors, or other API errors
        if (error.response) {
            if (error.response.status === 404) {
                server.createErrorResponse(`Agent not found: ${args.agent_id}`);
            }
            if (error.response.status === 422) {
                server.createErrorResponse(
                    `Validation error updating agent ${args.agent_id}: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for modify_agent
 * Note: The input schema for update_data should ideally reflect the UpdateAgent schema
 * from the OpenAPI spec for better validation and clarity. For simplicity here,
 * it's defined as a generic object. A more robust implementation would generate
 * this schema dynamically or define it explicitly based on the spec.
 */
export const modifyAgentDefinition = {
    name: 'modify_agent',
    description:
        'Update an existing agent by ID with provided data. Use get_agent_summary to see current config, list_llm_models/list_embedding_models for model options. For tools, use attach_tool instead.',
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to modify',
            },
            update_data: {
                type: 'object',
                description:
                    'An object containing the fields to update (e.g., name, system, description, tool_ids, etc.)',
                additionalProperties: true, // Allow other properties from UpdateAgent schema
            },
        },
        required: ['agent_id', 'update_data'],
        additionalProperties: false,
    },
};
