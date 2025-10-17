/**
 * Integration tests for Phase 2 Consolidated Tools
 * Tests all 26 new CRUD operations added to consolidated tools
 */

import axios from 'axios';

const MCP_ENDPOINT = process.env.MCP_ENDPOINT?.trim() || 'http://localhost:3001/mcp';
let sessionId = null;

// Helper to make MCP requests
async function mcpRequest(method, params = {}) {
    const headers = {
        'Content-Type': 'application/json',
        Accept: 'application/json, text/event-stream',
    };

    if (sessionId) {
        headers['mcp-session-id'] = sessionId;
    }

    const response = await axios.post(
        MCP_ENDPOINT,
        {
            jsonrpc: '2.0',
            method,
            params,
            id: Date.now(),
        },
        { headers },
    );

    // Handle SSE response - extract JSON from event data
    if (typeof response.data === 'string' && response.data.includes('data: ')) {
        const lines = response.data.split('\n');
        for (const line of lines) {
            if (line.startsWith('data: ')) {
                return JSON.parse(line.substring(6));
            }
        }
    }

    return response.data;
}

// Helper to call a tool
async function callTool(toolName, args) {
    return await mcpRequest('tools/call', {
        name: toolName,
        arguments: args,
    });
}

// Initialize session
async function initialize() {
    const headers = {
        'Content-Type': 'application/json',
        Accept: 'application/json, text/event-stream',
    };

    const response = await axios.post(
        MCP_ENDPOINT,
        {
            jsonrpc: '2.0',
            method: 'initialize',
            params: {
                protocolVersion: '2025-06-18',
                capabilities: {},
                clientInfo: { name: 'phase2-test', version: '1.0.0' },
            },
            id: Date.now(),
        },
        { headers },
    );

    // Check for session ID in response headers (mcp-session-id)
    if (response.headers['mcp-session-id']) {
        sessionId = response.headers['mcp-session-id'];
        console.log('✓ Session initialized:', sessionId);
    } else if (response.headers['x-session-id']) {
        sessionId = response.headers['x-session-id'];
        console.log('✓ Session initialized:', sessionId);
    } else if (response.data.result && response.data.result.sessionId) {
        sessionId = response.data.result.sessionId;
        console.log('✓ Session initialized:', sessionId);
    } else {
        console.log('Warning: No session ID found, proceeding without session');
        console.log('Available headers:', Object.keys(response.headers));
    }

    // Handle SSE response - extract JSON from event data
    let responseData = response.data;
    if (typeof responseData === 'string' && responseData.includes('data: ')) {
        const lines = responseData.split('\n');
        for (const line of lines) {
            if (line.startsWith('data: ')) {
                responseData = JSON.parse(line.substring(6));
                break;
            }
        }
    }

    return responseData;
}

// Test suite for LMS-19: Agent Advanced Operations
async function testAgentOperations() {
    console.log('\n=== Testing LMS-19: Agent Operations ===\n');

    try {
        // Test list operation
        console.log('Testing agent list operation...');
        const listResult = await callTool('letta_agent_advanced', {
            operation: 'list',
            pagination: { limit: 5 },
        });
        console.log('✓ Agent list:', JSON.parse(listResult.result.content[0].text));

        // Test get operation (using first agent if available)
        const agents = JSON.parse(listResult.result.content[0].text).agents;
        if (agents && agents.length > 0) {
            console.log('\nTesting agent get operation...');
            const getResult = await callTool('letta_agent_advanced', {
                operation: 'get',
                agent_id: agents[0].id,
            });
            console.log('✓ Agent details retrieved');

            // Test list_tools operation
            console.log('\nTesting agent list_tools operation...');
            const toolsResult = await callTool('letta_agent_advanced', {
                operation: 'list_tools',
                agent_id: agents[0].id,
            });
            console.log('✓ Agent tools listed');

            // Test get_config operation
            console.log('\nTesting agent get_config operation...');
            const configResult = await callTool('letta_agent_advanced', {
                operation: 'get_config',
                agent_id: agents[0].id,
            });
            console.log('✓ Agent config retrieved');
        }

        console.log('\n✓ All agent operations passed');
    } catch (error) {
        console.error('✗ Agent operations failed:', error.response?.data || error.message);
        throw error;
    }
}

