# Contributing to Letta MCP Server

Thank you for your interest in contributing to the Letta MCP Server! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/Letta-MCP-server.git
   cd Letta-MCP-server
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/oculairmedia/Letta-MCP-server.git
   ```

## Development Setup

### Prerequisites

- **Rust 1.85+** (install via [rustup](https://rustup.rs/)) - required for edition 2024 support
- **mold linker** (optional, for faster builds):
  ```bash
  # Ubuntu/Debian
  sudo apt-get install mold
  
  # macOS
  brew install mold
  ```
- **just** (task runner, optional but recommended):
  ```bash
  cargo install just
  ```

### Quick Setup

```bash
# Install Rust toolchain
rustup update stable

# Build the project
cargo build

# Run tests
cargo test

# Run with justfile (if installed)
just build
just test
```

## Making Changes

### Branching Strategy

- Create feature branches from `master`
- Use descriptive branch names: `feat/add-streaming`, `fix/null-response`, `docs/contributing`
- Keep branches focused on a single feature or fix

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Build process, dependencies, etc.

**Examples:**
```
feat(memory): add archival search endpoint routing

fix(agent): handle null responses from attach/detach operations

docs: update README with coverage badge

test: add snapshot tests for MCP response shapes
```

## Coding Standards

### Rust Style

- **Format code** before committing:
  ```bash
  cargo fmt --all
  ```

- **Run clippy** and fix warnings:
  ```bash
  cargo clippy --all-targets --no-deps -- -D warnings
  ```

- **Follow Rust naming conventions**:
  - `snake_case` for functions, variables, modules
  - `CamelCase` for types, traits
  - `SCREAMING_SNAKE_CASE` for constants

### Code Quality

- **Write tests** for new features and bug fixes
- **Update snapshots** if response shapes change:
  ```bash
  cargo insta review
  ```
- **Add documentation** for public APIs
- **Keep functions focused** - one responsibility per function
- **Avoid unwrap()** - use proper error handling

### Performance

- Use `tokio::join!` for parallel operations
- Avoid blocking operations in async code
- Profile before optimizing

## Testing

### Running Tests

```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run snapshot tests
cargo test snapshots_test

# Run integration tests (requires Letta server)
cargo test --features integration-tests
```

### Writing Tests

1. **Unit tests** - in the same file as the code:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_something() {
           assert_eq!(2 + 2, 4);
       }
   }
   ```

2. **Snapshot tests** - for response shapes:
   ```rust
   #[test]
   fn test_response_shape() {
       let response = create_response();
       insta::assert_json_snapshot!("response_name", response);
   }
   ```

3. **Integration tests** - in `tests/` directory

### Test Coverage

- Aim for >60% coverage on new code
- Run coverage locally:
  ```bash
  cargo llvm-cov --package letta-server --lcov --output-path lcov.info
  ```

## Submitting Changes

### Before Submitting

1. **Update from upstream**:
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. **Run all checks**:
   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --no-deps -- -D warnings
   cargo test
   ```

3. **Update documentation** if needed

### Pull Request Process

1. **Push to your fork**:
   ```bash
   git push origin your-branch-name
   ```

2. **Create Pull Request** on GitHub with:
   - Clear title following conventional commit format
   - Description of changes
   - Link to related issues (`Closes #123`)
   - Screenshots/examples if applicable

3. **PR Checklist**:
   - [ ] Tests pass locally
   - [ ] Code formatted (`cargo fmt`)
   - [ ] No clippy warnings
   - [ ] Documentation updated
   - [ ] Snapshot tests reviewed (if applicable)
   - [ ] Commit messages follow conventions

4. **Code Review**:
   - Address reviewer feedback
   - Keep discussion constructive
   - Update PR based on feedback

5. **Merge**:
   - Maintainers will merge once approved
   - PRs are squash-merged to keep history clean

## Release Process

Releases are managed by maintainers:

1. Version bump in `Cargo.toml` and `package.json`
2. Update CHANGELOG.md
3. Create git tag: `vX.Y.Z`
4. GitHub Actions builds and publishes to npm
5. GitHub release created with changelog

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones
- Tag issues appropriately (`bug`, `enhancement`, `documentation`)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
