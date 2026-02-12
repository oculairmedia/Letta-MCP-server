"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const xmcp_1 = require("xmcp");
const dotenv_1 = __importDefault(require("dotenv"));
// Load environment variables
dotenv_1.default.config();
const app = (0, express_1.default)();
const port = process.env.PORT || 3001;
// Add middleware for JSON parsing
app.use(express_1.default.json({ limit: '10mb' }));
// Set up xmcp handler endpoints
app.use('/mcp', xmcp_1.xmcpHandler);
// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'healthy',
        service: 'letta-mcp-server',
        framework: 'xmcp',
        timestamp: new Date().toISOString(),
    });
});
app.listen(port, '0.0.0.0', () => {
    console.log(`🚀 Letta MCP Server running on port ${port}`);
    console.log(`📡 MCP endpoint: http://localhost:${port}/mcp`);
    console.log(`❤️  Health check: http://localhost:${port}/health`);
});
