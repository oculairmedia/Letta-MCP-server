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
 */
async function handleGetCoreMemory(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_core_memory operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/memory`,
                { headers }
            );
            return response.data;
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
 */
async function handleUpdateCoreMemory(server, args) {
    const { agent_id, memory_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for update_core_memory operation');
    }
    if (!memory_data) {
        throw new Error('memory_data is required for update_core_memory operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/memory`,
                memory_data,
                { headers }
            );
            return response.data;
        },
        'Updating core memory'
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
                    message: 'Core memory updated successfully',
                }),
            },
        ],
    };
}

/**
 * Get specific memory block by label
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
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/memory/blocks/${encodeURIComponent(block_label)}`,
                { headers }
            );
            return response.data;
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
 */
async function handleListBlocks(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_blocks operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/memory/blocks`,
                { headers }
            );
            return response.data;
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
            const headers = server.getApiHeaders();
            const response = await server.api.post('/memory/blocks', block_data, { headers });
            return response.data;
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
 */
async function handleGetBlock(server, args) {
    const { block_id } = args;

    if (!block_id) {
        throw new Error('block_id is required for get_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/memory/blocks/${encodeURIComponent(block_id)}`, { headers });
            return response.data;
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
            const headers = server.getApiHeaders();
            const response = await server.api.put(
                `/memory/blocks/${encodeURIComponent(block_id)}`,
                block_data,
                { headers }
            );
            return response.data;
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
 */
async function handleAttachBlock(server, args) {
    const { agent_id, block_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for attach_block operation');
    }
    if (!block_id) {
        throw new Error('block_id is required for attach_block operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/memory/blocks/${encodeURIComponent(block_id)}`,
                {},
                { headers }
            );
            return response.data;
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
                    message: 'Memory block attached to agent successfully',
                }),
            },
        ],
    };
}

/**
 * Detach memory block from agent
 */
async function handleDetachBlock(server, args) {
    const { agent_id, block_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for detach_block operation');
    }
    if (!block_id) {
        throw new Error('block_id is required for detach_block operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(
                `/agents/${encodeURIComponent(agent_id)}/memory/blocks/${encodeURIComponent(block_id)}`,
                { headers }
            );
            return response.data;
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
                    message: 'Memory block detached successfully',
                }),
            },
        ],
    };
}

/**
 * List agents using a specific memory block
 */
async function handleListAgentsUsingBlock(server, args) {
    const { block_id } = args;

    if (!block_id) {
        throw new Error('block_id is required for list_agents_using_block operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/memory/blocks/${encodeURIComponent(block_id)}/agents`,
                { headers }
            );
            return response.data;
        },
        'Listing agents using block'
    );

    const agents = Array.isArray(result) ? result : result.agents || [];

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
 */
async function handleSearchArchival(server, args) {
    const { agent_id, search_query, search_options = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for search_archival operation');
    }
    if (!search_query) {
        throw new Error('search_query is required for search_archival operation');
    }

    const queryParams = new URLSearchParams();
    queryParams.append('query', search_query);
    if (search_options.limit) queryParams.append('limit', search_options.limit);
    if (search_options.threshold) queryParams.append('threshold', search_options.threshold);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/archival/search?${queryParams.toString()}`,
                { headers }
            );
            return response.data;
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
 */
async function handleListPassages(server, args) {
    const { agent_id, pagination = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_passages operation');
    }

    const queryParams = new URLSearchParams();
    if (pagination.limit) queryParams.append('limit', pagination.limit);
    if (pagination.offset) queryParams.append('offset', pagination.offset);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/agents/${encodeURIComponent(agent_id)}/archival${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
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
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/archival`,
                passage_data,
                { headers }
            );
            return response.data;
        },
        'Creating passage'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create_passage',
                    agent_id,
                    passage_id: result.id,
                    passage: {
                        id: result.id,
                        text: result.text || result.content,
                        timestamp: result.timestamp || result.created_at,
                    },
                    message: 'Passage created successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing passage
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

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.put(
                `/agents/${encodeURIComponent(agent_id)}/archival/${encodeURIComponent(passage_id)}`,
                passage_data,
                { headers }
            );
            return response.data;
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
                    passage: result,
                    message: 'Passage updated successfully',
                }),
            },
        ],
    };
}

/**
 * Delete a passage from archival memory
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
            const headers = server.getApiHeaders();
            const response = await server.api.delete(
                `/agents/${encodeURIComponent(agent_id)}/archival/${encodeURIComponent(passage_id)}`,
                { headers }
            );
            return response.data;
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
