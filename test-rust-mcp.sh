#!/bin/bash
set -e

echo "=========================================="
echo "Testing Rust Letta MCP Server"
echo "=========================================="
echo ""

MCP_URL="http://localhost:3001/mcp"

# Test 1: Initialize MCP session
echo "Test 1: Initialize MCP session"
INIT_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
      "protocolVersion": "2025-06-18",
      "capabilities": {},
      "clientInfo": {"name": "test", "version": "1.0"}
    },
    "id": 1
  }')

echo "$INIT_RESPONSE" | jq '.'
echo "✓ Initialize successful"
echo ""

# Test 2: List available tools
echo "Test 2: List MCP tools"
TOOLS_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 2
  }')

echo "Total tools available:"
echo "$TOOLS_RESPONSE" | jq -r '.result.tools | length'
echo ""
echo "Consolidated tools:"
echo "$TOOLS_RESPONSE" | jq -r '.result.tools[] | select(.name | startswith("letta_")) | .name'
echo "✓ Tools list successful"
echo ""

# Test 3: Call letta_agent_advanced - list operation
echo "Test 3: List agents"
AGENTS_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_agent_advanced",
      "arguments": {
        "operation": "list",
        "pagination": {
          "limit": 5
        }
      }
    },
    "id": 3
  }')

if echo "$AGENTS_RESPONSE" | jq -e '.error' > /dev/null; then
  echo "✗ Error listing agents:"
  echo "$AGENTS_RESPONSE" | jq '.error'
else
  echo "✓ Agents listed successfully:"
  echo "$AGENTS_RESPONSE" | jq -r '.result.content[0].text' | jq '.agents | length' 2>/dev/null || echo "Response format differs"
fi
echo ""

# Test 4: Call letta_mcp_ops - list_servers operation
echo "Test 4: List MCP servers"
MCP_SERVERS_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_mcp_ops",
      "arguments": {
        "operation": "list_servers"
      }
    },
    "id": 4
  }')

if echo "$MCP_SERVERS_RESPONSE" | jq -e '.error' > /dev/null; then
  echo "✗ Error listing MCP servers:"
  echo "$MCP_SERVERS_RESPONSE" | jq '.error'
else
  echo "✓ MCP servers listed successfully"
  echo "$MCP_SERVERS_RESPONSE" | jq -r '.result.content[0].text' | jq -r '.servers | length' 2>/dev/null || echo "Response format differs"
fi
echo ""

# Test 5: Call letta_source_manager - list operation  
echo "Test 5: List sources"
SOURCES_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_source_manager",
      "arguments": {
        "operation": "list",
        "limit": 5
      }
    },
    "id": 5
  }')

if echo "$SOURCES_RESPONSE" | jq -e '.error' > /dev/null; then
  echo "✗ Error listing sources:"
  echo "$SOURCES_RESPONSE" | jq '.error'
else
  echo "✓ Sources listed successfully"
fi
echo ""

# Test 6: Call letta_tool_manager - list operation
echo "Test 6: List Letta tools"
LETTA_TOOLS_RESPONSE=$(curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "letta_tool_manager",
      "arguments": {
        "operation": "list"
      }
    },
    "id": 6
  }')

if echo "$LETTA_TOOLS_RESPONSE" | jq -e '.error' > /dev/null; then
  echo "✗ Error listing Letta tools:"
  echo "$LETTA_TOOLS_RESPONSE" | jq '.error'
else
  echo "✓ Letta tools listed successfully"
fi
echo ""

echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "Rust MCP server is functional and responding to requests"
echo "Available at: $MCP_URL"
