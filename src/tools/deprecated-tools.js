/**
 * Mapping of deprecated tools to their consolidated replacements
 * Used to add migration notices to old tool descriptions
 */

export const deprecatedToolsMigration = {
    // Agent tools → letta_agent_advanced
    'list_agents': {
        replacement: 'letta_agent_advanced',
        operation: 'list',
        note: 'Use letta_agent_advanced with operation: "list" instead'
    },
    'create_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'create',
        note: 'Use letta_agent_advanced with operation: "create" instead'
    },
    'prompt_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'send_message',
        note: 'Use letta_agent_advanced with operation: "send_message" instead'
    },
    'retrieve_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'get',
        note: 'Use letta_agent_advanced with operation: "get" instead'
    },
    'modify_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'update',
        note: 'Use letta_agent_advanced with operation: "update" instead'
    },
    'delete_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'delete',
        note: 'Use letta_agent_advanced with operation: "delete" instead'
    },
    'list_agent_tools': {
        replacement: 'letta_agent_advanced',
        operation: 'list_tools',
        note: 'Use letta_agent_advanced with operation: "list_tools" instead'
    },
    'export_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'export',
        note: 'Use letta_agent_advanced with operation: "export" instead'
    },
    'import_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'import',
        note: 'Use letta_agent_advanced with operation: "import" instead'
    },
    'clone_agent': {
        replacement: 'letta_agent_advanced',
        operation: 'clone',
        note: 'Use letta_agent_advanced with operation: "clone" instead'
    },
    'get_agent_summary': {
        replacement: 'letta_agent_advanced',
        operation: 'get_config',
        note: 'Use letta_agent_advanced with operation: "get_config" instead'
    },
    'bulk_delete_agents': {
        replacement: 'letta_agent_advanced',
        operation: 'bulk_delete',
        note: 'Use letta_agent_advanced with operation: "bulk_delete" instead'
    },

    // Memory tools → letta_memory_unified
    'list_memory_blocks': {
        replacement: 'letta_memory_unified',
        operation: 'list_blocks',
        note: 'Use letta_memory_unified with operation: "list_blocks" instead'
    },
    'read_memory_block': {
        replacement: 'letta_memory_unified',
        operation: 'get_block',
        note: 'Use letta_memory_unified with operation: "get_block" instead'
    },
    'update_memory_block': {
        replacement: 'letta_memory_unified',
        operation: 'update_block',
        note: 'Use letta_memory_unified with operation: "update_block" instead'
    },
    'attach_memory_block': {
        replacement: 'letta_memory_unified',
        operation: 'attach_block',
        note: 'Use letta_memory_unified with operation: "attach_block" instead'
    },
    'create_memory_block': {
        replacement: 'letta_memory_unified',
        operation: 'create_block',
        note: 'Use letta_memory_unified with operation: "create_block" instead'
    },

    // Passage tools → letta_memory_unified
    'list_passages': {
        replacement: 'letta_memory_unified',
        operation: 'list_passages',
        note: 'Use letta_memory_unified with operation: "list_passages" instead'
    },
    'create_passage': {
        replacement: 'letta_memory_unified',
        operation: 'create_passage',
        note: 'Use letta_memory_unified with operation: "create_passage" instead'
    },
    'modify_passage': {
        replacement: 'letta_memory_unified',
        operation: 'update_passage',
        note: 'Use letta_memory_unified with operation: "update_passage" instead'
    },
    'delete_passage': {
        replacement: 'letta_memory_unified',
        operation: 'delete_passage',
        note: 'Use letta_memory_unified with operation: "delete_passage" instead'
    },

    // Tool tools → letta_tool_manager
    'attach_tool': {
        replacement: 'letta_tool_manager',
        operation: 'attach',
        note: 'Use letta_tool_manager with operation: "attach" instead'
    },
    'upload_tool': {
        replacement: 'letta_tool_manager',
        operation: 'create',
        note: 'Use letta_tool_manager with operation: "create" instead'
    },
    'bulk_attach_tool_to_agents': {
        replacement: 'letta_tool_manager',
        operation: 'bulk_attach',
        note: 'Use letta_tool_manager with operation: "bulk_attach" instead'
    },

    // MCP tools → letta_mcp_ops
    'list_mcp_servers': {
        replacement: 'letta_mcp_ops',
        operation: 'list_servers',
        note: 'Use letta_mcp_ops with operation: "list_servers" instead'
    },
    'list_mcp_tools_by_server': {
        replacement: 'letta_mcp_ops',
        operation: 'list_tools',
        note: 'Use letta_mcp_ops with operation: "list_tools" instead'
    },
    'add_mcp_tool_to_letta': {
        replacement: 'letta_mcp_ops',
        operation: 'register_tool',
        note: 'Use letta_mcp_ops with operation: "register_tool" instead'
    },
};

/**
 * Check if a tool is deprecated
 * @param {string} toolName - Tool name to check
 * @returns {boolean}
 */
export function isDeprecated(toolName) {
    return toolName in deprecatedToolsMigration;
}

/**
 * Get deprecation notice for a tool
 * @param {string} toolName - Tool name
 * @returns {string|null} Deprecation notice or null if not deprecated
 */
export function getDeprecationNotice(toolName) {
    const migration = deprecatedToolsMigration[toolName];
    if (!migration) return null;

    return `⚠️ DEPRECATED: This tool is deprecated and will be removed in a future version. ${migration.note}`;
}

/**
 * Add deprecation notice to tool definition
 * @param {Object} toolDef - Tool definition
 * @returns {Object} Tool definition with deprecation notice
 */
export function addDeprecationNotice(toolDef) {
    const notice = getDeprecationNotice(toolDef.name);
    if (!notice) return toolDef;

    return {
        ...toolDef,
        description: `${notice}\n\n${toolDef.description}`,
    };
}