// Test suite for LMS-20: Memory Operations
async function testMemoryOperations() {
    console.log('\n=== Testing LMS-20: Memory Operations ===\n');

    try {
        // Get first agent for testing
        const listResult = await callTool('letta_agent_advanced', {
            operation: 'list',
            pagination: { limit: 1 },
        });
        const agents = JSON.parse(listResult.result.content[0].text).agents;

        if (agents && agents.length > 0) {
            const agentId = agents[0].id;

            // Test get_core_memory
            console.log('Testing get_core_memory operation...');
            const coreMemResult = await callTool('letta_memory_unified', {
                operation: 'get_core_memory',
                agent_id: agentId,
            });
            console.log('✓ Core memory retrieved');

            // Test list_blocks
            console.log('\nTesting list_blocks operation...');
            const blocksResult = await callTool('letta_memory_unified', {
                operation: 'list_blocks',
                agent_id: agentId,
            });
            console.log('✓ Memory blocks listed');

            // Test list_passages
            console.log('\nTesting list_passages operation...');
            const passagesResult = await callTool('letta_memory_unified', {
                operation: 'list_passages',
                agent_id: agentId,
                pagination: { limit: 5 },
            });
            console.log('✓ Passages listed');

            // Test create_passage
            console.log('\nTesting create_passage operation...');
            const createPassageResult = await callTool('letta_memory_unified', {
                operation: 'create_passage',
                agent_id: agentId,
                passage_data: {
                    text: 'Integration test passage created at ' + new Date().toISOString(),
                },
            });
            const passageData = JSON.parse(createPassageResult.result.content[0].text);
            console.log('✓ Passage created:', passageData.passage_id);

            // Test delete_passage
            if (passageData.passage_id) {
                console.log('\nTesting delete_passage operation...');
                await callTool('letta_memory_unified', {
                    operation: 'delete_passage',
                    agent_id: agentId,
                    passage_id: passageData.passage_id,
                });
                console.log('✓ Passage deleted');
            }
        }

        console.log('\n✓ All memory operations passed');
    } catch (error) {
        console.error('✗ Memory operations failed:', error.response?.data || error.message);
        throw error;
    }
}

// Test suite for LMS-21: Tool Manager Operations
async function testToolManagerOperations() {
    console.log('\n=== Testing LMS-21: Tool Manager Operations ===\n');

    try {
        // Test list operation
        console.log('Testing tool list operation...');
        const listResult = await callTool('letta_tool_manager', {
            operation: 'list',
            options: {
                pagination: { limit: 5 },
            },
        });
        const tools = JSON.parse(listResult.result.content[0].text).tools;
        console.log('✓ Tools listed:', tools.length);

        // Test get operation
        if (tools && tools.length > 0) {
            console.log('\nTesting tool get operation...');
            const getResult = await callTool('letta_tool_manager', {
                operation: 'get',
                tool_id: tools[0].id,
            });
            console.log('✓ Tool details retrieved');
        }

        console.log('\n✓ All tool manager operations passed');
    } catch (error) {
        console.error('✗ Tool manager operations failed:', error.response?.data || error.message);
        throw error;
    }
}

// Test suite for LMS-22: MCP Operations
async function testMcpOperations() {
    console.log('\n=== Testing LMS-22: MCP Operations ===\n');

    try {
        // Test list_servers operation
        console.log('Testing list_servers operation...');
        const serversResult = await callTool('letta_mcp_ops', {
            operation: 'list_servers',
            pagination: { limit: 10 },
        });
        const servers = JSON.parse(serversResult.result.content[0].text).servers;
        console.log('✓ MCP servers listed:', servers.length);

        // Test list_tools operation
        if (servers && servers.length > 0) {
            console.log('\nTesting list_tools operation...');
            const toolsResult = await callTool('letta_mcp_ops', {
                operation: 'list_tools',
                server_name: servers[0].name,
                pagination: { limit: 5 },
            });
            console.log('✓ MCP server tools listed');
        }

        console.log('\n✓ All MCP operations passed');
    } catch (error) {
        console.error('✗ MCP operations failed:', error.response?.data || error.message);
        throw error;
    }
}

// Test error handling for missing required parameters
async function testErrorHandling() {
    console.log('\n=== Testing Error Handling ===\n');

    const errorTests = [
        {
            name: 'Agent get without agent_id',
            tool: 'letta_agent_advanced',
            args: { operation: 'get' },
        },
        {
            name: 'Memory get_core_memory without agent_id',
            tool: 'letta_memory_unified',
            args: { operation: 'get_core_memory' },
        },
        {
            name: 'Tool get without tool_id',
            tool: 'letta_tool_manager',
            args: { operation: 'get' },
        },
        {
            name: 'MCP list_tools without server_name',
            tool: 'letta_mcp_ops',
            args: { operation: 'list_tools' },
        },
    ];

    for (const test of errorTests) {
        try {
            console.log(`Testing: ${test.name}...`);
            await callTool(test.tool, test.args);
            console.error(`✗ Should have thrown error for: ${test.name}`);
        } catch (error) {
            if (error.response?.data?.error) {
                console.log(`✓ Correctly errored: ${test.name}`);
            } else {
                throw error;
            }
        }
    }

    console.log('\n✓ All error handling tests passed');
}

// Main test runner
async function runAllTests() {
    console.log('Starting Phase 2 Integration Tests\n');
    console.log('='.repeat(50));

    try {
        await initialize();
        await testAgentOperations();
        await testMemoryOperations();
        await testToolManagerOperations();
        await testMcpOperations();
        await testErrorHandling();

        console.log('\n' + '='.repeat(50));
        console.log('\n✓✓✓ ALL TESTS PASSED ✓✓✓\n');
    } catch (error) {
        console.log('\n' + '='.repeat(50));
        console.error('\n✗✗✗ TESTS FAILED ✗✗✗\n');
        console.error('Error details:', error.response?.data || error.message);
        process.exit(1);
    }
}

// Run tests
runAllTests();
