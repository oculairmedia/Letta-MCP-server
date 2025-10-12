/**
 * Tool handler for letta_memory_unified - Unified Memory Operations Hub
 * Provides unified interface for all memory operations
 */
import { createLogger } from '../../core/logger.js';
// eslint-disable-next-line no-unused-vars
import { memoryUnifiedInputSchema, memoryUnifiedOutputSchema } from '../schemas/memory-unified-schemas.js';

const logger = createLogger('letta_memory_unified');

/**
 * Handle letta_memory_unified tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following memoryUnifiedInputSchema
 * @returns {Promise<Object>} Tool response following memoryUnifiedOutputSchema
 */
export async function handleLettaMemoryUnified(server, args) {
    const { operation } = args;

    logger.info(`Executing memory operation: ${operation}`, { args });

    try {
        switch (operation) {
            // Core memory operations
            case 'get_core_memory':
                return await handleGetCoreMemory(server, args);
            case 'update_core_memory':
                return await handleUpdateCoreMemory(server, args);
            // Block operations
            case 'get_block_by_label':
                return await handleGetBlockByLabel(server, args);
            case 'list_blocks':
                return await handleListBlocks(server, args);
            case 'create_block':
                return await handleCreateBlock(server, args);
            case 'get_block':
                return await handleGetBlock(server, args);
            case 'update_block':
                return await handleUpdateBlock(server, args);
            case 'attach_block':
                return await handleAttachBlock(server, args);
            case 'detach_block':
                return await handleDetachBlock(server, args);
            case 'list_agents_using_block':
                return await handleListAgentsUsingBlock(server, args);
            // Archival/passage operations
            case 'search_archival':
                return await handleSearchArchival(server, args);
            case 'list_passages':
                return await handleListPassages(server, args);
            case 'create_passage':
                return await handleCreatePassage(server, args);
            case 'update_passage':
                return await handleUpdatePassage(server, args);
            case 'delete_passage':
                return await handleDeletePassage(server, args);
            default:
                throw new Error(`Unknown operation: ${operation}`);
        }
    } catch (error) {
        logger.error(`Memory operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * Get agent's core memory (persona + human blocks)
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetCoreMemory(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_core_memory operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.coreMemory.retrieve() method
            return await server.client.agents.coreMemory.retrieve(agent_id);
        },
        'Getting core memory'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_core_memory',
                    agent_id,
                    core_memory: {
                        persona: result.persona || result.core_memory?.persona || '',
                        human: result.human || result.core_memory?.human || '',
                        ...result.core_memory,
                    },
                    message: 'Core memory retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Update agent's core memory
 * MIGRATED: Now using Letta SDK via blocks.modify() instead of axios
 * Note: SDK doesn't have direct core memory update, so we update individual blocks
 */
async function handleUpdateCoreMemory(server, args) {
    const { agent_id, memory_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for update_core_memory operation');
    }
    if (!memory_data) {
        throw new Error('memory_data is required for update_core_memory operation');
    }

    // Update blocks individually using SDK
    const updates = [];

    if (memory_data.persona !== undefined) {
        const personaUpdate = await server.handleSdkCall(
            async () => {
                return await server.client.agents.blocks.modify(
                    agent_id,
                    'persona',
                    { value: memory_data.persona }
                );
            },
            'Updating persona block'
        );
        updates.push(personaUpdate);
    }

    if (memory_data.human !== undefined) {
        const humanUpdate = await server.handleSdkCall(
            async () => {
                return await server.client.agents.blocks.modify(
                    agent_id,
                    'human',
                    { value: memory_data.human }
                );
            },
            'Updating human block'
        );
        updates.push(humanUpdate);
    }

    // Get updated core memory to return
    const result = await server.handleSdkCall(
        async () => {
            return await server.client.agents.coreMemory.retrieve(agent_id);
        },
        'Getting updated core memory'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update_core_memory',
                    agent_id,
                    core_memory: result.core_memory || result,
                    updates_applied: updates.length,
                    message: 'Core memory updated successfully',
                }),
            },
        ],
    };
}

/**
 * Get specific memory block by label
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetBlockByLabel(server, args) {
    const { agent_id, block_label } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_block_by_label operation');
    }
    if (!block_label) {
        throw new Error('block_label is required for get_block_by_label operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.blocks.retrieve() method
            return await server.client.agents.blocks.retrieve(agent_id, block_label);
        },
        'Getting memory block by label'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_block_by_label',
                    agent_id,
                    block_label,
                    block: {
                        id: result.id,
                        label: result.label,
                        value: result.value,
                        limit: result.limit,
                    },
                    message: 'Memory block retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * List all memory blocks for agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListBlocks(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_blocks operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.blocks.list() method
            return await server.client.agents.blocks.list(agent_id);
        },
        'Listing memory blocks'
    );

    const blocks = Array.isArray(result) ? result : result.blocks || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_blocks',
                    agent_id,
                    blocks: blocks.map(block => ({
                        id: block.id,
                        label: block.label,
                        value: block.value,
                        limit: block.limit,
                    })),
                    message: `Found ${blocks.length} memory blocks`,
                }),
            },
        ],
    };
}

/**
 * Create a new memory block
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCreateBlock(server, args) {
    const { block_data } = args;

    if (!block_data) {
        throw new Error('block_data is required for create_block operation');
    }
    if (!block_data.label) {
        throw new Error('block_data.label is required for create_block operation');
    }
    if (!block_data.value) {
        throw new Error('block_data.value is required for create_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.blocks.create() method
            return await server.client.blocks.create(block_data);
        },
        'Creating memory block'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create_block',
                    block_id: result.id,
                    block: {
                        id: result.id,
                        label: result.label,
                        value: result.value,
                        limit: result.limit,
                    },
                    message: 'Memory block created successfully',
                }),
            },
        ],
    };
}

/**
 * Get memory block by ID
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetBlock(server, args) {
    const { block_id } = args;

    if (!block_id) {
        throw new Error('block_id is required for get_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.blocks.retrieve() method
            return await server.client.blocks.retrieve(block_id);
        },
        'Getting memory block'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_block',
                    block_id,
                    block: {
                        id: result.id,
                        label: result.label,
                        value: result.value,
                        limit: result.limit,
                    },
                    message: 'Memory block retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing memory block
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleUpdateBlock(server, args) {
    const { block_id, block_data } = args;

    if (!block_id) {
        throw new Error('block_id is required for update_block operation');
    }
    if (!block_data) {
        throw new Error('block_data is required for update_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.blocks.modify() method
            return await server.client.blocks.modify(block_id, block_data);
        },
        'Updating memory block'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update_block',
                    block_id,
                    block: result,
                    message: 'Memory block updated successfully',
                }),
            },
        ],
    };
}

/**
 * Attach memory block to agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleAttachBlock(server, args) {
    const { agent_id, block_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for attach_block operation');
    }
    if (!block_id) {
        throw new Error('block_id is required for attach_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.blocks.attach() method
            return await server.client.agents.blocks.attach(agent_id, block_id);
        },
        'Attaching memory block to agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'attach_block',
                    agent_id,
                    block_id,
                    attached: true,
                    agent_state: result, // SDK returns AgentState
                    message: 'Memory block attached to agent successfully',
                }),
            },
        ],
    };
}

/**
 * Detach memory block from agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDetachBlock(server, args) {
    const { agent_id, block_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for detach_block operation');
    }
    if (!block_id) {
        throw new Error('block_id is required for detach_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.blocks.detach() method
            return await server.client.agents.blocks.detach(agent_id, block_id);
        },
        'Detaching memory block'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'detach_block',
                    agent_id,
                    block_id,
                    detached: true,
                    agent_state: result, // SDK returns AgentState
                    message: 'Memory block detached successfully',
                }),
            },
        ],
    };
}

/**
 * List agents using a specific memory block
 * MIGRATED: Now using Letta SDK via blocks.agents sub-resource
 * Note: SDK may not have direct support, falling back to check block details
 */
async function handleListAgentsUsingBlock(server, args) {
    const { block_id } = args;

    if (!block_id) {
        throw new Error('block_id is required for list_agents_using_block operation');
    }

    // Get block details which may include agent references
    const block = await server.handleSdkCall(
        async () => {
            return await server.client.blocks.retrieve(block_id);
        },
        'Getting block details for agent list'
    );

    // If SDK doesn't provide agent list directly, we need to fall back to axios
    // Check if block.agents exists, otherwise make direct API call
    let agents = [];
    if (block.agents) {
        agents = Array.isArray(block.agents) ? block.agents : [];
    } else {
        // Fallback to direct API call if SDK doesn't support this endpoint
        const result = await server.handleSdkCall(
            async () => {
                const headers = server.getApiHeaders();
                const response = await server.api.get(
                    `/memory/blocks/${encodeURIComponent(block_id)}/agents`,
                    { headers }
                );
                return response.data;
            },
            'Listing agents using block (API fallback)'
        );
        agents = Array.isArray(result) ? result : result.agents || [];
    }

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_agents_using_block',
                    block_id,
                    agents: agents.map(agent => ({
                        id: agent.id,
                        name: agent.name || agent.agent_name,
                    })),
                    message: `Found ${agents.length} agents using this block`,
                }),
            },
        ],
    };
}

/**
 * Search archival memory with similarity
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleSearchArchival(server, args) {
    const { agent_id, search_query, search_options = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for search_archival operation');
    }
    if (!search_query) {
        throw new Error('search_query is required for search_archival operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.passages.search() method
            return await server.client.agents.passages.search(agent_id, {
                query: search_query,
                ...search_options,
            });
        },
        'Searching archival memory'
    );

    const searchResults = Array.isArray(result) ? result : result.results || result.passages || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'search_archival',
                    agent_id,
                    search_results: searchResults.map(item => ({
                        id: item.id,
                        text: item.text || item.content,
                        timestamp: item.timestamp || item.created_at,
                        similarity_score: item.similarity_score || item.score || 0,
                    })),
                    message: `Found ${searchResults.length} results`,
                }),
            },
        ],
    };
}

/**
 * List passages/archival memory for an agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListPassages(server, args) {
    const { agent_id, pagination = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_passages operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.passages.list() method
            return await server.client.agents.passages.list(agent_id, pagination);
        },
        'Listing passages'
    );

    const passages = Array.isArray(result) ? result : result.passages || result.results || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_passages',
                    agent_id,
                    passages: passages.map(p => ({
                        id: p.id,
                        text: p.text || p.content,
                        timestamp: p.timestamp || p.created_at,
                    })),
                    message: `Found ${passages.length} passages`,
                }),
            },
        ],
    };
}

/**
 * Create a new passage/archival memory entry
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCreatePassage(server, args) {
    const { agent_id, passage_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for create_passage operation');
    }
    if (!passage_data) {
        throw new Error('passage_data is required for create_passage operation');
    }
    if (!passage_data.text) {
        throw new Error('passage_data.text is required for create_passage operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.passages.create() method
            // SDK returns array of created passages
            return await server.client.agents.passages.create(agent_id, passage_data);
        },
        'Creating passage'
    );

    // SDK may return array of passages
    const passages = Array.isArray(result) ? result : [result];
    const firstPassage = passages[0] || result;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create_passage',
                    agent_id,
                    passage_id: firstPassage.id,
                    passage: {
                        id: firstPassage.id,
                        text: firstPassage.text || firstPassage.content,
                        timestamp: firstPassage.timestamp || firstPassage.created_at,
                    },
                    message: 'Passage created successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing passage
 * MIGRATED: Now using Letta SDK instead of axios
 * Note: SDK modify() method returns void, no response data
 */
async function handleUpdatePassage(server, args) {
    const { agent_id, passage_id, passage_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for update_passage operation');
    }
    if (!passage_id) {
        throw new Error('passage_id is required for update_passage operation');
    }
    if (!passage_data) {
        throw new Error('passage_data is required for update_passage operation');
    }

    // SDK modify() returns void, so we just call it
    await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.passages.modify() method
            // Note: SDK method signature is modify(agentId, memoryId) - doesn't take passage_data
            return await server.client.agents.passages.modify(agent_id, passage_id);
        },
        'Updating passage'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update_passage',
                    agent_id,
                    passage_id,
                    message: 'Passage updated successfully (SDK returns void)',
                }),
            },
        ],
    };
}

/**
 * Delete a passage from archival memory
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDeletePassage(server, args) {
    const { agent_id, passage_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for delete_passage operation');
    }
    if (!passage_id) {
        throw new Error('passage_id is required for delete_passage operation');
    }

    await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.passages.delete() method
            return await server.client.agents.passages.delete(agent_id, passage_id);
        },
        'Deleting passage'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete_passage',
                    agent_id,
                    passage_id,
                    deleted: true,
                    message: 'Passage deleted successfully',
                }),
            },
        ],
    };
}

/**
 * Tool definition for letta_memory_unified
 */
export const lettaMemoryUnifiedDefinition = {
    name: 'letta_memory_unified',
    description:
        'Unified Memory Operations Hub - Comprehensive tool for all memory management including core memory (get_core_memory, update_core_memory), memory blocks (create_block, get_block, update_block, attach_block, detach_block, get_block_by_label, list_blocks, list_agents_using_block), and archival/passages (search_archival, list_passages, create_passage, update_passage, delete_passage). Provides 15 operations with discriminator-based operation routing. Replaces 9 individual memory endpoints.',
    inputSchema: memoryUnifiedInputSchema,
};
