#!/usr/bin/env node
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { McpError, ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import axios from 'axios';
import http from 'http';
import https from 'https';
import { LettaClient, LettaError, LettaTimeoutError } from '@letta-ai/letta-client';
import { createLogger } from './logger.js';

/**
 * Core LettaServer class that handles initialization and API communication
 */
export class LettaServer {
    /**
     * Initialize the Letta MCP server
     */
    constructor() {
        // Create logger for this module
        this.logger = createLogger('LettaServer');

        // Initialize MCP server
        this.server = new Server(
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

        // Set up error handler
        this.server.onerror = (error) => this.logger.error('MCP Error', { error });

        // Flag to track if handlers have been registered
        this.handlersRegistered = false;

        // Validate environment variables
        this.apiBase = process.env.LETTA_BASE_URL ?? '';
        this.password = process.env.LETTA_PASSWORD ?? '';
        if (!this.apiBase) {
            throw new Error('Missing required environment variable: LETTA_BASE_URL');
        }

        // Initialize axios instance (keep for backward compatibility)
        if (!this.apiBase.endsWith('/v1')) {
            this.apiBase = `${this.apiBase}/v1`;
        }

        // Configure HTTP/HTTPS agents with connection pooling
        // These settings follow best practices for production environments:
        // - keepAlive: Reuse TCP connections for multiple requests
        // - maxSockets: Limit concurrent connections per host (prevents exhaustion)
        // - maxFreeSockets: Keep warm connections in pool for faster requests
        // - timeout: Socket timeout for connection establishment
        const httpAgent = new http.Agent({
            keepAlive: true,
            maxSockets: 50, // Max concurrent connections per host
            maxFreeSockets: 10, // Keep 10 connections warm in pool
            timeout: 60000, // 60s socket timeout
        });

        const httpsAgent = new https.Agent({
            keepAlive: true,
            maxSockets: 50,
            maxFreeSockets: 10,
            timeout: 60000,
        });

        this.api = axios.create({
            baseURL: this.apiBase,
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            httpAgent,
            httpsAgent,
            timeout: 30000, // 30s request timeout (same as SDK)
        });

        // Initialize Letta SDK client
        try {
            this.client = new LettaClient({
                token: this.password,
                baseUrl: this.apiBase.replace('/v1', ''), // SDK adds /v1 automatically
                maxRetries: 2,
                timeoutInSeconds: 30,
            });
            this.logger.info('Letta SDK client initialized successfully');
        } catch (error) {
            this.logger.error('Failed to initialize Letta SDK client:', error);
            throw new Error(`SDK initialization failed: ${error.message}`);
        }
    }

    /**
     * Get standard headers for API requests
     * @returns {Object} Headers object
     */
    getApiHeaders() {
        return {
            'Content-Type': 'application/json',
            Accept: 'application/json',
            'X-BARE-PASSWORD': `password ${this.password}`,
            Authorization: `Bearer ${this.password}`,
        };
    }

    /**
     * Create a standard error response
     * @param {Error|string} error - The error object or message
     * @param {string} [context] - Additional context for the error
     * @throws {McpError} Always throws an McpError for proper JSON-RPC handling
     */
    createErrorResponse(error, context) {
        let errorMessage = '';
        let errorCode = ErrorCode.InternalError;

        if (typeof error === 'string') {
            errorMessage = error;
        } else if (error instanceof Error) {
            errorMessage = error.message;

            // Handle specific HTTP error codes
            if (error.response?.status === 404) {
                errorCode = ErrorCode.InvalidRequest;
                errorMessage = `Resource not found: ${error.message}`;
            } else if (error.response?.status === 422) {
                errorCode = ErrorCode.InvalidParams;
                errorMessage = `Validation error: ${error.message}`;
            } else if (error.response?.status === 401 || error.response?.status === 403) {
                errorCode = ErrorCode.InvalidRequest;
                errorMessage = `Authentication/Authorization error: ${error.message}`;
            }
        } else {
            errorMessage = 'Unknown error occurred';
        }

        // Add context if provided
        if (context) {
            errorMessage = `${context}: ${errorMessage}`;
        }

        // Add additional details if available
        if (error?.response?.data) {
            errorMessage += ` Details: ${JSON.stringify(error.response.data)}`;
        }

        throw new McpError(errorCode, errorMessage);
    }

    /**
     * Map HTTP status codes to MCP error codes
     * @param {number} statusCode - HTTP status code
     * @returns {ErrorCode} Corresponding MCP error code
     */
    mapErrorCode(statusCode) {
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
     * Wrapper for SDK calls that converts SDK errors to MCP errors
     * Handles both Letta SDK errors and axios errors for backward compatibility
     * @param {Function} sdkFunction - Async function that makes SDK calls
     * @param {string} [context] - Additional context for error messages
     * @returns {Promise<any>} Result from the SDK call
     * @throws {McpError} Always throws McpError on failure for proper JSON-RPC handling
     */
    async handleSdkCall(sdkFunction, context) {
        try {
            return await sdkFunction();
        } catch (error) {
            this.logger.error('SDK call failed:', { error, context });

            let errorMessage = '';
            let errorCode = ErrorCode.InternalError;

            // Handle Letta SDK errors (LettaError, LettaTimeoutError)
            if (error instanceof LettaError || error instanceof LettaTimeoutError) {
                // SDK error format: { statusCode, body, message }
                const statusCode = error.statusCode || 500;
                errorCode = this.mapErrorCode(statusCode);
                errorMessage = error.message || 'SDK request failed';

                // Include response body if available
                if (error.body) {
                    const bodyStr =
                        typeof error.body === 'string' ? error.body : JSON.stringify(error.body);
                    errorMessage += `\nBody: ${bodyStr}`;
                }
            }
            // Handle axios errors (for operations still using axios)
            else if (error.response) {
                // Axios error format: { response: { status, data } }
                const statusCode = error.response.status || 500;
                errorCode = this.mapErrorCode(statusCode);
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
    }
}
