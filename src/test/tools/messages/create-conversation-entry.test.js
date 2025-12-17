import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleCreateConversationEntry,
    createConversationEntryDefinition,
} from '../../../tools/messages/create-conversation-entry.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe('Create Conversation Entry', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(createConversationEntryDefinition.name).toBe('create_conversation_entry');
            expect(createConversationEntryDefinition.description).toContain(
                "Store a conversation entry in an agent's archival memory",
            );
            expect(createConversationEntryDefinition.inputSchema.required).toEqual([
                'agent_id',
                'role',
                'content',
            ]);
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty(
                'agent_id',
            );
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty('role');
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty(
                'content',
            );
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty(
                'timestamp',
            );
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty(
                'source',
            );
            expect(createConversationEntryDefinition.inputSchema.properties).toHaveProperty(
                'session_id',
            );
            expect(createConversationEntryDefinition.inputSchema.properties.role.enum).toEqual([
                'user',
                'assistant',
                'system',
            ]);
        });
    });

    describe('Functionality Tests', () => {
        it('should create user conversation entry successfully', async () => {
            const agentId = 'agent-123';
            const content = 'Hello, this is a test message from the user.';

            const mockPassage = {
                id: 'passage-123',
                text: expect.any(String),
                embedding: [0.1, 0.2, 0.3],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: content,
            });

            // Verify API call
            expect(mockServer.api.post).toHaveBeenCalledWith(
                `/agents/${agentId}/archival-memory`,
                expect.objectContaining({
                    text: expect.stringContaining('"role":"user"'),
                }),
                expect.objectContaining({
                    headers: expect.any(Object),
                }),
            );

            // Verify the stored text is JSON with correct format
            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.message_type).toBe('user_message');
            expect(storedData.role).toBe('user');
            expect(storedData.text).toBe(content);

            // Verify response
            const data = expectValidToolResponse(result);
            expect(data.success).toBe(true);
            expect(data.passage_id).toBe('passage-123');
            expect(data.role).toBe('user');
        });

        it('should create assistant conversation entry successfully', async () => {
            const agentId = 'agent-456';
            const content = 'This is the assistant response.';

            const mockPassage = {
                id: 'passage-456',
                text: expect.any(String),
                embedding: [0.1, 0.2],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'assistant',
                content: content,
            });

            // Verify the stored text has correct message_type
            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.message_type).toBe('assistant_message');
            expect(storedData.role).toBe('assistant');
            expect(storedData.text).toBe(content);

            const data = expectValidToolResponse(result);
            expect(data.role).toBe('assistant');
        });

        it('should create system conversation entry successfully', async () => {
            const agentId = 'agent-789';
            const content = 'System notification message.';

            const mockPassage = {
                id: 'passage-789',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'system',
                content: content,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.message_type).toBe('system_message');
            expect(storedData.role).toBe('system');

            const data = expectValidToolResponse(result);
            expect(data.role).toBe('system');
        });

        it('should include custom timestamp', async () => {
            const agentId = 'agent-ts';
            const customTimestamp = '2024-06-15T10:30:00Z';

            const mockPassage = {
                id: 'passage-ts',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Timestamped message',
                timestamp: customTimestamp,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.date).toBe(customTimestamp);

            const data = expectValidToolResponse(result);
            expect(data.timestamp).toBe(customTimestamp);
        });

        it('should include source metadata', async () => {
            const agentId = 'agent-source';
            const source = 'claude_code';

            const mockPassage = {
                id: 'passage-source',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Message from Claude Code',
                source: source,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.source).toBe(source);

            const data = expectValidToolResponse(result);
            expect(data.source).toBe(source);
        });

        it('should include session_id metadata', async () => {
            const agentId = 'agent-session';
            const sessionId = 'session-12345';

            const mockPassage = {
                id: 'passage-session',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'assistant',
                content: 'Session message',
                session_id: sessionId,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.session_id).toBe(sessionId);

            expectValidToolResponse(result);
        });

        it('should include all optional fields', async () => {
            const agentId = 'agent-all';
            const content = 'Full metadata message';
            const timestamp = '2024-01-15T12:00:00Z';
            const source = 'letta_ade';
            const sessionId = 'session-full-123';

            const mockPassage = {
                id: 'passage-all',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: content,
                timestamp: timestamp,
                source: source,
                session_id: sessionId,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.date).toBe(timestamp);
            expect(storedData.source).toBe(source);
            expect(storedData.session_id).toBe(sessionId);
            expect(storedData.text).toBe(content);
            expect(storedData.role).toBe('user');
            expect(storedData.message_type).toBe('user_message');

            const data = expectValidToolResponse(result);
            expect(data.timestamp).toBe(timestamp);
            expect(data.source).toBe(source);
        });

        it('should use defaults for optional fields', async () => {
            const agentId = 'agent-defaults';

            const mockPassage = {
                id: 'passage-defaults',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Minimal message',
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.source).toBe('unknown');
            expect(storedData.session_id).toBe('no-session');
            expect(storedData.date).toBeDefined(); // Should have auto-generated timestamp
        });

        it('should strip embeddings from response', async () => {
            const agentId = 'agent-embed';

            const mockPassage = {
                id: 'passage-embed',
                text: expect.any(String),
                embedding: [0.1, 0.2, 0.3, 0.4, 0.5],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Test message',
            });

            // Response should not contain embedding
            const data = expectValidToolResponse(result);
            expect(data.embedding).toBeUndefined();
        });

        it('should handle special characters in content', async () => {
            const agentId = 'agent-special';
            const specialContent =
                'Special chars: \n\t"quotes" \'apostrophes\' & symbols < > {} 🚀💬 中文';

            const mockPassage = {
                id: 'passage-special',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: specialContent,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.text).toBe(specialContent);

            expectValidToolResponse(result);
        });

        it('should handle very long content', async () => {
            const agentId = 'agent-long';
            const longContent = 'A'.repeat(10000);

            const mockPassage = {
                id: 'passage-long',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'assistant',
                content: longContent,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.text).toBe(longContent);

            expectValidToolResponse(result);
        });

        it('should handle special characters in agent ID', async () => {
            const agentId = 'agent@special#id';
            const encodedAgentId = encodeURIComponent(agentId);

            const mockPassage = {
                id: 'passage-encoded',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Test',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                `/agents/${encodedAgentId}/archival-memory`,
                expect.any(Object),
                expect.any(Object),
            );
        });
    });

    describe('Error Handling', () => {
        it('should handle missing agent_id', async () => {
            await expect(
                handleCreateConversationEntry(mockServer, {
                    role: 'user',
                    content: 'Test',
                }),
            ).rejects.toThrow('Missing required argument: agent_id');
        });

        it('should handle missing role', async () => {
            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'agent-123',
                    content: 'Test',
                }),
            ).rejects.toThrow('Missing required argument: role');
        });

        it('should handle missing content', async () => {
            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'agent-123',
                    role: 'user',
                }),
            ).rejects.toThrow('Missing required argument: content');
        });

        it('should handle null args', async () => {
            await expect(handleCreateConversationEntry(mockServer, null)).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle 404 agent not found error', async () => {
            const error = new Error('Not found');
            error.response = {
                status: 404,
                data: { error: 'Agent not found' },
            };
            mockServer.api.post.mockRejectedValueOnce(error);

            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'non-existent-agent',
                    role: 'user',
                    content: 'Test',
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
            mockServer.api.post.mockRejectedValueOnce(error);

            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'agent-123',
                    role: 'user',
                    content: 'Test',
                }),
            ).rejects.toThrow();

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(error);
        });

        it('should handle network errors without response', async () => {
            const error = new Error('Network error: Connection refused');
            mockServer.api.post.mockRejectedValueOnce(error);

            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'agent-123',
                    role: 'user',
                    content: 'Test',
                }),
            ).rejects.toThrow();

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(error);
        });
    });

    describe('Edge Cases', () => {
        it('should handle UUID format agent IDs', async () => {
            const agentId = '550e8400-e29b-41d4-a716-446655440000';

            const mockPassage = {
                id: 'passage-uuid',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'UUID test',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                `/agents/${agentId}/archival-memory`,
                expect.any(Object),
                expect.any(Object),
            );
        });

        it('should reject empty content string', async () => {
            await expect(
                handleCreateConversationEntry(mockServer, {
                    agent_id: 'agent-empty',
                    role: 'user',
                    content: '',
                }),
            ).rejects.toThrow('Missing required argument: content');
        });

        it('should handle multiline content', async () => {
            const agentId = 'agent-multiline';
            const multilineContent = `Line 1: Introduction
Line 2: Main content
Line 3: Additional details

Line 5: After blank line
\tLine 6: With tab
  Line 7: With spaces`;

            const mockPassage = {
                id: 'passage-multiline',
                text: expect.any(String),
                embedding: [0.1],
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: multilineContent,
            });

            const callArgs = mockServer.api.post.mock.calls[0][1];
            const storedData = JSON.parse(callArgs.text);
            expect(storedData.text).toBe(multilineContent);

            expectValidToolResponse(result);
        });

        it('should handle passage with null embedding', async () => {
            const agentId = 'agent-null-embed';

            const mockPassage = {
                id: 'passage-null-embed',
                text: expect.any(String),
                embedding: null,
            };

            mockServer.api.post.mockResolvedValueOnce({ data: mockPassage });

            const result = await handleCreateConversationEntry(mockServer, {
                agent_id: agentId,
                role: 'user',
                content: 'Test',
            });

            // Should not error even with null embedding
            expectValidToolResponse(result);
        });
    });
});
