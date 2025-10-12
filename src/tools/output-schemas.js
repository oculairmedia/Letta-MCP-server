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
                        description: { type: 'string' },
                        created_at: { type: 'string' },
                        model: { type: 'string' },
                        embedding_model: { type: 'string' },
                    },
                    required: ['id', 'name'],
                },
            },
            additionalProperties: false,
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
                                    arguments: { type: 'object', additionalProperties: true },
                                },
                                additionalProperties: false,
                            },
                        },
                        tool_call_id: { type: 'string' },
                        additionalProperties: false,
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
                additionalProperties: false,
            },
            additionalProperties: false,
        },
        required: ['messages'],
    },

    get_agent_summary: {
        type: 'object',
        properties: {
            agent_id: { type: 'string' },
            name: { type: 'string' },
            description: { type: 'string' },
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
                    additionalProperties: false,
                },
            },
            tools: {
                type: 'array',
                items: { type: 'string' },
            },
            last_activity: { type: 'string' },
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
        },
        required: ['servers'],
    },

    letta_tool_manager: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            tool_id: {
                type: 'string',
                description: 'Tool ID',
            },
            tool: {
                type: 'object',
                properties: {
                    id: { type: 'string' },
                    name: { type: 'string' },
                    description: { type: 'string' },
                    source_code: { type: 'string' },
                    tags: {
                        type: 'array',
                        items: { type: 'string' },
                    },
                    json_schema: { type: 'object', additionalProperties: true },
                },
                additionalProperties: false,
                description: 'Tool details (for get operation)',
            },
            tools: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        description: { type: 'string' },
                        tags: {
                            type: 'array',
                            items: { type: 'string' },
                        },
                        source_type: { type: 'string' },
                        created_at: { type: 'string' },
                        updated_at: { type: 'string' },
                    },
                    required: ['id', 'name'],
                    additionalProperties: false,
                },
                description: 'List of tools (for list operation)',
            },
            pagination: {
                type: 'object',
                properties: {
                    total: { type: 'integer' },
                    limit: { type: 'integer' },
                    offset: { type: 'integer' },
                    has_more: { type: 'boolean' },
                },
                additionalProperties: false,
                description: 'Pagination info for list operation',
            },
            generated_code: {
                type: 'string',
                description: 'Generated source code (for generate_from_prompt)',
            },
            generated_schema: {
                type: 'object',
                additionalProperties: true,
                description: 'Generated JSON schema (for generate_schema)',
            },
            execution_result: {
                type: 'object',
                additionalProperties: true,
                description: 'Result from tool execution (for run_from_source)',
            },
            detached_from_agent: {
                type: 'string',
                description: 'Agent ID tool was detached from',
            },
            added_tools_count: {
                type: 'integer',
                description: 'Number of base tools added',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_mcp_ops: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            server_name: {
                type: 'string',
                description: 'Name of the MCP server',
            },
            server_config: {
                type: 'object',
                additionalProperties: true,
                description: 'Server configuration (for add/update/test operations)',
            },
            servers: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        name: { type: 'string' },
                        type: { type: 'string', enum: ['stdio', 'sse'] },
                        status: { type: 'string' },
                    },
                    additionalProperties: false,
                },
                description: 'List of servers (for list operations)',
            },
            test_result: {
                type: 'object',
                properties: {
                    connected: { type: 'boolean' },
                    latency_ms: { type: 'number' },
                    error: { type: 'string' },
                },
                additionalProperties: false,
                description: 'Test connection result',
            },
            execution_result: {
                type: 'object',
                additionalProperties: true,
                description: 'Result from tool execution',
            },
            oauth_url: {
                type: 'string',
                format: 'uri',
                description: 'OAuth authorization URL (for connect operation)',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_agent_advanced: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            agent_id: {
                type: 'string',
                description: 'Agent ID',
            },
            agent: {
                type: 'object',
                additionalProperties: true,
                description: 'Agent details (for read/update operations)',
            },
            agents: {
                type: 'array',
                items: { type: 'object', additionalProperties: true },
                description: 'List of agents (for search/list operations)',
            },
            health: {
                type: 'object',
                properties: {
                    status: { type: 'string' },
                    timestamp: { type: 'string' },
                    memory_usage: { type: 'object', additionalProperties: true },
                    response_time_ms: { type: 'number' },
                },
                additionalProperties: false,
                description: 'Health check result',
            },
            paused: {
                type: 'boolean',
                description: 'Whether agent was paused',
            },
            resumed: {
                type: 'boolean',
                description: 'Whether agent was resumed',
            },
            reset: {
                type: 'boolean',
                description: 'Whether agent was reset',
            },
            tags: {
                type: 'array',
                items: { type: 'string' },
                description: 'List of tags',
            },
            archived: {
                type: 'boolean',
                description: 'Whether agent was archived',
            },
            restored: {
                type: 'boolean',
                description: 'Whether agent was restored',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_memory_unified: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            agent_id: {
                type: 'string',
                description: 'Agent ID',
            },
            block_id: {
                type: 'string',
                description: 'Memory block ID',
            },
            block_label: {
                type: 'string',
                description: 'Memory block label',
            },
            core_memory: {
                type: 'object',
                properties: {
                    persona: { type: 'string' },
                    human: { type: 'string' },
                },
                additionalProperties: true,
                description: 'Core memory contents',
            },
            block: {
                type: 'object',
                properties: {
                    id: { type: 'string' },
                    label: { type: 'string' },
                    value: { type: 'string' },
                    limit: { type: 'integer' },
                },
                additionalProperties: false,
                description: 'Memory block details',
            },
            blocks: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        label: { type: 'string' },
                        value: { type: 'string' },
                        limit: { type: 'integer' },
                    },
                    additionalProperties: false,
                },
                description: 'List of memory blocks',
            },
            agents: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                    },
                    additionalProperties: false,
                },
                description: 'List of agents using block',
            },
            search_results: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        text: { type: 'string' },
                        timestamp: { type: 'string' },
                        similarity_score: { type: 'number' },
                    },
                    additionalProperties: false,
                },
                description: 'Archival memory search results',
            },
            detached: {
                type: 'boolean',
                description: 'Whether block was detached',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_source_manager: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            source_id: {
                type: 'string',
                description: 'Source ID',
            },
            source_name: {
                type: 'string',
                description: 'Source name',
            },
            agent_id: {
                type: 'string',
                description: 'Agent ID',
            },
            file_id: {
                type: 'string',
                description: 'File ID',
            },
            source: {
                type: 'object',
                additionalProperties: true,
                description: 'Source details',
            },
            sources: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        description: { type: 'string' },
                        created_at: { type: 'string' },
                        num_passages: { type: 'integer' },
                        num_files: { type: 'integer' },
                    },
                    additionalProperties: false,
                },
                description: 'List of sources',
            },
            files: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        filename: { type: 'string' },
                        size: { type: 'integer' },
                        mime_type: { type: 'string' },
                        uploaded_at: { type: 'string' },
                    },
                    additionalProperties: false,
                },
                description: 'List of files',
            },
            passages: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        text: { type: 'string' },
                        doc_id: { type: 'string' },
                        metadata: { type: 'object', additionalProperties: true },
                    },
                    additionalProperties: false,
                },
                description: 'List of passages',
            },
            metadata: {
                type: 'object',
                additionalProperties: true,
                description: 'Source metadata',
            },
            count: {
                type: 'integer',
                description: 'Total count of sources',
            },
            attached: {
                type: 'boolean',
                description: 'Whether source was attached',
            },
            detached: {
                type: 'boolean',
                description: 'Whether source was detached',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_job_monitor: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            job_id: {
                type: 'string',
                description: 'Job ID',
            },
            job: {
                type: 'object',
                properties: {
                    id: { type: 'string' },
                    status: {
                        type: 'string',
                        enum: ['pending', 'running', 'completed', 'failed', 'cancelled'],
                    },
                    job_type: { type: 'string' },
                    agent_id: { type: 'string' },
                    created_at: { type: 'string' },
                    started_at: { type: 'string' },
                    completed_at: { type: 'string' },
                    progress: {
                        type: 'object',
                        properties: {
                            current: { type: 'integer' },
                            total: { type: 'integer' },
                            percentage: { type: 'number' },
                        },
                        additionalProperties: false,
                    },
                    error: { type: 'string' },
                    result: { type: 'object', additionalProperties: true },
                },
                additionalProperties: false,
                description: 'Job details',
            },
            jobs: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        status: {
                            type: 'string',
                            enum: ['pending', 'running', 'completed', 'failed', 'cancelled'],
                        },
                        job_type: { type: 'string' },
                        agent_id: { type: 'string' },
                        created_at: { type: 'string' },
                        started_at: { type: 'string' },
                        completed_at: { type: 'string' },
                        progress: {
                            type: 'object',
                            properties: {
                                current: { type: 'integer' },
                                total: { type: 'integer' },
                                percentage: { type: 'number' },
                            },
                            additionalProperties: false,
                        },
                    },
                    additionalProperties: false,
                },
                description: 'List of jobs',
            },
            cancelled: {
                type: 'boolean',
                description: 'Whether job was cancelled',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
    },

    letta_file_folder_ops: {
        type: 'object',
        properties: {
            success: {
                type: 'boolean',
                description: 'Whether the operation succeeded',
            },
            operation: {
                type: 'string',
                description: 'Operation that was performed',
            },
            agent_id: {
                type: 'string',
                description: 'Agent ID',
            },
            file_id: {
                type: 'string',
                description: 'File ID',
            },
            folder_id: {
                type: 'string',
                description: 'Folder ID',
            },
            files: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        filename: { type: 'string' },
                        size: { type: 'integer' },
                        mime_type: { type: 'string' },
                        is_open: { type: 'boolean' },
                        opened_at: { type: 'string' },
                    },
                    additionalProperties: false,
                },
                description: 'List of files',
            },
            folders: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                        path: { type: 'string' },
                        file_count: { type: 'integer' },
                        agent_count: { type: 'integer' },
                    },
                    additionalProperties: false,
                },
                description: 'List of folders',
            },
            agents: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        id: { type: 'string' },
                        name: { type: 'string' },
                    },
                    additionalProperties: false,
                },
                description: 'List of agents in folder',
            },
            opened: {
                type: 'boolean',
                description: 'Whether file was opened',
            },
            closed: {
                type: 'boolean',
                description: 'Whether file was closed',
            },
            closed_count: {
                type: 'integer',
                description: 'Number of files closed',
            },
            attached: {
                type: 'boolean',
                description: 'Whether folder was attached',
            },
            detached: {
                type: 'boolean',
                description: 'Whether folder was detached',
            },
            message: {
                type: 'string',
                description: 'Status or error message',
            },
        },
        required: ['success', 'operation'],
        additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            additionalProperties: false,
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
            description: { type: 'string' },
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
        additionalProperties: false,
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
            additionalProperties: false,
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
