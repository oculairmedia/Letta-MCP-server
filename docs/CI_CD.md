# CI/CD Pipeline Documentation

## Overview

The Letta MCP Server Rust implementation uses GitHub Actions for automated testing, building, and deployment. The CI/CD pipeline ensures code quality, security, and reliable deployments.

## Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust CI/CD Pipeline                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │         Stage 1: Tests & Quality        │
        │  ┌───────────────────────────────────┐  │
        │  │  • Run 312 tests                  │  │
        │  │  • Code formatting (rustfmt)      │  │
        │  │  • Linting (clippy)              │  │
        │  │  • Security audit (cargo-audit)   │  │
        │  └───────────────────────────────────┘  │
        └─────────────────────────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │      Stage 2: Coverage & Benchmarks     │
        │  ┌───────────────────────────────────┐  │
        │  │  • Generate coverage report       │  │
        │  │  • Upload to Codecov              │  │
        │  │  • Run performance benchmarks     │  │
        │  └───────────────────────────────────┘  │
        └─────────────────────────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │        Stage 3: Docker Build            │
        │  ┌───────────────────────────────────┐  │
        │  │  • Build multi-arch images        │  │
        │  │  • Push to GHCR                   │  │
        │  │  • Security scan (Trivy)          │  │
        │  └───────────────────────────────────┘  │
        └─────────────────────────────────────────┘
```

## Workflows

### 1. Rust Tests (`rust-test.yml`)

**Triggers:**
- Push to `rust-implementation`, `main`, or `master` branches
- Pull requests to these branches
- Changes to Rust code, tests, or dependencies

**Jobs:**
- **test**: Runs all 312 tests on stable and nightly Rust
  - Checks code formatting
  - Runs clippy linter
  - Executes test suite
  - Generates test report

- **coverage**: Generates code coverage report
  - Uses `cargo-tarpaulin`
  - Uploads to Codecov
  - Targets 85% coverage

- **security**: Security audit
  - Runs `cargo audit`
  - Checks for vulnerable dependencies

- **benchmark**: Performance benchmarks
  - Measures test execution time
  - Target: < 2 seconds

**Matrix Testing:**
```yaml
strategy:
  matrix:
    rust: [stable, nightly]
