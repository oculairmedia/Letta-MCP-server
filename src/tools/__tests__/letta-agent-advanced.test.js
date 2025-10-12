/**
 * Tests for letta_agent_advanced tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaAgentAdvanced, lettaAgentAdvancedDefinition } from '../agents/letta-agent-advanced.js';

// Mock server with API client
const createMockServer = () => ({
    api: {
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
    },
    getApiHeaders: vi.fn(() => ({
        'Content-Type': 'application/json',
        Authorization: 'Bearer test-token',
    })),
    handleSdkCall: vi.fn(async (fn) => await fn()),
});

describe('letta_agent_advanced', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaAgentAdvancedDefinition.name).toBe('letta_agent_advanced');
        });

        it('should have proper input schema with all operations', () => {
            expect(lettaAgentAdvancedDefinition.inputSchema.properties.operation.enum).toEqual([
                'context',
                'reset_messages',
                'summarize',
                'stream',
                'async_message',
                'cancel_message',
                'preview_payload',
                'search_messages',
                'get_message',
                'count',
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaAgentAdvancedDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('Context Operation', () => {
        it('should successfully get agent context', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    messages: [
                        { role: 'user', content: 'Hello', timestamp: '2025-10-12T00:00:00Z' },
                        { role: 'assistant', content: 'Hi!', timestamp: '2025-10-12T00:00:01Z' },
                    ],
                    token_count: 150,
                    context_window_size: 4096,
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'context',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/context',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('context');
            expect(response.context.messages).toHaveLength(2);
            expect(response.context.token_count).toBe(150);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'context',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Reset Messages Operation', () => {
        it('should successfully reset agent messages', async () => {
            mockServer.api.post.mockResolvedValue({
                data: { reset_count: 10 },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'reset_messages',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/reset',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('reset_messages');
            expect(response.reset_count).toBe(10);
        });

        it('should handle response with deleted count', async () => {
            mockServer.api.post.mockResolvedValue({
                data: { deleted: 5 },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'reset_messages',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.reset_count).toBe(5);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'reset_messages',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Summarize Operation', () => {
        it('should successfully generate conversation summary', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    summary: 'This is a conversation about implementing features.',
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'summarize',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/summarize',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('summarize');
            expect(response.summary).toBe('This is a conversation about implementing features.');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'summarize',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Stream Operation', () => {
        it('should successfully initiate message stream', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    stream_url: 'https://example.com/stream/abc123',
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'stream',
                agent_id: 'agent-123',
                message_data: {
                    messages: [{ role: 'user', content: 'Hello' }],
                },
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/stream',
                {
                    messages: [{ role: 'user', content: 'Hello' }],
                    stream: true,
                },
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('stream');
            expect(response.stream_url).toBe('https://example.com/stream/abc123');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'stream',
                    message_data: { messages: [] },
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when message_data is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'stream',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('message_data is required');
        });
    });

    describe('Async Message Operation', () => {
        it('should successfully send async message', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    job_id: 'job-abc123',
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'async_message',
                agent_id: 'agent-123',
                message_data: {
                    messages: [{ role: 'user', content: 'Process this' }],
                },
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/async',
                {
                    messages: [{ role: 'user', content: 'Process this' }],
                },
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('async_message');
            expect(response.async_job_id).toBe('job-abc123');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'async_message',
                    message_data: { messages: [] },
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when message_data is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'async_message',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('message_data is required');
        });
    });

    describe('Cancel Message Operation', () => {
        it('should successfully cancel message', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'cancel_message',
                agent_id: 'agent-123',
                message_id: 'msg-456',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/msg-456/cancel',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('cancel_message');
            expect(response.cancelled).toBe(true);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'cancel_message',
                    message_id: 'msg-456',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when message_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'cancel_message',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('message_id is required');
        });
    });

    describe('Preview Payload Operation', () => {
        it('should successfully generate payload preview', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    model: 'gpt-4',
                    messages: [{ role: 'user', content: 'Test' }],
                    temperature: 0.7,
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'preview_payload',
                agent_id: 'agent-123',
                message_data: {
                    messages: [{ role: 'user', content: 'Test' }],
                },
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/messages/preview',
                expect.any(Object),
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('preview_payload');
            expect(response.raw_payload).toBeDefined();
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'preview_payload',
                    message_data: { messages: [] },
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when message_data is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'preview_payload',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('message_data is required');
        });
    });

    describe('Search Messages Operation', () => {
        it('should successfully search messages', async () => {
            const mockMessages = [
                { id: 'msg-1', role: 'user', content: 'Hello', timestamp: '2025-10-12T00:00:00Z' },
                { id: 'msg-2', role: 'assistant', content: 'Hi!', timestamp: '2025-10-12T00:00:01Z' },
            ];

            mockServer.api.get.mockResolvedValue({ data: mockMessages });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'search_messages',
                agent_id: 'agent-123',
                search_query: 'hello',
                filters: {
                    role: 'user',
                    start_date: '2025-10-01',
                },
            });

            const callArg = mockServer.api.get.mock.calls[0][0];
            expect(callArg).toContain('query=hello');
            expect(callArg).toContain('role=user');
            expect(callArg).toContain('start_date=2025-10-01');

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('search_messages');
            expect(response.messages).toHaveLength(2);
        });

        it('should handle search without filters', async () => {
            mockServer.api.get.mockResolvedValue({ data: [] });

            await handleLettaAgentAdvanced(mockServer, {
                operation: 'search_messages',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/messages/search',
                expect.any(Object)
            );
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'search_messages',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Get Message Operation', () => {
        it('should successfully get message details', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    id: 'msg-123',
                    role: 'assistant',
                    content: 'Response text',
                    timestamp: '2025-10-12T00:00:00Z',
                    tool_calls: [{ tool_name: 'search', arguments: { query: 'test' } }],
                },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'get_message',
                agent_id: 'agent-123',
                message_id: 'msg-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/messages/msg-123',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('get_message');
            expect(response.message.id).toBe('msg-123');
            expect(response.message.tool_calls).toHaveLength(1);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'get_message',
                    message_id: 'msg-123',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when message_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'get_message',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('message_id is required');
        });
    });

    describe('Count Operation', () => {
        it('should successfully count messages', async () => {
            mockServer.api.get.mockResolvedValue({
                data: { count: 42 },
            });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'count',
                agent_id: 'agent-123',
                filters: {
                    role: 'user',
                    start_date: '2025-10-01',
                },
            });

            const callArg = mockServer.api.get.mock.calls[0][0];
            expect(callArg).toContain('role=user');
            expect(callArg).toContain('start_date=2025-10-01');

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('count');
            expect(response.count).toBe(42);
        });

        it('should handle count without filters', async () => {
            mockServer.api.get.mockResolvedValue({ data: { total: 10 } });

            const result = await handleLettaAgentAdvanced(mockServer, {
                operation: 'count',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.count).toBe(10);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'count',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'invalid',
                })
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.api.get.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'context',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.api.post.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaAgentAdvanced(mockServer, {
                    operation: 'summarize',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
