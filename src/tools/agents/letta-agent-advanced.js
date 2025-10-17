/**
 * Tool handler for letta_agent_advanced - Advanced Agent Operations Hub
 * Provides unified interface for advanced agent capabilities
 */
import { createLogger } from '../../core/logger.js';
// eslint-disable-next-line no-unused-vars
import {
    agentAdvancedInputSchema,
    agentAdvancedOutputSchema,
} from '../schemas/agent-advanced-schemas.js';

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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.list() with pagination
        return await server.client.agents.list({
            limit: pagination.limit,
            offset: pagination.offset,
        });
    }, 'Listing agents');

    // SDK returns array directly
    const agents = Array.isArray(result) ? result : [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list',
                    agents: agents.map((agent) => ({
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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.create() method
        return await server.client.agents.create(agent_data);
    }, 'Creating agent');

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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.retrieve() method
        return await server.client.agents.retrieve(agent_id);
    }, 'Getting agent details');

    // Trim agent object to essential fields only to reduce token usage
    // Full AgentState objects contain 30+ fields including full tools array, memory blocks, etc.
    // We only need 10-12 essential fields for agent details
    const trimmedAgent = {
        id: result.id,
        name: result.name,
        description: result.description,
        system: result.system,
        llm_config: result.llm_config,
        embedding_config: result.embedding_config,
        tool_ids: result.tool_ids || [], // Just IDs, not full tool objects
        source_ids: result.source_ids || [], // Just IDs, not full source objects
        block_ids: result.block_ids || [], // Just IDs, not full block objects
        message_count: result.message_count,
        created_at: result.created_at,
        updated_at: result.updated_at,
    };

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'get',
                    agent_id,
                    agent: trimmedAgent,
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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.modify() method
        return await server.client.agents.modify(agent_id, agent_data);
    }, 'Updating agent');

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

    await server.handleSdkCall(async () => {
        // Use SDK client.agents.delete() method
        return await server.client.agents.delete(agent_id);
    }, 'Deleting agent');

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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.tools.list() method
        return await server.client.agents.tools.list(agent_id);
    }, 'Listing agent tools');

    const tools = Array.isArray(result) ? result : [];

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'list_tools',
                    agent_id,
                    tools: tools.map((tool) => ({
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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.create() method
        return await server.client.agents.messages.create(agent_id, {
            messages: message_data.messages,
        });
    }, 'Sending message to agent');

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleExportAgent(server, args) {
    const { agent_id, file_path } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for export operation');
    }

    const exportData = await server.handleSdkCall(async () => {
        // Use SDK client.agents.exportFile() method
        return await server.client.agents.exportFile(agent_id);
    }, 'Exporting agent configuration');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'export',
                    agent_id,
                    export_data: JSON.parse(exportData),
                    file_path: file_path || `agent_${agent_id}.json`,
                    message: 'Agent exported successfully',
                }),
            },
        ],
    };
}

/**
 * Import agent from JSON configuration
 * MIGRATED: Now using Letta SDK (falls back to create for JSON object)
 */