```

### 2. Rust Docker Build (`rust-docker-build.yml`)

**Triggers:**
- Push to `rust-implementation` branch
- Pull requests
- After tests pass (workflow_run)

**Features:**
- Multi-architecture builds (amd64, arm64)
- Layer caching for faster builds
- Security scanning with Trivy
- Push to GitHub Container Registry

**Image Tags:**
- `rust-latest` - Latest build from rust-implementation branch
- `rust-{branch}` - Branch-specific tag
- `rust-{sha}` - Commit SHA tag
- `rust-{branch}-{sha}` - Combined tag

**Pull Command:**
```bash
docker pull ghcr.io/oculairmedia/letta-mcp-server-rust:rust-latest
```

### 3. Rust CI/CD Pipeline (`rust-ci-cd.yml`)

**Triggers:**
- Push to `rust-implementation`, `main`, or `master` branches
- Pull requests

**Orchestration:**
1. Runs tests workflow
2. If tests pass, builds Docker image
3. Sends success notification

**Features:**
- Reusable workflow composition
- Sequential execution (tests → build)
- Comprehensive status reporting

## Test Coverage

The test suite includes **312 tests** covering:

| Tool | Tests | Operations |
|------|-------|------------|
| Agent Advanced | 32 | 23 |
| Memory Unified | 59 | 15 |
| Tool Manager | 53 | 13 |
| Source Manager | 50 | 13 |
| MCP Ops | 42 | 10 |
| Job Monitor | 34 | 4 |
| File/Folder Ops | 16 | 8 |
| Optimization Tests | 18 | - |
| Test Helpers | 8 | - |
| **TOTAL** | **312** | **91** |

**Estimated Coverage:** ~85%  
**Execution Time:** < 1 second

## Security

### Vulnerability Scanning

**Trivy:** Scans Docker images for:
- CRITICAL and HIGH severity vulnerabilities
- Results uploaded to GitHub Security tab

**Cargo Audit:** Checks dependencies for:
- Known security vulnerabilities
- Unmaintained crates
- Yanked versions

### Supply Chain Security

- **Dependabot:** Automated dependency updates
- **CodeQL:** Static analysis for security issues
- **SARIF Upload:** Security results in GitHub Security

## Caching Strategy

### Cargo Registry Cache
```yaml
key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
```

### Cargo Build Cache
```yaml
key: ${{ runner.os }}-cargo-build-${{ matrix.rust }}-${{ hashFiles('**/Cargo.lock') }}
```

### Docker Build Cache
```yaml
cache-from: type=gha,scope=rust
cache-to: type=gha,mode=max,scope=rust
```

**Benefits:**
- Faster CI runs (30-60% time reduction)
- Reduced bandwidth usage
- Lower GitHub Actions costs

## Branch Strategy

### rust-implementation
- All Rust development happens here
- Full CI/CD pipeline runs on every push
- Docker images tagged with `rust-*` prefix

### main/master
- Production-ready code
- Node.js implementation (legacy)
- Separate Docker workflow

## Status Badges

Add to README.md:

```markdown
![Rust Tests](https://github.com/oculairmedia/Letta-MCP-server/workflows/Rust%20Tests/badge.svg)
![Docker Build](https://github.com/oculairmedia/Letta-MCP-server/workflows/Rust%20Docker%20Build%20and%20Push/badge.svg)
[![codecov](https://codecov.io/gh/oculairmedia/Letta-MCP-server/branch/rust-implementation/graph/badge.svg)](https://codecov.io/gh/oculairmedia/Letta-MCP-server)
```

## Local Development

### Running Tests Locally
```bash
# All tests
cargo test --tests

# Specific test file
cargo test --test source_manager_test

# With output
cargo test -- --nocapture

# Fast mode (no optimizations)
cargo test --lib
```

### Checking Before Push
```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run all checks
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --tests
```

### Generate Coverage Locally
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --out Html --output-dir coverage

# Open in browser
open coverage/index.html  # macOS
firefox coverage/index.html  # Linux
```

## Secrets Required

Configure in GitHub Settings → Secrets and variables → Actions:

| Secret | Purpose | Required |
|--------|---------|----------|
| `GITHUB_TOKEN` | Automatic, for GHCR push | ✅ Auto |
| `CODECOV_TOKEN` | Upload coverage to Codecov | ⚠️ Optional |

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Test Execution | < 2s | ~1s ✅ |
| Code Coverage | ≥ 85% | ~85% ✅ |
| Docker Build | < 10m | ~8m ✅ |
| Total Pipeline | < 15m | ~12m ✅ |

## Troubleshooting

### Tests Failing in CI but Passing Locally

**Issue:** Different Rust versions

**Solution:**
```bash
# Use same version as CI
rustup install stable
rustup default stable
cargo test
```

### Docker Build Timeout

**Issue:** Large dependencies, no cache

**Solution:**
- Check cache hit rate in workflow logs
- Verify Cargo.lock is committed
- Use `docker/build-push-action@v6` (includes optimizations)

### Coverage Upload Fails

**Issue:** Missing CODECOV_TOKEN

**Solution:**
1. Get token from https://codecov.io
2. Add to GitHub Secrets
3. Workflow marked as `fail_ci_if_error: false` so non-blocking

## Future Enhancements

- [ ] Automated releases with semantic versioning
- [ ] Integration tests against live Letta API
- [ ] Performance regression testing
- [ ] Automated changelog generation
- [ ] Multi-environment deployments (staging, production)
- [ ] Slack/Discord notifications
- [ ] Deployment to Kubernetes cluster

## Monitoring

### GitHub Actions Dashboard
View workflow runs: https://github.com/oculairmedia/Letta-MCP-server/actions

### Codecov Dashboard
View coverage trends: https://codecov.io/gh/oculairmedia/Letta-MCP-server

### Security Dashboard
View security alerts: https://github.com/oculairmedia/Letta-MCP-server/security

## Contributing

When submitting PRs:
1. ✅ All tests must pass
2. ✅ Code must be formatted (`cargo fmt`)
3. ✅ No clippy warnings (`cargo clippy`)
4. ✅ Coverage should not decrease
5. ✅ Security scan must pass

## Support

For CI/CD issues:
1. Check workflow logs in GitHub Actions
2. Review error messages
3. Test locally with same Rust version
4. Open issue with `ci/cd` label

---

**Last Updated:** December 15, 2024  
**Pipeline Version:** 1.0  
**Test Count:** 312  
**Coverage:** ~85%
