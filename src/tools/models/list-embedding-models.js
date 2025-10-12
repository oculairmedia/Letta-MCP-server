/**
 * Tool handler for listing available embedding models
 */
export async function handleListEmbeddingModels(server, _args) {
    try {
        const headers = server.getApiHeaders();

        // Use the specific endpoint from the OpenAPI spec
        const response = await server.api.get('/models/embedding', { headers });
        const models = response.data; // Assuming response.data is an array of EmbeddingConfig objects

        // Trim response to only essential fields to reduce token usage
        // Full embedding model objects can contain 10+ fields, we only need 5 essential ones
        const trimmedModels = models.map(model => ({
            handle: model.handle,                            // What to use when referencing the model
            embedding_model: model.embedding_model,          // Model name
            provider: model.embedding_endpoint_type,         // Provider type (openai, google_ai, etc)
            embedding_dim: model.embedding_dim,              // Critical: dimension of embeddings
            embedding_chunk_size: model.embedding_chunk_size,// Important: chunk size for text processing
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
 * Tool definition for list_embedding_models
 */
export const listEmbeddingModelsDefinition = {
    name: 'list_embedding_models',
    description:
        'List available embedding models configured on the Letta server. Use with create_agent or modify_agent to set agent embedding preferences.',
    inputSchema: {
        type: 'object',
        properties: {}, // No input arguments needed
        required: [],
        additionalProperties: false,
    },
};
