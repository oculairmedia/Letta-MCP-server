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

    // Get first agent
    const listResponse = await axios.post(MCP_BASE_URL, {
        jsonrpc: '2.0',
        method: 'tools/call',
        params: {
            name: 'letta_agent_advanced',
            arguments: {
                operation: 'list',
                pagination: { limit: 1 },
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

    // Parse SSE response
    let listData = listResponse.data;
    if (typeof listData === 'string' && listData.includes('data: ')) {
        const lines = listData.split('\n');
        for (const line of lines) {
            if (line.startsWith('data: ')) {
                listData = JSON.parse(line.substring(6));
                break;
            }
        }
    }

    const agents = JSON.parse(listData.result.content[0].text).agents;
    const agentId = agents[0].id;
    console.log('Using agent:', agentId);

    // Test create_passage
    console.log('\nTesting create_passage...');
    const createResponse = await axios.post(MCP_BASE_URL, {
        jsonrpc: '2.0',
        method: 'tools/call',
        params: {
            name: 'letta_memory_unified',
            arguments: {
                operation: 'create_passage',
                agent_id: agentId,
                passage_data: {
                    text: 'Debug test passage created at ' + new Date().toISOString(),
                },
            },
        },
        id: 3,
    }, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json, text/event-stream',
            'mcp-session-id': sessionId,
        },
    });

    console.log('\nRaw response:');
    console.log(JSON.stringify(createResponse.data, null, 2));

    // Parse SSE if needed
    let createData = createResponse.data;
    if (typeof createData === 'string' && createData.includes('data: ')) {
        const lines = createData.split('\n');
        for (const line of lines) {
            if (line.startsWith('data: ')) {
                createData = JSON.parse(line.substring(6));
                break;
            }
        }
    }

    console.log('\nParsed response:');
    console.log(JSON.stringify(createData, null, 2));
}

test().catch(console.error);
