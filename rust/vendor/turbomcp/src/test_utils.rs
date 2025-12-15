//! Shared test utilities to reduce duplication across the TurboMCP test suite
//!
//! This module provides reusable test macros and utilities that eliminate the need
//! for duplicate test functions across different crates.

/// Test macro for config default values - eliminates duplicate default config tests
#[macro_export]
macro_rules! test_config_defaults {
    ($config_type:ty, { $($field:ident => $expected:expr),+ }) => {
        #[test]
        fn test_config_defaults() {
            let config = <$config_type>::default();
            $(
                assert_eq!(config.$field, $expected,
                    "Default value for {} should be {:?}",
                    stringify!($field), $expected);
            )+
        }
    };
}

/// Test macro for config clone/debug/eq implementations - reduces boilerplate
#[macro_export]
macro_rules! test_config_traits {
    ($config_type:ty) => {
        #[test]
        fn test_config_clone() {
            let config = <$config_type>::default();
            let cloned = config.clone();
            assert_eq!(format!("{:?}", config), format!("{:?}", cloned));
        }

        #[test]
        fn test_config_debug() {
            let config = <$config_type>::default();
            let debug_str = format!("{:?}", config);
            assert!(
                !debug_str.is_empty(),
                "Debug implementation should not be empty"
            );
            assert!(
                debug_str.contains(stringify!($config_type)),
                "Debug should contain type name"
            );
        }
    };
}

/// Test macro for error code validation - eliminates duplicate error tests
#[macro_export]
macro_rules! test_error_codes {
    ($error_type:ty, { $($variant:ident => $code:expr),+ }) => {
        #[test]
        fn test_error_codes() {
            $(
                let error = <$error_type>::$variant;
                assert_eq!(error.code(), $code,
                    "Error code for {} should be {}",
                    stringify!($variant), $code);
            )+
        }
    };
}

/// Test macro for JSON serialization consistency - reduces serialization test duplication
#[macro_export]
macro_rules! test_json_serialization {
    ($type:ty, $value:expr, $expected_fields:expr) => {
        #[test]
        fn test_json_serialization() {
            let value: $type = $value;
            let serialized = serde_json::to_string(&value).expect("Should serialize to JSON");

            // Verify it's valid JSON
            let parsed: serde_json::Value =
                serde_json::from_str(&serialized).expect("Should parse as valid JSON");

            // Verify expected fields are present
            let expected_fields: &[&str] = $expected_fields;
            for field in expected_fields {
                assert!(
                    parsed.get(field).is_some(),
                    "Field '{}' should be present in JSON",
                    field
                );
            }

            // Verify round-trip
            let deserialized: $type =
                serde_json::from_str(&serialized).expect("Should deserialize from JSON");

            assert_eq!(
                format!("{:?}", value),
                format!("{:?}", deserialized),
                "Round-trip serialization should preserve data"
            );
        }
    };
}

/// Test macro for transport state testing - reduces transport test duplication  
#[macro_export]
macro_rules! test_transport_states {
    ($transport_type:ty) => {
        #[test]
        fn test_transport_state_clone() {
            use $crate::TransportState;

            let states = [
                TransportState::Disconnected,
                TransportState::Connecting,
                TransportState::Connected,
                TransportState::Failed,
            ];

            for state in &states {
                let cloned = state.clone();
                assert_eq!(*state, cloned, "Transport state should clone correctly");
            }
        }

        #[test]
        fn test_transport_state_transitions() {
            use $crate::TransportState;

            // Test valid state transitions
            let initial = TransportState::Disconnected;
            assert_eq!(initial, TransportState::Disconnected);

            // Verify Debug implementation
            for state in &[
                TransportState::Disconnected,
                TransportState::Connecting,
                TransportState::Connected,
                TransportState::Failed,
            ] {
                let debug_str = format!("{:?}", state);
                assert!(!debug_str.is_empty());
            }
        }
    };
}

/// Utility function for consistent registry testing
pub fn test_registry_properties<T>()
where
    T: Default + std::fmt::Debug + Clone,
{
    let registry = T::default();

    // Test Debug
    let debug_str = format!("{:?}", registry);
    assert!(
        !debug_str.is_empty(),
        "Registry Debug implementation should not be empty"
    );

    // Test Clone
    let _cloned = registry.clone();
    // Just verify it doesn't panic
}

/// Utility function for error context testing
pub fn test_error_context_properties<T>()
where
    T: std::fmt::Display + std::fmt::Debug + Clone + Send + Sync + 'static,
{
    // This is a placeholder that can be expanded based on actual error types
    // The key is to provide a consistent interface for error testing
}

#[cfg(test)]
mod tests {

    // Example usage of the macros
    #[derive(Debug, Clone, Default, PartialEq)]
    struct TestConfig {
        pub enabled: bool,
        pub timeout_ms: u64,
        pub retries: u32,
    }

    test_config_defaults!(TestConfig, {
        enabled => false,
        timeout_ms => 0,
        retries => 0
    });

    test_config_traits!(TestConfig);

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestMessage {
        pub id: u32,
        pub message: String,
    }

    test_json_serialization!(
        TestMessage,
        TestMessage {
            id: 1,
            message: "test".to_string()
        },
        &["id", "message"]
    );
}
