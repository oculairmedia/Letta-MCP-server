/**
 * Schemas for letta_job_monitor tool
 * Provides discriminator-based schemas for job monitoring and management
 */

/**
 * Job filters schema
 */
export const JobFiltersSchema = {
    type: 'object',
    properties: {
        status: {
            type: 'string',
            enum: ['pending', 'running', 'completed', 'failed', 'cancelled'],
            description: 'Filter by job status',
        },
        job_type: {
            type: 'string',
            description: 'Filter by job type',
        },
        agent_id: {
            type: 'string',
            description: 'Filter by agent ID',
        },
    },
    additionalProperties: false,
};

/**
 * Input schema for letta_job_monitor tool
 */
export const jobMonitorInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: ['list', 'get', 'list_active', 'cancel'],
            description: 'Job monitoring operation to perform',
        },
        job_id: {
            type: 'string',
            description: 'Job ID (required for get, cancel operations)',
        },
        filters: {
            ...JobFiltersSchema,
            description: 'Filter options for list operations',
        },
        request_heartbeat: {
            type: 'boolean',
            description: 'Ignored parameter (for MCP client compatibility)',
        },
    },
    required: ['operation'],
    additionalProperties: true,
};

/**
 * Job status schema
 */
export const JobStatusSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        status: {
            type: 'string',
            enum: ['pending', 'running', 'completed', 'failed', 'cancelled'],
        },
        job_type: { type: 'string' },
        agent_id: { type: 'string' },
        created_at: { type: 'string' },
        started_at: { type: 'string' },
        completed_at: { type: 'string' },
        progress: {
            type: 'object',
            properties: {
                current: { type: 'integer' },
                total: { type: 'integer' },
                percentage: { type: 'number' },
            },
            additionalProperties: false,
        },
        error: {
            type: 'string',
            description: 'Error message if job failed',
        },
        result: {
            type: 'object',
            additionalProperties: true,
            description: 'Job result if completed',
        },
    },
    required: ['id', 'status', 'job_type'],
    additionalProperties: false,
};

/**
 * Output schema for letta_job_monitor tool
 */
export const jobMonitorOutputSchema = {
    type: 'object',
    properties: {
        success: {
            type: 'boolean',
            description: 'Whether the operation succeeded',
        },
        operation: {
            type: 'string',
            description: 'Operation that was performed',
        },
        job: {
            ...JobStatusSchema,
            description: 'Job details (for get operation)',
        },
        jobs: {
            type: 'array',
            items: JobStatusSchema,
            description: 'List of jobs (for list operations)',
        },
        cancelled: {
            type: 'boolean',
            description: 'Whether job was cancelled',
        },
        message: {
            type: 'string',
            description: 'Status or error message',
        },
    },
    required: ['success', 'operation'],
    additionalProperties: false,
};
