#!/usr/bin/env node

/**
 * Integration test for consolidated tools with real Letta instance
 * Tests all 7 consolidated tools with actual API calls
 */

import axios from 'axios';

const MCP_ENDPOINT = 'http://localhost:3001/mcp';
const TEST_RESULTS = [];
let SESSION_ID = null;

/**
 * Initialize MCP session
 */
async function initializeSession() {
    console.log('\n🔌 Initializing MCP session...');
    const response = await axios.post(MCP_ENDPOINT, {
        jsonrpc: '2.0',
        method: 'initialize',
        params: {
            protocolVersion: '2025-06-18',
            capabilities: {
                tools: {}
            },
            clientInfo: {
                name: 'integration-test',
                version: '1.0.0'
            }
        },
        id: 1
    }, {
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        }
    });

    if (response.data.error) {
        throw new Error(`Session init error: ${JSON.stringify(response.data.error)}`);
    }

    // Extract session ID from response headers
    SESSION_ID = response.headers['x-mcp-session-id'] || response.headers['x-session-id'];
    console.log(`✓ Session initialized: ${SESSION_ID || '(stdio mode)'}`);
    return SESSION_ID;
}

/**
 * Send MCP tool request
 */
async function callTool(toolName, args = {}) {
    const headers = {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
    };

    if (SESSION_ID) {
        headers['x-mcp-session-id'] = SESSION_ID;
    }

    const response = await axios.post(MCP_ENDPOINT, {
        jsonrpc: '2.0',
        method: 'tools/call',
        params: {
            name: toolName,
            arguments: args
        },
        id: Date.now()
    }, { headers });

    if (response.data.error) {
        throw new Error(`Tool error: ${JSON.stringify(response.data.error)}`);
    }

    return response.data.result;
}

/**
 * Run a test case
 */
async function runTest(name, fn) {
    try {
        console.log(`\n🧪 Testing: ${name}`);
        await fn();
        console.log(`✅ PASS: ${name}`);
        TEST_RESULTS.push({ name, status: 'PASS' });
        return true;
    } catch (error) {
        console.error(`❌ FAIL: ${name}`);
        console.error(`   Error: ${error.message}`);
        TEST_RESULTS.push({ name, status: 'FAIL', error: error.message });
        return false;
    }
}

/**
 * Test letta_agent_advanced
 */
async function testAgentAdvanced() {
    await runTest('letta_agent_advanced: list', async () => {
        const result = await callTool('letta_agent_advanced', {
            operation: 'list',
            limit: 5
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.agents)) throw new Error('Expected agents array');
        console.log(`   Found ${data.agents.length} agents`);
    });

    await runTest('letta_agent_advanced: list_models', async () => {
        const result = await callTool('letta_agent_advanced', {
            operation: 'list_models'
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.models)) throw new Error('Expected models array');
        console.log(`   Found ${data.models.length} models`);
    });
}

/**
 * Test letta_memory_unified
 */
