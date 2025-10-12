#!/usr/bin/env node

import axios from 'axios';
import dotenv from 'dotenv';

// Load environment variables
dotenv.config();

// Remove /v1 from base URL since endpoints already include it
const LETTA_BASE_URL = process.env.LETTA_BASE_URL?.replace(/\/v1\/?$/, '') || 'https://letta2.oculair.ca';
const LETTA_PASSWORD = process.env.LETTA_PASSWORD;

if (!LETTA_PASSWORD) {
    console.error('❌ LETTA_PASSWORD environment variable is required');
    process.exit(1);
}

// MCP servers to register - properly formatted for the API
const mcpServers = [
    { 
        server_name: 'filesystem', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:8100/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'letta', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3001/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'bookstack', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3054/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'claude-code-mcp', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3456/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'context7', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3054/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'ghost', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3064/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'hayhooks', 
        type: 'sse',
        server_url: 'http://192.168.50.90:1416/sse',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'jopbspy', 
        type: 'sse',
        server_url: 'http://192.168.50.90:9232/sse',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'lettatoolsselector', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3020/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'matrix', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:8016/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'plane', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3094/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'postiz', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:3084/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'resumerx', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:9722/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    },
    { 
        server_name: 'huly', 
        type: 'streamable_http',
        server_url: 'http://192.168.50.90:5439/mcp',
        auth_header: null,
        auth_token: null,
        custom_headers: {}
    }
];

async function registerMCPServer(server) {
    try {
        console.log(`📝 Registering ${server.server_name}...`);
        
        // Use PUT to add server to config with /v1 in path
        const response = await axios.put(`${LETTA_BASE_URL}/v1/tools/mcp/servers`, server, {
            headers: {
                'Authorization': `Bearer ${LETTA_PASSWORD}`,
                'Content-Type': 'application/json'
            }
        });
        
        console.log(`✅ Successfully registered ${server.server_name}`);
        return response.data;
        
    } catch (error) {
        if (error.response?.status === 422) {
            console.error(`❌ Validation error for ${server.server_name}:`, error.response.data.detail?.[0]?.msg || JSON.stringify(error.response.data));
        } else {
            console.error(`❌ Failed to register ${server.server_name}: ${error.response?.data?.detail || error.message}`);
        }
        return null;
    }
}

async function main() {
    console.log('🚀 Starting MCP server registration...');
    console.log(`🔗 Letta API: ${LETTA_BASE_URL}`);
    
    try {
        // Register servers one by one
        let successCount = 0;
        for (const server of mcpServers) {
            const result = await registerMCPServer(server);
            if (result) successCount++;
        }
        
        console.log(`\n✨ Registration complete! ${successCount}/${mcpServers.length} servers registered.`);
        
        // Get all registered servers
        console.log('\n📋 Fetching all registered MCP servers...');
        try {
            const response = await axios.get(`${LETTA_BASE_URL}/v1/tools/mcp/servers`, {
                headers: {
                    'Authorization': `Bearer ${LETTA_PASSWORD}`
                }
            });
            
            if (response.data && Array.isArray(response.data)) {
                console.log(`Found ${response.data.length} registered servers:`);
                response.data.forEach(server => {
                    console.log(`   - ${server.server_name}: ${server.type} (${server.server_url || server.command || 'stdio'})`);
                });
            }
        } catch (error) {
            console.log('⚠️  Could not fetch server list');
        }
        
        // Now test listing tools from each server
        console.log('\n🔍 Checking tools from each server:');
        for (const server of mcpServers) {
            try {
                const response = await axios.get(`${LETTA_BASE_URL}/v1/tools/mcp/servers/${server.server_name}/tools`, {
                    headers: {
                        'Authorization': `Bearer ${LETTA_PASSWORD}`
                    }
                });
                console.log(`   ${server.server_name}: ${response.data.length || 0} tools available`);
            } catch (error) {
                console.log(`   ${server.server_name}: ⚠️  ${error.response?.status === 404 ? 'Not found' : 'Error checking tools'}`);
            }
        }
        
    } catch (error) {
        console.error('\n❌ Registration failed:', error.message);
    }
}

// Run the script
main().catch(error => {
    console.error('❌ Script failed:', error);
    process.exit(1);
});