import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleListMessages,
    listMessagesDefinition,
} from '../../../tools/messages/list-messages.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe('List Messages', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(listMessagesDefinition.name).toBe('list_messages');
            expect(listMessagesDefinition.description).toContain(
                "Retrieve messages from an agent's conversation history",
            );
            expect(listMessagesDefinition.inputSchema.required).toEqual(['agent_id']);
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('agent_id');
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('limit');
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('order');
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('before');
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('after');
            expect(listMessagesDefinition.inputSchema.properties).toHaveProperty('group_id');
            expect(listMessagesDefinition.inputSchema.properties.order.enum).toEqual([
                'asc',
                'desc',
            ]);
        });
    });

    describe('Functionality Tests', () => {
        it('should list messages successfully', async () => {
            const agentId = 'agent-123';
            const mockMessages = [
                {
                    id: 'msg-1',
                    message_type: 'user_message',
                    date: '2024-01-01T00:00:00Z',
                    role: 'user',
                    text: 'Hello agent',
                },
                {
                    id: 'msg-2',
                    message_type: 'assistant_message',
                    date: '2024-01-01T00:00:01Z',
                    role: 'assistant',
                    text: 'Hello! How can I help you?',
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            // Verify API call
            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    headers: expect.any(Object),
                    params: {},
                }),
            );

            // Verify response
            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(2);
            expect(data.count).toBe(2);
            expect(data.messages[0].id).toBe('msg-1');
            expect(data.messages[0].role).toBe('user');
            expect(data.messages[1].role).toBe('assistant');
        });

        it('should handle limit parameter', async () => {
            const agentId = 'agent-limit';
            const limit = 10;
            const mockMessages = Array.from({ length: 10 }, (_, i) => ({
                id: `msg-${i}`,
                message_type: 'user_message',
                text: `Message ${i}`,
            }));

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                limit: limit,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { limit: limit },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(10);
            expect(data.count).toBe(10);
        });

        it('should handle order parameter asc', async () => {
            const agentId = 'agent-asc';
            const mockMessages = [
                { id: 'msg-old', date: '2024-01-01T00:00:00Z', text: 'Oldest' },
                { id: 'msg-new', date: '2024-01-02T00:00:00Z', text: 'Newest' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                order: 'asc',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { order: 'asc' },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages[0].id).toBe('msg-old');
        });

        it('should handle order parameter desc', async () => {
            const agentId = 'agent-desc';
            const mockMessages = [
                { id: 'msg-new', date: '2024-01-02T00:00:00Z', text: 'Newest' },
                { id: 'msg-old', date: '2024-01-01T00:00:00Z', text: 'Oldest' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                order: 'desc',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { order: 'desc' },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages[0].id).toBe('msg-new');
        });

        it('should handle before parameter', async () => {
            const agentId = 'agent-before';
            const beforeId = 'msg-50';
            const mockMessages = [
                { id: 'msg-48', text: 'Before msg 50' },
                { id: 'msg-49', text: 'Before msg 50' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                before: beforeId,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { before: beforeId },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(2);
        });

        it('should handle after parameter', async () => {
            const agentId = 'agent-after';
            const afterId = 'msg-100';
            const mockMessages = [
                { id: 'msg-101', text: 'After msg 100' },
                { id: 'msg-102', text: 'After msg 101' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                after: afterId,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { after: afterId },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages[0].id).toBe('msg-101');
        });

        it('should handle group_id parameter', async () => {
            const agentId = 'agent-group';
            const groupId = 'group-123';
            const mockMessages = [
                { id: 'msg-1', group_id: groupId, text: 'Message in group' },
                { id: 'msg-2', group_id: groupId, text: 'Another message in group' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                group_id: groupId,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: { group_id: groupId },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(2);
        });

        it('should handle all parameters combined', async () => {
            const agentId = 'agent-all';
            const params = {
                limit: 5,
                order: 'desc',
                before: 'msg-20',
                after: 'msg-10',
                group_id: 'group-456',
            };

            const mockMessages = [{ id: 'msg-15', text: 'Message in range' }];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
                ...params,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: params,
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(1);
        });

        it('should handle empty message list', async () => {
            const agentId = 'agent-empty';
            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            const data = expectValidToolResponse(result);
            expect(data.messages).toEqual([]);
            expect(data.count).toBe(0);
        });

        it('should handle messages with tool calls', async () => {
            const agentId = 'agent-tools';
            const mockMessages = [
                {
                    id: 'msg-1',
                    message_type: 'tool_call',
                    role: 'assistant',
                    tool_calls: [
                        {
                            function: { name: 'search', arguments: '{"query": "test"}' },
                        },
                    ],
                },
                {
                    id: 'msg-2',
                    message_type: 'tool_return',
                    role: 'tool',
                    tool_call_id: 'call-123',
                    content: 'Search results here',
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            const data = expectValidToolResponse(result);
            expect(data.messages).toHaveLength(2);
            expect(data.messages[0].message_type).toBe('tool_call');
            expect(data.messages[1].message_type).toBe('tool_return');
        });

        it('should handle special characters in agent ID', async () => {
            const agentId = 'agent@special#id';
            const encodedAgentId = encodeURIComponent(agentId);

            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${encodedAgentId}/messages`,
                expect.any(Object),
            );
        });
    });

    describe('Error Handling', () => {
        it('should handle missing agent_id', async () => {
            await expect(handleListMessages(mockServer, {})).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle null args', async () => {
            await expect(handleListMessages(mockServer, null)).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle 404 agent not found error', async () => {
            const agentId = 'non-existent-agent';
            const error = new Error('Not found');
            error.response = {
                status: 404,
                data: { error: 'Agent not found' },
            };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleListMessages(mockServer, {
                    agent_id: agentId,
                }),
            ).rejects.toThrow();

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(error);
        });

        it('should handle generic API errors', async () => {
            const error = new Error('Internal server error');
            error.response = {
                status: 500,
                data: { error: 'Database error' },
            };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleListMessages(mockServer, {
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow();

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(error);
        });

        it('should handle network errors without response', async () => {
            const error = new Error('Network error: Connection refused');
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleListMessages(mockServer, {
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow();

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(error);
        });
    });

    describe('Edge Cases', () => {
        it('should handle UUID format agent IDs', async () => {
            const agentId = '550e8400-e29b-41d4-a716-446655440000';
            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.any(Object),
            );
        });

        it('should not include undefined parameters in API call', async () => {
            const agentId = 'agent-undefined';
            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            await handleListMessages(mockServer, {
                agent_id: agentId,
                limit: undefined,
                order: undefined,
                before: undefined,
                after: undefined,
                group_id: undefined,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                `/agents/${agentId}/messages`,
                expect.objectContaining({
                    params: {},
                }),
            );
        });

        it('should handle messages with complex content', async () => {
            const agentId = 'agent-complex';
            const mockMessages = [
                {
                    id: 'msg-complex',
                    message_type: 'user_message',
                    role: 'user',
                    text: 'Special chars: \n\t"quotes" \'apostrophes\' & symbols < > {} 🚀💬',
                    metadata: {
                        nested: { value: 'test' },
                        array: [1, 2, 3],
                    },
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockMessages });

            const result = await handleListMessages(mockServer, {
                agent_id: agentId,
            });

            const data = expectValidToolResponse(result);
            expect(data.messages[0].text).toContain('🚀');
            expect(data.messages[0].metadata.nested.value).toBe('test');
        });
    });
});
