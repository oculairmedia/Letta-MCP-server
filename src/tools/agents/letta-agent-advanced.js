/**
 * Tool handler for letta_agent_advanced - Advanced Agent Operations Hub
 * Provides unified interface for advanced agent capabilities
 */
import { createLogger } from '../../core/logger.js';
// eslint-disable-next-line no-unused-vars
import { agentAdvancedInputSchema, agentAdvancedOutputSchema } from '../schemas/agent-advanced-schemas.js';

const logger = createLogger('letta_agent_advanced');

/**
 * Handle letta_agent_advanced tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following agentAdvancedInputSchema
 * @returns {Promise<Object>} Tool response following agentAdvancedOutputSchema
 */
export async function handleLettaAgentAdvanced(server, args) {
    const { operation } = args;

    logger.info(`Executing agent operation: ${operation}`, { args });

    try {
        switch (operation) {
            // CRUD operations
            case 'list':
                return await handleListAgents(server, args);
            case 'create':
                return await handleCreateAgent(server, args);
            case 'get':
                return await handleGetAgent(server, args);
            case 'update':
                return await handleUpdateAgent(server, args);
            case 'delete':
                return await handleDeleteAgent(server, args);
            case 'list_tools':
                return await handleListAgentTools(server, args);
            case 'send_message':
                return await handleSendMessage(server, args);
            case 'export':
                return await handleExportAgent(server, args);
            case 'import':
                return await handleImportAgent(server, args);
            case 'clone':
                return await handleCloneAgent(server, args);
            case 'get_config':
                return await handleGetConfig(server, args);
            case 'bulk_delete':
                return await handleBulkDelete(server, args);
            // Advanced operations
            case 'context':
                return await handleGetContext(server, args);
            case 'reset_messages':
                return await handleResetMessages(server, args);
            case 'summarize':
                return await handleSummarize(server, args);
            case 'stream':
                return await handleStream(server, args);
            case 'async_message':
                return await handleAsyncMessage(server, args);
            case 'cancel_message':
                return await handleCancelMessage(server, args);
            case 'preview_payload':
                return await handlePreviewPayload(server, args);
            case 'search_messages':
                return await handleSearchMessages(server, args);
            case 'get_message':
                return await handleGetMessage(server, args);
            case 'count':
                return await handleCount(server, args);
            default:
                throw new Error(`Unknown operation: ${operation}`);
        }
    } catch (error) {
        logger.error(`Agent operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * List all agents with pagination
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListAgents(server, args) {
    const { pagination = {} } = args;

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.list() with pagination
            return await server.client.agents.list({
                limit: pagination.limit,
                offset: pagination.offset,
            });
        },
        'Listing agents'
    );

    // SDK returns array directly
    const agents = Array.isArray(result) ? result : [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list',
                    agents: agents.map(agent => ({
                        id: agent.id,
                        name: agent.name,
                        description: agent.description,
                        created_at: agent.created_at,
                        updated_at: agent.updated_at,
                    })),
                    total: agents.length,
                    message: `Found ${agents.length} agents`,
                }),
            },
        ],
    };
}

/**
 * Create a new agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCreateAgent(server, args) {
    const { agent_data } = args;

    if (!agent_data) {
        throw new Error('agent_data is required for create operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.create() method
            return await server.client.agents.create(agent_data);
        },
        'Creating agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'create',
                    agent_id: result.id,
                    agent: {
                        id: result.id,
                        name: result.name,
                        description: result.description,
                        created_at: result.created_at,
                    },
                    message: 'Agent created successfully',
                }),
            },
        ],
    };
}

/**
 * Get agent details
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetAgent(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.retrieve() method
            return await server.client.agents.retrieve(agent_id);
        },
        'Getting agent details'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get',
                    agent_id,
                    agent: result,
                    message: 'Agent retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Update an existing agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleUpdateAgent(server, args) {
    const { agent_id, agent_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for update operation');
    }
    if (!agent_data) {
        throw new Error('agent_data is required for update operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.modify() method
            return await server.client.agents.modify(agent_id, agent_data);
        },
        'Updating agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'update',
                    agent_id,
                    agent: result,
                    message: 'Agent updated successfully',
                }),
            },
        ],
    };
}

/**
 * Delete an agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleDeleteAgent(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for delete operation');
    }

    await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.delete() method
            return await server.client.agents.delete(agent_id);
        },
        'Deleting agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'delete',
                    agent_id,
                    deleted: true,
                    message: 'Agent deleted successfully',
                }),
            },
        ],
    };
}

/**
 * List agent's tools
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleListAgentTools(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for list_tools operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.tools.list() method
            return await server.client.agents.tools.list(agent_id);
        },
        'Listing agent tools'
    );

    const tools = Array.isArray(result) ? result : [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_tools',
                    agent_id,
                    tools: tools.map(tool => ({
                        id: tool.id,
                        name: tool.name,
                        description: tool.description,
                        tags: tool.tags || [],
                    })),
                    message: `Found ${tools.length} tools`,
                }),
            },
        ],
    };
}

/**
 * Send message to agent
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleSendMessage(server, args) {
    const { agent_id, message_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for send_message operation');
    }
    if (!message_data) {
        throw new Error('message_data is required for send_message operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.create() method
            return await server.client.agents.messages.create(agent_id, { messages: message_data.messages });
        },
        'Sending message to agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'send_message',
                    agent_id,
                    response: result,
                    message: 'Message sent successfully',
                }),
            },
        ],
    };
}

/**
 * Export agent configuration to JSON
 */
async function handleExportAgent(server, args) {
    const { agent_id, file_path } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for export operation');
    }

    // First get the agent details
    const agent = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}`, { headers });
            return response.data;
        },
        'Getting agent for export'
    );

    // Export to JSON format
    const exportData = {
        agent,
        exported_at: new Date().toISOString(),
        version: '1.0',
    };

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'export',
                    agent_id,
                    export_data: exportData,
                    file_path: file_path || `agent_${agent_id}.json`,
                    message: 'Agent exported successfully',
                }),
            },
        ],
    };
}

