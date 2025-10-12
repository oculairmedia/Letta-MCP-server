/**
 * Tests for letta_memory_unified tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaMemoryUnified, lettaMemoryUnifiedDefinition } from '../memory/letta-memory-unified.js';

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

describe('letta_memory_unified', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaMemoryUnifiedDefinition.name).toBe('letta_memory_unified');
        });

        it('should have proper input schema with all operations', () => {
            expect(lettaMemoryUnifiedDefinition.inputSchema.properties.operation.enum).toEqual([
                'get_core_memory',
                'update_core_memory',
                'get_block_by_label',
                'list_blocks',
                'detach_block',
                'list_agents_using_block',
                'search_archival',
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaMemoryUnifiedDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('Get Core Memory Operation', () => {
        it('should successfully get agent core memory', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    persona: 'I am a helpful assistant',
                    human: 'User is a developer working on AI projects',
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'get_core_memory',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/memory',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('get_core_memory');
            expect(response.core_memory.persona).toBe('I am a helpful assistant');
            expect(response.core_memory.human).toBe('User is a developer working on AI projects');
        });

        it('should handle nested core_memory structure', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    core_memory: {
                        persona: 'Nested persona',
                        human: 'Nested human',
                    },
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'get_core_memory',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.core_memory.persona).toBe('Nested persona');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'get_core_memory',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Update Core Memory Operation', () => {
        it('should successfully update core memory', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    core_memory: {
                        persona: 'Updated persona',
                        human: 'Updated human',
                    },
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'update_core_memory',
                agent_id: 'agent-123',
                memory_data: {
                    persona: 'Updated persona',
                    human: 'Updated human',
                },
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/memory',
                {
                    persona: 'Updated persona',
                    human: 'Updated human',
                },
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('update_core_memory');
            expect(response.core_memory).toBeDefined();
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'update_core_memory',
                    memory_data: { persona: 'test' },
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when memory_data is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'update_core_memory',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('memory_data is required');
        });
    });

    describe('Get Block By Label Operation', () => {
        it('should successfully get memory block by label', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    id: 'block-123',
                    label: 'persona',
                    value: 'I am a helpful assistant',
                    limit: 2000,
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'get_block_by_label',
                agent_id: 'agent-123',
                block_label: 'persona',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/memory/blocks/persona',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('get_block_by_label');
            expect(response.block.label).toBe('persona');
            expect(response.block.id).toBe('block-123');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'get_block_by_label',
                    block_label: 'persona',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when block_label is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'get_block_by_label',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('block_label is required');
        });
    });

    describe('List Blocks Operation', () => {
        it('should successfully list all memory blocks', async () => {
            const mockBlocks = [
                { id: 'block-1', label: 'persona', value: 'Persona content', limit: 2000 },
                { id: 'block-2', label: 'human', value: 'Human content', limit: 2000 },
            ];

            mockServer.api.get.mockResolvedValue({ data: mockBlocks });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'list_blocks',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/memory/blocks',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('list_blocks');
            expect(response.blocks).toHaveLength(2);
            expect(response.blocks[0].label).toBe('persona');
        });

        it('should handle response with blocks array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    blocks: [
                        { id: 'block-1', label: 'custom', value: 'Custom content', limit: 1000 },
                    ],
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'list_blocks',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.blocks).toHaveLength(1);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'list_blocks',
                })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Detach Block Operation', () => {
        it('should successfully detach memory block', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'detach_block',
                agent_id: 'agent-123',
                block_id: 'block-456',
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                '/agents/agent-123/memory/blocks/block-456',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('detach_block');
            expect(response.detached).toBe(true);
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'detach_block',
                    block_id: 'block-456',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when block_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'detach_block',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('block_id is required');
        });
    });

    describe('List Agents Using Block Operation', () => {
        it('should successfully list agents using a block', async () => {
            const mockAgents = [
                { id: 'agent-1', name: 'Agent One' },
                { id: 'agent-2', name: 'Agent Two' },
            ];

            mockServer.api.get.mockResolvedValue({ data: mockAgents });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'list_agents_using_block',
                block_id: 'block-789',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/memory/blocks/block-789/agents',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('list_agents_using_block');
            expect(response.agents).toHaveLength(2);
            expect(response.agents[0].name).toBe('Agent One');
        });

        it('should handle response with agents array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    agents: [{ id: 'agent-3', agent_name: 'Agent Three' }],
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'list_agents_using_block',
                block_id: 'block-789',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.agents).toHaveLength(1);
            expect(response.agents[0].name).toBe('Agent Three');
        });

        it('should throw error when block_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'list_agents_using_block',
                })
            ).rejects.toThrow('block_id is required');
        });
    });

    describe('Search Archival Operation', () => {
        it('should successfully search archival memory', async () => {
            const mockResults = [
                {
                    id: 'passage-1',
                    text: 'This is relevant content',
                    timestamp: '2025-10-12T00:00:00Z',
                    similarity_score: 0.95,
                },
                {
                    id: 'passage-2',
                    text: 'Another relevant passage',
                    timestamp: '2025-10-12T00:01:00Z',
                    similarity_score: 0.85,
                },
            ];

            mockServer.api.get.mockResolvedValue({ data: mockResults });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'search_archival',
                agent_id: 'agent-123',
                search_query: 'relevant content',
                search_options: {
                    limit: 10,
                    threshold: 0.7,
                },
            });

            const callArg = mockServer.api.get.mock.calls[0][0];
            expect(callArg).toContain('query=relevant+content');
            expect(callArg).toContain('limit=10');
            expect(callArg).toContain('threshold=0.7');

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('search_archival');
            expect(response.search_results).toHaveLength(2);
            expect(response.search_results[0].similarity_score).toBe(0.95);
        });

        it('should handle response with results array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    results: [
                        {
                            id: 'p-1',
                            content: 'Content text',
                            created_at: '2025-10-12T00:00:00Z',
                            score: 0.9,
                        },
                    ],
                },
            });

            const result = await handleLettaMemoryUnified(mockServer, {
                operation: 'search_archival',
                agent_id: 'agent-123',
                search_query: 'test',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.search_results).toHaveLength(1);
            expect(response.search_results[0].text).toBe('Content text');
        });

        it('should search without search_options', async () => {
            mockServer.api.get.mockResolvedValue({ data: [] });

            await handleLettaMemoryUnified(mockServer, {
                operation: 'search_archival',
                agent_id: 'agent-123',
                search_query: 'test',
            });

            const callArg = mockServer.api.get.mock.calls[0][0];
            expect(callArg).toContain('query=test');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'search_archival',
                    search_query: 'test',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when search_query is missing', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'search_archival',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('search_query is required');
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'invalid',
                })
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.api.get.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'get_core_memory',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.api.post.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaMemoryUnified(mockServer, {
                    operation: 'update_core_memory',
                    agent_id: 'agent-123',
                    memory_data: { persona: 'test' },
                })
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
