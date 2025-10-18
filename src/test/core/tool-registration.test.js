import { describe, it, expect, beforeEach, vi } from 'vitest';
import { LettaServer } from '../../core/server.js';
import { registerToolHandlers } from '../../tools/index.js';

// Mock dependencies
vi.mock('@modelcontextprotocol/sdk/server/index.js');
vi.mock('axios');
vi.mock('../../core/logger.js');

describe.skip('Tool Registration (LMP-84)', () => {
    let server;
    let registeredHandlers;

    beforeEach(() => {
        // Set required env vars
        process.env.LETTA_BASE_URL = 'https://test.letta.com';
        process.env.LETTA_PASSWORD = 'test-password';

        // Track registered handlers
        registeredHandlers = [];

        // Create server instance
        server = new LettaServer();

        // Mock setRequestHandler to capture registrations
        server.server.setRequestHandler = vi.fn((schema, handler) => {
            registeredHandlers.push({ schema, handler });
        });
    });

    describe('Handler Registration', () => {
        it('should register tool handlers', () => {
            registerToolHandlers(server);

            // Should register exactly 2 handlers
            expect(registeredHandlers).toHaveLength(2);
            expect(server.server.setRequestHandler).toHaveBeenCalledTimes(2);
        });

        it('should register tools/list handler first', () => {
            registerToolHandlers(server);

            // First handler should be list tools
            const firstHandler = registeredHandlers[0];
            expect(firstHandler).toBeDefined();
            expect(firstHandler.handler).toBeTypeOf('function');
        });

        it('should register tools/call handler second', () => {
            registerToolHandlers(server);

            // Second handler should be call tool
            const secondHandler = registeredHandlers[1];
            expect(secondHandler).toBeDefined();
            expect(secondHandler.handler).toBeTypeOf('function');
        });
    });

    describe('List Tools Handler', () => {
        it('should return list of available tools', async () => {
            registerToolHandlers(server);

            // Get the list tools handler (first one registered)
            const listToolsHandler = registeredHandlers[0].handler;
            const response = await listToolsHandler({});

            expect(response).toHaveProperty('tools');
            expect(Array.isArray(response.tools)).toBe(true);
            expect(response.tools.length).toBeGreaterThan(0);
        });

        it('should return tools with correct structure', async () => {
            registerToolHandlers(server);

            const listToolsHandler = registeredHandlers[0].handler;
            const response = await listToolsHandler({});

            // Check first tool structure
            const firstTool = response.tools[0];
            expect(firstTool).toHaveProperty('name');
            expect(firstTool).toHaveProperty('description');
            expect(firstTool).toHaveProperty('inputSchema');
        });

        it('should include expected tool names', async () => {
            registerToolHandlers(server);

            const listToolsHandler = registeredHandlers[0].handler;
            const response = await listToolsHandler({});

            const toolNames = response.tools.map((t) => t.name);
            expect(toolNames).toContain('list_agents');
            expect(toolNames).toContain('prompt_agent');
            expect(toolNames).toContain('list_memory_blocks');
        });
    });

    describe('Call Tool Handler', () => {
        it('should be registered as second handler', () => {
            registerToolHandlers(server);

            // Get the call tool handler (second one registered)
            const callToolHandler = registeredHandlers[1].handler;
            expect(callToolHandler).toBeTypeOf('function');
        });

        it('should throw error for unknown tool', async () => {
            registerToolHandlers(server);

            const callToolHandler = registeredHandlers[1].handler;

            const request = {
                params: {
                    name: 'unknown_tool',
                    arguments: {},
                },
            };

            await expect(callToolHandler(request)).rejects.toThrow();
        });
    });

    describe('Error Handling', () => {
        it('should handle registration on server without MCP server', () => {
            const invalidServer = { server: null };

            expect(() => {
                registerToolHandlers(invalidServer);
            }).toThrow();
        });

        it('should handle registration on undefined server', () => {
            expect(() => {
                registerToolHandlers(undefined);
            }).toThrow();
        });
    });
});
