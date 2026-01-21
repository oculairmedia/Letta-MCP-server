import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
    handleListAgentTools,
    listAgentToolsDefinition,
} from '../../../tools/agents/list-agent-tools.js';

describe('List Agent Tools (LMP-95)', () => {
    let mockServer;

    beforeEach(() => {
        vi.clearAllMocks();

        mockServer = {
            api: {
                get: vi.fn(),
            },
            getApiHeaders: vi.fn().mockReturnValue({
                Authorization: 'Bearer test-password',
            }),
            createErrorResponse: vi.fn((error) => {
                throw error;
            }),
        };
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(listAgentToolsDefinition.name).toBe('list_agent_tools');
        });

        it('should have a description', () => {
            expect(listAgentToolsDefinition.description).toBeDefined();
            expect(listAgentToolsDefinition.description).toContain('tools');
            expect(listAgentToolsDefinition.description).toContain('agent');
        });

        it('should have input schema', () => {
            expect(listAgentToolsDefinition.inputSchema).toBeDefined();
            expect(listAgentToolsDefinition.inputSchema.type).toBe('object');
            expect(listAgentToolsDefinition.inputSchema.properties).toHaveProperty('agent_id');
        });

        it('should require agent_id parameter', () => {
            expect(listAgentToolsDefinition.inputSchema.required).toContain('agent_id');
        });
    });

    describe('List Agent Tools Handler', () => {
        it('should successfully list tools for an agent', async () => {
            const mockAgentData = {
                id: 'agent-123',
                name: 'Test Agent',
                tools: ['tool-1', 'tool-2', 'tool-3'],
            };

            mockServer.api.get.mockResolvedValueOnce({ data: mockAgentData });

            const result = await handleListAgentTools(mockServer, {
                agent_id: 'agent-123',
            });

            // Verify API call
            expect(mockServer.api.get).toHaveBeenCalledWith('/agents/agent-123', {
                headers: {
                    Authorization: 'Bearer test-password',
                },
            });

            // Verify result format
            expect(result).toEqual({
                content: [
                    {
                        type: 'text',
                        text: JSON.stringify({
                            agent_id: 'agent-123',
                            agent_name: 'Test Agent',
                            tool_count: 3,
                            tools: ['tool-1', 'tool-2', 'tool-3'],
                        }),
                    },
                ],
                structuredContent: {
                    agent_id: 'agent-123',
                    agent_name: 'Test Agent',
                    tool_count: 3,
                    tools: ['tool-1', 'tool-2', 'tool-3'],
                },
            });
        });

        it('should handle empty tool list', async () => {
            mockServer.api.get.mockResolvedValueOnce({
                data: {
                    id: 'agent-empty',
                    name: 'Empty Agent',
                    tools: [],
                },
            });

            const result = await handleListAgentTools(mockServer, {
                agent_id: 'agent-empty',
            });

            const parsedText = JSON.parse(result.content[0].text);
            expect(parsedText.tool_count).toBe(0);
            expect(parsedText.tools).toEqual([]);
            expect(parsedText.agent_name).toBe('Empty Agent');
        });

        it('should handle agent without tools field', async () => {
            mockServer.api.get.mockResolvedValueOnce({
                data: {
                    id: 'agent-no-tools',
                    name: 'No Tools Agent',
                    // No tools field
                },
            });

            const result = await handleListAgentTools(mockServer, {
                agent_id: 'agent-no-tools',
            });

            const parsedText = JSON.parse(result.content[0].text);
            expect(parsedText.tool_count).toBe(0);
            expect(parsedText.tools).toEqual([]);
        });

        it('should handle API errors', async () => {
            const mockError = new Error('Failed to fetch agent');
            mockError.response = {
                status: 404,
                data: { detail: 'Agent not found' },
            };
            mockServer.api.get.mockRejectedValueOnce(mockError);

            await expect(
                handleListAgentTools(mockServer, {
                    agent_id: 'nonexistent-agent',
                }),
            ).rejects.toThrow('Failed to fetch agent');

            expect(mockServer.createErrorResponse).toHaveBeenCalledWith(mockError);
        });

        it('should handle missing agent_id', async () => {
            await expect(handleListAgentTools(mockServer, {})).rejects.toThrow(
                'Missing required argument: agent_id',
            );
        });

        it('should handle network errors', async () => {
            const mockError = new Error('Network error');
            mockError.code = 'ECONNREFUSED';
            mockServer.api.get.mockRejectedValueOnce(mockError);

            await expect(
                handleListAgentTools(mockServer, {
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow('Network error');
        });

        it('should handle server errors gracefully', async () => {
            const mockError = new Error('Internal server error');
            mockError.response = {
                status: 500,
                data: { detail: 'Database connection failed' },
            };
            mockServer.api.get.mockRejectedValueOnce(mockError);

            await expect(
                handleListAgentTools(mockServer, {
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow('Internal server error');
        });

        it('should handle unauthorized errors', async () => {
            const mockError = new Error('Unauthorized');
            mockError.response = {
                status: 401,
                data: { detail: 'Invalid authentication credentials' },
            };
            mockServer.api.get.mockRejectedValueOnce(mockError);

            await expect(
                handleListAgentTools(mockServer, {
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow('Unauthorized');
        });

        it('should handle agent with many tools', async () => {
            const manyTools = Array.from({ length: 50 }, (_, i) => `tool-${i + 1}`);
            mockServer.api.get.mockResolvedValueOnce({
                data: {
                    id: 'agent-many-tools',
                    name: 'Many Tools Agent',
                    tools: manyTools,
                },
            });

            const result = await handleListAgentTools(mockServer, {
                agent_id: 'agent-many-tools',
            });

            const parsedText = JSON.parse(result.content[0].text);
            expect(parsedText.tool_count).toBe(50);
            expect(parsedText.tools).toHaveLength(50);
            expect(parsedText.tools[0]).toBe('tool-1');
            expect(parsedText.tools[49]).toBe('tool-50');
        });
    });

    describe('Input Validation', () => {
        it('should validate required parameters', () => {
            const inputSchema = listAgentToolsDefinition.inputSchema;

            // Check schema structure
            expect(inputSchema.type).toBe('object');
            expect(inputSchema.required).toContain('agent_id');
            expect(inputSchema.properties.agent_id.type).toBe('string');

            // The actual validation happens in the handler
            // These tests verify the schema definition is correct
        });
    });
});
