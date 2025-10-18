import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleRetrieveAgent,
    retrieveAgentDefinition,
} from '../../../tools/agents/retrieve-agent.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { fixtures } from '../../utils/test-fixtures.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe.skip('Retrieve Agent', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(retrieveAgentDefinition.name).toBe('retrieve_agent');
            expect(retrieveAgentDefinition.description).toContain(
                'Get the full state of a specific agent',
            );
            expect(retrieveAgentDefinition.inputSchema.required).toEqual(['agent_id']);
            expect(retrieveAgentDefinition.inputSchema.properties).toHaveProperty('agent_id');
        });

        it('should require agent_id parameter', () => {
            const agentIdProp = retrieveAgentDefinition.inputSchema.properties.agent_id;
            expect(agentIdProp.type).toBe('string');
            expect(agentIdProp.description).toContain('ID of the agent');
        });
    });

    describe('Functionality Tests', () => {
        it('should retrieve agent successfully', async () => {
            const fullAgentState = {
                ...fixtures.agent.basic,
                memory: {
                    human: 'User information',
                    persona: 'Assistant persona',
                },
                tools: ['tool1', 'tool2', 'tool3'],
                messages_count: 42,
                last_message_at: '2024-01-15T10:30:00Z',
            };

            mockServer.api.get.mockResolvedValueOnce({ data: fullAgentState });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: 'agent-123',
            });

            // Verify API call
            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123',
                expect.objectContaining({ headers: expect.any(Object) }),
            );

            // Verify response
            const data = expectValidToolResponse(result);
            expect(data.agent).toEqual(fullAgentState);
            expect(data.agent.id).toBe('agent-123');
            expect(data.agent.memory).toBeDefined();
            expect(data.agent.messages_count).toBe(42);
        });

        it('should handle special characters in agent_id', async () => {
            const agentData = fixtures.agent.basic;
            mockServer.api.get.mockResolvedValueOnce({ data: agentData });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: 'agent@special#123',
            });

            // Verify URL encoding
            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent%40special%23123',
                expect.any(Object),
            );

            const data = expectValidToolResponse(result);
            expect(data.agent).toEqual(agentData);
        });

        it('should retrieve minimal agent state', async () => {
            const minimalAgent = fixtures.agent.minimal;
            mockServer.api.get.mockResolvedValueOnce({ data: minimalAgent });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: 'agent-456',
            });

            const data = expectValidToolResponse(result);
            expect(data.agent).toEqual(minimalAgent);
            expect(data.agent.id).toBe('agent-456');
        });

        it('should retrieve agent with full configuration', async () => {
            const fullConfigAgent = {
                ...fixtures.agent.basic,
                llm_config: {
                    model: 'gpt-4',
                    temperature: 0.7,
                    max_tokens: 2000,
                    frequency_penalty: 0.5,
                    presence_penalty: 0.5,
                },
                embedding_config: {
                    model: 'text-embedding-ada-002',
                    dimension: 1536,
                },
                memory_blocks: fixtures.memory.blocks,
                tool_rules: [
                    { tool_name: 'search', enabled: true },
                    { tool_name: 'calculator', enabled: false },
                ],
            };

            mockServer.api.get.mockResolvedValueOnce({ data: fullConfigAgent });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: 'config-agent',
            });

            const data = expectValidToolResponse(result);
            expect(data.agent).toEqual(fullConfigAgent);
            expect(data.agent.llm_config).toBeDefined();
            expect(data.agent.embedding_config).toBeDefined();
            expect(data.agent.memory_blocks).toHaveLength(2);
        });
    });

    describe('Error Handling', () => {
        it('should throw error for missing agent_id', async () => {
            await expect(handleRetrieveAgent(mockServer, {})).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should throw error for null agent_id', async () => {
            await expect(handleRetrieveAgent(mockServer, { agent_id: null })).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should throw error for undefined args', async () => {
            await expect(handleRetrieveAgent(mockServer, undefined)).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle agent not found (404)', async () => {
            const error = new Error('Not found');
            error.response = { status: 404 };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleRetrieveAgent(mockServer, { agent_id: 'nonexistent' }),
            ).rejects.toThrow('Agent not found: nonexistent');
        });

        it('should handle unauthorized access (401)', async () => {
            const error = new Error('Unauthorized');
            error.response = { status: 401, data: { error: 'Invalid credentials' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleRetrieveAgent(mockServer, { agent_id: 'agent-123' }),
            ).rejects.toThrow('Unauthorized');
        });

        it('should handle server errors (500)', async () => {
            const error = new Error('Internal server error');
            error.response = { status: 500, data: { error: 'Database connection failed' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleRetrieveAgent(mockServer, { agent_id: 'agent-123' }),
            ).rejects.toThrow('Internal server error');
        });

        it('should handle network errors', async () => {
            const error = new Error('Network error');
            // No response property for network errors
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleRetrieveAgent(mockServer, { agent_id: 'agent-123' }),
            ).rejects.toThrow('Network error');
        });
    });

    describe('Edge Cases', () => {
        it('should handle empty agent_id string', async () => {
            await expect(handleRetrieveAgent(mockServer, { agent_id: '' })).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle very long agent_id', async () => {
            const longId = 'a'.repeat(1000);
            const agentData = { ...fixtures.agent.basic, id: longId };
            mockServer.api.get.mockResolvedValueOnce({ data: agentData });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: longId,
            });

            const data = expectValidToolResponse(result);
            expect(data.agent.id).toBe(longId);
        });

        it('should handle agent with unicode characters in ID', async () => {
            const unicodeId = 'agent-🤖-123';
            const agentData = { ...fixtures.agent.basic, id: unicodeId };
            mockServer.api.get.mockResolvedValueOnce({ data: agentData });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: unicodeId,
            });

            // Verify proper encoding
            expect(mockServer.api.get).toHaveBeenCalledWith(
                expect.stringContaining('agent-'),
                expect.any(Object),
            );

            const data = expectValidToolResponse(result);
            expect(data.agent.id).toBe(unicodeId);
        });

        it('should preserve all agent state fields', async () => {
            const complexAgent = {
                id: 'complex-agent',
                name: 'Complex Agent',
                description: 'Test agent with many fields',
                created_at: '2024-01-01T00:00:00Z',
                last_modified: '2024-01-15T12:00:00Z',
                metadata: {
                    custom_field: 'value',
                    nested: {
                        data: 'test',
                    },
                },
                settings: {
                    feature_flags: ['flag1', 'flag2'],
                    experimental: true,
                },
                statistics: {
                    total_messages: 1000,
                    average_response_time: 1.5,
                },
            };

            mockServer.api.get.mockResolvedValueOnce({ data: complexAgent });

            const result = await handleRetrieveAgent(mockServer, {
                agent_id: 'complex-agent',
            });

            const data = expectValidToolResponse(result);
            expect(data.agent).toEqual(complexAgent);
            // Verify nested structures are preserved
            expect(data.agent.metadata.nested.data).toBe('test');
            expect(data.agent.settings.feature_flags).toHaveLength(2);
            expect(data.agent.statistics.average_response_time).toBe(1.5);
        });
    });
});
