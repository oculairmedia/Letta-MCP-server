// Agent-related imports
// import { handleListAgents, listAgentsToolDefinition } from './agents/list-agents.js';
// import { handlePromptAgent, promptAgentToolDefinition } from './agents/prompt-agent.js';
// import { handleListAgentTools, listAgentToolsDefinition } from './agents/list-agent-tools.js';
// import { handleCreateAgent, createAgentToolDefinition } from './agents/create-agent.js';
// import { handleRetrieveAgent, retrieveAgentDefinition } from './agents/retrieve-agent.js';
// import { handleModifyAgent, modifyAgentDefinition } from './agents/modify-agent.js';
// import { handleDeleteAgent, deleteAgentDefinition } from './agents/delete-agent.js';
// import { handleExportAgent, exportAgentDefinition } from './agents/export-agent.js';
// import { handleImportAgent, importAgentDefinition } from './agents/import-agent.js';
// import { handleCloneAgent, cloneAgentDefinition } from './agents/clone-agent.js';
// import { handleGetAgentSummary, getAgentSummaryDefinition } from './agents/get-agent-summary.js';
// import { handleBulkDeleteAgents, bulkDeleteAgentsDefinition } from './agents/bulk-delete-agents.js';

// Memory-related imports
// import {
//     handleListMemoryBlocks,
//     listMemoryBlocksToolDefinition,
// } from './memory/list-memory-blocks.js';
// import {
//     handleReadMemoryBlock,
//     readMemoryBlockToolDefinition,
// } from './memory/read-memory-block.js';
// import {
//     handleUpdateMemoryBlock,
//     updateMemoryBlockToolDefinition,
// } from './memory/update-memory-block.js';
// import {
//     handleAttachMemoryBlock,
//     attachMemoryBlockToolDefinition,
// } from './memory/attach-memory-block.js';
// import {
//     handleCreateMemoryBlock,
//     createMemoryBlockToolDefinition,
// } from './memory/create-memory-block.js';

// Passage-related imports
// import { handleListPassages, listPassagesDefinition } from './passages/list-passages.js';
// import { handleCreatePassage, createPassageDefinition } from './passages/create-passage.js';
// import { handleModifyPassage, modifyPassageDefinition } from './passages/modify-passage.js';
// import { handleDeletePassage, deletePassageDefinition } from './passages/delete-passage.js';

// Tool-related imports
// import { handleAttachTool, attachToolToolDefinition } from './tools/attach-tool.js';
// import {
//     handleBulkAttachToolToAgents,
//     bulkAttachToolDefinition,
// } from './tools/bulk-attach-tool.js';
// import { handleUploadTool, uploadToolToolDefinition } from './tools/upload-tool.js';
import { handleLettaToolManager, lettaToolManagerDefinition } from './tools/letta-tool-manager.js';
import {
    handleLettaAgentAdvanced,
    lettaAgentAdvancedDefinition,
} from './agents/letta-agent-advanced.js';
import {
    handleLettaMemoryUnified,
    lettaMemoryUnifiedDefinition,
} from './memory/letta-memory-unified.js';

// Source-related imports
import {
    handleLettaSourceManager,
    lettaSourceManagerDefinition,
} from './sources/letta-source-manager.js';

// Job-related imports
import { handleLettaJobMonitor, lettaJobMonitorDefinition } from './jobs/letta-job-monitor.js';

// File-related imports
import {
    handleLettaFileFolderOps,
    lettaFileFolderOpsDefinition,
} from './files/letta-file-folder-ops.js';

// MCP-related imports
// import {
//     handleListMcpToolsByServer,
//     listMcpToolsByServerDefinition,
// } from './mcp/list-mcp-tools-by-server.js';
// import { handleListMcpServers, listMcpServersDefinition } from './mcp/list-mcp-servers.js';
// import {
//     handleAddMcpToolToLetta,
//     addMcpToolToLettaDefinition,
// } from './mcp/add-mcp-tool-to-letta.js';
import { handleLettaMcpOps, lettaMcpOpsDefinition } from './mcp/letta-mcp-ops.js';

