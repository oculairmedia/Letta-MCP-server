/**
 * Tests for letta_tool_manager tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaToolManager, lettaToolManagerDefinition } from '../tools/letta-tool-manager.js';

// Mock server with API client and SDK client
const createMockServer = () => ({
    api: {
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
        patch: vi.fn(),
    },
    client: {
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
    },
    getApiHeaders: vi.fn(() => ({
        'Content-Type': 'application/json',
        Authorization: 'Bearer test-token',
    })),
    handleSdkCall: vi.fn(async (fn) => await fn()),
});

describe('letta_tool_manager', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaToolManagerDefinition.name).toBe('letta_tool_manager');
        });

        it('should have proper input schema with all operations', () => {
            expect(lettaToolManagerDefinition.inputSchema.properties.operation.enum).toEqual([
                'list',
                'get',
                'create',
                'update',
                'delete',
                'upsert',
                'attach',
                'detach',
                'bulk_attach',
                'generate_from_prompt',
                'generate_schema',
                'run_from_source',
                'add_base_tools',
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaToolManagerDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('List Operation', () => {
        it('should successfully list all tools', async () => {
            const mockTools = [
                { id: 'tool-1', name: 'Tool 1', description: 'Test tool 1', tags: ['test'] },
                { id: 'tool-2', name: 'Tool 2', description: 'Test tool 2', tags: ['demo'] },
            ];

            mockServer.client.tools.list.mockResolvedValue(mockTools);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'list',
            });

            expect(mockServer.client.tools.list).toHaveBeenCalledWith({});

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('list');
            expect(response.tools).toHaveLength(2);
            expect(response.pagination).toBeDefined();
        });

        it('should apply pagination parameters', async () => {
            mockServer.client.tools.list.mockResolvedValue([]);

            await handleLettaToolManager(mockServer, {
                operation: 'list',
                options: {
                    pagination: {
                        limit: 10,
                    },
                },
            });

            expect(mockServer.client.tools.list).toHaveBeenCalledWith({
                limit: 10,
            });
        });

        it('should apply filter parameters', async () => {
            mockServer.client.tools.list.mockResolvedValue([]);

            await handleLettaToolManager(mockServer, {
                operation: 'list',
                options: {
                    filters: {
                        name: 'search',
                        tags: ['python', 'utility'],
                        source_type: 'python',
                    },
                },
            });

            expect(mockServer.client.tools.list).toHaveBeenCalledWith({
                name: 'search',
                tags: ['python', 'utility'],
                source_type: 'python',
            });
        });
    });

    describe('Get Operation', () => {
        it('should successfully get tool details', async () => {
            const mockTool = {
                id: 'tool-123',
                name: 'Test Tool',
                description: 'A test tool',
                source_code: 'def test(): pass',
                tags: ['test'],
                json_schema: { type: 'object' },
            };

            mockServer.client.tools.retrieve.mockResolvedValue(mockTool);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'get',
                tool_id: 'tool-123',
            });

            expect(mockServer.client.tools.retrieve).toHaveBeenCalledWith('tool-123');

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('get');
            expect(response.tool_id).toBe('tool-123');
            expect(response.tool.name).toBe('Test Tool');
        });

        it('should throw error when tool_id is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'get',
                }),
            ).rejects.toThrow('tool_id is required');
        });
    });

    describe('Update Operation', () => {
        it('should successfully update a tool', async () => {
            const toolData = {
                name: 'Updated Tool',
                description: 'Updated description',
                source_code: 'def updated(): pass',
            };

            mockServer.client.tools.modify.mockResolvedValue({
                id: 'tool-123',
                ...toolData,
            });

            const result = await handleLettaToolManager(mockServer, {
                operation: 'update',
                tool_id: 'tool-123',
                tool_data: toolData,
            });

            expect(mockServer.client.tools.modify).toHaveBeenCalledWith('tool-123', toolData);

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('update');
            expect(response.tool_id).toBe('tool-123');
        });

        it('should throw error when tool_id is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'update',
                    tool_data: { name: 'test' },
                }),
            ).rejects.toThrow('tool_id is required');
        });

        it('should throw error when tool_data is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'update',
                    tool_id: 'tool-123',
                }),
            ).rejects.toThrow('tool_data is required');
        });
    });

    describe('Delete Operation', () => {
        it('should successfully delete a tool', async () => {
            mockServer.client.tools.delete.mockResolvedValue({});

            const result = await handleLettaToolManager(mockServer, {
                operation: 'delete',
                tool_id: 'tool-123',
            });

            expect(mockServer.client.tools.delete).toHaveBeenCalledWith('tool-123');

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('delete');
            expect(response.tool_id).toBe('tool-123');
        });

        it('should throw error when tool_id is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'delete',
                }),
            ).rejects.toThrow('tool_id is required');
        });
    });

    describe('Upsert Operation', () => {
        it('should create new tool when it does not exist', async () => {
            const mockTool = { id: 'tool-new', name: 'New Tool', was_updated: false };
            mockServer.client.tools.upsert.mockResolvedValue(mockTool);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'upsert',
                tool_data: {
                    name: 'New Tool',
                    description: 'A new tool',
                    source_code: 'def new(): pass',
                },
            });

            expect(mockServer.client.tools.upsert).toHaveBeenCalledWith({
                name: 'New Tool',
                description: 'A new tool',
                sourceCode: 'def new(): pass',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('upsert');
            expect(response.tool_id).toBe('tool-new');
        });

        it('should update existing tool when it exists', async () => {
            const mockTool = { id: 'tool-123', name: 'Existing Tool', was_updated: true };
            mockServer.client.tools.upsert.mockResolvedValue(mockTool);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'upsert',
                tool_data: {
                    name: 'Existing Tool',
                    description: 'Updated',
                },
            });

            expect(mockServer.client.tools.upsert).toHaveBeenCalledWith({
                name: 'Existing Tool',
                description: 'Updated',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.tool_id).toBe('tool-123');
        });

        it('should throw error when tool_data is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'upsert',
                }),
            ).rejects.toThrow('tool_data is required');
        });

        it('should throw error when tool_data.name is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'upsert',
                    tool_data: { description: 'test' },
                }),
            ).rejects.toThrow('tool_data.name is required');
        });
    });

    describe('Detach Operation', () => {
        it('should successfully detach tool from agent', async () => {
            const mockAgentState = { id: 'agent-123', name: 'Test Agent' };
            mockServer.client.agents.tools.detach.mockResolvedValue(mockAgentState);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'detach',
                agent_id: 'agent-123',
                tool_id: 'tool-456',
            });

            expect(mockServer.client.agents.tools.detach).toHaveBeenCalledWith(
                'agent-123',
                'tool-456',
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('detach');
            expect(response.detached_from_agent).toBe('agent-123');
        });

        it('should throw error when agent_id is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'detach',
                    tool_id: 'tool-456',
                }),
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when tool_id is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'detach',
                    agent_id: 'agent-123',
                }),
            ).rejects.toThrow('tool_id is required');
        });
    });

    describe('Generate From Prompt Operation', () => {
        it('should successfully generate tool from prompt', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    source_code: 'def generated(): pass',
                    name: 'Generated Tool',
                },
            });

            const result = await handleLettaToolManager(mockServer, {
                operation: 'generate_from_prompt',
                prompt: 'Create a tool that calculates fibonacci numbers',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/tools/generate',
                { prompt: 'Create a tool that calculates fibonacci numbers' },
                expect.any(Object),
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('generate_from_prompt');
            expect(response.generated_code).toBeDefined();
        });

        it('should throw error when prompt is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'generate_from_prompt',
                }),
            ).rejects.toThrow('prompt is required');
        });
    });

    describe('Generate Schema Operation', () => {
        it('should successfully generate schema from source code', async () => {
            mockServer.api.post.mockResolvedValue({
                data: {
                    schema: {
                        type: 'object',
                        properties: { arg1: { type: 'string' } },
                    },
                },
            });

            const result = await handleLettaToolManager(mockServer, {
                operation: 'generate_schema',
                source_code: 'def my_tool(arg1: str): pass',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/tools/schema',
                { source_code: 'def my_tool(arg1: str): pass' },
                expect.any(Object),
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('generate_schema');
            expect(response.generated_schema).toBeDefined();
        });

        it('should throw error when source_code is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'generate_schema',
                }),
            ).rejects.toThrow('source_code is required');
        });
    });

    describe('Run From Source Operation', () => {
        it('should successfully run tool from source code', async () => {
            const mockResult = {
                result: 'success',
                output: 'Tool executed successfully',
            };
            mockServer.client.tools.runToolFromSource.mockResolvedValue(mockResult);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'run_from_source',
                source_code: 'def my_tool(): return "Hello"',
                tool_args: { arg1: 'value1' },
            });

            expect(mockServer.client.tools.runToolFromSource).toHaveBeenCalledWith({
                sourceCode: 'def my_tool(): return "Hello"',
                args: { arg1: 'value1' },
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('run_from_source');
            expect(response.execution_result).toBeDefined();
        });

        it('should run without tool_args if not provided', async () => {
            mockServer.client.tools.runToolFromSource.mockResolvedValue({
                result: 'success',
            });

            await handleLettaToolManager(mockServer, {
                operation: 'run_from_source',
                source_code: 'def my_tool(): pass',
            });

            expect(mockServer.client.tools.runToolFromSource).toHaveBeenCalledWith({
                sourceCode: 'def my_tool(): pass',
                args: {},
            });
        });

        it('should throw error when source_code is missing', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'run_from_source',
                }),
            ).rejects.toThrow('source_code is required');
        });
    });

    describe('Add Base Tools Operation', () => {
        it('should successfully add base tools', async () => {
            const mockResult = { count: 5, added: 5 };
            mockServer.client.tools.upsertBaseTools.mockResolvedValue(mockResult);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'add_base_tools',
            });

            expect(mockServer.client.tools.upsertBaseTools).toHaveBeenCalledWith();

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('add_base_tools');
            expect(response.added_tools_count).toBe(5);
        });

        it('should handle response with tools array', async () => {
            const mockResult = ['tool1', 'tool2', 'tool3'];
            mockServer.client.tools.upsertBaseTools.mockResolvedValue(mockResult);

            const result = await handleLettaToolManager(mockServer, {
                operation: 'add_base_tools',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.added_tools_count).toBe(3);
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'invalid',
                }),
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.client.tools.list.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'list',
                }),
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.client.tools.retrieve.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaToolManager(mockServer, {
                    operation: 'get',
                    tool_id: 'tool-123',
                }),
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
