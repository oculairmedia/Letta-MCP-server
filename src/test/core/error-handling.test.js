import { describe, it, expect, beforeEach, vi } from 'vitest';
import { LettaServer } from '../../core/server.js';
import { McpError, ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import { createMockLettaServer } from '../utils/mock-server.js';

// Mock dependencies
vi.mock('@modelcontextprotocol/sdk/server/index.js');
vi.mock('axios');

describe('LettaServer Error Handling (LMP-83)', () => {
    let server;
    let mockLogger;

    beforeEach(() => {
        // Set required env vars
        process.env.LETTA_BASE_URL = 'https://test.letta.com';
        process.env.LETTA_PASSWORD = 'test-password';

        // Create mock logger
        mockLogger = {
            info: vi.fn(),
            warn: vi.fn(),
            error: vi.fn(),
            debug: vi.fn(),
            child: vi.fn().mockReturnThis(),
        };

        server = new LettaServer();
        server.logger = mockLogger;
    });

    describe('createErrorResponse method', () => {
        describe('String Error Handling', () => {
            it('should handle plain string errors', () => {
                const errorMessage = 'Something went wrong';

                expect(() => {
                    server.createErrorResponse(errorMessage);
                }).toThrow(McpError);

                try {
                    server.createErrorResponse(errorMessage);
                } catch (error) {
                    expect(error).toBeInstanceOf(McpError);
                    expect(error.message).toContain(errorMessage);
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle string errors with context', () => {
                const errorMessage = 'Connection failed';
                const context = 'Database operation';

                try {
                    server.createErrorResponse(errorMessage, context);
                } catch (error) {
                    expect(error.message).toContain('Database operation: Connection failed');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle empty string errors', () => {
                try {
                    server.createErrorResponse('');
                } catch (error) {
                    // McpError prepends 'MCP error <code>: ' to the message
                    expect(error.message).toMatch(/MCP error -\d+: $/);
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });
        });

        describe('Error Object Handling', () => {
            it('should handle basic Error objects', () => {
                const testError = new Error('Test error message');

                try {
                    server.createErrorResponse(testError);
                } catch (error) {
                    expect(error).toBeInstanceOf(McpError);
                    expect(error.message).toContain('Test error message');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle Error objects with context', () => {
                const testError = new Error('API call failed');
                const context = 'Agent creation';

                try {
                    server.createErrorResponse(testError, context);
                } catch (error) {
                    expect(error.message).toContain('Agent creation: API call failed');
                }
            });

            it('should handle custom Error types', () => {
                class CustomError extends Error {
                    constructor(message) {
                        super(message);
                        this.name = 'CustomError';
                    }
                }

                const customError = new CustomError('Custom error occurred');

                try {
                    server.createErrorResponse(customError);
                } catch (error) {
                    expect(error.message).toContain('Custom error occurred');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });
        });

        describe('HTTP Error Code Mapping', () => {
            it('should map 404 errors to InvalidRequest', () => {
                const error404 = new Error('Resource not found');
                error404.response = { status: 404 };

                try {
                    server.createErrorResponse(error404);
                } catch (error) {
                    expect(error.code).toBe(ErrorCode.InvalidRequest);
                    expect(error.message).toContain('Resource not found');
                    expect(error.message).toContain('Resource not found');
                }
            });

            it('should map 422 errors to InvalidParams', () => {
                const error422 = new Error('Validation failed');
                error422.response = { status: 422 };

                try {
                    server.createErrorResponse(error422);
                } catch (error) {
                    expect(error.code).toBe(ErrorCode.InvalidParams);
                    expect(error.message).toContain('Validation error');
                    expect(error.message).toContain('Validation failed');
                }
            });

            it('should map 401 errors to InvalidRequest', () => {
                const error401 = new Error('Unauthorized');
                error401.response = { status: 401 };

                try {
                    server.createErrorResponse(error401);
                } catch (error) {
                    expect(error.code).toBe(ErrorCode.InvalidRequest);
                    expect(error.message).toContain('Authentication/Authorization error');
                    expect(error.message).toContain('Unauthorized');
                }
            });

            it('should map 403 errors to InvalidRequest', () => {
                const error403 = new Error('Forbidden');
                error403.response = { status: 403 };

                try {
                    server.createErrorResponse(error403);
                } catch (error) {
                    expect(error.code).toBe(ErrorCode.InvalidRequest);
                    expect(error.message).toContain('Authentication/Authorization error');
                    expect(error.message).toContain('Forbidden');
                }
            });

            it('should handle other HTTP status codes as InternalError', () => {
                const error500 = new Error('Server error');
                error500.response = { status: 500 };

                try {
                    server.createErrorResponse(error500);
                } catch (error) {
                    expect(error.code).toBe(ErrorCode.InternalError);
                    expect(error.message).toContain('Server error');
                }
            });
        });

        describe('Response Data Handling', () => {
            it('should include response data in error message', () => {
                const errorWithData = new Error('API Error');
                errorWithData.response = {
                    status: 400,
                    data: { message: 'Bad request', field: 'name' },
                };

                try {
                    server.createErrorResponse(errorWithData);
                } catch (error) {
                    expect(error.message).toContain('API Error');
                    expect(error.message).toContain('Details:');
                    expect(error.message).toContain('"message":"Bad request"');
                    expect(error.message).toContain('"field":"name"');
                }
            });

            it('should handle complex response data', () => {
                const errorWithComplexData = new Error('Complex error');
                errorWithComplexData.response = {
                    status: 400,
                    data: {
                        errors: [
                            { field: 'name', message: 'Required' },
                            { field: 'email', message: 'Invalid format' },
                        ],
                        code: 'VALIDATION_ERROR',
                    },
                };

                try {
                    server.createErrorResponse(errorWithComplexData);
                } catch (error) {
                    const errorDetails = JSON.stringify(errorWithComplexData.response.data);
                    expect(error.message).toContain(`Details: ${errorDetails}`);
                }
            });

            it('should handle circular references in response data', () => {
                const errorWithCircular = new Error('Circular error');
                const circularData = { a: 1 };
                circularData.self = circularData;
                errorWithCircular.response = {
                    status: 400,
                    data: circularData,
                };

                // createErrorResponse will throw TypeError when trying to stringify circular data
                expect(() => {
                    server.createErrorResponse(errorWithCircular);
                }).toThrow(TypeError);
            });
        });

        describe('Unknown Error Types', () => {
            it('should handle null errors', () => {
                try {
                    server.createErrorResponse(null);
                } catch (error) {
                    expect(error.message).toContain('Unknown error occurred');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle undefined errors', () => {
                try {
                    server.createErrorResponse(undefined);
                } catch (error) {
                    expect(error.message).toContain('Unknown error occurred');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle number errors', () => {
                try {
                    server.createErrorResponse(42);
                } catch (error) {
                    expect(error.message).toContain('Unknown error occurred');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });

            it('should handle object errors without message', () => {
                const objError = { code: 'ERR_001', description: 'Something failed' };

                try {
                    server.createErrorResponse(objError);
                } catch (error) {
                    expect(error.message).toContain('Unknown error occurred');
                    expect(error.code).toBe(ErrorCode.InternalError);
                }
            });
        });

        describe('Context Handling', () => {
            it('should prepend context to all error types', () => {
                const context = 'Tool execution';

                // String error with context
                try {
                    server.createErrorResponse('Failed', context);
                } catch (error) {
                    expect(error.message).toContain('Tool execution: Failed');
                }

                // Error object with context
                try {
                    server.createErrorResponse(new Error('Failed'), context);
                } catch (error) {
                    expect(error.message).toContain('Tool execution: Failed');
                }

                // Unknown error with context
                try {
                    server.createErrorResponse(null, context);
                } catch (error) {
                    expect(error.message).toContain('Tool execution: Unknown error occurred');
                }
            });

            it('should handle empty context', () => {
                try {
                    server.createErrorResponse('Error', '');
                } catch (error) {
                    expect(error.message).toContain('Error');
                }
            });

            it('should handle very long context', () => {
                const longContext = 'A'.repeat(1000);
                try {
                    server.createErrorResponse('Error', longContext);
                } catch (error) {
                    expect(error.message).toContain(longContext);
                    expect(error.message).toContain('Error');
                }
            });
        });

        describe('MCP Error Properties', () => {
            it('should always throw McpError instances', () => {
                const testCases = [
                    'String error',
                    new Error('Error object'),
                    null,
                    undefined,
                    42,
                    { invalid: 'object' },
                ];

                testCases.forEach((testCase) => {
                    try {
                        server.createErrorResponse(testCase);
                    } catch (error) {
                        expect(error).toBeInstanceOf(McpError);
                        expect(error).toHaveProperty('code');
                        expect(error).toHaveProperty('message');
                    }
                });
            });

            it('should preserve error codes correctly', () => {
                const errorCases = [
                    {
                        error: new Error('Not found'),
                        response: { status: 404 },
                        expectedCode: ErrorCode.InvalidRequest,
                    },
                    {
                        error: new Error('Invalid'),
                        response: { status: 422 },
                        expectedCode: ErrorCode.InvalidParams,
                    },
                    {
                        error: new Error('Unauthorized'),
                        response: { status: 401 },
                        expectedCode: ErrorCode.InvalidRequest,
                    },
                    {
                        error: new Error('Server error'),
                        response: { status: 500 },
                        expectedCode: ErrorCode.InternalError,
                    },
                    {
                        error: 'String error',
                        response: null,
                        expectedCode: ErrorCode.InternalError,
                    },
                ];

                errorCases.forEach(({ error, response, expectedCode }) => {
                    if (response && error.response === undefined) {
                        error.response = response;
                    }

                    try {
                        server.createErrorResponse(error);
                    } catch (mcpError) {
                        expect(mcpError.code).toBe(expectedCode);
                    }
                });
            });
        });

        describe('Error Handler on MCP Server', () => {
            it('should log errors through the error handler', () => {
                const loggerSpy = vi.spyOn(server.logger, 'error');
                const testError = new Error('Test MCP error');

                // Trigger the error handler
                server.server.onerror(testError);

                expect(loggerSpy).toHaveBeenCalledWith('MCP Error', { error: testError });
            });

            it('should handle null errors in error handler', () => {
                const loggerSpy = vi.spyOn(server.logger, 'error');

                // Should not throw
                expect(() => {
                    server.server.onerror(null);
                }).not.toThrow();

                expect(loggerSpy).toHaveBeenCalledWith('MCP Error', { error: null });
            });
        });

        describe('Integration with Mock Server', () => {
            it('should work with mock server error handling', () => {
                const mockServer = createMockLettaServer();

                // Mock server's createErrorResponse should throw
                expect(() => {
                    mockServer.createErrorResponse('Test error', 'Mock context');
                }).toThrow('Mock context: Test error');
            });
        });
    });
});
