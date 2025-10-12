/**
 * Tests for letta_file_folder_ops tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaFileFolderOps, lettaFileFolderOpsDefinition } from '../files/letta-file-folder-ops.js';

const createMockServer = () => ({
    api: {
        get: vi.fn(),
        post: vi.fn(),
        put: vi.fn(),
        delete: vi.fn(),
    },
    getApiHeaders: vi.fn(() => ({
        'Content-Type': 'application/json',
        Authorization: 'Bearer test-token',
    })),
    handleSdkCall: vi.fn(async (fn) => await fn()),
});

describe('letta_file_folder_ops', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaFileFolderOpsDefinition.name).toBe('letta_file_folder_ops');
        });

        it('should have all 8 operations', () => {
            expect(lettaFileFolderOpsDefinition.inputSchema.properties.operation.enum).toEqual([
                'list_files',
                'open_file',
                'close_file',
                'close_all_files',
                'list_folders',
                'attach_folder',
                'detach_folder',
                'list_agents_in_folder',
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaFileFolderOpsDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('File Operations', () => {
        it('should list files for an agent', async () => {
            mockServer.api.get.mockResolvedValue({
                data: [
                    {
                        id: 'file-1',
                        filename: 'document.pdf',
                        size: 1024,
                        mime_type: 'application/pdf',
                        is_open: true,
                        opened_at: '2025-10-12T00:00:00Z',
                    },
                    {
                        id: 'file-2',
                        name: 'data.csv',
                        size: 2048,
                        type: 'text/csv',
                        is_open: false,
                    },
                ],
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_files',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/agents/agent-123/files', expect.any(Object));

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.files).toHaveLength(2);
            expect(response.files[0].filename).toBe('document.pdf');
            expect(response.files[1].filename).toBe('data.csv');
        });

        it('should handle response with files array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    files: [
                        { id: 'file-3', filename: 'test.txt', size: 512 },
                    ],
                },
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_files',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.files).toHaveLength(1);
        });

        it('should throw error when agent_id is missing for list_files', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, { operation: 'list_files' })
            ).rejects.toThrow('agent_id is required');
        });

        it('should open a file', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'open_file',
                agent_id: 'agent-123',
                file_id: 'file-1',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/files/file-1/open',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.opened).toBe(true);
        });

        it('should throw error when agent_id is missing for open_file', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'open_file',
                    file_id: 'file-1',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when file_id is missing for open_file', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'open_file',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('file_id is required');
        });

        it('should close a file', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'close_file',
                agent_id: 'agent-123',
                file_id: 'file-1',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/files/file-1/close',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.closed).toBe(true);
        });

        it('should throw error when agent_id is missing for close_file', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'close_file',
                    file_id: 'file-1',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when file_id is missing for close_file', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'close_file',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('file_id is required');
        });

        it('should close all files', async () => {
            mockServer.api.post.mockResolvedValue({
                data: { closed_count: 5 },
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'close_all_files',
                agent_id: 'agent-123',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/files/close-all',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.closed_count).toBe(5);
        });

        it('should handle close_all_files with count field', async () => {
            mockServer.api.post.mockResolvedValue({
                data: { count: 3 },
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'close_all_files',
                agent_id: 'agent-123',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.closed_count).toBe(3);
        });

        it('should throw error when agent_id is missing for close_all_files', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, { operation: 'close_all_files' })
            ).rejects.toThrow('agent_id is required');
        });
    });

    describe('Folder Operations', () => {
        it('should list all folders', async () => {
            mockServer.api.get.mockResolvedValue({
                data: [
                    {
                        id: 'folder-1',
                        name: 'Documents',
                        path: '/documents',
                        file_count: 10,
                        agent_count: 2,
                    },
                    {
                        id: 'folder-2',
                        name: 'Data',
                        path: '/data',
                        file_count: 5,
                        agent_count: 1,
                    },
                ],
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_folders',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/folders', expect.any(Object));

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.folders).toHaveLength(2);
            expect(response.folders[0].name).toBe('Documents');
        });

        it('should handle response with folders array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    folders: [
                        { id: 'folder-3', name: 'Test', path: '/test' },
                    ],
                },
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_folders',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.folders).toHaveLength(1);
        });

        it('should attach folder to agent', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'attach_folder',
                agent_id: 'agent-123',
                folder_id: 'folder-1',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/agents/agent-123/folders/folder-1',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.attached).toBe(true);
        });

        it('should throw error when agent_id is missing for attach_folder', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'attach_folder',
                    folder_id: 'folder-1',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when folder_id is missing for attach_folder', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'attach_folder',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('folder_id is required');
        });

        it('should detach folder from agent', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'detach_folder',
                agent_id: 'agent-123',
                folder_id: 'folder-1',
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                '/agents/agent-123/folders/folder-1',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.detached).toBe(true);
        });

        it('should throw error when agent_id is missing for detach_folder', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'detach_folder',
                    folder_id: 'folder-1',
                })
            ).rejects.toThrow('agent_id is required');
        });

        it('should throw error when folder_id is missing for detach_folder', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'detach_folder',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('folder_id is required');
        });

        it('should list agents in folder', async () => {
            mockServer.api.get.mockResolvedValue({
                data: [
                    { id: 'agent-1', name: 'Agent One' },
                    { id: 'agent-2', agent_name: 'Agent Two' },
                ],
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_agents_in_folder',
                folder_id: 'folder-1',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith(
                '/folders/folder-1/agents',
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.agents).toHaveLength(2);
            expect(response.agents[0].name).toBe('Agent One');
            expect(response.agents[1].name).toBe('Agent Two');
        });

        it('should handle response with agents array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    agents: [
                        { id: 'agent-3', name: 'Agent Three' },
                    ],
                },
            });

            const result = await handleLettaFileFolderOps(mockServer, {
                operation: 'list_agents_in_folder',
                folder_id: 'folder-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.agents).toHaveLength(1);
        });

        it('should throw error when folder_id is missing for list_agents_in_folder', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, { operation: 'list_agents_in_folder' })
            ).rejects.toThrow('folder_id is required');
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaFileFolderOps(mockServer, { operation: 'invalid' })
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.api.get.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'list_files',
                    agent_id: 'agent-123',
                })
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.api.post.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaFileFolderOps(mockServer, {
                    operation: 'open_file',
                    agent_id: 'agent-123',
                    file_id: 'file-1',
                })
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
