#!/usr/bin/env node
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { McpError, ErrorCode } from '@modelcontextprotocol/sdk/types.js';
import axios from 'axios';
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

        // Initialize axios instance
        this.apiBase = `${this.apiBase}/v1`;
        this.api = axios.create({
            baseURL: this.apiBase,
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });
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
            // Identify as SDK v1.0 compatible client for proper API behavior
            'User-Agent': 'letta-mcp-server/2.0.1 (sdk-v1.0-compatible)',
            'X-Letta-SDK-Version': '1.0',
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
        let troubleshooting = '';

        if (typeof error === 'string') {
            errorMessage = error;
        } else if (error instanceof Error) {
            errorMessage = error.message;

            // Handle specific HTTP error codes with actionable troubleshooting
            if (error.response?.status === 404) {
                errorCode = ErrorCode.InvalidRequest;
                errorMessage = `Resource not found: ${error.message}`;
                troubleshooting = 'Check that the agent_id or resource ID exists and is correct.';
            } else if (error.response?.status === 422) {
                errorCode = ErrorCode.InvalidParams;
                errorMessage = `Validation error: ${error.message}`;
                troubleshooting = 'Check the request parameters match the expected schema.';
            } else if (error.response?.status === 401 || error.response?.status === 403) {
                errorCode = ErrorCode.InvalidRequest;
                errorMessage = `Authentication/Authorization error: ${error.message}`;
                troubleshooting = 'Check LETTA_PASSWORD environment variable and API credentials.';
            } else if (error.response?.status === 500) {
                // Parse Letta's generic 500 errors and provide helpful context
                const detail = error.response?.data?.detail || '';
                if (detail === 'An unknown error occurred' || detail === '') {
                    errorMessage = 'Letta server internal error';
                    troubleshooting = [
                        'Common causes:',
                        '1. Agent embedding model misconfigured (check agent embedding_config)',
                        '2. Invalid API key for embedding provider (OpenAI, Ollama, etc.)',
                        '3. Embedding service unreachable',
                        '4. Agent was created with different embedding model than currently configured',
                        '',
                        'To diagnose: Check Letta server logs with: docker logs <letta-container>',
                    ].join('\n');
                } else {
                    errorMessage = `Letta server error: ${detail}`;
                }
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
            const data = error.response.data;
            // Don't repeat generic "unknown error" message for 500 errors
            const isGeneric500 =
                error.response?.status === 500 && data.detail === 'An unknown error occurred';
            if (!isGeneric500) {
                try {
                    errorMessage += ` Details: ${JSON.stringify(data)}`;
                } catch {
                    // Handle circular references gracefully
                    errorMessage += ' Details: [complex data - could not serialize]';
                }
            }
        }

        // Add troubleshooting hints
        if (troubleshooting) {
            errorMessage += `\n\nTroubleshooting:\n${troubleshooting}`;
        }

        throw new McpError(errorCode, errorMessage);
    }
}
