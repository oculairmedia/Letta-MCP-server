import axios from 'axios';

const MCP_BASE_URL = 'http://192.168.50.90:3001/mcp';

async function test() {
    // Initialize
    const initResponse = await axios.post(MCP_BASE_URL, {
        jsonrpc: '2.0',
        method: 'initialize',
        params: {
            protocolVersion: '2025-06-18',
            capabilities: {},
            clientInfo: { name: 'test', version: '1.0.0' },
        },
        id: 1,
    }, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json, text/event-stream',
        },
    });

    const sessionId = initResponse.headers['mcp-session-id'];
    console.log('Session ID:', sessionId);

    // Call tool
    const toolResponse = await axios.post(MCP_BASE_URL, {
        jsonrpc: '2.0',
        method: 'tools/call',
        params: {
            name: 'letta_agent_advanced',
            arguments: {
                operation: 'list',
                pagination: { limit: 2 },
            },
        },
        id: 2,
    }, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json, text/event-stream',
            'mcp-session-id': sessionId,
        },
    });

    console.log('\nTool Response:');
    console.log(JSON.stringify(toolResponse.data, null, 2));
}

test().catch(console.error);
