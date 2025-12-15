#!/bin/bash
set -e

echo "=========================================="
echo "Rust Letta MCP Server - Functional Test"
echo "=========================================="
echo ""

MCP_URL="http://localhost:3001/mcp"

TEST_COUNT=0
PASS_COUNT=0
FAIL_COUNT=0

run_test() {
  local test_name="$1"
  local tool_name="$2"
  local operation="$3"
  local extra_args="${4:-}"
  
  TEST_COUNT=$((TEST_COUNT + 1))
  echo "Test $TEST_COUNT: $test_name"
  
  local args="{\"operation\": \"$operation\""
  if [ -n "$extra_args" ]; then
    args="$args, $extra_args"
  fi
  args="$args}"
  
  RESPONSE=$(curl -s -X POST "$MCP_URL" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"jsonrpc\": \"2.0\",
      \"method\": \"tools/call\",
      \"params\": {
        \"name\": \"$tool_name\",
        \"arguments\": $args
      },
      \"id\": $TEST_COUNT
    }")
  
  if echo "$RESPONSE" | jq -e '.error' > /dev/null 2>&1; then
    echo "  ✗ FAIL: $(echo "$RESPONSE" | jq -r '.error.message' | head -c 80)"
    FAIL_COUNT=$((FAIL_COUNT + 1))
  else
    echo "  ✓ PASS"
    PASS_COUNT=$((PASS_COUNT + 1))
  fi
  echo ""
}

# Initialize
echo "Initializing MCP session..."
curl -s -X POST "$MCP_URL" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}},"id":1}' > /dev/null
echo "✓ Session initialized"
echo ""

echo "Running functional tests..."
echo ""

# Test letta_agent_advanced operations
run_test "List agents" "letta_agent_advanced" "list" "\"pagination\": {\"limit\": 5}"
run_test "Count agents" "letta_agent_advanced" "count"

# Test letta_memory_unified operations  
run_test "List memory blocks (templates)" "letta_memory_unified" "list_blocks" "\"templates_only\": true, \"limit\": 5"

# Test letta_tool_manager operations
run_test "List Letta tools" "letta_tool_manager" "list" "\"limit\": 10"

# Test letta_mcp_ops operations
run_test "List MCP servers" "letta_mcp_ops" "list_servers"

# Test letta_source_manager operations
run_test "List sources" "letta_source_manager" "list" "\"limit\": 5"

# Test letta_job_monitor operations
run_test "List jobs" "letta_job_monitor" "list"

# Test letta_file_folder_ops operations
run_test "List folders" "letta_file_folder_ops" "list_folders"

echo "=========================================="
echo "Test Results"
echo "=========================================="
echo "Total:  $TEST_COUNT"
echo "Passed: $PASS_COUNT"
echo "Failed: $FAIL_COUNT"
echo ""

if [ $FAIL_COUNT -eq 0 ]; then
  echo "✓ All tests passed!"
  exit 0
else
  echo "✗ Some tests failed"
  echo ""
  echo "Note: Some failures may be due to:"
  echo "- Letta API response format changes"
  echo "- Rust SDK deserialization issues"
  echo "- Missing required parameters"
  exit 1
fi
