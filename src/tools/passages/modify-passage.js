import { createLogger } from '../../core/logger.js';

const logger = createLogger('modify_passage');

/**
 * Tool handler for modifying a passage in an agent's archival memory
 */
export async function handleModifyPassage(server, args) {
    if (!args?.agent_id) {
        server.createErrorResponse('Missing required argument: agent_id');
    }
    if (!args?.memory_id) {
        server.createErrorResponse('Missing required argument: memory_id');
    }
    if (!args?.update_data || typeof args.update_data.text !== 'string') {
        // Ensure text is provided and is a string
        server.createErrorResponse(
            "Missing or invalid required argument: update_data must contain a 'text' field (string).",
        );
    }

    try {
        const headers = server.getApiHeaders();
        const agentId = encodeURIComponent(args.agent_id);
        const memoryId = args.memory_id; // Use for finding, encode for URL

        // Step 1: Fetch ALL passages for the agent to find the target one
        let existingPassage = null;
        try {
            // Use list_passages logic internally, ensuring embeddings are included for the PATCH
            const listResponse = await server.api.get(`/agents/${agentId}/archival-memory`, {
                headers,
                params: { include_embeddings: true }, // Ensure we get embeddings
            });
            const allPassages = listResponse.data;
            if (Array.isArray(allPassages)) {
                existingPassage = allPassages.find((p) => p.id === memoryId);
            }
            if (!existingPassage) {
                throw new Error(`Could not find passage ${memoryId} for agent ${agentId}.`);
            }
            // Basic check for required fields based on the schema provided by the user
            if (
                !existingPassage.embedding_config ||
                !existingPassage.id ||
                existingPassage.text === undefined
            ) {
                logger.error('Fetched passage object is missing required fields:', existingPassage);
                throw new Error(
                    `Fetched passage ${memoryId} is missing required fields (embedding_config, id, text).`,
                );
            }
        } catch (fetchError) {
            if (fetchError.response && fetchError.response.status === 404) {
                server.createErrorResponse(
                    `Agent not found when listing passages: ${args.agent_id}`,
                );
            }
            logger.error('Error fetching passages:', fetchError);
            server.createErrorResponse(
                `Failed to fetch passages for agent ${args.agent_id}: ${fetchError.message}`,
            );
        }

        // Step 2: Construct the full update payload based on the fetched passage, modifying only the text
        const updatePayload = {
            ...existingPassage, // Copy all fields from the fetched passage
            text: args.update_data.text, // Update the text field
        };

        // Remove fields that shouldn't be sent in PATCH body if necessary (adjust based on API behavior)
        // delete updatePayload.created_at;
        // delete updatePayload.updated_at;
        // delete updatePayload.created_by_id;
        // delete updatePayload.last_updated_by_id;
        // delete updatePayload.organization_id;
        // delete updatePayload.agent_id; // agent_id is in the URL path

        logger.info(
            `[modify_passage] Sending payload for memory_id ${memoryId}:`,
            JSON.stringify(updatePayload),
        );

        // Step 3: Send the PATCH request with the complete payload
        const patchResponse = await server.api.patch(
            `/agents/${agentId}/archival-memory/${encodeURIComponent(memoryId)}`,
            updatePayload,
            { headers },
        );
        let modifiedPassages = patchResponse.data; // API returns an array of modified Passage objects

        // Optionally remove embeddings from the response based on the flag
        const includeEmbeddings = args?.include_embeddings ?? false;
        if (!includeEmbeddings && Array.isArray(modifiedPassages)) {
            modifiedPassages = modifiedPassages.map((passage) => {
                // eslint-disable-next-line no-unused-vars
                const { embedding, ...rest } = passage;
                return rest;
            });
        }

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        passages: modifiedPassages,
                    }),
                },
            ],
        };
    } catch (error) {
        logger.error('[modify_passage] Error:', error.response?.data || error.message);
        if (error.response) {
            if (error.response.status === 404) {
                server.createErrorResponse(
                    `Agent or Passage not found during update: agent_id=${args.agent_id}, memory_id=${args.memory_id}`,
                );
            }
            if (error.response.status === 422) {
                server.createErrorResponse(
                    `Validation error modifying passage ${args.memory_id}: ${JSON.stringify(error.response.data)}`,
                );
            }
        }
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for modify_passage
 */
export const modifyPassageDefinition = {
    name: 'modify_passage',
    description:
        "Modify a memory in the agent's archival memory store. Use list_passages to find memory IDs. Currently only supports updating the text content.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose passage to modify',
            },
            memory_id: {
                type: 'string',
                description: 'ID of the passage (memory) to modify',
            },
            update_data: {
                type: 'object',
                description:
                    "Object containing the fields to update. Currently only supports updating the 'text' field.",
                properties: {
                    text: {
                        type: 'string',
                        description: 'The new text content for the passage.',
                    },
                    // Add other fields here if the API supports updating them via PassageUpdate schema
                },
                required: ['text'], // Require 'text' within the update_data object
            },
            include_embeddings: {
                type: 'boolean',
                description:
                    'Whether to include the full embedding vectors in the response (default: false).',
                default: false,
            },
            additionalProperties: false,
        },
        required: ['agent_id', 'memory_id', 'update_data'],
        additionalProperties: false,
    },
};
