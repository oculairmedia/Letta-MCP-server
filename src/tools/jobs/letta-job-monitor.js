/**
 * Tool handler for letta_job_monitor - Job Monitoring and Management Hub
 * Provides unified interface for tracking and managing long-running jobs
 */
import { createLogger } from '../../core/logger.js';
import { jobMonitorInputSchema } from '../schemas/job-monitor-schemas.js';

const logger = createLogger('letta_job_monitor');

/**
 * Handle letta_job_monitor tool requests
 * @param {Object} server - LettaServer instance
 * @param {Object} args - Tool arguments following jobMonitorInputSchema
 * @returns {Promise<Object>} Tool response
 */
export async function handleLettaJobMonitor(server, args) {
    const { operation } = args;
    logger.info(`Executing job operation: ${operation}`, { args });

    try {
        const handlers = {
            list: handleList,
            get: handleGet,
            list_active: handleListActive,
            cancel: handleCancel,
        };

        if (!handlers[operation]) {
            throw new Error(`Unknown operation: ${operation}`);
        }

        return await handlers[operation](server, args);
    } catch (error) {
        logger.error(`Job operation failed: ${operation}`, { error, args });
        throw error;
    }
}

/**
 * List all jobs with optional filters
 */
async function handleList(server, args) {
    const { filters = {} } = args;

    const queryParams = new URLSearchParams();
    if (filters.status) queryParams.append('status', filters.status);
    if (filters.job_type) queryParams.append('job_type', filters.job_type);
    if (filters.agent_id) queryParams.append('agent_id', filters.agent_id);

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const url = `/jobs/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
            const response = await server.api.get(url, { headers });
            return response.data;
        },
        'Listing jobs'
    );

    const jobs = Array.isArray(result) ? result : result.jobs || [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list',
                jobs: jobs.map(job => ({
                    id: job.id,
                    status: job.status,
                    job_type: job.job_type || job.type,
                    agent_id: job.agent_id,
                    created_at: job.created_at,
                    started_at: job.started_at,
                    completed_at: job.completed_at,
                    progress: job.progress,
                    error: job.error,
                    result: job.result,
                })),
                message: `Found ${jobs.length} jobs`,
            }),
        }],
    };
}

/**
 * Get details of a specific job
 */
async function handleGet(server, args) {
    const { job_id } = args;

    if (!job_id) {
        throw new Error('job_id is required for get operation');
    }

    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get(`/jobs/${encodeURIComponent(job_id)}`, { headers });
            return response.data;
        },
        'Getting job details'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'get',
                job: {
                    id: result.id,
                    status: result.status,
                    job_type: result.job_type || result.type,
                    agent_id: result.agent_id,
                    created_at: result.created_at,
                    started_at: result.started_at,
                    completed_at: result.completed_at,
                    progress: result.progress,
                    error: result.error,
                    result: result.result,
                },
                message: 'Job details retrieved successfully',
            }),
        }],
    };
}

/**
 * List only active/running jobs
 */
async function handleListActive(server, _args) {
    const result = await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.get('/jobs?status=running', { headers });
            return response.data;
        },
        'Listing active jobs'
    );

    const jobs = Array.isArray(result) ? result : result.jobs || [];

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'list_active',
                jobs: jobs.map(job => ({
                    id: job.id,
                    status: job.status,
                    job_type: job.job_type || job.type,
                    agent_id: job.agent_id,
                    created_at: job.created_at,
                    started_at: job.started_at,
                    progress: job.progress,
                })),
                message: `Found ${jobs.length} active jobs`,
            }),
        }],
    };
}

/**
 * Cancel a running job
 */
async function handleCancel(server, args) {
    const { job_id } = args;

    if (!job_id) {
        throw new Error('job_id is required for cancel operation');
    }

    await server.handleSdkCall(
        async () => {
            const headers = server.getApiHeaders();
            const response = await server.api.post(
                `/jobs/${encodeURIComponent(job_id)}/cancel`,
                {},
                { headers }
            );
            return response.data;
        },
        'Cancelling job'
    );

    return {
        content: [{
            type: 'text',
            text: JSON.stringify({
                success: true,
                operation: 'cancel',
                job_id,
                cancelled: true,
                message: 'Job cancelled successfully',
            }),
        }],
    };
}

/**
 * Tool definition for letta_job_monitor
 */
export const lettaJobMonitorDefinition = {
    name: 'letta_job_monitor',
    description: 'Job Monitoring and Management Hub - Tool for tracking and managing long-running jobs with 4 operations: list (with filters), get (job details), list_active (running jobs only), and cancel (cancel running job). Provides job status, progress tracking, and lifecycle management with discriminator-based operation routing.',
    inputSchema: jobMonitorInputSchema,
};
