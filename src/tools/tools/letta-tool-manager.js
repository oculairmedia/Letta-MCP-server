/**
 * Tool handler for letta_tool_manager - Tool Lifecycle Management Hub
 * Provides unified interface for complete tool management operations
 */
import { createLogger } from '../../core/logger.js';
// eslint-disable-next-line no-unused-vars
import { toolManagerInputSchema, toolManagerOutputSchema } from '../schemas/tool-manager-schemas.js';

const logger = createLogger('letta_tool_manager');

/**
 * Handle letta_tool_manager tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following toolManagerInputSchema
 * @returns {Promise<Object>} Tool response following toolManagerOutputSchema
 */
export async function handleLettaToolManager(server, args) {
    const { operation } = args;

    logger.info(`Executing tool operation: ${operation}`, { args });

    try {
        switch (operation) {
            case 'list':
                return await handleListTools(server, args);
            case 'get':
                return await handleGetTool(server, args);
            case 'create':
                return await handleCreateTool(server, args);
            case 'update':
                return await handleUpdateTool(server, args);
            case 'delete':
                return await handleDeleteTool(server, args);
            case 'upsert':
                return await handleUpsertTool(server, args);
            case 'attach':
                return await handleAttachTool(server, args);
            case 'detach':
                return await handleDetachTool(server, args);
            case 'bulk_attach':
                return await handleBulkAttach(server, args);
            case 'generate_from_prompt':
                return await handleGenerateFromPrompt(server, args);
            case 'generate_schema':
                return await handleGenerateSchema(server, args);
            case 'run_from_source':
                return await handleRunFromSource(server, args);
            case 'add_base_tools':
                return await handleAddBaseTools(server, args);
            default:
                throw new Error(`Unknown operation: ${operation}`);
        }
    } catch (error) {
        logger.error(`Tool operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * List all tools with pagination and filters
 */
async function handleListTools(server, args) {
    const { options = {} } = args;
    const { pagination = {}, filters = {} } = options;

    const queryParams = new URLSearchParams();

    // Apply pagination
    if (pagination.limit) queryParams.append('limit', pagination.limit);
    if (pagination.offset) queryParams.append('offset', pagination.offset);
    if (pagination.after) queryParams.append('after', pagination.after);

    // Apply filters
    if (filters.name) queryParams.append('name', filters.name);
    if (filters.tags && filters.tags.length > 0) {
        filters.tags.forEach(tag => queryParams.append('tags', tag));
    }
    if (filters.source_type) queryParams.append('source_type', filters.source_type);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/tools/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Listing tools'
    );

    // Handle both array and paginated response formats
    const tools = Array.isArray(result) ? result : result.tools || result.data || [];
    const total = result.total || tools.length;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list',
                    tools: tools.map(tool => ({
                        id: tool.id,
                        name: tool.name,
                        description: tool.description,
                        tags: tool.tags || [],
                        source_type: tool.source_type,
                        created_at: tool.created_at,
                        updated_at: tool.updated_at,
                    })),
                    pagination: {
                        total,
                        limit: pagination.limit || 50,
                        offset: pagination.offset || 0,
                        has_more: tools.length === (pagination.limit || 50),
                    },
                    message: `Found ${tools.length} tools`,
                }),
            },
        ],
    };
}

/**
 * Get tool details by ID
 */
async function handleGetTool(server, args) {
    const { tool_id } = args;

    if (!tool_id) {
        throw new Error('tool_id is required for get operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/tools/${encodeURIComponent(tool_id)}`, { headers });
            return response.data;
        },
        'Getting tool details'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get',
                    tool_id,
                    tool: {
                        id: result.id,
                        name: result.name,
                        description: result.description,
                        source_code: result.source_code,
                        tags: result.tags || [],
                        json_schema: result.json_schema || result.schema,
                    },
                    message: 'Tool details retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Create a new tool
 */
async function handleCreateTool(server, args) {
    const { tool_data } = args;

    if (!tool_data) {
        throw new Error('tool_data is required for create operation');
    }
    if (!tool_data.name) {
        throw new Error('tool_data.name is required for create operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post('/tools/', tool_data, { headers });
            return response.data;
        },
        'Creating tool'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create',
                    tool_id: result.id,
                    tool: result,
                    message: 'Tool created successfully',
                }),
            },
        ],
    };
}

