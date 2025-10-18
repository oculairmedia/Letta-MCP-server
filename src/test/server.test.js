import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { LettaServer } from '../core/server.js';

describe('LettaServer', () => {
    let originalEnv;

    beforeEach(() => {
        // Save original env vars
        originalEnv = {
            LETTA_BASE_URL: process.env.LETTA_BASE_URL,
            LETTA_PASSWORD: process.env.LETTA_PASSWORD,
        };
    });

    afterEach(() => {
        // Restore original env vars
        process.env.LETTA_BASE_URL = originalEnv.LETTA_BASE_URL;
        process.env.LETTA_PASSWORD = originalEnv.LETTA_PASSWORD;
    });

    it('should throw error when LETTA_BASE_URL is not set', () => {
        delete process.env.LETTA_BASE_URL;

        expect(() => {
            new LettaServer();
        }).toThrow(/Missing required environment variable: LETTA_BASE_URL/);
    });

    it('should create server instance with valid config', () => {
        process.env.LETTA_BASE_URL = 'https://test.letta.com/v1';
        process.env.LETTA_PASSWORD = 'test-password';

        const server = new LettaServer();
        expect(server).toBeDefined();
        expect(server.api).toBeDefined();
        expect(server.server).toBeDefined();
        expect(server.logger).toBeDefined();
    });

    it('should create error response with proper format', () => {
        process.env.LETTA_BASE_URL = 'https://test.letta.com/v1';
        process.env.LETTA_PASSWORD = 'test-password';

        const server = new LettaServer();
        const error = new Error('Test error');

        expect(() => {
            server.createErrorResponse(error, 'Test context');
        }).toThrow(/Test context.*Test error/);
    });

    it('should properly configure axios instance', () => {
        process.env.LETTA_BASE_URL = 'https://test.letta.com/v1';
        process.env.LETTA_PASSWORD = 'test-password';

        const server = new LettaServer();

        expect(server.api.defaults.baseURL).toBe('https://test.letta.com/v1');
        // No default timeout is set in the current implementation
    });

    it('should return correct API headers', () => {
        process.env.LETTA_BASE_URL = 'https://test.letta.com/v1';
        process.env.LETTA_PASSWORD = 'test-password';

        const server = new LettaServer();
        const headers = server.getApiHeaders();

        expect(headers).toEqual({
            'Content-Type': 'application/json',
            Accept: 'application/json',
            'X-BARE-PASSWORD': 'password test-password',
            Authorization: 'Bearer test-password',
        });
    });
});
