# API Response Fixtures

This directory contains real API responses from the Letta server for regression testing.

## Purpose

When the Letta API returns unexpected response shapes (like `null` instead of an object), we capture them here to:
1. Ensure our deserializers handle all known response shapes
2. Document edge cases discovered in production
3. Prevent regressions when SDK types change

## Adding New Fixtures

When you discover a new API response shape:

1. **Capture the raw response**
   ```bash
   curl -H "Authorization: Bearer $TOKEN" \
        https://api.letta.com/v1/agents > new_response.json
   ```

2. **Add the fixture file**
   ```bash
   cp new_response.json rust/vendor/letta/tests/fixtures/
   ```

3. **Add a test case** in `../api_fixtures_test.rs`:
   ```rust
   #[test]
   fn test_new_response() {
       let json = include_str!("fixtures/new_response.json");
       let result: Result<ExpectedType, _> = serde_json::from_str(json);
       assert!(result.is_ok(), "Failed to deserialize: {:?}", result.err());
   }
   ```

## Current Fixtures

- `attach_tool_null.json` - Null response from attach/detach operations (Issue #134)
- `attach_tool_valid.json` - Valid AgentState response from attach
- `list_agents_response.json` - Array of AgentState objects
- `list_tools_response.json` - Array of Tool objects  
- `archival_memory_search.json` - Array of Passage objects

## Running Tests

```bash
cd rust/vendor/letta
cargo test api_fixtures
```

All fixtures are tested during normal `cargo test` runs - no live server required.
