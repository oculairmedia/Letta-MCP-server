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
exports.default = listAgents;
const zod_1 = require("zod");
const letta_server_js_1 = require("../core/letta-server.js");
// Define the schema for tool parameters
exports.schema = zod_1.z.object({
    limit: zod_1.z.number().optional().describe('Maximum number of agents to return'),
    offset: zod_1.z.number().optional().describe('Offset for pagination'),
});
// Define tool metadata
exports.metadata = {
    name: 'list_agents',
    description: 'List all Letta agents with pagination support',
    inputSchema: exports.schema,
};
// Tool implementation
function listAgents(args) {
    return __awaiter(this, void 0, void 0, function* () {
        const server = new letta_server_js_1.LettaServer();
        try {
            const { limit = 20, offset = 0 } = args;
            // Use handleSdkCall for proper error handling
            const result = yield server.handleSdkCall(() => __awaiter(this, void 0, void 0, function* () {
                return yield server.client.agents.list({
                    limit,
                    offset,
                });
            }), 'Listing agents');
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
            server.logger.error('Error listing agents:', error);
            throw error;
        }
    });
}
