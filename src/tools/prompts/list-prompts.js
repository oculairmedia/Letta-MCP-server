import { promptRegistry } from '../../handlers/prompts.js';

/**
 * Tool handler for listing available prompts
 */
export async function handleListPrompts(server) {
    try {
        const prompts = Array.from(promptRegistry.values()).map((p) => ({
            name: p.name,
            title: p.title || p.name,
            description: p.description,
            arguments: p.arguments || [],
        }));

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(
                        {
                            total_prompts: prompts.length,
                            prompts,
                        },
                        null,
                        2,
                    ),
                },
            ],
            structuredContent: {
                total_prompts: prompts.length,
                prompts,
            },
        };
    } catch (error) {
        return server.createErrorResponse(error, 'Failed to list prompts');
    }
}

/**
 * Tool definition for list_prompts
 */
export const listPromptsToolDefinition = {
    name: 'list_prompts',
    description: 'List all available prompt templates including wizards and workflows',
    inputSchema: {
        type: 'object',
        properties: {},
        additionalProperties: false,
    },
    outputSchema: {
        type: 'object',
        properties: {
            total_prompts: {
                type: 'integer',
                description: 'Total number of available prompts',
            },
            prompts: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        title: { type: 'string' },
                        description: { type: 'string' },
                        arguments: {
                            type: 'array',
                            items: {
                                type: 'object',
                                properties: {
                                    name: { type: 'string' },
                                    title: { type: 'string' },
                                    description: { type: 'string' },
                                    required: { type: 'boolean' },
                                },
                            },
                        },
                    },
                    additionalProperties: false,
                },
            },
        },
        additionalProperties: false,
        required: ['total_prompts', 'prompts'],
    },
};
