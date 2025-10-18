import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { LettaServer } from '../../core/server.js';
import axios from 'axios';
import { createMockLettaServer } from '../utils/mock-server.js';

// Mock dependencies
vi.mock('@modelcontextprotocol/sdk/server/index.js');
vi.mock('axios');
vi.mock('../../core/logger.js');

describe('API Client Configuration (LMP-85)', () => {
    let mockAxiosInstance;
    let originalEnv;

    beforeEach(() => {
        // Save original env vars
        originalEnv = {
            LETTA_BASE_URL: process.env.LETTA_BASE_URL,
            LETTA_PASSWORD: process.env.LETTA_PASSWORD,
        };

        // Set up mock axios instance
        mockAxiosInstance = {
            get: vi.fn(),
            post: vi.fn(),
            put: vi.fn(),
            patch: vi.fn(),
            delete: vi.fn(),
            request: vi.fn(),
            defaults: {
                baseURL: '',
                headers: {},
            },
        };

        axios.create.mockReturnValue(mockAxiosInstance);
    });

    afterEach(() => {
        // Restore original env vars
        process.env.LETTA_BASE_URL = originalEnv.LETTA_BASE_URL;
        process.env.LETTA_PASSWORD = originalEnv.LETTA_PASSWORD;

        vi.clearAllMocks();
    });

    describe('Axios Instance Creation', () => {
        it('should create axios instance with correct base URL', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://api.letta.com/v1',
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
        });

        it('should handle base URL with trailing slash', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com/';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://api.letta.com//v1',
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
        });

        it('should handle base URL already containing /v1', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com/v1';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            // Expect the implementation to not duplicate /v1 when base already contains it
            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://api.letta.com/v1',
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
        });

        it('should set default headers', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            const createCall = axios.create.mock.calls[0][0];
            expect(createCall.headers).toEqual({
                'Content-Type': 'application/json',
                Accept: 'application/json',
            });
        });

        it('should store axios instance as api property', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            const server = new LettaServer();
            expect(server.api).toBe(mockAxiosInstance);
        });
    });

    describe('getApiHeaders method', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
        });

        it('should return correct headers with password', () => {
            process.env.LETTA_PASSWORD = 'my-secret-password';
            const server = new LettaServer();

            const headers = server.getApiHeaders();

            expect(headers).toEqual({
                'Content-Type': 'application/json',
                Accept: 'application/json',
                'X-BARE-PASSWORD': 'password my-secret-password',
                Authorization: 'Bearer my-secret-password',
            });
        });

        it('should handle empty password', () => {
            process.env.LETTA_PASSWORD = '';
            const server = new LettaServer();

            const headers = server.getApiHeaders();

            expect(headers).toEqual({
                'Content-Type': 'application/json',
                Accept: 'application/json',
                'X-BARE-PASSWORD': 'password ',
                Authorization: 'Bearer ',
            });
        });

        it('should handle special characters in password', () => {
            process.env.LETTA_PASSWORD = 'p@$$w0rd!#$%^&*()';
            const server = new LettaServer();

            const headers = server.getApiHeaders();

            expect(headers).toEqual({
                'Content-Type': 'application/json',
                Accept: 'application/json',
                'X-BARE-PASSWORD': 'password p@$$w0rd!#$%^&*()',
                Authorization: 'Bearer p@$$w0rd!#$%^&*()',
            });
        });

        it('should return new object each time', () => {
            process.env.LETTA_PASSWORD = 'test-password';
            const server = new LettaServer();

            const headers1 = server.getApiHeaders();
            const headers2 = server.getApiHeaders();

            expect(headers1).not.toBe(headers2);
            expect(headers1).toEqual(headers2);
        });

        it('should not modify returned headers object', () => {
            process.env.LETTA_PASSWORD = 'test-password';
            const server = new LettaServer();

            const headers = server.getApiHeaders();
            const originalHeaders = { ...headers };

            // Modify headers
            headers['Custom-Header'] = 'custom-value';
            delete headers.Authorization;

            // Next call should return original headers
            const newHeaders = server.getApiHeaders();
            expect(newHeaders).toEqual(originalHeaders);
            expect(newHeaders).not.toHaveProperty('Custom-Header');
        });
    });

    describe('URL Construction', () => {
        it('should handle various URL formats', () => {
            const urlTests = [
                { input: 'http://localhost:3000', expected: 'http://localhost:3000/v1' },
                { input: 'https://api.letta.com', expected: 'https://api.letta.com/v1' },
                { input: 'https://letta.com/api', expected: 'https://letta.com/api/v1' },
                { input: 'http://192.168.1.1:8080', expected: 'http://192.168.1.1:8080/v1' },
            ];

            urlTests.forEach(({ input, expected }) => {
                process.env.LETTA_BASE_URL = input;
                process.env.LETTA_PASSWORD = 'test';

                new LettaServer();

                expect(axios.create).toHaveBeenCalledWith(
                    expect.objectContaining({
                        baseURL: expected,
                    }),
                );

                vi.clearAllMocks();
            });
        });

        it('should store the constructed API base URL', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            const server = new LettaServer();
            expect(server.apiBase).toBe('https://api.letta.com/v1');
        });
    });

    describe('API Method Availability', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should provide all HTTP method functions', () => {
            const server = new LettaServer();

            expect(server.api.get).toBeDefined();
            expect(server.api.post).toBeDefined();
            expect(server.api.put).toBeDefined();
            expect(server.api.patch).toBeDefined();
            expect(server.api.delete).toBeDefined();
            expect(server.api.request).toBeDefined();
        });

        it('should allow calling HTTP methods', async () => {
            const server = new LettaServer();

            // Mock responses
            mockAxiosInstance.get.mockResolvedValue({ data: 'get response' });
            mockAxiosInstance.post.mockResolvedValue({ data: 'post response' });
            mockAxiosInstance.put.mockResolvedValue({ data: 'put response' });
            mockAxiosInstance.patch.mockResolvedValue({ data: 'patch response' });
            mockAxiosInstance.delete.mockResolvedValue({ data: 'delete response' });

            // Test each method
            expect(await server.api.get('/test')).toEqual({ data: 'get response' });
            expect(await server.api.post('/test', {})).toEqual({ data: 'post response' });
            expect(await server.api.put('/test', {})).toEqual({ data: 'put response' });
            expect(await server.api.patch('/test', {})).toEqual({ data: 'patch response' });
            expect(await server.api.delete('/test')).toEqual({ data: 'delete response' });
        });
    });

    describe('Integration with Tool Handlers', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should work with tool handler patterns', async () => {
            const server = new LettaServer();
            const headers = server.getApiHeaders();

            // Simulate a tool handler making an API call
            mockAxiosInstance.get.mockResolvedValue({
                data: { agents: [] },
            });

            await server.api.get('/agents', { headers });

            expect(mockAxiosInstance.get).toHaveBeenCalledWith('/agents', {
                headers: expect.objectContaining({
                    Authorization: 'Bearer test-password',
                    'X-BARE-PASSWORD': 'password test-password',
                }),
            });
        });

        it('should handle API errors properly', async () => {
            const server = new LettaServer();
            const apiError = new Error('API Error');
            apiError.response = {
                status: 404,
                data: { error: 'Not found' },
            };

            mockAxiosInstance.get.mockRejectedValue(apiError);

            await expect(server.api.get('/agents')).rejects.toThrow('API Error');
        });
    });

    describe('Mock Server API Client', () => {
        it('should work with mock server api', () => {
            const mockServer = createMockLettaServer();

            expect(mockServer.api).toBeDefined();
            expect(mockServer.api.get).toBeDefined();
            expect(mockServer.getApiHeaders).toBeDefined();

            const headers = mockServer.getApiHeaders();
            expect(headers).toEqual({
                Authorization: 'Bearer test-token',
                'Content-Type': 'application/json',
            });
        });
    });

    describe('Security Considerations', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
        });

        it('should not expose password in axios defaults', () => {
            process.env.LETTA_PASSWORD = 'secret-password';
            new LettaServer();

            const createCall = axios.create.mock.calls[0][0];
            expect(JSON.stringify(createCall)).not.toContain('secret-password');
        });

        it('should only include auth headers when explicitly requested', () => {
            process.env.LETTA_PASSWORD = 'secret-password';
            const server = new LettaServer();

            // The created axios instance should not have auth headers by default
            const createCall = axios.create.mock.calls[0][0];
            expect(createCall.headers).not.toHaveProperty('Authorization');
            expect(createCall.headers).not.toHaveProperty('X-BARE-PASSWORD');

            // Auth headers should only be available through getApiHeaders
            const authHeaders = server.getApiHeaders();
            expect(authHeaders.Authorization).toBe('Bearer secret-password');
        });
    });

    describe('Environment Variable Edge Cases', () => {
        it('should handle very long base URLs', () => {
            const longUrl = 'https://' + 'a'.repeat(1000) + '.com';
            process.env.LETTA_BASE_URL = longUrl;
            process.env.LETTA_PASSWORD = 'test';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: longUrl + '/v1',
                }),
            );
        });

        it('should handle URLs with query parameters', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com?key=value';
            process.env.LETTA_PASSWORD = 'test';

            new LettaServer();

            // Should append /v1 before query params (current behavior)
            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://api.letta.com?key=value/v1',
                }),
            );
        });

        it('should handle URLs with fragments', () => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com#section';
            process.env.LETTA_PASSWORD = 'test';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://api.letta.com#section/v1',
                }),
            );
        });
    });

    describe('Multiple Instance Handling', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://api.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should create separate axios instances for each server', () => {
            // Create different mock instances for each call
            const mockInstance1 = {
                get: vi.fn(),
                post: vi.fn(),
                put: vi.fn(),
                patch: vi.fn(),
                delete: vi.fn(),
                request: vi.fn(),
            };
            const mockInstance2 = {
                get: vi.fn(),
                post: vi.fn(),
                put: vi.fn(),
                patch: vi.fn(),
                delete: vi.fn(),
                request: vi.fn(),
            };

            axios.create.mockReturnValueOnce(mockInstance1).mockReturnValueOnce(mockInstance2);

            const server1 = new LettaServer();
            const server2 = new LettaServer();

            expect(axios.create).toHaveBeenCalledTimes(2);
            expect(server1.api).toBe(mockInstance1);
            expect(server2.api).toBe(mockInstance2);
            expect(server1.api).not.toBe(server2.api);
        });

        it('should allow different passwords for different instances', () => {
            const server1 = new LettaServer();

            process.env.LETTA_PASSWORD = 'different-password';
            const server2 = new LettaServer();

            const headers1 = server1.getApiHeaders();
            const headers2 = server2.getApiHeaders();

            expect(headers1.Authorization).toBe('Bearer test-password');
            expect(headers2.Authorization).toBe('Bearer different-password');
        });
    });
});