/**
 * Import agent from JSON configuration
 */
async function handleImportAgent(server, args) {
    const { agent_data } = args;

    if (!agent_data) {
        throw new Error('agent_data is required for import operation');
    }

    // Create new agent with imported data
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post('/agents/', agent_data, { headers });
            return response.data;
        },
        'Importing agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'import',
                    agent_id: result.id,
                    agent: result,
                    message: 'Agent imported successfully',
                }),
            },
        ],
    };
}

/**
 * Clone an existing agent
 */
async function handleCloneAgent(server, args) {
    const { agent_id, new_agent_name } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for clone operation');
    }
    if (!new_agent_name) {
        throw new Error('new_agent_name is required for clone operation');
    }

    // Get source agent
    const sourceAgent = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}`, { headers });
            return response.data;
        },
        'Getting source agent for cloning'
    );

    // Create new agent with cloned config
    const cloneData = {
        ...sourceAgent,
        name: new_agent_name,
        id: undefined, // Remove ID to create new agent
    };

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post('/agents/', cloneData, { headers });
            return response.data;
        },
        'Creating cloned agent'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'clone',
                    source_agent_id: agent_id,
                    new_agent_id: result.id,
                    new_agent_name,
                    message: 'Agent cloned successfully',
                }),
            },
        ],
    };
}

/**
 * Get agent configuration summary
 */
async function handleGetConfig(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_config operation');
    }

    const agent = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}`, { headers });
            return response.data;
        },
        'Getting agent configuration'
    );

    // Get agent's tools
    const tools = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/agents/${encodeURIComponent(agent_id)}/tools`, { headers });
            return response.data;
        },
        'Getting agent tools'
    ).catch(() => []);

    const toolsList = Array.isArray(tools) ? tools : tools.tools || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_config',
                    agent_id,
                    config: {
                        name: agent.name,
                        description: agent.description,
                        system: agent.system,
                        llm_config: agent.llm_config,
                        embedding_config: agent.embedding_config,
                        tools: toolsList.map(t => ({ id: t.id, name: t.name })),
                        created_at: agent.created_at,
                        updated_at: agent.updated_at,
                    },
                    message: 'Agent configuration retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Bulk delete agents by filters
 */
async function handleBulkDelete(server, args) {
    const { bulk_delete_filters } = args;

    if (!bulk_delete_filters) {
        throw new Error('bulk_delete_filters is required for bulk_delete operation');
    }

    // Get list of all agents
    const allAgents = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get('/agents/', { headers });
            return response.data;
        },
        'Listing agents for bulk delete'
    );

    const agents = Array.isArray(allAgents) ? allAgents : allAgents.agents || [];

    // Filter agents based on criteria
    let agentsToDelete = [];

    if (bulk_delete_filters.agent_ids && bulk_delete_filters.agent_ids.length > 0) {
        agentsToDelete = agents.filter(a => bulk_delete_filters.agent_ids.includes(a.id));
    } else {
        agentsToDelete = agents.filter(agent => {
            if (bulk_delete_filters.agent_name_filter && !agent.name.includes(bulk_delete_filters.agent_name_filter)) {
                return false;
            }
            if (bulk_delete_filters.agent_tag_filter && !agent.tags?.includes(bulk_delete_filters.agent_tag_filter)) {
                return false;
            }
            return true;
        });
    }

    // Delete filtered agents
    const deleteResults = [];
    for (const agent of agentsToDelete) {
        try {
            await server.handleSdkCall(
                async () => {
                    const headers = server.getApiHeaders();
                    const response = await server.api.delete(`/agents/${encodeURIComponent(agent.id)}`, { headers });
                    return response.data;
                },
                `Deleting agent ${agent.id}`
            );
            deleteResults.push({ id: agent.id, name: agent.name, success: true });
        } catch (error) {
            deleteResults.push({ id: agent.id, name: agent.name, success: false, error: error.message });
        }
    }

    const successCount = deleteResults.filter(r => r.success).length;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'bulk_delete',
                    deleted_count: successCount,
                    failed_count: deleteResults.length - successCount,
                    results: deleteResults,
                    message: `Deleted ${successCount} agents successfully`,
                }),
            },
        ],
    };
}

/**
 * Get agent's context window with token count
 */
async function handleGetContext(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for context operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/context`,
                { headers }
            );
            return response.data;
        },
        'Getting agent context'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'context',
                    agent_id,
                    context: {
                        messages: result.messages || [],
                        token_count: result.token_count || result.tokens || 0,
                        context_window_size: result.context_window_size || result.max_tokens || 0,
                    },
                    message: 'Context retrieved successfully',
                }),
            },
        ],
    };
}