async function testMemoryUnified() {
    await runTest('letta_memory_unified: list_blocks', async () => {
        const result = await callTool('letta_memory_unified', {
            operation: 'list_blocks',
            templates_only: true,
            limit: 5
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        console.log(`   Found ${data.total || 0} memory blocks`);
    });
}

/**
 * Test letta_tool_manager
 */
async function testToolManager() {
    await runTest('letta_tool_manager: list', async () => {
        const result = await callTool('letta_tool_manager', {
            operation: 'list',
            limit: 5
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.tools)) throw new Error('Expected tools array');
        console.log(`   Found ${data.tools.length} tools`);
    });
}

/**
 * Test letta_mcp_ops
 */
async function testMcpOps() {
    await runTest('letta_mcp_ops: list_servers', async () => {
        const result = await callTool('letta_mcp_ops', {
            operation: 'list_servers'
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.servers)) throw new Error('Expected servers array');
        console.log(`   Found ${data.servers.length} MCP servers`);
    });
}

/**
 * Test letta_source_manager
 */
async function testSourceManager() {
    await runTest('letta_source_manager: list', async () => {
        const result = await callTool('letta_source_manager', {
            operation: 'list',
            limit: 5
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        console.log(`   Found ${data.total || 0} sources`);
    });
}

/**
 * Test letta_job_monitor
 */
async function testJobMonitor() {
    await runTest('letta_job_monitor: list', async () => {
        const result = await callTool('letta_job_monitor', {
            operation: 'list'
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.jobs)) throw new Error('Expected jobs array');
        console.log(`   Found ${data.jobs.length} jobs`);
    });
}

/**
 * Test letta_file_folder_ops
 */
async function testFileFolderOps() {
    await runTest('letta_file_folder_ops: list_folders', async () => {
        const result = await callTool('letta_file_folder_ops', {
            operation: 'list_folders'
        });
        const data = JSON.parse(result.content[0].text);
        if (!data.success) throw new Error('Operation failed');
        if (!Array.isArray(data.folders)) throw new Error('Expected folders array');
        console.log(`   Found ${data.folders.length} folders`);
    });
}

/**
 * Test tool listing
 */
async function testToolListing() {
    await runTest('MCP tools/list endpoint', async () => {
        const headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        };

        if (SESSION_ID) {
            headers['x-mcp-session-id'] = SESSION_ID;
        }

        const response = await axios.post(MCP_ENDPOINT, {
            jsonrpc: '2.0',
            method: 'tools/list',
            id: Date.now()
        }, { headers });

        if (response.data.error) {
            throw new Error(`Error: ${JSON.stringify(response.data.error)}`);
        }

        const tools = response.data.result.tools;
        if (!Array.isArray(tools)) throw new Error('Expected tools array');

        // Check for consolidated tools
        const consolidatedTools = [
            'letta_agent_advanced',
            'letta_memory_unified',
            'letta_tool_manager',
            'letta_mcp_ops',
            'letta_source_manager',
            'letta_job_monitor',
            'letta_file_folder_ops'
        ];

        for (const toolName of consolidatedTools) {
            const tool = tools.find(t => t.name === toolName);
            if (!tool) throw new Error(`Missing consolidated tool: ${toolName}`);
        }

        console.log(`   Found all 7 consolidated tools in ${tools.length} total tools`);

        // Check for deprecation warnings
        const deprecatedTool = tools.find(t => t.name === 'create_agent');
        if (deprecatedTool && !deprecatedTool.description.includes('DEPRECATED')) {
            console.warn('   ⚠️  Warning: Deprecated tools missing deprecation notice');
        } else if (deprecatedTool) {
            console.log('   ✓ Deprecation warnings are working');
        }
    });
}

/**
 * Main test runner
 */
async function runAllTests() {
    console.log('═══════════════════════════════════════════════════════════');
    console.log('  Letta MCP Server - Integration Tests');
    console.log('  Testing consolidated tools with real Letta instance');
    console.log('═══════════════════════════════════════════════════════════');

    // Initialize MCP session
    await initializeSession();

    // Test tool listing first
    await testToolListing();

    // Test each consolidated tool
    await testAgentAdvanced();
    await testMemoryUnified();
    await testToolManager();
    await testMcpOps();
    await testSourceManager();
    await testJobMonitor();
    await testFileFolderOps();

    // Summary
    console.log('\n═══════════════════════════════════════════════════════════');
    console.log('  Test Results');
    console.log('═══════════════════════════════════════════════════════════');

    const passed = TEST_RESULTS.filter(r => r.status === 'PASS').length;
    const failed = TEST_RESULTS.filter(r => r.status === 'FAIL').length;
    const total = TEST_RESULTS.length;

    console.log(`\n✅ Passed: ${passed}/${total}`);
    if (failed > 0) {
        console.log(`❌ Failed: ${failed}/${total}`);
        console.log('\nFailed tests:');
        TEST_RESULTS.filter(r => r.status === 'FAIL').forEach(r => {
            console.log(`  - ${r.name}`);
            console.log(`    ${r.error}`);
        });
    }

    console.log('\n═══════════════════════════════════════════════════════════\n');

    process.exit(failed > 0 ? 1 : 0);
}

// Run tests
runAllTests().catch(error => {
    console.error('\n❌ Integration test suite failed:');
    console.error(error);
    process.exit(1);
});