// Model-related imports
import { handleListLlmModels, listLlmModelsDefinition } from './models/list-llm-models.js';
import {
    handleListEmbeddingModels,
    listEmbeddingModelsDefinition,
} from './models/list-embedding-models.js';

// Prompt-related imports
import { handleListPrompts, listPromptsToolDefinition } from './prompts/list-prompts.js';
import { handleUsePrompt, usePromptToolDefinition } from './prompts/use-prompt.js';

import {
    CallToolRequestSchema,
    ListToolsRequestSchema,
    McpError,
    ErrorCode,
} from '@modelcontextprotocol/sdk/types.js';
import { enhanceAllTools } from './enhance-tools.js';

/**
 * Register all tool handlers with the server
 * @param {Object} server - The LettaServer instance (should likely be typed more specifically if possible)
 */
export function registerToolHandlers(server) {
    // Collect all tool definitions
    const allTools = [
        listLlmModelsDefinition,
        listEmbeddingModelsDefinition,
        listPromptsToolDefinition,
        usePromptToolDefinition,
        lettaMcpOpsDefinition,
        lettaToolManagerDefinition,
        lettaAgentAdvancedDefinition,
        lettaMemoryUnifiedDefinition,
        lettaSourceManagerDefinition,
        lettaJobMonitorDefinition,
        lettaFileFolderOpsDefinition,
    ];

    // Enhance all tools with output schemas and improved descriptions
    const enhancedTools = enhanceAllTools(allTools);

    // Register tool definitions
    server.server.setRequestHandler(ListToolsRequestSchema, async () => ({
        tools: enhancedTools,
    }));

    // Register tool call handler
    server.server.setRequestHandler(CallToolRequestSchema, async (request) => {
        switch (request.params.name) {
            case 'list_llm_models':
                return handleListLlmModels(server, request.params.arguments);
            case 'list_embedding_models':
                return handleListEmbeddingModels(server, request.params.arguments);
            case 'list_prompts':
                return handleListPrompts(server, request.params.arguments);
            case 'use_prompt':
                return handleUsePrompt(server, request.params.arguments);
            case 'letta_mcp_ops':
                return handleLettaMcpOps(server, request.params.arguments);
            case 'letta_tool_manager':
                return handleLettaToolManager(server, request.params.arguments);
            case 'letta_agent_advanced':
                return handleLettaAgentAdvanced(server, request.params.arguments);
            case 'letta_memory_unified':
                return handleLettaMemoryUnified(server, request.params.arguments);
            case 'letta_source_manager':
                return handleLettaSourceManager(server, request.params.arguments);
            case 'letta_job_monitor':
                return handleLettaJobMonitor(server, request.params.arguments);
            case 'letta_file_folder_ops':
                return handleLettaFileFolderOps(server, request.params.arguments);
            default:
                throw new McpError(
                    ErrorCode.MethodNotFound,
                    `Unknown tool: ${request.params.name}`,
                );
        }
    });
}

// Export all tool definitions (enhanced)
export const toolDefinitions = enhanceAllTools([
    listLlmModelsDefinition,
    listEmbeddingModelsDefinition,
    listPromptsToolDefinition,
    usePromptToolDefinition,
    lettaMcpOpsDefinition,
    lettaToolManagerDefinition,
    lettaAgentAdvancedDefinition,
    lettaMemoryUnifiedDefinition,
    lettaSourceManagerDefinition,
    lettaJobMonitorDefinition,
    lettaFileFolderOpsDefinition,
]);

// Export all tool handlers
export const toolHandlers = {
    handleListLlmModels,
    handleListEmbeddingModels,
    handleListPrompts,
    handleUsePrompt,
    handleLettaMcpOps,
    handleLettaToolManager,
    handleLettaAgentAdvanced,
    handleLettaMemoryUnified,
    handleLettaSourceManager,
    handleLettaJobMonitor,
    handleLettaFileFolderOps,
};
