#!/usr/bin/env bash
# Test script for running cloud API integration tests

set -euo pipefail

# Check if API key is provided
if [ -z "${LETTA_API_KEY:-}" ]; then
    echo "❌ LETTA_API_KEY environment variable is required for cloud API tests"
    echo "Usage: LETTA_API_KEY=your-key $0"
    exit 1
fi

echo "🌩️  Running cloud API integration tests..."

# Run only the cloud API tests (which are marked with #[ignore])
echo "🧪 Running ignored tests (cloud API tests)..."
cargo test --test '*cloud*' -- --ignored --nocapture

echo "✅ Cloud API tests completed!"