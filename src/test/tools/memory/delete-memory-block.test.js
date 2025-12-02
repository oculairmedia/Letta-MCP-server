import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
    handleDeleteMemoryBlock,
    deleteMemoryBlockToolDefinition,
} from '../../../tools/memory/delete-memory-block.js';
import { createMockLettaServer } from '../../utils/mock-server.js';
import { expectValidToolResponse } from '../../utils/test-helpers.js';

describe('Delete Memory Block', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockLettaServer();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool definition', () => {
            expect(deleteMemoryBlockToolDefinition.name).toBe('delete_memory_block');
            expect(deleteMemoryBlockToolDefinition.description).toContain('Delete a memory block');
            expect(deleteMemoryBlockToolDefinition.description).toContain('WARNING');
            expect(deleteMemoryBlockToolDefinition.inputSchema.required).toEqual(['block_id']);
            expect(deleteMemoryBlockToolDefinition.inputSchema.properties).toHaveProperty('block_id');
            expect(deleteMemoryBlockToolDefinition.inputSchema.properties).toHaveProperty('agent_id');
        });
    });

    describe('Functionality Tests', () => {
        it('should delete memory block successfully', async () => {
            mockServer.api.delete.mockResolvedValueOnce({ data: null });

            const result = await handleDeleteMemoryBlock(mockServer, {
                block_id: 'block-123',
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                '/blocks/block-123',
                expect.objectContaining({
                    headers: expect.any(Object),
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.success).toBe(true);
            expect(data.deleted_block_id).toBe('block-123');
        });

        it('should delete memory block with agent_id authorization', async () => {
            const agentId = 'agent-456';
            mockServer.api.delete.mockResolvedValueOnce({ data: null });

            const result = await handleDeleteMemoryBlock(mockServer, {
                block_id: 'agent-block-123',
                agent_id: agentId,
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                '/blocks/agent-block-123',
                expect.objectContaining({
                    headers: expect.objectContaining({
                        user_id: agentId,
                    }),
                }),
            );

            const data = expectValidToolResponse(result);
            expect(data.success).toBe(true);
            expect(data.deleted_block_id).toBe('agent-block-123');
        });

        it('should handle UUID format block IDs', async () => {
            const uuidBlockId = '550e8400-e29b-41d4-a716-446655440000';
            mockServer.api.delete.mockResolvedValueOnce({ data: null });

            const result = await handleDeleteMemoryBlock(mockServer, {
                block_id: uuidBlockId,
            });

            expect(mockServer.api.delete).toHaveBeenCalledWith(
                `/blocks/${uuidBlockId}`,
                expect.any(Object),
            );

            const data = expectValidToolResponse(result);
            expect(data.success).toBe(true);
            expect(data.deleted_block_id).toBe(uuidBlockId);
        });

        it('should handle special characters in block_id', async () => {
            const blockId = 'block-with-special_chars.123';
            mockServer.api.delete.mockResolvedValueOnce({ data: null });

            const result = await handleDeleteMemoryBlock(mockServer, {
                block_id: blockId,
            });

            const data = expectValidToolResponse(result);
            expect(data.success).toBe(true);
            expect(data.deleted_block_id).toBe(blockId);
        });
    });

    describe('Error Handling', () => {
        it('should throw error for missing block_id', async () => {
            await expect(handleDeleteMemoryBlock(mockServer, {})).rejects.toThrow(
                'Missing required argument: block_id',
            );
        });

        it('should throw error for null block_id', async () => {
            await expect(handleDeleteMemoryBlock(mockServer, { block_id: null })).rejects.toThrow(
                'Missing required argument: block_id',
            );
        });

        it('should throw error for undefined args', async () => {
            await expect(handleDeleteMemoryBlock(mockServer, undefined)).rejects.toThrow(
                'Missing required argument: block_id',
            );
        });

        it('should throw error for empty string block_id', async () => {
            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: '',
                }),
            ).rejects.toThrow('Missing required argument: block_id');
        });

        it('should handle 404 error when block not found', async () => {
            const error = new Error('Not found');
            error.response = {
                status: 404,
                data: { error: 'Memory block not found' },
            };
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'non-existent-block',
                }),
            ).rejects.toThrow('Not found');
        });

        it('should handle 401 unauthorized error', async () => {
            const error = new Error('Unauthorized');
            error.response = {
                status: 401,
                data: { error: 'Unauthorized access' },
            };
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'protected-block',
                }),
            ).rejects.toThrow('Unauthorized');
        });

        it('should handle 403 forbidden error', async () => {
            const error = new Error('Forbidden');
            error.response = {
                status: 403,
                data: { error: 'Cannot delete this block' },
            };
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'forbidden-block',
                    agent_id: 'agent-no-access',
                }),
            ).rejects.toThrow('Forbidden');
        });

        it('should handle network errors', async () => {
            const error = new Error('Network error: Connection refused');
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'block-123',
                }),
            ).rejects.toThrow('Network error');
        });

        it('should handle server errors', async () => {
            const error = new Error('Internal server error');
            error.response = {
                status: 500,
                data: { error: 'Database error' },
            };
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'block-123',
                }),
            ).rejects.toThrow('Internal server error');
        });

        it('should handle 409 conflict error for blocks in use', async () => {
            const error = new Error('Conflict');
            error.response = {
                status: 409,
                data: { error: 'Block is currently attached to agents and cannot be deleted' },
            };
            mockServer.api.delete.mockRejectedValueOnce(error);

            await expect(
                handleDeleteMemoryBlock(mockServer, {
                    block_id: 'in-use-block',
                }),
            ).rejects.toThrow('Conflict');
        });
    });
});
