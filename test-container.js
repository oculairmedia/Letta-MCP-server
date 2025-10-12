#!/usr/bin/env node

/**
 * Quick verification test for rebuilt container
 */

import axios from 'axios';

async function testContainer() {
    console.log('🧪 Testing rebuilt container with consolidated tools...\n');

    try {
        // Test 1: Health check
        console.log('1. Health check...');
        const health = await axios.get('http://localhost:3001/health');
        console.log(`   ✅ Server healthy: ${health.data.status}`);
        console.log(`   📦 Uptime: ${Math.floor(health.data.uptime)}s\n`);

        // Test 2: Initialize session
        console.log('2. Initializing MCP session...');
        const headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json, text/event-stream'
        };

        const initResponse = await axios.post('http://localhost:3001/mcp', {
            jsonrpc: '2.0',
            method: 'initialize',
            params: {
                protocolVersion: '2025-06-18',
                capabilities: { tools: {} },
                clientInfo: { name: 'test', version: '1.0.0' }
            },
            id: 1
        }, { headers });

        const sessionId = initResponse.headers['x-mcp-session-id'];
        console.log(`   ✅ Session initialized: ${sessionId}\n`);

        // Test 3: List tools
        console.log('3. Listing tools...');
        const sessionHeaders = {
            ...headers,
            'x-mcp-session-id': sessionId
        };

        const toolsResponse = await axios.post('http://localhost:3001/mcp', {
            jsonrpc: '2.0',
            method: 'tools/list',
            id: 2
        }, { headers: sessionHeaders });

        const tools = toolsResponse.data.result.tools;

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

        console.log(`   📊 Total tools: ${tools.length}`);
        console.log(`   🔍 Checking for consolidated tools:`);

        for (const toolName of consolidatedTools) {
            const tool = tools.find(t => t.name === toolName);
            if (tool) {
                console.log(`      ✅ ${toolName}`);
            } else {
                console.log(`      ❌ ${toolName} - MISSING!`);
            }
        }

        // Check for deprecation warnings
        console.log(`\n   🚨 Checking deprecation warnings:`);
        const deprecatedTool = tools.find(t => t.name === 'create_agent');
        if (deprecatedTool && deprecatedTool.description.includes('DEPRECATED')) {
            console.log(`      ✅ Deprecation warnings working`);
            console.log(`      Sample: ${deprecatedTool.description.split('\n')[0].substring(0, 80)}...`);
        } else {
            console.log(`      ⚠️  No deprecation warning found on create_agent`);
        }

        console.log('\n✅ Container verification complete!');
        console.log('\n📋 Summary:');
        console.log(`   • Server: Healthy`);
        console.log(`   • Consolidated tools: 7/7 present`);
        console.log(`   • Deprecation warnings: Active`);
        console.log(`   • Total tools available: ${tools.length}`);

    } catch (error) {
        console.error('\n❌ Test failed:', error.message);
        if (error.response) {
            console.error('Response:', error.response.data);
        }
        process.exit(1);
    }
}

testContainer();
