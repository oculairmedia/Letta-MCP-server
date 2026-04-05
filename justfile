# Letta MCP Server - Development Tasks
# Install just: cargo install just
# Run: just <recipe>

# Default recipe - show available commands
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run specific test
test-one TEST:
    cargo test {{TEST}}

# Run snapshot tests
test-snapshots:
    cargo test snapshots_test --lib

# Review snapshot changes
review-snapshots:
    cargo insta review

# Run integration tests (requires Letta server)
test-integration:
    cargo test -p letta --features integration-tests

# Run fixture tests
test-fixtures:
    cargo test --test api_fixtures_test -p letta

# Format code
fmt:
    cargo fmt --all

# Check formatting without changing files
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-targets --no-deps -- -D warnings

# Run all checks (fmt, clippy, test)
check: fmt-check clippy test

# Generate coverage report
coverage:
    cargo llvm-cov --package letta-server --package letta-types --lcov --output-path lcov.info

# Clean build artifacts
clean:
    cargo clean

# Build Docker image
docker-build:
    docker build -f Dockerfile.rust -t letta-mcp-server:local .

# Run Docker container
docker-run:
    docker run -p 6507:6507 --env-file .env letta-mcp-server:local

# Start via docker-compose
docker-up:
    cd rust && docker-compose up -d

# Stop docker-compose
docker-down:
    cd rust && docker-compose down

# View docker logs
docker-logs:
    cd rust && docker-compose logs -f

# Run the server locally (stdio mode)
run:
    cargo run --bin letta-server

# Run with HTTP transport
run-http PORT="6507":
    TRANSPORT=http PORT={{PORT}} cargo run --bin letta-server

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Watch and run tests on changes
watch-test:
    cargo watch -x test

# Install development dependencies
install-deps:
    cargo install cargo-watch cargo-insta cargo-llvm-cov just

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Generate documentation
docs:
    cargo doc --no-deps --open

# Run security audit
audit:
    cargo audit

# Fix clippy warnings automatically
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Prepare for PR (format, clippy, test)
pr-ready: fmt clippy test
    @echo "✅ Ready for PR!"

# Show project statistics
stats:
    @echo "Lines of code:"
    @tokei
    @echo "\nDependencies:"
    @cargo tree --depth 1