async function handleImportAgent(server, args) {
    const { agent_data } = args;

    if (!agent_data) {
        throw new Error('agent_data is required for import operation');
    }

    // SDK importFile expects a file, but we have JSON object
    // Use create() as it's functionally equivalent for JSON imports
    const result = await server.handleSdkCall(async () => {
        return await server.client.agents.create(agent_data);
    }, 'Importing agent');

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleCloneAgent(server, args) {
    const { agent_id, new_agent_name } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for clone operation');
    }
    if (!new_agent_name) {
        throw new Error('new_agent_name is required for clone operation');
    }

    // Get source agent using SDK
    const sourceAgent = await server.handleSdkCall(async () => {
        return await server.client.agents.retrieve(agent_id);
    }, 'Getting source agent for cloning');

    // Create new agent with cloned config
    const cloneData = {
        ...sourceAgent,
        name: new_agent_name,
    };
    delete cloneData.id;

    const result = await server.handleSdkCall(async () => {
        return await server.client.agents.create(cloneData);
    }, 'Creating cloned agent');

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetConfig(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_config operation');
    }

    const agent = await server.handleSdkCall(async () => {
        return await server.client.agents.retrieve(agent_id);
    }, 'Getting agent configuration');

    // Get agent's tools using SDK
    const tools = await server
        .handleSdkCall(async () => {
            return await server.client.agents.tools.list(agent_id);
        }, 'Getting agent tools')
        .catch(() => []);

    const toolsList = Array.isArray(tools) ? tools : [];

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
                        tools: toolsList.map((t) => ({ id: t.id, name: t.name })),
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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleBulkDelete(server, args) {
    const { bulk_delete_filters } = args;

    if (!bulk_delete_filters) {
        throw new Error('bulk_delete_filters is required for bulk_delete operation');
    }

    // Get list of all agents using SDK
    const allAgents = await server.handleSdkCall(async () => {
        return await server.client.agents.list();
    }, 'Listing agents for bulk delete');

    const agents = Array.isArray(allAgents) ? allAgents : [];

    // Filter agents based on criteria
    let agentsToDelete = [];

    if (bulk_delete_filters.agent_ids && bulk_delete_filters.agent_ids.length > 0) {
        agentsToDelete = agents.filter((a) => bulk_delete_filters.agent_ids.includes(a.id));
    } else {
        agentsToDelete = agents.filter((agent) => {
            if (
                bulk_delete_filters.agent_name_filter &&
                !agent.name.includes(bulk_delete_filters.agent_name_filter)
            ) {
                return false;
            }
            if (
                bulk_delete_filters.agent_tag_filter &&
                !agent.tags?.includes(bulk_delete_filters.agent_tag_filter)
            ) {
                return false;
            }
            return true;
        });
    }

    // Delete filtered agents using SDK
    const deleteResults = [];
    for (const agent of agentsToDelete) {
        try {
            await server.handleSdkCall(async () => {
                return await server.client.agents.delete(agent.id);
            }, `Deleting agent ${agent.id}`);
            deleteResults.push({ id: agent.id, name: agent.name, success: true });
        } catch (error) {
            deleteResults.push({
                id: agent.id,
                name: agent.name,
                success: false,
                error: error.message,
            });
        }
    }

    const successCount = deleteResults.filter((r) => r.success).length;

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
 * MIGRATED: Now using Letta SDK instead of axios
 */
async function handleGetContext(server, args) {
    const { agent_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for context operation');
    }

    const result = await server.handleSdkCall(async () => {
        return await server.client.agents.context.retrieve(agent_id);
    }, 'Getting agent context');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'context',
                    agent_id,
                    context: result,
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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.reset() method
        return await server.client.agents.messages.reset(agent_id);
    }, 'Resetting agent messages');

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

    await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.summarize() method
        // Note: SDK method returns void, just triggers the summarization
        return await server.client.agents.messages.summarize(agent_id, { maxMessageLength: 1000 });
    }, 'Generating conversation summary');

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
 * MIGRATED: Now using Letta SDK instead of axios
 * Note: SDK returns Stream object, not URL. MCP transport may need special handling.
 */
async function handleStream(server, args) {
    const { agent_id, message_data } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for stream operation');
    }
    if (!message_data) {
        throw new Error('message_data is required for stream operation');
    }

    // SDK createStream returns a Stream object for direct consumption
    // For MCP protocol, we may need to handle this differently
    const stream = await server.handleSdkCall(async () => {
        return await server.client.agents.messages.createStream(agent_id, {
            messages: message_data.messages,
            streamTokens: message_data.stream || true,
        });
    }, 'Starting message stream');

    // Note: Stream object cannot be easily serialized over MCP
    // This operation may need transport-specific handling
    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'stream',
                    agent_id,
                    message:
                        'Stream initiated via SDK. Note: Stream object requires special handling for MCP transport.',
                    stream_type: typeof stream,
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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.createAsync() method
        return await server.client.agents.messages.createAsync(agent_id, {
            messages: message_data.messages,
        });
    }, 'Sending async message');

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

    await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.cancel() method
        // Note: message_id here is actually run_id for canceling async runs
        return await server.client.agents.messages.cancel(agent_id, { runIds: [message_id] });
    }, 'Cancelling message');

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

    const result = await server.handleSdkCall(async () => {
        // Use SDK client.agents.messages.preview() method
        return await server.client.agents.messages.preview(agent_id, {
            messages: message_data.messages,
        });
    }, 'Generating payload preview');

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
 * MIGRATED: Now using Letta SDK instead of axios
 * Note: messages.search() is a cloud-only feature per SDK docs
 */
