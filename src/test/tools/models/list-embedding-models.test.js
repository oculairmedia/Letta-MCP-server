import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleListEmbeddingModels,
    listEmbeddingModelsDefinition,
} from '../../../tools/models/list-embedding-models.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe.skip('List Embedding Models', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(listEmbeddingModelsDefinition.name).toBe('list_embedding_models');
            expect(listEmbeddingModelsDefinition.description).toContain(
                'List available embedding models',
            );
            expect(listEmbeddingModelsDefinition.description).toContain(
                'create_agent or modify_agent',
            );
            expect(listEmbeddingModelsDefinition.inputSchema.properties).toEqual({});
            expect(listEmbeddingModelsDefinition.inputSchema.required).toEqual([]);
        });
    });

    describe('Functionality Tests', () => {
        it('should list embedding models successfully', async () => {
            const mockModels = [
                {
                    name: 'text-embedding-ada-002',
                    provider: 'openai',
                    dimensions: 1536,
                    max_input_tokens: 8192,
                    cost_per_million_tokens: 0.1,
                },
                {
                    name: 'text-embedding-3-small',
                    provider: 'openai',
                    dimensions: 1536,
                    max_input_tokens: 8192,
                    cost_per_million_tokens: 0.02,
                },
                {
                    name: 'text-embedding-3-large',
                    provider: 'openai',
                    dimensions: 3072,
                    max_input_tokens: 8192,
                    cost_per_million_tokens: 0.13,
                },
                {
                    name: 'voyage-2',
                    provider: 'voyage',
                    dimensions: 1024,
                    max_input_tokens: 4000,
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            // Verify API call
            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/models/embedding',
                expect.objectContaining({ headers: expect.any(Object) }),
            );

            // Verify response
            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(4);
            expect(data.models).toEqual(mockModels);
            expect(data.models[0].name).toBe('text-embedding-ada-002');
            expect(data.models[2].dimensions).toBe(3072);
            expect(data.models[3].provider).toBe('voyage');
        });

        it('should handle empty model list', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(0);
            expect(data.models).toEqual([]);
        });

        it('should handle models with minimal properties', async () => {
            const mockModels = [
                { name: 'basic-embedding' },
                { name: 'embedding-with-dims', dimensions: 768 },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(2);
            expect(data.models[0]).toEqual({ name: 'basic-embedding' });
            expect(data.models[1].dimensions).toBe(768);
        });

        it('should ignore any input arguments', async () => {
            const mockModels = [{ name: 'test-embedding' }];
            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            // Pass some random args that should be ignored
            const result = await handleListEmbeddingModels(mockServer, {
                unused: 'argument',
                filter: 'should-be-ignored',
                provider: 'openai',
            });

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(1);
            expect(data.models[0].name).toBe('test-embedding');
        });

        it('should handle models with complex configuration', async () => {
            const mockModels = [
                {
                    name: 'advanced-embedding',
                    provider: 'custom-ai',
                    dimensions: 2048,
                    max_input_tokens: 16384,
                    cost_per_million_tokens: 0.05,
                    supported_languages: ['en', 'es', 'fr', 'de', 'ja', 'zh', 'ar', 'ru'],
                    model_type: 'transformer',
                    training_data: {
                        size: '100B tokens',
                        sources: ['web', 'books', 'academic'],
                        cutoff_date: '2024-01',
                    },
                    features: {
                        multilingual: true,
                        fine_tunable: true,
                        batch_processing: true,
                        async_processing: true,
                    },
                    performance_metrics: {
                        mteb_score: 0.85,
                        retrieval_accuracy: 0.92,
                        semantic_similarity: 0.88,
                    },
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.models[0]).toEqual(mockModels[0]);
            expect(data.models[0].supported_languages).toHaveLength(8);
            expect(data.models[0].features.multilingual).toBe(true);
            expect(data.models[0].performance_metrics.mteb_score).toBe(0.85);
        });
    });

    describe('Error Handling', () => {
        it('should handle API errors gracefully', async () => {
            const error = new Error('Network error');
            error.response = { status: 500, data: { error: 'Internal server error' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListEmbeddingModels(mockServer, {})).rejects.toThrow(
                'Network error',
            );
        });

        it('should handle authentication errors', async () => {
            const error = new Error('Unauthorized');
            error.response = { status: 401, data: { error: 'Invalid API key' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListEmbeddingModels(mockServer, {})).rejects.toThrow('Unauthorized');
        });

        it('should handle rate limit errors', async () => {
            const error = new Error('Rate limit exceeded');
            error.response = {
                status: 429,
                data: { error: 'Too many requests' },
                headers: { 'retry-after': '60' },
            };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListEmbeddingModels(mockServer, {})).rejects.toThrow(
                'Rate limit exceeded',
            );
        });

        it('should handle malformed response data', async () => {
            // If API returns non-array data
            mockServer.api.get.mockResolvedValueOnce({ data: 'not-an-array' });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(12); // length of string 'not-an-array' is 12
            expect(data.models).toBe('not-an-array');
        });
    });

    describe('Edge Cases', () => {
        it('should handle model names with special characters', async () => {
            const mockModels = [
                { name: 'embedding-v1.0' },
                { name: 'embedding_multilingual' },
                { name: 'embedding@latest' },
                { name: 'text/embedding-base' },
                { name: 'embedding-512d-v2' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(5);
            expect(data.models.map((m) => m.name)).toEqual([
                'embedding-v1.0',
                'embedding_multilingual',
                'embedding@latest',
                'text/embedding-base',
                'embedding-512d-v2',
            ]);
        });

        it('should handle very large model lists', async () => {
            const mockModels = Array.from({ length: 50 }, (_, i) => ({
                name: `embedding-model-${i}`,
                provider: `provider-${i % 5}`,
                dimensions: 256 * ((i % 8) + 1), // 256, 512, 768, ..., 2048
                max_input_tokens: 1000 * ((i % 10) + 1),
            }));

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(50);
            expect(data.models).toHaveLength(50);
            expect(data.models[49].name).toBe('embedding-model-49');
        });

        it('should preserve all model properties including nulls and empty values', async () => {
            const mockModels = [
                {
                    name: 'test-embedding',
                    provider: '',
                    dimensions: 0,
                    max_input_tokens: null,
                    metadata: {
                        version: null,
                        tags: [],
                        custom: {},
                    },
                    experimental: undefined,
                    deprecated: false,
                    aliases: ['test-emb', 'test-embedding-v1'],
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.models[0].provider).toBe('');
            expect(data.models[0].dimensions).toBe(0);
            expect(data.models[0].max_input_tokens).toBeNull();
            expect(data.models[0].metadata.tags).toEqual([]);
            expect(data.models[0].deprecated).toBe(false);
            expect(data.models[0].aliases).toHaveLength(2);
        });

        it('should handle models with dimension variations', async () => {
            const mockModels = [
                { name: 'tiny-embedding', dimensions: 64 },
                { name: 'small-embedding', dimensions: 256 },
                { name: 'medium-embedding', dimensions: 768 },
                { name: 'large-embedding', dimensions: 1536 },
                { name: 'xlarge-embedding', dimensions: 3072 },
                { name: 'massive-embedding', dimensions: 8192 },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(6);
            const dimensions = data.models.map((m) => m.dimensions);
            expect(dimensions).toEqual([64, 256, 768, 1536, 3072, 8192]);
        });

        it('should handle response with additional unexpected fields', async () => {
            const mockModels = [
                {
                    name: 'future-embedding',
                    provider: 'next-gen-ai',
                    dimensions: 4096,
                    // Future fields that might be added
                    quantum_enhanced: true,
                    neural_architecture: 'transformer-v5',
                    compression_ratio: 0.95,
                    supported_modalities: ['text', 'code', 'math'],
                    hardware_requirements: {
                        gpu_memory: '16GB',
                        compute_capability: 8.0,
                    },
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListEmbeddingModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.models[0]).toEqual(mockModels[0]);
            expect(data.models[0].quantum_enhanced).toBe(true);
            expect(data.models[0].supported_modalities).toContain('code');
        });
    });
});
