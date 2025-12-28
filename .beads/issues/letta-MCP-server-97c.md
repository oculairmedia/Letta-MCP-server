# Upgrade TurboMCP SDK from 2.0.0-rc.3 to 2.3.5 fork

## Summary
Upgrade letta-MCP-server to use the latest TurboMCP SDK (2.3.5 base with our patches) to match vibe-kanban and benefit from upstream improvements.

## Current State
- **letta-MCP-server**: Using vendored `turbomcp 2.0.0-rc.3` from git fork
- **vibe-kanban**: Using `turbomcp 2.3` with `[patch.crates-io]` pointing to fork
- **Our fork**: `oculairmedia/turbomcp` branch `feature/flatten-structs`
  - Based on upstream v2.3.5
  - Adds `$defs` support to ToolInputSchema
  - Fixes for flattened parameter schemas
  - 269 commits ahead of main (contains all upstream + our patches)

## Why Upgrade
1. **2 major versions behind** - Missing 269 commits of improvements
2. **Consistency** - Match vibe-kanban's approach
3. **Maintenance** - Remove vendored code (~50+ files in `rust/vendor/turbomcp*`)
4. **Features** - Upstream has removed progress reporting, added elicitation API, improved feature flags

## Migration Steps

### Phase 1: Update Cargo.toml
1. Remove vendored dependencies from `rust/vendor/turbomcp*`
2. Update `Cargo.toml` to use crates.io version with patch:
   ```toml
   [dependencies]
   turbomcp = { version = "2.3", features = ["http", "schemars"] }
   turbomcp-macros = "2.3"
   turbomcp-protocol = "2.3"
   turbomcp-server = "2.3"
   turbomcp-transport = { version = "2.3", features = ["http"] }
   
   [patch.crates-io]
   turbomcp = { git = "https://github.com/oculairmedia/turbomcp.git", branch = "feature/flatten-structs" }
   turbomcp-macros = { git = "https://github.com/oculairmedia/turbomcp.git", branch = "feature/flatten-structs" }
   turbomcp-protocol = { git = "https://github.com/oculairmedia/turbomcp.git", branch = "feature/flatten-structs" }
   turbomcp-server = { git = "https://github.com/oculairmedia/turbomcp.git", branch = "feature/flatten-structs" }
   turbomcp-transport = { git = "https://github.com/oculairmedia/turbomcp.git", branch = "feature/flatten-structs" }
   ```

### Phase 2: Fix Breaking Changes
1. Compile and identify API changes
2. Update imports (module paths may have changed)
3. Update any deprecated APIs
4. Key areas to check:
   - `#[tool]` macro usage
   - `CallToolResult` construction
   - `ServerError` handling
   - Schema generation with `schemars`

### Phase 3: Testing
1. Run existing test suite
2. Verify all 7 tools work correctly
3. Test schema generation (no $ref issues)
4. Integration test with MCP clients

### Phase 4: Cleanup
1. Remove `rust/vendor/turbomcp*` directories
2. Update `.gitignore` if needed
3. Update documentation

## Files to Modify
- `Cargo.toml` (root)
- `letta-server/Cargo.toml`
- `letta-types/Cargo.toml` (if applicable)
- Remove: `rust/vendor/turbomcp*` (~50+ files)

## Reference
- vibe-kanban Cargo.toml: Uses same patch approach
- Fork: https://github.com/oculairmedia/turbomcp/tree/feature/flatten-structs
- Upstream: https://crates.io/crates/turbomcp (v2.3.5)

## Acceptance Criteria
- [ ] Using turbomcp 2.3.x with patch
- [ ] All 7 tools functional
- [ ] All tests passing
- [ ] No vendored turbomcp code
- [ ] Schema validation tests still pass
- [ ] Docker build succeeds
