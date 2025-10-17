import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { LettaServer } from '../../core/server.js';
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import axios from 'axios';
import { createLogger } from '../../core/logger.js';

// Mock dependencies
vi.mock('@modelcontextprotocol/sdk/server/index.js');
vi.mock('axios');
vi.mock('../../core/logger.js');

describe('LettaServer Initialization (LMP-82)', () => {
    let originalEnv;
    let mockLogger;
    let mockMCPServer;
    let mockAxiosInstance;

    beforeEach(() => {
        // Save original env vars
        originalEnv = {
            LETTA_BASE_URL: process.env.LETTA_BASE_URL,
            LETTA_PASSWORD: process.env.LETTA_PASSWORD,
        };

        // Set up mock logger
        mockLogger = {
            info: vi.fn(),
            warn: vi.fn(),
            error: vi.fn(),
            debug: vi.fn(),
            child: vi.fn().mockReturnThis(),
        };
        createLogger.mockReturnValue(mockLogger);

        // Set up mock MCP server
        mockMCPServer = {
            setRequestHandler: vi.fn(),
            onerror: null,
            connect: vi.fn(),
            close: vi.fn(),
        };
        Server.mockImplementation(() => mockMCPServer);

        // Set up mock axios instance
        mockAxiosInstance = {
            get: vi.fn(),
            post: vi.fn(),
            put: vi.fn(),
            patch: vi.fn(),
            delete: vi.fn(),
            request: vi.fn(),
        };
        axios.create.mockReturnValue(mockAxiosInstance);
    });

    afterEach(() => {
        // Restore original env vars
        process.env.LETTA_BASE_URL = originalEnv.LETTA_BASE_URL;
        process.env.LETTA_PASSWORD = originalEnv.LETTA_PASSWORD;

        // Clear all mocks
        vi.clearAllMocks();
    });

    describe('Environment Variable Validation', () => {
        it('should throw error when LETTA_BASE_URL is not set', () => {
            delete process.env.LETTA_BASE_URL;
            process.env.LETTA_PASSWORD = 'test-password';

            expect(() => {
                new LettaServer();
            }).toThrow('Missing required environment variable: LETTA_BASE_URL');
        });

        it('should throw error when LETTA_BASE_URL is empty string', () => {
            process.env.LETTA_BASE_URL = '';
            process.env.LETTA_PASSWORD = 'test-password';

            expect(() => {
                new LettaServer();
            }).toThrow('Missing required environment variable: LETTA_BASE_URL');
        });

        it('should not throw error when LETTA_PASSWORD is not set', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            delete process.env.LETTA_PASSWORD;

            expect(() => {
                new LettaServer();
            }).not.toThrow();
        });

        it('should use empty string for password when not set', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            delete process.env.LETTA_PASSWORD;

            const server = new LettaServer();
            expect(server.password).toBe('');
        });
    });

    describe('MCP Server Initialization', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should create MCP server with correct configuration', () => {
            new LettaServer();

            expect(Server).toHaveBeenCalledWith(
                {
                    name: 'letta-server',
                    version: '0.1.0',
                },
                {
                    capabilities: {
                        tools: {
                            listChanged: true,
                        },
                        prompts: {
                            listChanged: true,
                        },
                        resources: {
                            subscribe: true,
                            listChanged: true,
                        },
                    },
                },
            );
        });

        it('should set error handler on MCP server', () => {
            new LettaServer();
            const testError = new Error('Test error');

            // Trigger the error handler
            mockMCPServer.onerror(testError);

            expect(mockLogger.error).toHaveBeenCalledWith('MCP Error', { error: testError });
        });

        it('should store reference to MCP server instance', () => {
            const server = new LettaServer();
            expect(server.server).toBe(mockMCPServer);
        });
    });

    describe('Logger Initialization', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should create logger with correct module name', () => {
            new LettaServer();
            expect(createLogger).toHaveBeenCalledWith('LettaServer');
        });

        it('should store logger reference', () => {
            const server = new LettaServer();
            expect(server.logger).toBe(mockLogger);
        });
    });

    describe('API Client Configuration', () => {
        it('should append /v1 to base URL if not present', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            const server = new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://test.letta.com/v1',
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
            expect(server.apiBase).toBe('https://test.letta.com/v1');
        });

        it('should not duplicate /v1 if already present', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com/v1';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'https://test.letta.com/v1',
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
        });

        it('should create axios instance with correct default headers', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            new LettaServer();

            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    headers: {
                        'Content-Type': 'application/json',
                        Accept: 'application/json',
                    },
                }),
            );
        });

        it('should store axios instance reference', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';

            const server = new LettaServer();
            expect(server.api).toBe(mockAxiosInstance);
        });
    });

    describe('Property Initialization', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should initialize all required properties', () => {
            const server = new LettaServer();

            expect(server).toHaveProperty('logger');
            expect(server).toHaveProperty('server');
            expect(server).toHaveProperty('apiBase');
            expect(server).toHaveProperty('password');
            expect(server).toHaveProperty('api');
        });

        it('should store password from environment', () => {
            const server = new LettaServer();
            expect(server.password).toBe('test-password');
        });

        it('should handle special characters in password', () => {
            process.env.LETTA_PASSWORD = 'p@$$w0rd!#$%^&*()';
            const server = new LettaServer();
            expect(server.password).toBe('p@$$w0rd!#$%^&*()');
        });
    });

    describe('Edge Cases', () => {
        it('should handle malformed URLs gracefully', () => {
            process.env.LETTA_BASE_URL = 'not-a-valid-url';
            process.env.LETTA_PASSWORD = 'test-password';

            // Should not throw during construction
            expect(() => {
                new LettaServer();
            }).not.toThrow();

            // axios.create should still be called with the malformed URL
            expect(axios.create).toHaveBeenCalledWith(
                expect.objectContaining({
                    baseURL: 'not-a-valid-url/v1',
                }),
            );
        });

        it('should handle very long passwords', () => {
            const longPassword = 'a'.repeat(1000);
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = longPassword;

            const server = new LettaServer();
            expect(server.password).toBe(longPassword);
        });

        it('should handle URLs with trailing slashes', () => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com/';
            process.env.LETTA_PASSWORD = 'test-password';

            const server = new LettaServer();
            expect(server.apiBase).toBe('https://test.letta.com//v1');
        });
    });

    describe('Multiple Instance Creation', () => {
        beforeEach(() => {
            process.env.LETTA_BASE_URL = 'https://test.letta.com';
            process.env.LETTA_PASSWORD = 'test-password';
        });

        it('should allow creating multiple server instances', () => {
            // Reset mocks to return different instances
            const mockMCPServer2 = {
                setRequestHandler: vi.fn(),
                onerror: null,
                connect: vi.fn(),
                close: vi.fn(),
            };
            const mockAxiosInstance2 = {
                get: vi.fn(),
                post: vi.fn(),
                put: vi.fn(),
                patch: vi.fn(),
                delete: vi.fn(),
                request: vi.fn(),
            };
            const mockLogger2 = {
                info: vi.fn(),
                warn: vi.fn(),
                error: vi.fn(),
                debug: vi.fn(),
                child: vi.fn().mockReturnThis(),
            };

            Server.mockImplementationOnce(() => mockMCPServer);
            Server.mockImplementationOnce(() => mockMCPServer2);
            axios.create.mockReturnValueOnce(mockAxiosInstance);
            axios.create.mockReturnValueOnce(mockAxiosInstance2);
            createLogger.mockReturnValueOnce(mockLogger);
            createLogger.mockReturnValueOnce(mockLogger2);

            const server1 = new LettaServer();
            const server2 = new LettaServer();

            expect(server1).not.toBe(server2);
            expect(server1.server).toBe(mockMCPServer);
            expect(server2.server).toBe(mockMCPServer2);
            expect(server1.api).toBe(mockAxiosInstance);
            expect(server2.api).toBe(mockAxiosInstance2);
            expect(server1.logger).toBe(mockLogger);
            expect(server2.logger).toBe(mockLogger2);
        });

        it('should create separate MCP server instances', () => {
            new LettaServer();
            new LettaServer();

            expect(Server).toHaveBeenCalledTimes(2);
        });

        it('should create separate axios instances', () => {
            new LettaServer();
            new LettaServer();

            expect(axios.create).toHaveBeenCalledTimes(2);
        });
    });
});
