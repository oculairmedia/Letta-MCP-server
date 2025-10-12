/**
 * Integration test for Phase A SDK pilot migration
 * Tests the 3 migrated operations: list, get, delete
 */

import axios from 'axios';

const MCP_BASE_URL = 'http://192.168.50.90:3001/mcp';

// ANSI color codes
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    reset: '\x1b[0m',
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

async function initializeSession() {
    log('\n=== Initializing MCP Session ===', 'blue');

    const response = await axios.post(MCP_BASE_URL, {
        jsonrpc: '2.0',
        method: 'initialize',
        params: {
            protocolVersion: '2025-06-18',
            capabilities: {},
            clientInfo: { name: 'sdk-pilot-test', version: '1.0.0' },
        },
        id: 1,
    }, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json, text/event-stream',
        },
    });

    const sessionId = response.headers['mcp-session-id'];
    log(`✓ Session initialized: ${sessionId}`, 'green');
    return sessionId;
}

function parseSSEResponse(data) {
    if (typeof data === 'string' && data.includes('data: ')) {
        const lines = data.split('\n');
        for (const line of lines) {
            if (line.startsWith('data: ')) {
                return JSON.parse(line.substring(6));
            }
        }
    }
    return data;
}

async function testListAgents(sessionId) {
    log('\n=== Test 1: List Agents (SDK: client.agents.list) ===', 'blue');

    try {
        const response = await axios.post(MCP_BASE_URL, {
            jsonrpc: '2.0',
            method: 'tools/call',
            params: {
                name: 'letta_agent_advanced',
                arguments: {
                    operation: 'list',
                    pagination: { limit: 5 },
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

        const result = parseSSEResponse(response.data);
        const content = JSON.parse(result.result.content[0].text);

        if (content.success && content.operation === 'list') {
            log(`✓ List operation succeeded`, 'green');
            log(`  Found ${content.agents.length} agents`, 'green');
            if (content.agents.length > 0) {
                log(`  First agent: ${content.agents[0].name} (${content.agents[0].id})`, 'green');
                return content.agents[0].id; // Return first agent ID for next test
            }
        } else {
            log(`✗ List operation failed`, 'red');
            console.log(content);
        }
    } catch (error) {
        log(`✗ List operation error: ${error.message}`, 'red');
        if (error.response) {
            console.log(error.response.data);
        }
    }

    return null;
}

async function testGetAgent(sessionId, agentId) {
    log('\n=== Test 2: Get Agent (SDK: client.agents.retrieve) ===', 'blue');

    if (!agentId) {
        log('⚠ Skipping get test - no agent ID available', 'yellow');
        return;
    }

    try {
        const response = await axios.post(MCP_BASE_URL, {
            jsonrpc: '2.0',
            method: 'tools/call',
            params: {
                name: 'letta_agent_advanced',
                arguments: {
                    operation: 'get',
                    agent_id: agentId,
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

        const result = parseSSEResponse(response.data);
        const content = JSON.parse(result.result.content[0].text);

        if (content.success && content.operation === 'get') {
            log(`✓ Get operation succeeded`, 'green');
            log(`  Agent: ${content.agent.name}`, 'green');
            log(`  Description: ${content.agent.description || 'N/A'}`, 'green');
            log(`  Created: ${content.agent.created_at}`, 'green');
        } else {
            log(`✗ Get operation failed`, 'red');
            console.log(content);
        }
    } catch (error) {
        log(`✗ Get operation error: ${error.message}`, 'red');
        if (error.response) {
            console.log(error.response.data);
        }
    }
}

async function testDeleteAgent(sessionId) {
    log('\n=== Test 3: Delete Agent (SDK: client.agents.delete) ===', 'blue');
    log('⚠ Skipping delete test to preserve existing agents', 'yellow');
    log('  Note: Delete operation migrated but not tested to avoid data loss', 'yellow');
    log('  Manual verification: Create a test agent and delete it', 'yellow');
}

async function runTests() {
    log('\n╔════════════════════════════════════════════════════════════╗', 'blue');
    log('║  Phase A SDK Pilot Migration - Integration Tests          ║', 'blue');
    log('║  Testing 3 migrated operations in letta_agent_advanced    ║', 'blue');
    log('╚════════════════════════════════════════════════════════════╝', 'blue');

    try {
        const sessionId = await initializeSession();

        // Test 1: List agents
        const agentId = await testListAgents(sessionId);

        // Test 2: Get agent
        await testGetAgent(sessionId, agentId);

        // Test 3: Delete agent (skipped for safety)
        await testDeleteAgent(sessionId);

        log('\n╔════════════════════════════════════════════════════════════╗', 'green');
        log('║  Phase A SDK Pilot Migration Tests Completed              ║', 'green');
        log('╚════════════════════════════════════════════════════════════╝', 'green');

    } catch (error) {
        log('\n✗ Test suite failed', 'red');
        console.error(error);
        process.exit(1);
    }
}

runTests();
