/**
 * Tests for letta_job_monitor tool
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { handleLettaJobMonitor, lettaJobMonitorDefinition } from '../jobs/letta-job-monitor.js';

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

describe('letta_job_monitor', () => {
    let mockServer;

    beforeEach(() => {
        mockServer = createMockServer();
        vi.clearAllMocks();
    });

    describe('Tool Definition', () => {
        it('should have correct tool name', () => {
            expect(lettaJobMonitorDefinition.name).toBe('letta_job_monitor');
        });

        it('should have all 4 operations', () => {
            expect(lettaJobMonitorDefinition.inputSchema.properties.operation.enum).toEqual([
                'list', 'get', 'list_active', 'cancel'
            ]);
        });

        it('should have additionalProperties set to false', () => {
            expect(lettaJobMonitorDefinition.inputSchema.additionalProperties).toBe(false);
        });
    });

    describe('List Operation', () => {
        it('should list all jobs', async () => {
            mockServer.api.get.mockResolvedValue({
                data: [
                    {
                        id: 'job-1',
                        status: 'running',
                        type: 'data_import',
                        agent_id: 'agent-123',
                        created_at: '2025-10-12T00:00:00Z',
                        progress: { current: 50, total: 100, percentage: 50 },
                    },
                    {
                        id: 'job-2',
                        status: 'completed',
                        type: 'model_training',
                        agent_id: 'agent-456',
                        created_at: '2025-10-11T00:00:00Z',
                        completed_at: '2025-10-11T01:00:00Z',
                    },
                ],
            });

            const result = await handleLettaJobMonitor(mockServer, { operation: 'list' });
            const response = JSON.parse(result.content[0].text);

            expect(response.success).toBe(true);
            expect(response.jobs).toHaveLength(2);
            expect(response.jobs[0].id).toBe('job-1');
            expect(response.jobs[0].status).toBe('running');
        });

        it('should list jobs with filters', async () => {
            mockServer.api.get.mockResolvedValue({ data: [] });

            await handleLettaJobMonitor(mockServer, {
                operation: 'list',
                filters: {
                    status: 'running',
                    job_type: 'data_import',
                    agent_id: 'agent-123',
                },
            });

            const callArg = mockServer.api.get.mock.calls[0][0];
            expect(callArg).toContain('status=running');
            expect(callArg).toContain('job_type=data_import');
            expect(callArg).toContain('agent_id=agent-123');
        });

        it('should handle response with jobs array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    jobs: [
                        { id: 'job-3', status: 'pending', job_type: 'test', agent_id: 'agent-789' },
                    ],
                },
            });

            const result = await handleLettaJobMonitor(mockServer, { operation: 'list' });
            const response = JSON.parse(result.content[0].text);

            expect(response.jobs).toHaveLength(1);
        });
    });

    describe('Get Operation', () => {
        it('should get job details', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    id: 'job-1',
                    status: 'running',
                    job_type: 'data_import',
                    agent_id: 'agent-123',
                    created_at: '2025-10-12T00:00:00Z',
                    started_at: '2025-10-12T00:01:00Z',
                    progress: {
                        current: 75,
                        total: 100,
                        percentage: 75,
                    },
                },
            });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'get',
                job_id: 'job-1',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/jobs/job-1', expect.any(Object));

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.job.id).toBe('job-1');
            expect(response.job.progress.percentage).toBe(75);
        });

        it('should handle job with error', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    id: 'job-2',
                    status: 'failed',
                    job_type: 'test',
                    agent_id: 'agent-456',
                    created_at: '2025-10-12T00:00:00Z',
                    error: 'Connection timeout',
                },
            });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'get',
                job_id: 'job-2',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.job.status).toBe('failed');
            expect(response.job.error).toBe('Connection timeout');
        });

        it('should handle job with result', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    id: 'job-3',
                    status: 'completed',
                    job_type: 'data_import',
                    agent_id: 'agent-789',
                    created_at: '2025-10-12T00:00:00Z',
                    completed_at: '2025-10-12T01:00:00Z',
                    result: {
                        records_imported: 1000,
                        duration_seconds: 3600,
                    },
                },
            });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'get',
                job_id: 'job-3',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.job.status).toBe('completed');
            expect(response.job.result.records_imported).toBe(1000);
        });

        it('should throw error when job_id is missing', async () => {
            await expect(
                handleLettaJobMonitor(mockServer, { operation: 'get' })
            ).rejects.toThrow('job_id is required');
        });
    });

    describe('List Active Operation', () => {
        it('should list only active jobs', async () => {
            mockServer.api.get.mockResolvedValue({
                data: [
                    {
                        id: 'job-1',
                        status: 'running',
                        type: 'data_import',
                        agent_id: 'agent-123',
                        created_at: '2025-10-12T00:00:00Z',
                        started_at: '2025-10-12T00:01:00Z',
                        progress: { current: 50, total: 100, percentage: 50 },
                    },
                    {
                        id: 'job-2',
                        status: 'running',
                        type: 'model_training',
                        agent_id: 'agent-456',
                        created_at: '2025-10-12T00:05:00Z',
                        started_at: '2025-10-12T00:06:00Z',
                        progress: { current: 25, total: 100, percentage: 25 },
                    },
                ],
            });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'list_active',
            });

            expect(mockServer.api.get).toHaveBeenCalledWith('/jobs?status=running', expect.any(Object));

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('list_active');
            expect(response.jobs).toHaveLength(2);
            expect(response.jobs[0].status).toBe('running');
            expect(response.jobs[1].status).toBe('running');
        });

        it('should handle empty active jobs list', async () => {
            mockServer.api.get.mockResolvedValue({ data: [] });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'list_active',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.jobs).toHaveLength(0);
            expect(response.message).toContain('Found 0 active jobs');
        });

        it('should handle response with jobs array', async () => {
            mockServer.api.get.mockResolvedValue({
                data: {
                    jobs: [
                        { id: 'job-5', status: 'running', job_type: 'test', agent_id: 'agent-999' },
                    ],
                },
            });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'list_active',
            });

            const response = JSON.parse(result.content[0].text);
            expect(response.jobs).toHaveLength(1);
        });
    });

    describe('Cancel Operation', () => {
        it('should cancel a running job', async () => {
            mockServer.api.post.mockResolvedValue({ data: {} });

            const result = await handleLettaJobMonitor(mockServer, {
                operation: 'cancel',
                job_id: 'job-1',
            });

            expect(mockServer.api.post).toHaveBeenCalledWith(
                '/jobs/job-1/cancel',
                {},
                expect.any(Object)
            );

            const response = JSON.parse(result.content[0].text);
            expect(response.success).toBe(true);
            expect(response.operation).toBe('cancel');
            expect(response.cancelled).toBe(true);
        });

        it('should throw error when job_id is missing', async () => {
            await expect(
                handleLettaJobMonitor(mockServer, { operation: 'cancel' })
            ).rejects.toThrow('job_id is required');
        });
    });

    describe('Error Handling', () => {
        it('should throw error for unknown operation', async () => {
            await expect(
                handleLettaJobMonitor(mockServer, { operation: 'invalid' })
            ).rejects.toThrow('Unknown operation: invalid');
        });

        it('should propagate API errors', async () => {
            mockServer.api.get.mockRejectedValue(new Error('API Error'));

            await expect(
                handleLettaJobMonitor(mockServer, { operation: 'list' })
            ).rejects.toThrow('API Error');
        });

        it('should handle network timeout', async () => {
            mockServer.api.post.mockRejectedValue(new Error('ETIMEDOUT'));

            await expect(
                handleLettaJobMonitor(mockServer, {
                    operation: 'cancel',
                    job_id: 'job-1',
                })
            ).rejects.toThrow('ETIMEDOUT');
        });
    });
});
