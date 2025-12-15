#!/usr/bin/env bash
# Test script for running integration tests with local Letta server

set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Project root is one level up from nix/
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 Starting local Letta server for integration tests..."

# Change to project root where compose.yml is located
cd "$PROJECT_ROOT"

# Start docker compose in the background
docker compose up -d

# Function to cleanup on exit
cleanup() {
    echo "🛑 Stopping local Letta server..."
    cd "$PROJECT_ROOT"
    docker compose down
}
trap cleanup EXIT

# Wait for server to be ready
echo "⏳ Waiting for server to be ready..."
max_attempts=30
attempt=0

while ! curl -s http://localhost:8283/v1/health >/dev/null 2>&1; do
    attempt=$((attempt + 1))
    if [ $attempt -ge $max_attempts ]; then
        echo "❌ Server failed to start after $max_attempts attempts"
        exit 1
    fi
    echo "  Attempt $attempt/$max_attempts..."
    sleep 2
done

echo "✅ Server is ready!"

# Run the tests that depend on local server
echo "🧪 Running integration tests..."
cargo test --test '*' -- --nocapture

echo "✅ All tests completed!"