async function handleSearchMessages(server, args) {
    const { agent_id, search_query, filters = {} } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for search_messages operation');
    }

    try {
        const result = await server.handleSdkCall(async () => {
            // SDK messages.search() - cloud-only feature
            return await server.client.agents.messages.search({
                query: search_query,
                agentId: agent_id,
                ...filters,
            });
        }, 'Searching messages');

        const messages = Array.isArray(result) ? result : [];

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        success: true,
                        operation: 'search_messages',
                        agent_id,
                        messages: messages.map((msg) => ({
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
    } catch (error) {
        // If search fails (e.g., on self-hosted), fall back to listing messages
        const messages = await server.handleSdkCall(async () => {
            return await server.client.agents.messages.list(agent_id);
        }, 'Listing messages (search fallback)');

        const filtered = Array.isArray(messages)
            ? messages.filter((msg) => {
                  if (
                      search_query &&
                      !JSON.stringify(msg).toLowerCase().includes(search_query.toLowerCase())
                  ) {
                      return false;
                  }
                  if (filters.role && msg.role !== filters.role) {
                      return false;
                  }
                  return true;
              })
            : [];

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify({
                        success: true,
                        operation: 'search_messages',
                        agent_id,
                        messages: filtered.map((msg) => ({
                            id: msg.id,
                            role: msg.role,
                            content: msg.content || msg.text,
                            timestamp: msg.timestamp || msg.created_at,
                            agent_id: msg.agent_id,
                        })),
                        message: `Found ${filtered.length} messages (using list fallback)`,
                        note: 'Search API unavailable, used list with client-side filtering',
                    }),
                },
            ],
        };
    }
}

/**
 * Get specific message details
 * PARTIAL MIGRATION: SDK messages.list() doesn't have individual message retrieval
 * Falls back to listing and filtering by ID
 */
async function handleGetMessage(server, args) {
    const { agent_id, message_id } = args;

    if (!agent_id) {
        throw new Error('agent_id is required for get_message operation');
    }
    if (!message_id) {
        throw new Error('message_id is required for get_message operation');
    }

    // SDK doesn't have messages.retrieve(message_id), use list and filter
    const messages = await server.handleSdkCall(async () => {
        return await server.client.agents.messages.list(agent_id);
    }, 'Listing messages to find specific message');

    const result = Array.isArray(messages) ? messages.find((m) => m.id === message_id) : null;

    if (!result) {
        throw new Error(`Message ${message_id} not found`);
    }

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
                    note: 'Retrieved via list and filter (SDK has no direct message retrieve method)',
                }),
            },
        ],
    };
}

/**
 * Count total agents (not messages)
 * MIGRATED: Now using Letta SDK instead of axios
 * Note: This is agent count, not message count. For message counting, use list with pagination.
 */
async function handleCount(server, args) {
    const { agent_id } = args;

    // If agent_id provided, this was intended as message count - not supported by SDK
    // Count agents instead as that's what SDK provides
    const count = await server.handleSdkCall(async () => {
        return await server.client.agents.count();
    }, 'Counting agents');

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify({
                    success: true,
                    operation: 'count',
                    count,
                    message: `Total agents: ${count}`,
                    note: agent_id
                        ? 'Agent-specific message counting requires using list operation with filters'
                        : undefined,
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
