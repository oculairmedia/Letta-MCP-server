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
exports.default = createAgent;
const zod_1 = require("zod");
const letta_server_js_1 = require("../core/letta-server.js");
// Define the schema for tool parameters
exports.schema = zod_1.z.object({
    name: zod_1.z.string().describe('Name of the agent to create'),
    description: zod_1.z.string().optional().describe('Description of the agent'),
    system_prompt: zod_1.z.string().optional().describe('System prompt for the agent'),
    user_prompt: zod_1.z.string().optional().describe('User prompt for the agent'),
});
// Define tool metadata
exports.metadata = {
    name: 'create_agent',
    description: 'Create a new Letta agent',
    inputSchema: exports.schema,
};
// Tool implementation
function createAgent(args) {
    return __awaiter(this, void 0, void 0, function* () {
        const server = new letta_server_js_1.LettaServer();
        try {
            const { name, description, system_prompt, user_prompt } = args;
            // Use handleSdkCall for proper error handling
            const result = yield server.handleSdkCall(() => __awaiter(this, void 0, void 0, function* () {
                return yield server.client.agents.create({
                    name,
                    description,
                    systemPrompt: system_prompt,
                    userPrompt: user_prompt,
                });
            }), 'Creating agent');
            return {
                content: [
                    {
                        type: 'text',
                        text: JSON.stringify(result, null, 2),
                    },
                ],
            };
        }
        catch (error) {
            server.logger.error('Error creating agent:', error);
            throw error;
        }
    });
}
