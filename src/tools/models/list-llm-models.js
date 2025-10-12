/**
 * Tool handler for listing available LLM models
 */
export async function handleListLlmModels(server, _args) {
    try {
        const headers = server.getApiHeaders();

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.get('/models/', { headers });
        const models = response.data; // Assuming response.data is an array of LLMConfig objects

        // Trim response to only essential fields to reduce token usage
        // Full model objects can contain 15+ fields, we only need 5-6 essential ones
        const trimmedModels = models.map(model => ({
            handle: model.handle,                    // What to use when referencing the model
            model: model.model,                      // Model name
            provider: model.provider_name,           // Provider (openai, anthropic, etc)
            context_window: model.context_window,    // Critical for capacity planning
            max_tokens: model.max_tokens,            // Output limits
            temperature: model.temperature,          // Default temperature (can be overridden)
        }));

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        model_count: models.length,
                        models: trimmedModels,
                    }),
                },
            ],
        };
    } catch (error) {
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for list_llm_models
 */
export const listLlmModelsDefinition = {
    name: 'list_llm_models',
    description:
        'List available LLM models configured on the Letta server. Use with create_agent or modify_agent to set agent model preferences.',
    inputSchema: {
        type: 'object',
        properties: {}, // No input arguments needed
        required: [],
        additionalProperties: false,
    },
};
