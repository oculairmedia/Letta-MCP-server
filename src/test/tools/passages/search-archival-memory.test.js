import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleSearchArchivalMemory,
    searchArchivalMemoryDefinition,
} from '../../../tools/passages/search-archival-memory.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe('Search Archival Memory', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(searchArchivalMemoryDefinition.name).toBe('search_archival_memory');
            expect(searchArchivalMemoryDefinition.description).toContain('semantic similarity');
            expect(searchArchivalMemoryDefinition.inputSchema.required).toEqual(['agent_id', 'query']);
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('agent_id');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('query');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('tags');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('tag_match_mode');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('top_k');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('start_datetime');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('end_datetime');
            expect(searchArchivalMemoryDefinition.inputSchema.properties).toHaveProperty('include_embeddings');
        });
    });

    describe('Functionality Tests', () => {
        it('should search archival memory successfully', async () => {
            const mockResults = {
                results: [
                    {
                        id: 'passage-1',
                        text: 'First matching passage',
                        metadata: {},
                    },
                    {
                        id: 'passage-2',
                        text: 'Second matching passage',
                        metadata: {},
                    },
                ],
                count: 2,
            };

            mockServer.api.get.mockResolvedValueOnce({ data: mockResults });

            const result = await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test query',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-123/archival-memory/search',
                expect.objectContaining({
                    headers: expect.any(Object),
                    params: { query: 'test query' },
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.passages).toHaveLength(2);
            expect(data.passages[0].id).toBe('passage-1');
        });

        it('should exclude embeddings by default', async () => {
            const mockResults = {
                results: [
                    {
                        id: 'passage-1',
                        text: 'Passage with embedding',
                        embedding: [0.1, 0.2, 0.3],
                    },
                ],
                count: 1,
            };

            mockServer.api.get.mockResolvedValueOnce({ data: mockResults });

            const result = await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
            });

            const data = expectValidToolResponse(result);
            expect(data.passages[0]).not.toHaveProperty('embedding');
        });

        it('should include embeddings when requested', async () => {
            const mockResults = {
                results: [
                    {
                        id: 'passage-1',
                        text: 'Passage with embedding',
                        embedding: [0.1, 0.2, 0.3],
                    },
                ],
                count: 1,
            };

            mockServer.api.get.mockResolvedValueOnce({ data: mockResults });

            const result = await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
                include_embeddings: true,
            });

            const data = expectValidToolResponse(result);
            expect(data.passages[0].embedding).toEqual([0.1, 0.2, 0.3]);
        });

        it('should pass top_k parameter', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: [], count: 0 } });

            await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
                top_k: 5,
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                expect.any(String),
                expect.objectContaining({
                    params: expect.objectContaining({ top_k: 5 }),
                }),
            );
        });

        it('should pass tags and tag_match_mode parameters', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: [], count: 0 } });

            await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
                tags: ['important', 'work'],
                tag_match_mode: 'all',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                expect.any(String),
                expect.objectContaining({
                    params: expect.objectContaining({
                        tags: ['important', 'work'],
                        tag_match_mode: 'all',
                    }),
                }),
            );
        });

        it('should pass datetime filter parameters', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: [], count: 0 } });

            await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
                start_datetime: '2024-01-01T00:00:00Z',
                end_datetime: '2024-12-31T23:59:59Z',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                expect.any(String),
                expect.objectContaining({
                    params: expect.objectContaining({
                        start_datetime: '2024-01-01T00:00:00Z',
                        end_datetime: '2024-12-31T23:59:59Z',
                    }),
                }),
            );
        });

        it('should handle empty results', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: [], count: 0 } });

            const result = await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'nonexistent',
            });

            const data = expectValidToolResponse(result);
            expect(data.passages).toHaveLength(0);
        });

        it('should handle special characters in agent_id', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: [], count: 0 } });

            await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-with-special/chars',
                query: 'test',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/agents/agent-with-special%2Fchars/archival-memory/search',
                expect.any(Object),
            );
        });

        it('should handle null results array', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: { results: null, count: 0 } });

            const result = await handleSearchArchivalMemory(mockServer, {
                agent_id: 'agent-123',
                query: 'test',
            });

            const data = expectValidToolResponse(result);
            expect(data.passages).toHaveLength(0);
        });
    });

    describe('Error Handling', () => {
        it('should throw error for missing agent_id', async () => {
            await expect(
                handleSearchArchivalMemory(mockServer, { query: 'test' }),
            ).rejects.toThrow('Missing required argument: agent_id');
        });

        it('should throw error for missing query', async () => {
            await expect(
                handleSearchArchivalMemory(mockServer, { agent_id: 'agent-123' }),
            ).rejects.toThrow('Missing required argument: query');
        });

        it('should throw error for null args', async () => {
            await expect(handleSearchArchivalMemory(mockServer, null)).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should throw error for undefined args', async () => {
            await expect(handleSearchArchivalMemory(mockServer, undefined)).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle 404 error', async () => {
            const error = new Error('Not found');
            error.response = { status: 404, data: { error: 'Agent not found' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleSearchArchivalMemory(mockServer, {
                    agent_id: 'nonexistent',
                    query: 'test',
                }),
            ).rejects.toThrow('Not found');
        });

        it('should handle network errors', async () => {
            const error = new Error('Network error: Connection refused');
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleSearchArchivalMemory(mockServer, {
                    agent_id: 'agent-123',
                    query: 'test',
                }),
            ).rejects.toThrow('Network error');
        });

        it('should handle server errors', async () => {
            const error = new Error('Internal server error');
            error.response = { status: 500, data: { error: 'Database error' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(
                handleSearchArchivalMemory(mockServer, {
                    agent_id: 'agent-123',
                    query: 'test',
                }),
            ).rejects.toThrow('Internal server error');
        });
    });
});