/**
 * Attach a tool to an agent
 */
async function handleAttachTool(server, args) {
    const { agent_id, tool_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for attach operation');
    }
    if (!tool_id) {
        throw new Error('tool_id is required for attach operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/tools/${encodeURIComponent(tool_id)}`,
                {},
                { headers }
            );
            return response.data;
        },
        'Attaching tool to agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'attach',
                    agent_id,
                    tool_id,
                    attached: true,
                    message: 'Tool attached to agent successfully',
                }),
            },
        ],
    };
}

/**
 * Bulk attach tool to multiple agents
 */
async function handleBulkAttach(server, args) {
    const { tool_id, bulk_attach_filters } = args;

    if (!tool_id) {
        throw new Error('tool_id is required for bulk_attach operation');
    }
    if (!bulk_attach_filters) {
        throw new Error('bulk_attach_filters is required for bulk_attach operation');
    }

    // Get list of all agents
    const allAgents = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get('/agents/', { headers });
            return response.data;
        },
        'Listing agents for bulk attach'
    );

    const agents = Array.isArray(allAgents) ? allAgents : allAgents.agents || [];

    // Filter agents based on criteria
    let agentsToAttach = [];

    if (bulk_attach_filters.agent_ids && bulk_attach_filters.agent_ids.length > 0) {
        agentsToAttach = agents.filter(a => bulk_attach_filters.agent_ids.includes(a.id));
    } else {
        agentsToAttach = agents.filter(agent => {
            if (bulk_attach_filters.agent_name_filter && !agent.name.includes(bulk_attach_filters.agent_name_filter)) {
                return false;
            }
            if (bulk_attach_filters.agent_tag_filter && !agent.tags?.includes(bulk_attach_filters.agent_tag_filter)) {
                return false;
            }
            return true;
        });
    }

    // Attach tool to filtered agents
    const attachResults = [];
    for (const agent of agentsToAttach) {
        try {
            await server.handleSdkCall(
                async () => {
                    const headers = server.getApiHeaders();
                    const response = await server.api.post(
                        `/agents/${encodeURIComponent(agent.id)}/tools/${encodeURIComponent(tool_id)}`,
                        {},
                        { headers }
                    );
                    return response.data;
                },
                `Attaching tool to agent ${agent.id}`
            );
            attachResults.push({ agent_id: agent.id, agent_name: agent.name, success: true });
        } catch (error) {
            attachResults.push({ agent_id: agent.id, agent_name: agent.name, success: false, error: error.message });
        }
    }

    const successCount = attachResults.filter(r => r.success).length;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'bulk_attach',
                    tool_id,
                    attached_count: successCount,
                    failed_count: attachResults.length - successCount,
                    results: attachResults,
                    message: `Attached tool to ${successCount} agents successfully`,
                }),
            },
        ],
    };
}

/**
 * Update an existing tool
 */
async function handleUpdateTool(server, args) {
    const { tool_id, tool_data } = args;

    if (!tool_id) {
        throw new Error('tool_id is required for update operation');
    }
    if (!tool_data) {
        throw new Error('tool_data is required for update operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.put(
                `/tools/${encodeURIComponent(tool_id)}`,
                tool_data,
                { headers }
            );
            return response.data;
        },
        'Updating tool'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update',
                    tool_id,
                    tool: result,
                    message: 'Tool updated successfully',
                }),
            },
        ],
    };
}

/**
 * Delete a tool
 */
async function handleDeleteTool(server, args) {
    const { tool_id } = args;

    if (!tool_id) {
        throw new Error('tool_id is required for delete operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.delete(
                `/tools/${encodeURIComponent(tool_id)}`,
                { headers }
            );
            return response.data;
        },
        'Deleting tool'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete',
                    tool_id,
                    message: 'Tool deleted successfully',
                }),
            },
        ],
    };
}

/**
 * Upsert a tool (create or update if exists)
 */
