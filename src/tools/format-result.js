import { getOutputSchema } from './output-schemas.js';

/**
 * Format a tool result with proper structuredContent for MCP protocol compliance.
 *
 * When a tool declares an outputSchema, the MCP specification (2025-06-18) requires
 * that CallToolResult includes a structuredContent field containing the structured
 * data matching that schema.
 *
 * @param {Object} data - The data object to return
 * @param {string} toolName - The name of the tool (used to check for outputSchema)
 * @param {Object} [options] - Additional options
 * @param {boolean} [options.isError] - Whether this result represents an error
 * @returns {Object} MCP-compliant CallToolResult object
 *
 * @example
 * // Tool with outputSchema - returns both content and structuredContent
 * return formatToolResult({ agents: [...], count: 5 }, 'list_agents');
 * // => { content: [{ type: 'text', text: '...' }], structuredContent: { agents: [...], count: 5 } }
 *
 * @example
 * // Tool without outputSchema - returns only content
 * return formatToolResult({ message: 'done' }, 'some_tool');
 * // => { content: [{ type: 'text', text: '...' }] }
 *
 * @example
 * // Tool with error flag
 * return formatToolResult({ error: 'failed' }, 'some_tool', { isError: true });
 * // => { content: [...], isError: true }
 */
export function formatToolResult(data, toolName, options = {}) {
    const result = {
        content: [
            {
                type: 'text',
                text: JSON.stringify(data),
            },
        ],
    };

    // If tool has outputSchema, add structuredContent per MCP spec
    const outputSchema = getOutputSchema(toolName);
    if (outputSchema) {
        result.structuredContent = data;
    }

    // Add isError flag if specified
    if (options.isError !== undefined) {
        result.isError = options.isError;
    }

    return result;
}
