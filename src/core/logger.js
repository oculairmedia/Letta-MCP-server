import winston from 'winston';

const { combine, timestamp, printf, colorize, errors } = winston.format;

// Custom format for console output
const consoleFormat = printf(({ level, message, timestamp, stack, ...metadata }) => {
    let log = `${timestamp} [${level}]`;

    // Add metadata context if present
    if (metadata.context) {
        log += ` [${metadata.context}]`;
    }

    log += `: ${message}`;

    // Add stack trace for errors
    if (stack) {
        log += `\n${stack}`;
    }

    // Add any additional metadata
    const additionalData = Object.keys(metadata).filter((key) => key !== 'context');
    if (additionalData.length > 0) {
        const metadataStr = additionalData.reduce((acc, key) => {
            acc[key] = metadata[key];
            return acc;
        }, {});
        log += ` ${JSON.stringify(metadataStr)}`;
    }

    return log;
});

// Create logger instance
const logger = winston.createLogger({
    level: process.env.LOG_LEVEL || 'info',
    format: combine(errors({ stack: true }), timestamp({ format: 'YYYY-MM-DD HH:mm:ss' })),
    transports: [
        // Console transport - output to stderr to avoid interfering with MCP protocol (stdout)
        new winston.transports.Console({
            format: combine(colorize(), consoleFormat),
            // Force all log levels to stderr to keep stdout clean for MCP JSON communication
            stderrLevels: ['error', 'warn', 'info', 'verbose', 'debug', 'silly'],
        }),
    ],
});

// Add file transport in production
if (process.env.NODE_ENV === 'production') {
    logger.add(
        new winston.transports.File({
            filename: 'logs/error.log',
            level: 'error',
            format: combine(timestamp(), winston.format.json()),
        }),
    );

    logger.add(
        new winston.transports.File({
            filename: 'logs/combined.log',
            format: combine(timestamp(), winston.format.json()),
        }),
    );
}

// Create child loggers for different modules
export function createLogger(context) {
    return logger.child({ context });
}

// Export default logger
export default logger;
