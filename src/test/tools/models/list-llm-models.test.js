import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleListLlmModels,
    listLlmModelsDefinition,
} from '../../../tools/models/list-llm-models.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe.skip('List LLM Models', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(listLlmModelsDefinition.name).toBe('list_llm_models');
            expect(listLlmModelsDefinition.description).toContain('List available LLM models');
            expect(listLlmModelsDefinition.description).toContain('create_agent or modify_agent');
            expect(listLlmModelsDefinition.inputSchema.properties).toEqual({});
            expect(listLlmModelsDefinition.inputSchema.required).toEqual([]);
        });
    });

    describe('Functionality Tests', () => {
        it('should list LLM models successfully', async () => {
            const mockModels = [
                {
                    name: 'gpt-4',
                    provider: 'openai',
                    context_window: 8192,
                    max_tokens: 4096,
                    capabilities: ['chat', 'completion'],
                },
                {
                    name: 'claude-3-opus',
                    provider: 'anthropic',
                    context_window: 200000,
                    max_tokens: 4096,
                    capabilities: ['chat', 'completion', 'vision'],
                },
                {
                    name: 'llama-2-70b',
                    provider: 'meta',
                    context_window: 4096,
                    max_tokens: 2048,
                    capabilities: ['chat', 'completion'],
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            // Verify API call
            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/models/',
                expect.objectContaining({ headers: expect.any(Object) }),
            );

            // Verify response
            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(3);
            expect(data.models).toEqual(mockModels);
            expect(data.models[0].name).toBe('gpt-4');
            expect(data.models[1].context_window).toBe(200000);
            expect(data.models[2].provider).toBe('meta');
        });

        it('should handle empty model list', async () => {
            mockServer.api.get.mockResolvedValueOnce({ data: [] });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(0);
            expect(data.models).toEqual([]);
        });

        it('should handle models with minimal properties', async () => {
            const mockModels = [
                { name: 'basic-model' },
                { name: 'model-with-provider', provider: 'custom' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(2);
            expect(data.models[0]).toEqual({ name: 'basic-model' });
            expect(data.models[1].provider).toBe('custom');
        });

        it('should ignore any input arguments', async () => {
            const mockModels = [{ name: 'test-model' }];
            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            // Pass some random args that should be ignored
            const result = await handleListLlmModels(mockServer, {
                unused: 'argument',
                filter: 'should-be-ignored',
            });

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(1);
            expect(data.models[0].name).toBe('test-model');
        });

        it('should handle models with complex configuration', async () => {
            const mockModels = [
                {
                    name: 'advanced-model',
                    provider: 'advanced-ai',
                    context_window: 32768,
                    max_tokens: 8192,
                    capabilities: ['chat', 'completion', 'vision', 'function-calling'],
                    pricing: {
                        input: 0.01,
                        output: 0.03,
                        currency: 'USD',
                        per_tokens: 1000,
                    },
                    metadata: {
                        version: '2.0',
                        release_date: '2024-01-15',
                        deprecated: false,
                    },
                    supported_languages: ['en', 'es', 'fr', 'de', 'ja', 'zh'],
                    fine_tuning_available: true,
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.models[0]).toEqual(mockModels[0]);
            expect(data.models[0].capabilities).toHaveLength(4);
            expect(data.models[0].pricing.input).toBe(0.01);
            expect(data.models[0].supported_languages).toContain('ja');
        });
    });

    describe('Error Handling', () => {
        it('should handle API errors gracefully', async () => {
            const error = new Error('Network error');
            error.response = { status: 500, data: { error: 'Internal server error' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListLlmModels(mockServer, {})).rejects.toThrow('Network error');
        });

        it('should handle authentication errors', async () => {
            const error = new Error('Unauthorized');
            error.response = { status: 401, data: { error: 'Invalid API key' } };
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListLlmModels(mockServer, {})).rejects.toThrow('Unauthorized');
        });

        it('should handle timeout errors', async () => {
            const error = new Error('Request timeout');
            error.code = 'ECONNABORTED';
            mockServer.api.get.mockRejectedValueOnce(error);

            await expect(handleListLlmModels(mockServer, {})).rejects.toThrow('Request timeout');
        });

        it('should handle malformed response data', async () => {
            // If API returns non-array data
            mockServer.api.get.mockResolvedValueOnce({ data: 'not-an-array' });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(12); // length of string 'not-an-array' is 12
            expect(data.models).toBe('not-an-array');
        });
    });

    describe('Edge Cases', () => {
        it('should handle model names with special characters', async () => {
            const mockModels = [
                { name: 'model-with-dash' },
                { name: 'model_with_underscore' },
                { name: 'model.with.dots' },
                { name: 'model@special#chars' },
                { name: 'model/with/slashes' },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(5);
            expect(data.models.map((m) => m.name)).toEqual([
                'model-with-dash',
                'model_with_underscore',
                'model.with.dots',
                'model@special#chars',
                'model/with/slashes',
            ]);
        });

        it('should handle very large model lists', async () => {
            const mockModels = Array.from({ length: 100 }, (_, i) => ({
                name: `model-${i}`,
                provider: `provider-${i % 10}`,
                context_window: 1024 * (i + 1),
            }));

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(100);
            expect(data.models).toHaveLength(100);
            expect(data.models[99].name).toBe('model-99');
        });

        it('should preserve all model properties', async () => {
            const mockModels = [
                {
                    name: 'test-model',
                    custom_field_1: 'value1',
                    nested: {
                        field: 'value',
                        array: [1, 2, 3],
                    },
                    boolean_field: true,
                    number_field: 42,
                    null_field: null,
                    experimental_features: {
                        feature1: true,
                        feature2: false,
                    },
                },
            ];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.models[0]).toEqual(mockModels[0]);
            expect(data.models[0].nested.array).toEqual([1, 2, 3]);
            expect(data.models[0].null_field).toBeNull();
            expect(data.models[0].experimental_features.feature1).toBe(true);
        });

        it('should handle models with numeric names', async () => {
            const mockModels = [{ name: '7b' }, { name: '13b' }, { name: '70b' }, { name: '175b' }];

            mockServer.api.get.mockResolvedValueOnce({ data: mockModels });

            const result = await handleListLlmModels(mockServer, {});

            const data = expectValidToolResponse(result);
            expect(data.model_count).toBe(4);
            expect(data.models.map((m) => m.name)).toEqual(['7b', '13b', '70b', '175b']);
        });
    });
});