async function handleUpsertTool(server, args) {
    const { tool_data } = args;

    if (!tool_data) {
        throw new Error('tool_data is required for upsert operation');
    }
    if (!tool_data.name) {
        throw new Error('tool_data.name is required for upsert operation');
    }

    // First, try to find existing tool by name
    const headers = server.getApiHeaders();
    let existingToolId = null;

    try {
        const listResponse = await server.api.get('/tools/', { headers });
        const tools = Array.isArray(listResponse.data) ? listResponse.data : listResponse.data.tools || [];

        const existingTool = tools.find(t => t.name === tool_data.name);
        if (existingTool) {
            existingToolId = existingTool.id;
            logger.info(`Found existing tool "${tool_data.name}" with ID ${existingToolId}`);
        }
    } catch (error) {
        logger.warn('Could not check for existing tools', { error });
    }

    // Update if exists, create if not
    const result = await server.handleSdkCall(
        async () => {
            if (existingToolId) {
                // Update existing tool
                const response = await server.api.put(
                    `/tools/${encodeURIComponent(existingToolId)}`,
                    tool_data,
                    { headers }
                );
                return { ...response.data, was_updated: true };
            } else {
                // Create new tool
                const response = await server.api.post('/tools/', tool_data, { headers });
                return { ...response.data, was_updated: false };
            }
        },
        'Upserting tool'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'upsert',
                    tool_id: result.id,
                    tool: result,
                    message: result.was_updated
                        ? `Tool "${tool_data.name}" updated successfully`
                        : `Tool "${tool_data.name}" created successfully`,
                }),
            },
        ],
    };
}

/**
 * Detach a tool from an agent
 */
async function handleDetachTool(server, args) {
    const { agent_id, tool_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for detach operation');
    }
    if (!tool_id) {
        throw new Error('tool_id is required for detach operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.patch(
                `/agents/${encodeURIComponent(agent_id)}/tools/detach/${encodeURIComponent(tool_id)}`,
                {},
                { headers }
            );
            return response.data;
        },
        'Detaching tool from agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'detach',
                    tool_id,
                    detached_from_agent: agent_id,
                    message: 'Tool detached from agent successfully',
                }),
            },
        ],
    };
}

/**
 * Generate tool from natural language prompt
 */
async function handleGenerateFromPrompt(server, args) {
    const { prompt } = args;

    if (!prompt) {
        throw new Error('prompt is required for generate_from_prompt operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                '/tools/generate',
                { prompt },
                { headers }
            );
            return response.data;
        },
        'Generating tool from prompt'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'generate_from_prompt',
                    generated_code: result.source_code || result.code,
                    tool: result,
                    message: 'Tool generated successfully from prompt',
                }),
            },
        ],
    };
}

/**
 * Generate JSON schema from tool source code
 */
async function handleGenerateSchema(server, args) {
    const { source_code } = args;

    if (!source_code) {
        throw new Error('source_code is required for generate_schema operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                '/tools/schema',
                { source_code },
                { headers }
            );
            return response.data;
        },
        'Generating schema from source code'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'generate_schema',
                    generated_schema: result.schema || result,
                    message: 'Schema generated successfully',
                }),
            },
        ],
    };
}

/**
 * Run tool from source code without saving
 */
async function handleRunFromSource(server, args) {
    const { source_code, tool_args = {} } = args;

    if (!source_code) {
        throw new Error('source_code is required for run_from_source operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                '/tools/run',
                {
                    source_code,
                    arguments: tool_args,
                },
                { headers }
            );
            return response.data;
        },
        'Running tool from source code'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'run_from_source',
                    execution_result: result,
                    message: 'Tool executed successfully',
                }),
            },
        ],
    };
}

/**
 * Add base tools to the system
 */
async function handleAddBaseTools(server, _args) {
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                '/tools/add-base-tools',
                {},
                { headers }
            );
            return response.data;
        },
        'Adding base tools'
    );

    const toolsCount = result.count || result.added || (result.tools ? result.tools.length : 0);

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'add_base_tools',
                    added_tools_count: toolsCount,
                    message: `Successfully added ${toolsCount} base tools`,
                }),
            },
        ],
    };
}

/**
 * Tool definition for letta_tool_manager
 */
export const lettaToolManagerDefinition = {
    name: 'letta_tool_manager',
    description:
        'Tool Lifecycle Management Hub - Unified tool for complete tool management including CRUD operations (list, get, create, update, delete, upsert), attachment operations (attach, detach, bulk_attach), and advanced features (generate_from_prompt, generate_schema, run_from_source, add_base_tools). Provides 13 operations with discriminator-based operation routing. Replaces 13 individual tool endpoints.',
    inputSchema: toolManagerInputSchema,
};
