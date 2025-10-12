import { createLogger } from '../../core/logger.js';

const logger = createLogger('attach_memory_block');

/**
 * Tool handler for attaching a memory block to an agent in the Letta system
 */
export async function handleAttachMemoryBlock(server, args) {
    try {
        // Validate arguments
        if (!args.block_id) {
            throw new Error('Missing required argument: block_id');
        }
        if (!args.agent_id) {
            throw new Error('Missing required argument: agent_id');
        }

        // Headers for API requests
        const headers = server.getApiHeaders();
        headers['user_id'] = args.agent_id;

        // Verify block exists
        const blockResponse = await server.api.get(`/blocks/${args.block_id}`, { headers });
        const blockData = blockResponse.data;
        const blockName = blockData.name || 'Unnamed Block';

        // Determine label to use
        const label = args.label || blockData.label || 'custom';

        // Attach block to agent
        logger.info(
            `Attaching memory block ${blockName} (${args.block_id}) to agent ${args.agent_id} with label ${label}...`,
        );

        // Use the core-memory/blocks/attach endpoint
        const attachUrl = `/agents/${args.agent_id}/core-memory/blocks/attach/${args.block_id}`;

        // Send an empty object as the request body
        await server.api.patch(attachUrl, {}, { headers });

        // Get updated agent data to verify attachment
        const agentInfoResponse = await server.api.get(`/agents/${args.agent_id}`, { headers });
        const agentData = agentInfoResponse.data;
        const agentName = agentData.name || 'Unknown';

        // Format the response
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        agent_id: args.agent_id,
                        agent_name: agentName,
                        block_id: args.block_id,
                        block_name: blockName,
                        label: label,
                    }),
                },
            ],
        };
    } catch (error) {
        server.createErrorResponse(error);
    }
}

/**
 * Tool definition for attach_memory_block
 */
export const attachMemoryBlockToolDefinition = {
    name: 'attach_memory_block',
    description:
        'Attach a memory block to an agent. Use list_memory_blocks to find blocks, create_memory_block to make new ones. Common labels: "persona", "human", "system".',
    inputSchema: {
        type: 'object',
        properties: {
            block_id: {
                type: 'string',
                description: 'The ID of the memory block to attach',
            },
            agent_id: {
                type: 'string',
                description: 'The ID of the agent to attach the memory block to',
            },
            label: {
                type: 'string',
                description:
                    'Optional label for the memory block (e.g., "persona", "human", "system")',
            },
        },
        required: ['block_id', 'agent_id'],
        additionalProperties: false,
    },
};