/**
 * Reset/clear agent's message history
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleResetMessages(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for reset_messages operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.reset() method
            return await server.client.agents.messages.reset(agent_id);
        },
        'Resetting agent messages'
    );

    // SDK returns AgentState, extract message count if available
    const resetCount = result.message_count || result.messages?.length || 0;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'reset_messages',
                    agent_id,
                    reset_count: resetCount,
                    message: `Messages reset successfully`,
                }),
            },
        ],
    };
}

/**
 * Generate conversation summary
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleSummarize(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for summarize operation');
    }

    await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.summarize() method
            // Note: SDK method returns void, just triggers the summarization
            return await server.client.agents.messages.summarize(agent_id, { maxMessageLength: 1000 });
        },
        'Generating conversation summary'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'summarize',
                    agent_id,
                    message: 'Summary generated successfully',
                }),
            },
        ],
    };
}

/**
 * Stream agent responses via SSE
 */
async function handleStream(server, args) {
    const { agent_id, message_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for stream operation');
    }
    if (!message_data) {
        throw new Error('message_data is required for stream operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/agents/${encodeURIComponent(agent_id)}/messages/stream`,
                {
                    messages: message_data.messages,
                    stream: true,
                },
                { headers }
            );
            return response.data;
        },
        'Starting message stream'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'stream',
                    agent_id,
                    stream_url: result.stream_url || result.url,
                    message: 'Stream initiated. Connect to stream_url to receive events.',
                }),
            },
        ],
    };
}

