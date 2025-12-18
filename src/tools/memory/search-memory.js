/**
 * Unified tool handler for searching both archival memory and messages
 */
export async function handleSearchMemory(server, args) {
    if (!args?.agent_id) {
        throw new Error('Missing required argument: agent_id');
    }
    if (!args?.query) {
        throw new Error('Missing required argument: query');
    }

    const results = {
        archival: null,
        messages: null,
    };

    const source = args.source || 'both';
    const headers = server.getApiHeaders();
    const agentId = encodeURIComponent(args.agent_id);

    try {
        // Search archival memory
        if (source === 'archival' || source === 'both') {
            const archivalParams = {
                query: args.query,
            };
            if (args.limit) archivalParams.top_k = args.limit;
            if (args.start_date) archivalParams.start_datetime = args.start_date;
            if (args.end_date) archivalParams.end_datetime = args.end_date;

            const archivalResponse = await server.api.get(
                `/agents/${agentId}/archival-memory/search`,
                {
                    headers,
                    params: archivalParams,
                }
            );

            let passages = archivalResponse.data.results || [];
            // Remove embeddings by default
            passages = passages.map((passage) => {
                const { embedding, ...rest } = passage;
                return rest;
            });

            results.archival = {
                passages,
                count: passages.length,
            };
        }

        // Search messages
        if (source === 'messages' || source === 'both') {
            results.messages = await searchMessages(
                server,
                agentId,
                headers,
                args.query,
                args.start_date,
                args.end_date,
                args.limit || 50
            );
        }

        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(results),
                },
            ],
        };
    } catch (error) {
        return server.createErrorResponse(error);
    }
}

/**
 * Search messages with client-side filtering and optimized pagination direction
 */
async function searchMessages(server, agentId, headers, query, startDate, endDate, limit) {
    // First, get boundary timestamps to determine optimal pagination direction
    const [firstMsg, lastMsg] = await Promise.all([
        fetchFirstMessage(server, agentId, headers),
        fetchLastMessage(server, agentId, headers),
    ]);

    if (!firstMsg || !lastMsg) {
        return { messages: [], count: 0 };
    }

    const firstTime = new Date(firstMsg.date).getTime();
    const lastTime = new Date(lastMsg.date).getTime();

    // Determine date range
    const rangeStart = startDate ? new Date(startDate).getTime() : firstTime;
    const rangeEnd = endDate ? new Date(endDate).getTime() : lastTime;
    const rangeCenter = (rangeStart + rangeEnd) / 2;

    // Choose pagination direction based on which end is closer to range center
    const distToFirst = Math.abs(rangeCenter - firstTime);
    const distToLast = Math.abs(rangeCenter - lastTime);
    const paginateAsc = distToFirst < distToLast;

    const queryLower = query.toLowerCase();
    const matchingMessages = [];
    let cursor = null;
    let hasMore = true;
    const pageSize = 100;

    while (hasMore && matchingMessages.length < limit) {
        const params = {
            limit: pageSize,
            order: paginateAsc ? 'asc' : 'desc',
        };
        if (cursor) {
            params[paginateAsc ? 'after' : 'before'] = cursor;
        }

        const response = await server.api.get(`/agents/${agentId}/messages`, {
            headers,
            params,
        });

        const messages = response.data || [];
        if (messages.length === 0) {
            hasMore = false;
            break;
        }

        for (const msg of messages) {
            const msgTime = new Date(msg.date).getTime();

            // Check if we've passed the date range boundary
            if (paginateAsc && msgTime > rangeEnd) {
                hasMore = false;
                break;
            }
            if (!paginateAsc && msgTime < rangeStart) {
                hasMore = false;
                break;
            }

            // Skip if outside date range
            if (msgTime < rangeStart || msgTime > rangeEnd) {
                continue;
            }

            // Check if content matches query
            const content = msg.content || '';
            if (content.toLowerCase().includes(queryLower)) {
                matchingMessages.push({
                    id: msg.id,
                    date: msg.date,
                    message_type: msg.message_type,
                    content: content.length > 500 ? content.substring(0, 500) + '...' : content,
                });

                if (matchingMessages.length >= limit) {
                    break;
                }
            }
        }

        // Update cursor for next page
        cursor = messages[messages.length - 1].id;

        // If we got fewer messages than requested, we've reached the end
        if (messages.length < pageSize) {
            hasMore = false;
        }
    }

    // Sort results by date (oldest first) for consistent output
    matchingMessages.sort((a, b) => new Date(a.date) - new Date(b.date));

    return {
        messages: matchingMessages,
        count: matchingMessages.length,
    };
}

async function fetchFirstMessage(server, agentId, headers) {
    const response = await server.api.get(`/agents/${agentId}/messages`, {
        headers,
        params: { limit: 1, order: 'asc' },
    });
    return response.data?.[0] || null;
}

async function fetchLastMessage(server, agentId, headers) {
    const response = await server.api.get(`/agents/${agentId}/messages`, {
        headers,
        params: { limit: 1, order: 'desc' },
    });
    return response.data?.[0] || null;
}

/**
 * Tool definition for search_memory
 */
export const searchMemoryDefinition = {
    name: 'search_memory',
    description:
        "Search an agent's memory across both archival passages and conversation messages. Supports text search with optional date filtering. Use source parameter to search only archival or messages for better performance.",
    inputSchema: {
        type: 'object',
        properties: {
            agent_id: {
                type: 'string',
                description: 'ID of the agent whose memory to search',
            },
            query: {
                type: 'string',
                description: 'Text to search for in memory content',
            },
            source: {
                type: 'string',
                enum: ['archival', 'messages', 'both'],
                description:
                    'Which memory source to search: "archival" (passages), "messages" (conversation history), or "both" (default)',
                default: 'both',
            },
            start_date: {
                type: 'string',
                format: 'date-time',
                description: 'Filter results after this datetime (ISO 8601 format)',
            },
            end_date: {
                type: 'string',
                format: 'date-time',
                description: 'Filter results before this datetime (ISO 8601 format)',
            },
            limit: {
                type: 'integer',
                description: 'Maximum number of results per source (default: 50)',
                default: 50,
            },
        },
        required: ['agent_id', 'query'],
    },
};
