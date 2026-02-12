"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.metadata = exports.schema = void 0;
exports.default = listPrompts;
const zod_1 = require("zod");
const letta_server_js_1 = require("../core/letta-server.js");
// Define the schema for prompt parameters
exports.schema = zod_1.z.object({});
// Define prompt metadata
exports.metadata = {
    name: 'list_prompts',
    description: 'List all available prompt templates including wizards and workflows',
    inputSchema: exports.schema,
    annotations: {
        title: 'List Prompts',
        readOnlyHint: true,
        destructiveHint: false,
        idempotentHint: true,
    },
};
// Prompt implementation
function listPrompts(args) {
    return __awaiter(this, void 0, void 0, function* () {
        // Create Letta server instance
        const server = new letta_server_js_1.LettaServer();
        try {
            // Get all tools from the tools/list endpoint
            const headers = server.getApiHeaders();
            const response = yield server.api.get('/tools/', { headers });
            const tools = response.data;
            // Filter for prompt-related tools
            const promptTools = tools.filter((tool) => tool.name.includes('prompt') || tool.description.includes('prompt'));
            return [
                {
                    role: 'user',
                    content: {
                        type: 'text',
                        text: JSON.stringify({
                            total_prompts: promptTools.length,
                            prompts: promptTools.map((tool) => ({
                                name: tool.name,
                                title: tool.title || tool.name,
                                description: tool.description,
                                arguments: tool.arguments || [],
                            })),
                        }, null, 2),
                    },
                },
            ];
        }
        catch (error) {
            server.logger.error('Error listing prompts:', error);
            throw error;
        }
    });
}
