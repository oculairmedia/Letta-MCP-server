import { promptRegistry } from '../../handlers/prompts.js';

/**
 * Tool handler for executing a registered prompt
 */
export async function handleUsePrompt(server, args) {
    try {
        const { prompt_name, arguments: promptArgs } = args;

        if (!prompt_name) {
            throw new Error('Missing required parameter: prompt_name');
        }

        // Get the prompt from registry
        const prompt = promptRegistry.get(prompt_name);
        if (!prompt) {
            // List available prompts for helpful error
            const availablePrompts = Array.from(promptRegistry.keys());
            throw new Error(
                `Prompt not found: ${prompt_name}. Available prompts: ${availablePrompts.join(', ')}`,
            );
        }

        // Execute the prompt handler
        const messages = await prompt.handler(promptArgs || {});

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(
                        {
                            prompt_name,
                            description: prompt.description,
                            messages: messages.length,
                            preview: messages[0]?.content?.text?.substring(0, 200) + '...',
                        },
                        null,
                        2,
                    ),
                },
            ],
            structuredContent: {
                prompt_name,
                description: prompt.description,
                messages,
            },
        };
    } catch (error) {
        return server.createErrorResponse(error, 'Failed to execute prompt');
    }
}

/**
 * Tool definition for use_prompt
 */
export const usePromptToolDefinition = {
    name: 'use_prompt',
    description:
        'Execute a registered prompt template. Use this to run wizards, workflows, and guided interactions.',
    inputSchema: {
        type: 'object',
        properties: {
            prompt_name: {
                type: 'string',
                description:
                    'Name of the prompt to execute (e.g., letta_agent_wizard, letta_memory_optimizer)',
            },
            arguments: {
                type: 'object',
                description: 'Arguments to pass to the prompt (depends on the specific prompt)',
                additionalProperties: true,
            },
        },
        required: ['prompt_name'],
    },
    outputSchema: {
        type: 'object',
        properties: {
            prompt_name: {
                type: 'string',
                description: 'Name of the executed prompt',
            },
            description: {
                type: 'string',
                description: 'Description of the prompt',
            },
            messages: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        role: { type: 'string' },
                        content: { type: 'object' },
                    },
                },
                description: 'Messages returned by the prompt',
            },
            additionalProperties: false,
        },
        required: ['prompt_name', 'messages'],
    },
};
