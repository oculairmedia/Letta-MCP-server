/**
 * Tool handler for modifying an existing agent
 */
export async function handleModifyAgent(server, args) {
    if (!args?.agent_id) {
        return server.createErrorResponse('Missing required argument: agent_id');
    }
    if (!args?.update_data) {
        return server.createErrorResponse('Missing required argument: update_data');
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
                return server.createErrorResponse(`Agent not found: ${args.agent_id}`);
            }
            if (error.response.status === 422) {
                return server.createErrorResponse(
                    `Validation error updating agent ${args.agent_id}: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        return server.createErrorResponse(error);
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
                properties: {
                    name: {
                        type: 'string',
                        description: 'Updated agent name',
                    },
                    system: {
                        type: 'string',
                        description: 'Updated system prompt for the agent',
                    },
                    description: {
                        type: 'string',
                        description: 'Updated agent description',
                    },
                    llm_config: {
                        type: 'object',
                        description: 'Updated LLM configuration',
                        properties: {
                            model: {
                                type: 'string',
                                description: 'Updated LLM model name',
                            },
                            temperature: {
                                type: 'number',
                                description: 'Updated temperature setting',
                            },
                            max_tokens: {
                                type: 'number',
                                description: 'Updated max tokens limit',
                            },
                        },
                        additionalProperties: true,
                    },
                    embedding_config: {
                        type: 'object',
                        description: 'Updated embedding configuration',
                        properties: {
                            model: {
                                type: 'string',
                                description: 'Updated embedding model name',
                            },
                        },
                        additionalProperties: true,
                    },
                    tool_ids: {
                        type: 'array',
                        items: {
                            type: 'string',
                        },
                        description: 'Updated list of tool IDs for the agent',
                    },
                    tags: {
                        type: 'array',
                        items: {
                            type: 'string',
                        },
                        description: 'Updated tags for the agent',
                    },
                },
                additionalProperties: true, // Allow other properties from UpdateAgent schema
            },
        },
        required: ['agent_id', 'update_data'],
        additionalProperties: false,
    },
};
