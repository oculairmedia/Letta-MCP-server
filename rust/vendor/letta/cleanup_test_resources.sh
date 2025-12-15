#!/usr/bin/env bash
# Cleanup test resources from Letta server

BASE_URL="http://localhost:8283"

echo "Starting cleanup of test resources..."

# Function to delete resources
delete_resource() {
    local endpoint=$1
    local id=$2
    local name=$3
    
    echo "  Deleting $name..."
    response=$(curl -s -X DELETE "$BASE_URL$endpoint/$id" -w "\n%{http_code}")
    http_code=$(echo "$response" | tail -n1)
    
    if [ "$http_code" = "200" ] || [ "$http_code" = "204" ]; then
        echo "    ✓ Deleted successfully"
    else
        echo "    ✗ Failed (HTTP $http_code)"
    fi
}

# Clean up agents
echo -e "\nCleaning up agents..."
agents=$(curl -s "$BASE_URL/v1/agents" | jq -r '.[] | select(.name | test("Test|test")) | "\(.id)|\(.name)"')

if [ -n "$agents" ]; then
    echo "$agents" | while IFS='|' read -r id name; do
        delete_resource "/v1/agents" "$id" "agent: $name ($id)"
    done
else
    echo "  No test agents found"
fi

# Clean up tools  
echo -e "\nCleaning up tools..."
tools=$(curl -s "$BASE_URL/v1/tools" | jq -r '.[] | select(.name | test("test|Test|echo|calculator|add_numbers|delete_test")) | "\(.id)|\(.name)"')

if [ -n "$tools" ]; then
    echo "$tools" | while IFS='|' read -r id name; do
        delete_resource "/v1/tools" "$id" "tool: $name ($id)"
    done
else
    echo "  No test tools found"
fi

# Clean up memory blocks
echo -e "\nCleaning up memory blocks..."
blocks=$(curl -s "$BASE_URL/v1/blocks" | jq -r '.[] | select((.label | test("test|Test")) or (.value | test("Test|test"))) | "\(.id)|\(.label)"')

if [ -n "$blocks" ]; then
    echo "$blocks" | while IFS='|' read -r id label; do
        delete_resource "/v1/blocks" "$id" "block: $label ($id)"
    done
else
    echo "  No test blocks found"
fi

# Clean up sources
echo -e "\nCleaning up sources..."
sources=$(curl -s "$BASE_URL/v1/sources" | jq -r '.[] | select(.name | test("test|Test")) | "\(.id)|\(.name)"')

if [ -n "$sources" ]; then
    echo "$sources" | while IFS='|' read -r id name; do
        delete_resource "/v1/sources" "$id" "source: $name ($id)"
    done
else
    echo "  No test sources found"
fi

# Clean up MCP servers
echo -e "\nCleaning up MCP servers..."
mcp_servers=$(curl -s "$BASE_URL/v1/tools/mcp/servers" | jq -r '.[] | select(.server_name | test("test")) | .server_name')

if [ -n "$mcp_servers" ]; then
    echo "$mcp_servers" | while read -r name; do
        echo "  Deleting MCP server: $name..."
        response=$(curl -s -X DELETE "$BASE_URL/v1/tools/mcp/servers/$name" -w "\n%{http_code}")
        http_code=$(echo "$response" | tail -n1)
        
        if [ "$http_code" = "200" ] || [ "$http_code" = "204" ]; then
            echo "    ✓ Deleted successfully"
        else
            echo "    ✗ Failed (HTTP $http_code)"
        fi
    done
else
    echo "  No test MCP servers found"
fi

echo -e "\nCleanup complete!"

# Check if server CPU is still high
echo -e "\nChecking server status..."
ps aux | grep "letta server" | grep -v grep | awk '{print "Letta server CPU usage: " $3 "%"}'