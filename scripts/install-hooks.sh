#!/usr/bin/env bash
# Install git pre-commit hooks

set -e

HOOK_DIR=".git/hooks"
HOOK_FILE="$HOOK_DIR/pre-commit"

echo "Installing pre-commit hooks..."

# Create hooks directory if it doesn't exist
mkdir -p "$HOOK_DIR"

# Create pre-commit hook
cat > "$HOOK_FILE" << 'HOOK'
#!/usr/bin/env bash
# Pre-commit hook for Letta MCP Server

set -e

echo "🔍 Running pre-commit checks..."

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ cargo not found. Please install Rust."
    exit 1
fi

# Run rustfmt
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed!"
    echo "💡 Run 'cargo fmt --all' to fix formatting"
    exit 1
fi

# Run clippy
echo "🔧 Running clippy..."
if ! cargo clippy --all-targets --no-deps -- -D warnings 2>&1 | grep -q "0 errors"; then
    echo "❌ Clippy found issues!"
    echo "💡 Run 'cargo clippy --fix --allow-dirty' to auto-fix some issues"
    exit 1
fi

echo "✅ All pre-commit checks passed!"
HOOK

# Make hook executable
chmod +x "$HOOK_FILE"

echo "✅ Pre-commit hooks installed successfully!"
echo ""
echo "The following checks will run before each commit:"
echo "  - Code formatting (rustfmt)"
echo "  - Linting (clippy)"
echo ""
echo "To skip hooks (not recommended): git commit --no-verify"