/**
 * Send message asynchronously with job tracking
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleAsyncMessage(server, args) {
    const { agent_id, message_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for async_message operation');
    }
    if (!message_data) {
        throw new Error('message_data is required for async_message operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.createAsync() method
            return await server.client.agents.messages.createAsync(agent_id, {
                messages: message_data.messages,
            });
        },
        'Sending async message'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'async_message',
                    agent_id,
                    async_job_id: result.id,
                    message: 'Async message sent. Use job_id to track progress.',
                }),
            },
        ],
    };
}

/**
 * Cancel an async message
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCancelMessage(server, args) {
    const { agent_id, message_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for cancel_message operation');
    }
    if (!message_id) {
        throw new Error('message_id is required for cancel_message operation');
    }

    await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.cancel() method
            // Note: message_id here is actually run_id for canceling async runs
            return await server.client.agents.messages.cancel(agent_id, { runIds: [message_id] });
        },
        'Cancelling message'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'cancel_message',
                    agent_id,
                    message_id,
                    cancelled: true,
                    message: 'Message cancelled successfully',
                }),
            },
        ],
    };
}

/**
 * Preview API payload before sending
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handlePreviewPayload(server, args) {
    const { agent_id, message_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for preview_payload operation');
    }
    if (!message_data) {
        throw new Error('message_data is required for preview_payload operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            // Use SDK client.agents.messages.preview() method
            return await server.client.agents.messages.preview(agent_id, {
                messages: message_data.messages,
            });
        },
        'Generating payload preview'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'preview_payload',
                    agent_id,
                    raw_payload: result,
                    message: 'Payload preview generated',
                }),
            },
        ],
    };
}

/**
 * Search messages by query and filters
 */
async function handleSearchMessages(server, args) {
    const { agent_id, search_query, filters = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for search_messages operation');
    }

    const queryParams = new URLSearchParams();
    if (search_query) queryParams.append('query', search_query);
    if (filters.start_date) queryParams.append('start_date', filters.start_date);
    if (filters.end_date) queryParams.append('end_date', filters.end_date);
    if (filters.role) queryParams.append('role', filters.role);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/agents/${encodeURIComponent(agent_id)}/messages/search${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Searching messages'
    );

    const messages = Array.isArray(result) ? result : result.messages || [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'search_messages',
                    agent_id,
                    messages: messages.map(msg => ({
                        id: msg.id,
                        role: msg.role,
                        content: msg.content || msg.text,
                        timestamp: msg.timestamp || msg.created_at,
                        agent_id: msg.agent_id,
                    })),
                    message: `Found ${messages.length} messages`,
                }),
            },
        ],
    };
}

/**
 * Get specific message details
 */
async function handleGetMessage(server, args) {
    const { agent_id, message_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_message operation');
    }
    if (!message_id) {
        throw new Error('message_id is required for get_message operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(
                `/agents/${encodeURIComponent(agent_id)}/messages/${encodeURIComponent(message_id)}`,
                { headers }
            );
            return response.data;
        },
        'Getting message details'
    );

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get_message',
                    agent_id,
                    message_id,
                    message: {
                        id: result.id,
                        role: result.role,
                        content: result.content || result.text,
                        timestamp: result.timestamp || result.created_at,
                        tool_calls: result.tool_calls || [],
                    },
                }),
            },
        ],
    };
}

/**
 * Count messages matching filters
 */
async function handleCount(server, args) {
    const { agent_id, filters = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for count operation');
    }

    const queryParams = new URLSearchParams();
    if (filters.start_date) queryParams.append('start_date', filters.start_date);
    if (filters.end_date) queryParams.append('end_date', filters.end_date);
    if (filters.role) queryParams.append('role', filters.role);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/agents/${encodeURIComponent(agent_id)}/messages/count${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Counting messages'
    );

    const count = result.count || result.total || 0;

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'count',
                    agent_id,
                    count,
                    message: `Found ${count} messages matching filters`,
                }),
            },
        ],
    };
}

/**
 * Tool definition for letta_agent_advanced
 */
export const lettaAgentAdvancedDefinition = {
    name: 'letta_agent_advanced',
    description:
        'Agent Operations Hub - Unified tool for complete agent lifecycle management including CRUD operations (list, create, get, update, delete, list_tools, send_message, export, import, clone, get_config, bulk_delete) and advanced capabilities (context management, message streaming, async messaging, conversation summaries, message search). Replaces 22 individual agent endpoints with discriminator-based operation routing.',
    inputSchema: agentAdvancedInputSchema,
};
