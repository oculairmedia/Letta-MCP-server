/**
 * Tests for letta_mcp_ops tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaMcpOps, lettaMcpOpsDefinition } from '../mcp/letta-mcp-ops.js';

// Mock server with API client
const createMockServer = () => ({
    api: {
        put: vi.fn(),
        patch: vi.fn(),
        delete: vi.fn(),
        post: vi.fn(),
        get: vi.fn(),
    },
    getApiHeaders: vi.fn(() => ({
        'Content-Type': 'application/json',
        Authorization: 'Bearer test-token',
    })),
    handleSdkCall: vi.fn(async (fn) => await fn()),
});

describe('letta_mcp_ops', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaMcpOpsDefinition.name).toBe('letta_mcp_ops');
        });

        it('should have proper input schema with all operations', () => {
            expect(lettaMcpOpsDefinition.inputSchema.properties.operation.enum).toEqual([
                'add',
                'update',
                'delete',
                'test',
                'connect',
                'resync',
                'execute',
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaMcpOpsDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('Add Operation', () => {
        it('should successfully add a stdio MCP server', async () => {
            const serverConfig = {
                type: 'stdio',
                command: 'node',
                args: ['server.js'],
                env: { NODE_ENV: 'production' },
            };

            mockServer.api.put.mockResolvedValue({
                data: { ...serverConfig, id: 'server-123' },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'add',
                server_config: serverConfig,
            });

            expect(mockServer.api.put).toHaveBeenCalledWith(
                '/tools/mcp/servers',
                serverConfig,
                expect.objectContaining({ headers: expect.any(Object) })
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('add');
            expect(response.server_config.id).toBe('server-123');
        });

        it('should successfully add an SSE MCP server', async () => {
            const serverConfig = {
                type: 'sse',
                url: 'https://example.com/mcp',
                headers: { 'X-API-Key': 'secret' },
            };

            mockServer.api.put.mockResolvedValue({
                data: { ...serverConfig, id: 'server-456' },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'add',
                server_config: serverConfig,
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.server_config.type).toBe('sse');
        });

        it('should throw error when server_config is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'add',
                })
            ).rejects.toThrow('server_config is required');
        });
    });

    describe('Update Operation', () => {
        it('should successfully update an MCP server', async () => {
            const serverConfig = {
                type: 'stdio',
                command: 'python',
                args: ['-m', 'server'],
            };

            mockServer.api.patch.mockResolvedValue({
                data: serverConfig,
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'update',
                server_name: 'my-server',
                server_config: serverConfig,
            });

            expect(mockServer.api.patch).toHaveBeenCalledWith(
                '/tools/mcp/servers/my-server',
                serverConfig,
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('update');
            expect(response.server_name).toBe('my-server');
        });

        it('should throw error when server_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'update',
                    server_config: { type: 'stdio', command: 'node' },
                })
            ).rejects.toThrow('server_name is required');
        });

        it('should throw error when server_config is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'update',
                    server_name: 'my-server',
                })
            ).rejects.toThrow('server_config is required');
        });

        it('should handle special characters in server name', async () => {
            mockServer.api.patch.mockResolvedValue({ data: {} });

            await handleLettaMcpOps(mockServer, {
                operation: 'update',
                server_name: 'my-server@v2',
                server_config: { type: 'stdio', command: 'node' },
            });

            expect(mockServer.api.patch).toHaveBeenCalledWith(
                '/tools/mcp/servers/my-server%40v2',
                expect.any(Object),
                expect.any(Object)
            );
        });
    });

    describe('Delete Operation', () => {
        it('should successfully delete an MCP server', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'delete',
                server_name: 'old-server',
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                '/tools/mcp/servers/old-server',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('delete');
            expect(response.server_name).toBe('old-server');
        });

        it('should throw error when server_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'delete',
                })
            ).rejects.toThrow('server_name is required');
        });
    });

    describe('Test Operation', () => {
        it('should successfully test an MCP server connection', async () => {
            const serverConfig = {
                type: 'sse',
                url: 'https://example.com/mcp',
            };

            mockServer.api.post.mockResolvedValue({
                data: { status: 'connected', version: '1.0.0' },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'test',
                server_config: serverConfig,
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/tools/mcp/servers/test',
                serverConfig,
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('test');
            expect(response.test_result.connected).toBe(true);
            expect(response.test_result.latency_ms).toBeGreaterThanOrEqual(0);
        });

        it('should throw error when server_config is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'test',
                })
            ).rejects.toThrow('server_config is required');
        });
    });

    describe('Connect Operation', () => {
        it('should successfully initiate OAuth connection', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    authorization_url: 'https://oauth.example.com/authorize?client_id=123',
                    session_id: 'session-abc',
                },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'connect',
                server_name: 'oauth-server',
                oauth_config: {
                    callback_url: 'https://myapp.com/callback',
                    session_id: 'session-abc',
                    scopes: ['read', 'write'],
                },
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('connect');
            expect(response.oauth_url).toBeTruthy();
        });

        it('should throw error when server_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'connect',
                    oauth_config: { callback_url: 'http://test', session_id: '123' },
                })
            ).rejects.toThrow('server_name is required');
        });

        it('should throw error when oauth_config is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'connect',
                    server_name: 'oauth-server',
                })
            ).rejects.toThrow('oauth_config is required');
        });
    });

    describe('Resync Operation', () => {
        it('should successfully resync MCP server tools', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    tools: ['tool1', 'tool2', 'tool3'],
                    synced_at: '2025-10-12T00:00:00Z',
                },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'resync',
                server_name: 'my-server',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/tools/mcp/servers/my-server/resync',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('resync');
            expect(response.servers).toBeTruthy();
        });

        it('should throw error when server_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'resync',
                })
            ).rejects.toThrow('server_name is required');
        });
    });

    describe('Execute Operation', () => {
        it('should successfully execute a tool with arguments', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    result: 'success',
                    output: 'Tool executed successfully',
                },
            });

            const result = await handleLettaMcpOps(mockServer, {
                operation: 'execute',
                server_name: 'my-server',
                tool_name: 'my-tool',
                tool_args: {
                    param1: 'value1',
                    param2: 42,
                },
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/tools/mcp/servers/my-server/tools/my-tool/execute',
                { param1: 'value1', param2: 42 },
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('execute');
            expect(response.execution_result.result).toBe('success');
        });

        it('should execute tool without arguments', async () => {
            mockServer.api.post.mockResolvedValue({
                data: { result: 'success' },
            });

            await handleLettaMcpOps(mockServer, {
                operation: 'execute',
                server_name: 'my-server',
                tool_name: 'my-tool',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                expect.any(String),
                {},
                expect.any(Object)
            );
        });

        it('should throw error when server_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'execute',
                    tool_name: 'my-tool',
                })
            ).rejects.toThrow('server_name is required');
        });

        it('should throw error when tool_name is missing', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'execute',
                    server_name: 'my-server',
                })
            ).rejects.toThrow('tool_name is required');
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'invalid',
                })
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.api.put.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'add',
                    server_config: { type: 'stdio', command: 'node' },
                })
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.api.post.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaMcpOps(mockServer, {
                    operation: 'test',
                    server_config: { type: 'sse', url: 'https://slow.example.com' },
                })
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
