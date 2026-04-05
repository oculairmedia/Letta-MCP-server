# rust/ Directory

This directory contains:

- **`vendor/`** - Vendored Letta Rust SDK with local compatibility patches
- **`docker-compose.yml`** - Production deployment configuration for letta-mcp-rust container

## Important Notes

- **DO NOT** create a Cargo workspace here - the ROOT workspace is the active one
- **DO NOT** duplicate source code here - edit `letta-server/` and `letta-types/` at the repo root
- The vendored SDK is referenced by the root `Cargo.toml` via `path = "rust/vendor/letta"`

## Historical Context

This directory previously contained a duplicate TurboMCP v2 workspace. That has been removed.
The current codebase uses TurboMCP v3 defined in the root workspace.
