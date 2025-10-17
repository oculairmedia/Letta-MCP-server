import { vi } from 'vitest';
import { ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import { McpError } from '@modelcontextprotocol/sdk/types.js';

/**
 * Creates a mock LettaServer instance for testing
 */
export function createMockLettaServer(overrides = {}) {
    const mockApi = {
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        patch: vi.fn(),
        delete: vi.fn(),
        request: vi.fn(),
    };

    const mockServer = {
        api: mockApi,
        client: createMockLettaClient(),
        server: createMockMCPServer(),
        logger: createMockLogger(),
        getApiHeaders: vi.fn().mockReturnValue({
            Authorization: 'Bearer test-token',
            'Content-Type': 'application/json',
        }),
        createErrorResponse: vi.fn((errorOrMessage, context) => {
            let message =
                typeof errorOrMessage === 'string'
                    ? errorOrMessage
                    : errorOrMessage.message || 'Unknown error';
            if (context) {
                message = `${context}: ${message}`;
            }
            throw new Error(message);
        }),
        handleSdkCall: vi.fn(async (sdkFunction, context) => {
            try {
                return await sdkFunction();
            } catch (error) {
                let errorMessage = '';
                let errorCode = ErrorCode.InternalError;

                // Handle axios errors (for operations still using axios)
                if (error.response) {
                    // Axios error format: { response: { status, data } }
                    const statusCode = error.response.status || 500;
                    errorCode = mapErrorCode(statusCode);
                    errorMessage = error.message || 'Request failed';

                    // Include response data if available
                    if (error.response.data) {
                        const dataStr =
                            typeof error.response.data === 'string'
                                ? error.response.data
                                : JSON.stringify(error.response.data);
                        errorMessage += ` - ${dataStr}`;
                    }
                }
                // Handle generic errors
                else if (error instanceof Error) {
                    errorMessage = error.message || 'Unknown error occurred';
                } else {
                    errorMessage = 'Unknown SDK error occurred';
                }

                // Add context if provided
                if (context) {
                    errorMessage = `${context}: ${errorMessage}`;
                }

                throw new McpError(errorCode, errorMessage);
            }
        }),
        ...overrides,
    };

    return mockServer;
}

/**
 * Creates a mock Letta SDK client
 */
export function createMockLettaClient() {
    // Create nested mock client structure matching the real SDK
    const mockClient = {
        // Top-level tools methods
        tools: {
            list: vi.fn(),
            retrieve: vi.fn(),
            create: vi.fn(),
            modify: vi.fn(),
            delete: vi.fn(),
            upsert: vi.fn(),
            addMcpServer: vi.fn(),
            updateMcpServer: vi.fn(),
            deleteMcpServer: vi.fn(),
            testMcpServer: vi.fn(),
            connectMcpServer: vi.fn(),
            listMcpServers: vi.fn(),
            listMcpToolsByServer: vi.fn(),
            addMcpTool: vi.fn(),
            upsertBaseTools: vi.fn(),
            runToolFromSource: vi.fn(),
        },

        // Agents methods
        agents: {
            list: vi.fn(),
            retrieve: vi.fn(),
            create: vi.fn(),
            modify: vi.fn(),
            delete: vi.fn(),
            destroy: vi.fn(),

            // Agent tools methods
            tools: {
                list: vi.fn(),
                attach: vi.fn(),
                detach: vi.fn(),
            },

            // Agent core memory methods
            coreMemory: {
                get: vi.fn(),
                update: vi.fn(),
            },

            // Agent blocks methods
            blocks: {
                list: vi.fn(),
                retrieve: vi.fn(),
                create: vi.fn(),
                update: vi.fn(),
                destroy: vi.fn(),
                attach: vi.fn(),
                detach: vi.fn(),
            },

            // Agent passages methods
            passages: {
                list: vi.fn(),
                retrieve: vi.fn(),
                create: vi.fn(),
                update: vi.fn(),
                destroy: vi.fn(),
            },

            // Agent messages methods
            messages: {
                list: vi.fn(),
                create: vi.fn(),
            },

            // Agent sources methods
            sources: {
                list: vi.fn(),
                attach: vi.fn(),
                detach: vi.fn(),
            },

            // Agent files methods
            files: {
                list: vi.fn(),
                attach: vi.fn(),
                detach: vi.fn(),
            },

            // Agent folders methods
            folders: {
                list: vi.fn(),
                attach: vi.fn(),
                detach: vi.fn(),
            },
        },

        // Standalone blocks methods
        blocks: {
            list: vi.fn(),
            retrieve: vi.fn(),
            create: vi.fn(),
            update: vi.fn(),
            destroy: vi.fn(),
        },

        // Sources methods
        sources: {
            list: vi.fn(),
            retrieve: vi.fn(),
            create: vi.fn(),
            modify: vi.fn(),
            delete: vi.fn(),
        },

        // Jobs methods
        jobs: {
            list: vi.fn(),
            retrieve: vi.fn(),
            cancel: vi.fn(),
        },

        // Folders methods
        folders: {
            list: vi.fn(),
            retrieve: vi.fn(),
            create: vi.fn(),
            update: vi.fn(),
            delete: vi.fn(),
        },
    };

    return mockClient;
}

/**
 * Map HTTP status codes to MCP error codes
 * @param {number} statusCode - HTTP status code
 * @returns {ErrorCode} Corresponding MCP error code
 */
function mapErrorCode(statusCode) {
    switch (statusCode) {
        case 400:
            return ErrorCode.InvalidParams;
        case 401:
        case 403:
            return ErrorCode.InvalidRequest;
        case 404:
            return ErrorCode.InvalidRequest;
        case 422:
            return ErrorCode.InvalidParams;
        case 429:
            return ErrorCode.InvalidRequest;
        case 500:
        case 502:
        case 503:
        case 504:
            return ErrorCode.InternalError;
        default:
            return ErrorCode.InternalError;
    }
}

/**
 * Creates a mock MCP Server instance
 */
export function createMockMCPServer() {
    const mockMCPServer = {
        setRequestHandler: vi.fn(),
        onerror: vi.fn(),
        connect: vi.fn(),
        close: vi.fn(),
        handleRequest: vi.fn(),
        _handlers: new Map(),
    };

    // Mock the setRequestHandler to store handlers
    mockMCPServer.setRequestHandler.mockImplementation((type, handler) => {
        mockMCPServer._handlers.set(type, handler);
    });

    // Helper to trigger a handler
    mockMCPServer.triggerHandler = async (type, args) => {
        const handler = mockMCPServer._handlers.get(type);
        if (!handler) {
            throw new Error(`No handler registered for ${type}`);
        }
        return await handler(args);
    };

    // Mock handleRequest for transport tests
    mockMCPServer.handleRequest.mockImplementation(async (request) => {
        if (request.method === 'initialize') {
            return {
                protocolVersion: '2025-06-18',
                capabilities: {},
                serverInfo: {
                    name: 'letta-mcp-server',
                    version: '1.0.0',
                },
            };
        }
        return { result: {} };
    });

    return mockMCPServer;
}

/**
 * Creates a mock logger
 */
export function createMockLogger() {
    return {
        info: vi.fn(),
        warn: vi.fn(),
        error: vi.fn(),
        debug: vi.fn(),
        child: vi.fn().mockReturnThis(),
    };
}

/**
 * Creates a mock transport
 */
export function createMockTransport() {
    return {
        start: vi.fn(),
        close: vi.fn(),
        send: vi.fn(),
    };
}
