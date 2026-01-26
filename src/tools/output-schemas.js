/**
 * Output schemas for all tools to enable structured responses
 */

export const outputSchemas = {
    // Agent Management
    create_agent: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'Unique identifier of the created agent',
            },
            capabilities: {
                type: 'array',
                items: { type: 'string' },
                description: 'List of tool names attached to the agent',
            },
        },
        required: ['agent_id'],
    },

    list_agents: {
        type: 'object',
        properties: {
            agents: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        description: { type: ['string', 'null'] },
                        created_at: { type: 'string' },
                        model: { type: 'string' },
                        embedding_model: { type: 'string' },
                    },
                    required: ['id', 'name'],
                },
            },
        },
        required: ['agents'],
    },

    prompt_agent: {
        type: 'object',
        properties: {
            messages: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        role: { type: 'string', enum: ['user', 'assistant', 'system', 'tool'] },
                        text: { type: 'string' },
                        tool_calls: {
                            type: 'array',
                            items: {
                                type: 'object',
                                properties: {
                                    id: { type: 'string' },
                                    tool_name: { type: 'string' },
                                    arguments: { type: 'object' },
                                },
                            },
                        },
                        tool_call_id: { type: 'string' },
                    },
                    required: ['role'],
                },
            },
            usage: {
                type: 'object',
                properties: {
                    completion_tokens: { type: 'integer' },
                    prompt_tokens: { type: 'integer' },
                    total_tokens: { type: 'integer' },
                    step_count: { type: 'integer' },
                },
            },
        },
        required: ['messages'],
    },

    get_agent_summary: {
        type: 'object',
        properties: {
            agent_id: { type: 'string' },
            name: { type: 'string' },
            description: { type: ['string', 'null'] },
            model: { type: 'string' },
            memory_summary: {
                type: 'object',
                properties: {
                    core_memory: {
                        type: 'object',
                        properties: {
                            persona: { type: 'string' },
                            human: { type: 'string' },
                        },
                    },
                    archival_memory_size: { type: 'integer' },
                },
            },
            tools: {
                type: 'array',
                items: { type: 'string' },
            },
            last_activity: { type: 'string' },
        },
        required: ['agent_id', 'name'],
    },

    // Memory Management
    create_memory_block: {
        type: 'object',
        properties: {
            id: { type: 'string', description: 'Unique identifier of the created memory block' },
            name: { type: 'string' },
            label: { type: 'string' },
            value: { type: 'string' },
            metadata: { type: 'object' },
        },
        required: ['id', 'name', 'label'],
    },

    list_memory_blocks: {
        type: 'object',
        properties: {
            blocks: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        label: { type: 'string' },
                        value: { type: 'string' },
                        is_template: { type: 'boolean' },
                        metadata: { type: 'object' },
                    },
                    required: ['id', 'name', 'label'],
                },
            },
            total: { type: 'integer' },
            page: { type: 'integer' },
            pageSize: { type: 'integer' },
        },
        required: ['blocks'],
    },

    // Passages
    create_passage: {
        type: 'object',
        properties: {
            id: { type: 'string', description: 'Unique identifier of the created passage' },
            text: { type: 'string' },
            embedding_model: { type: 'string' },
            created_at: { type: 'string' },
            metadata: { type: 'object' },
        },
        required: ['id', 'text'],
    },

    list_passages: {
        type: 'object',
        properties: {
            passages: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        text: { type: 'string' },
                        created_at: { type: 'string' },
                        metadata: { type: 'object' },
                    },
                    required: ['id', 'text'],
                },
            },
            total: { type: 'integer' },
            has_more: { type: 'boolean' },
        },
        required: ['passages'],
    },

    // Tools
    attach_tool: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            attached_tools: {
                type: 'array',
                items: { type: 'string' },
                description: 'List of tool IDs that were successfully attached',
            },
            errors: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        tool: { type: 'string' },
                        error: { type: 'string' },
                    },
                },
            },
        },
        required: ['success'],
    },

    list_mcp_tools_by_server: {
        type: 'object',
        properties: {
            server_name: { type: 'string' },
            tools: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        description: { type: 'string' },
                        category: { type: 'string' },
                        input_schema: { type: 'object' },
                    },
                    required: ['name'],
                },
            },
            total: { type: 'integer' },
        },
        required: ['server_name', 'tools'],
    },

    list_mcp_servers: {
        type: 'object',
        properties: {
            servers: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        url: { type: 'string' },
                        transport: { type: 'string' },
                        status: { type: 'string', enum: ['connected', 'disconnected', 'error'] },
                    },
                    required: ['name'],
                },
            },
        },
        required: ['servers'],
    },

    // Models
    list_llm_models: {
        type: 'object',
        properties: {
            models: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        provider: { type: 'string' },
                        context_window: { type: 'integer' },
                        supports_functions: { type: 'boolean' },
                    },
                    required: ['name'],
                },
            },
        },
        required: ['models'],
    },

    list_embedding_models: {
        type: 'object',
        properties: {
            models: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        provider: { type: 'string' },
                        dimensions: { type: 'integer' },
                    },
                    required: ['name'],
                },
            },
        },
        required: ['models'],
    },

    // Import/Export
    export_agent: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            file_path: { type: 'string' },
            upload_url: { type: 'string' },
            base64_content: { type: 'string' },
            agent_data: {
                type: 'object',
                properties: {
                    agent_id: { type: 'string' },
                    name: { type: 'string' },
                    version: { type: 'string' },
                    exported_at: { type: 'string' },
                },
            },
        },
        required: ['success'],
    },

    import_agent: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            agent_id: { type: 'string' },
            name: { type: 'string' },
            warnings: {
                type: 'array',
                items: { type: 'string' },
            },
        },
        required: ['success', 'agent_id'],
    },

    clone_agent: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            original_agent_id: { type: 'string' },
            new_agent_id: { type: 'string' },
            new_agent_name: { type: 'string' },
        },
        required: ['success', 'new_agent_id'],
    },

    // Bulk Operations
    bulk_attach_tool_to_agents: {
        type: 'object',
        properties: {
            tool_id: { type: 'string' },
            total_agents: { type: 'integer' },
            successful_attachments: { type: 'integer' },
            failed_attachments: { type: 'integer' },
            results: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        agent_id: { type: 'string' },
                        agent_name: { type: 'string' },
                        success: { type: 'boolean' },
                        error: { type: 'string' },
                    },
                    required: ['agent_id', 'success'],
                },
            },
        },
        required: ['tool_id', 'total_agents', 'successful_attachments'],
    },

    bulk_delete_agents: {
        type: 'object',
        properties: {
            total_agents: { type: 'integer' },
            deleted: { type: 'integer' },
            failed: { type: 'integer' },
            results: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        agent_id: { type: 'string' },
                        agent_name: { type: 'string' },
                        success: { type: 'boolean' },
                        error: { type: 'string' },
                    },
                    required: ['agent_id', 'success'],
                },
            },
        },
        required: ['total_agents', 'deleted'],
    },

    // Operations
    upload_tool: {
        type: 'object',
        properties: {
            tool_id: { type: 'string' },
            name: { type: 'string' },
            category: { type: 'string' },
            attached_to_agent: { type: 'boolean' },
        },
        required: ['tool_id', 'name'],
    },

    add_mcp_tool_to_letta: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            tool_name: { type: 'string' },
            tool_id: { type: 'string' },
            attached_to_agent: { type: 'boolean' },
            agent_id: { type: 'string' },
        },
        required: ['success', 'tool_name'],
    },

    // Simple operations that return basic success/data
    retrieve_agent: {
        type: 'object',
        properties: {
            id: { type: 'string' },
            name: { type: 'string' },
            description: { type: ['string', 'null'] },
            created_at: { type: 'string' },
            model: { type: 'string' },
            embedding_model: { type: 'string' },
            memory: { type: 'object' },
            tools: { type: 'array', items: { type: 'object' } },
        },
        required: ['id', 'name'],
    },

    modify_agent: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            agent_id: { type: 'string' },
            updated_fields: {
                type: 'array',
                items: { type: 'string' },
            },
        },
        required: ['success', 'agent_id'],
    },

    delete_agent: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            agent_id: { type: 'string' },
            message: { type: 'string' },
        },
        required: ['success'],
    },

    // Memory block operations
    read_memory_block: {
        type: 'object',
        properties: {
            id: { type: 'string' },
            name: { type: 'string' },
            label: { type: 'string' },
            value: { type: 'string' },
            metadata: { type: 'object' },
        },
        required: ['id', 'name', 'label', 'value'],
    },

    update_memory_block: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            block_id: { type: 'string' },
            updated_fields: {
                type: 'array',
                items: { type: 'string' },
            },
        },
        required: ['success', 'block_id'],
    },

    attach_memory_block: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            agent_id: { type: 'string' },
            block_id: { type: 'string' },
            label: { type: 'string' },
        },
        required: ['success'],
    },

    // Passage operations
    modify_passage: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            passage_id: { type: 'string' },
            new_text: { type: 'string' },
        },
        required: ['success', 'passage_id'],
    },

    delete_passage: {
        type: 'object',
        properties: {
            success: { type: 'boolean' },
            passage_id: { type: 'string' },
            message: { type: 'string' },
        },
        required: ['success'],
    },

    list_agent_tools: {
        type: 'object',
        properties: {
            agent_id: { type: 'string' },
            tools: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        description: { type: 'string' },
                        source: { type: 'string' },
                    },
                    required: ['id', 'name'],
                },
            },
        },
        required: ['agent_id', 'tools'],
    },
};

/**
 * Get output schema for a tool
 * @param {string} toolName - Name of the tool
 * @returns {Object|null} Output schema object or null if not found
 */
export function getOutputSchema(toolName) {
    return outputSchemas[toolName] || null;
}
