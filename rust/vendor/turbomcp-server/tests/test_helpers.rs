//! Test helper functions to eliminate DRY violations in server tests
//!
//! This module provides common test utilities and patterns to reduce
//! boilerplate code across the test suite.

use turbomcp_server::{McpServer, ServerBuilder};

/// Create a default ServerBuilder for tests with optional name override
pub fn test_server_builder() -> ServerBuilder {
    ServerBuilder::new()
}

/// Create a ServerBuilder with a specific test name
pub fn test_server_builder_named(name: &str) -> ServerBuilder {
    ServerBuilder::new().name(name)
}

/// Create a ServerBuilder with test name and version
pub fn test_server_builder_versioned(name: &str, version: &str) -> ServerBuilder {
    ServerBuilder::new().name(name).version(version)
}

/// Create a minimal test server with default configuration
pub fn test_server() -> McpServer {
    test_server_builder().build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_creates_builder() {
        let builder = test_server_builder();
        let debug_str = format!("{builder:?}");
        assert!(debug_str.contains("ServerBuilder"));
    }

    #[tokio::test]
    async fn test_helper_creates_named_builder() {
        let builder = test_server_builder_named("test-server");
        let server = builder.build();
        assert_eq!(server.config().name, "test-server");
    }

    #[tokio::test]
    async fn test_helper_creates_versioned_builder() {
        let builder = test_server_builder_versioned("test-server", "2.0.0");
        let server = builder.build();
        assert_eq!(server.config().name, "test-server");
        assert_eq!(server.config().version, "2.0.0");
    }

    #[tokio::test]
    async fn test_helper_creates_server() {
        let server = test_server();
        assert_eq!(server.config().name, "turbomcp-server");
    }
}
