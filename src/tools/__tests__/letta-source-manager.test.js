/**
 * Tests for letta_source_manager tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaSourceManager, lettaSourceManagerDefinition } from '../sources/letta-source-manager.js';

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

describe('letta_source_manager', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaSourceManagerDefinition.name).toBe('letta_source_manager');
        });

        it('should have all 15 operations', () => {
            expect(lettaSourceManagerDefinition.inputSchema.properties.operation.enum).toEqual([
                'list', 'create', 'get', 'update', 'delete', 'count', 'get_by_name',
                'upload_file', 'list_files', 'delete_file', 'list_passages',
                'get_metadata', 'attach_to_agent', 'detach_from_agent', 'list_agent_sources',
            ]);
        });
    });

    describe('CRUD Operations', () => {
        it('should list sources', async () => {
            mockServer.api.get.mockResolvedValue({ data: [
                { id: 's-1', name: 'Source 1', description: 'Test', num_passages: 5 }
            ]});

            const result = await handleLettaSourceManager(mockServer, { operation: 'list' });
            const response = JSON.parse(result.content[0].text);

            expect(response.success).toBe(true);
            expect(response.sources).toHaveLength(1);
        });

        it('should create source', async () => {
            mockServer.api.post.mockResolvedValue({ data: { id: 's-new', name: 'New Source' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'create',
                source_data: { name: 'New Source', description: 'Test' },
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.source_id).toBe('s-new');
        });

        it('should get source', async () => {
            mockServer.api.get.mockResolvedValue({ data: { id: 's-1', name: 'Source' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'get',
                source_id: 's-1',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/sources/s-1', expect.any(Object));
            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
        });

        it('should update source', async () => {
            mockServer.api.put.mockResolvedValue({ data: { id: 's-1', name: 'Updated' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'update',
                source_id: 's-1',
                source_data: { name: 'Updated' },
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
        });

        it('should delete source', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'delete',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
        });

        it('should count sources', async () => {
            mockServer.api.get.mockResolvedValue({ data: { count: 42 }});

            const result = await handleLettaSourceManager(mockServer, { operation: 'count' });
            const response = JSON.parse(result.content[0].text);

            expect(response.count).toBe(42);
        });

        it('should get source by name', async () => {
            mockServer.api.get.mockResolvedValue({ data: { id: 's-1', name: 'Test' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'get_by_name',
                source_name: 'Test',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/sources/name/Test', expect.any(Object));
            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
        });
    });

    describe('File Operations', () => {
        it('should upload file', async () => {
            mockServer.api.post.mockResolvedValue({ data: { id: 'f-1' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'upload_file',
                source_id: 's-1',
                file_data: { file: 'base64data', filename: 'test.txt' },
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.file_id).toBe('f-1');
        });

        it('should list files', async () => {
            mockServer.api.get.mockResolvedValue({ data: [
                { id: 'f-1', filename: 'file1.txt', size: 1024 }
            ]});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'list_files',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.files).toHaveLength(1);
        });

        it('should delete file', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'delete_file',
                source_id: 's-1',
                file_id: 'f-1',
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith('/sources/s-1/files/f-1', expect.any(Object));
            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
        });
    });

    describe('Passage Operations', () => {
        it('should list passages', async () => {
            mockServer.api.get.mockResolvedValue({ data: [
                { id: 'p-1', text: 'Passage text', doc_id: 'd-1' }
            ]});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'list_passages',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.passages).toHaveLength(1);
        });

        it('should get metadata', async () => {
            mockServer.api.get.mockResolvedValue({ data: { key: 'value' }});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'get_metadata',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.metadata).toEqual({ key: 'value' });
        });
    });

    describe('Agent Operations', () => {
        it('should attach source to agent', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'attach_to_agent',
                agent_id: 'a-1',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.attached).toBe(true);
        });

        it('should detach source from agent', async () => {
            mockServer.api.delete.mockResolvedValue({ data: {} });

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'detach_from_agent',
                agent_id: 'a-1',
                source_id: 's-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.detached).toBe(true);
        });

        it('should list agent sources', async () => {
            mockServer.api.get.mockResolvedValue({ data: [
                { id: 's-1', name: 'Source 1' }
            ]});

            const result = await handleLettaSourceManager(mockServer, {
                operation: 'list_agent_sources',
                agent_id: 'a-1',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.sources).toHaveLength(1);
        });
    });

    describe('Error Handling', () => {
        it('should require source_id for get', async () => {
            await expect(
                handleLettaSourceManager(mockServer, { operation: 'get' })
            ).rejects.toThrow('source_id is required');
        });

        it('should require source_data for create', async () => {
            await expect(
                handleLettaSourceManager(mockServer, { operation: 'create' })
            ).rejects.toThrow('source_data is required');
        });

        it('should throw for unknown operation', async () => {
            await expect(
                handleLettaSourceManager(mockServer, { operation: 'invalid' })
            ).rejects.toThrow('Unknown operation');
        });

        it('should propagate API errors', async () => {
            mockServer.api.get.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaSourceManager(mockServer, { operation: 'count' })
            ).rejects.toThrow('API Error');
        });
    });
});
