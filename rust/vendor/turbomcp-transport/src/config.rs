//! Transport configuration utilities.

use std::collections::HashMap;
use std::time::Duration;

use crate::core::{TransportConfig, TransportError, TransportResult, TransportType};

/// Builder for transport configurations
#[derive(Debug, Clone)]
pub struct TransportConfigBuilder {
    transport_type: TransportType,
    connect_timeout: Duration,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    keep_alive: Option<Duration>,
    max_connections: Option<usize>,
    compression: bool,
    compression_algorithm: Option<String>,
    custom: HashMap<String, serde_json::Value>,
}

impl TransportConfigBuilder {
    /// Create a new config builder
    #[must_use]
    pub fn new(transport_type: TransportType) -> Self {
        Self {
            transport_type,
            connect_timeout: Duration::from_secs(30),
            read_timeout: None,
            write_timeout: None,
            keep_alive: None,
            max_connections: None,
            compression: false,
            compression_algorithm: None,
            custom: HashMap::new(),
        }
    }

    /// Set connection timeout
    #[must_use]
    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set read timeout
    #[must_use]
    pub const fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    /// Set write timeout
    #[must_use]
    pub const fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = Some(timeout);
        self
    }

    /// Set keep-alive interval
    #[must_use]
    pub const fn keep_alive(mut self, interval: Duration) -> Self {
        self.keep_alive = Some(interval);
        self
    }

    /// Set maximum connections
    #[must_use]
    pub const fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Enable compression
    #[must_use]
    pub const fn enable_compression(mut self) -> Self {
        self.compression = true;
        self
    }

    /// Set compression algorithm
    pub fn compression_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.compression_algorithm = Some(algorithm.into());
        self
    }

    /// Add custom configuration
    pub fn custom(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> TransportResult<TransportConfig> {
        // Validate configuration
        if self.connect_timeout < Duration::from_millis(100) {
            return Err(TransportError::ConfigurationError(
                "Connect timeout must be at least 100ms".to_string(),
            ));
        }

        if let Some(max_connections) = self.max_connections
            && max_connections == 0
        {
            return Err(TransportError::ConfigurationError(
                "Max connections must be greater than 0".to_string(),
            ));
        }

        Ok(TransportConfig {
            transport_type: self.transport_type,
            connect_timeout: self.connect_timeout,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            keep_alive: self.keep_alive,
            max_connections: self.max_connections,
            compression: self.compression,
            compression_algorithm: self.compression_algorithm,
            custom: self.custom,
        })
    }
}

/// Predefined transport configurations
#[derive(Debug)]
pub struct Configs;

impl Configs {
    /// Default stdio configuration
    #[must_use]
    pub fn stdio() -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Stdio)
            .build()
            .expect("Default stdio config should be valid")
    }

    /// Fast stdio configuration (shorter timeouts)
    #[must_use]
    pub fn stdio_fast() -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Stdio)
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("Fast stdio config should be valid")
    }

    /// Default HTTP configuration
    #[cfg(feature = "http")]
    #[must_use]
    pub fn http(port: u16) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Http)
            .custom("port", port)
            .build()
            .expect("Default HTTP config should be valid")
    }

    /// HTTP configuration with TLS
    #[cfg(all(feature = "http", feature = "tls"))]
    #[must_use]
    pub fn https(port: u16) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Http)
            .custom("port", port)
            .custom("tls", true)
            .build()
            .expect("HTTPS config should be valid")
    }

    /// Default WebSocket configuration
    #[cfg(feature = "websocket")]
    pub fn websocket(url: impl Into<String>) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::WebSocket)
            .custom("url", url.into())
            .build()
            .expect("Default WebSocket config should be valid")
    }

    /// WebSocket configuration with compression
    #[cfg(all(feature = "websocket", feature = "compression"))]
    pub fn websocket_compressed(url: impl Into<String>) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::WebSocket)
            .enable_compression()
            .custom("url", url.into())
            .build()
            .expect("Compressed WebSocket config should be valid")
    }

    /// Default TCP configuration
    #[cfg(feature = "tcp")]
    pub fn tcp(host: impl Into<String>, port: u16) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Tcp)
            .custom("host", host.into())
            .custom("port", port)
            .build()
            .expect("Default TCP config should be valid")
    }

    /// Default Unix socket configuration
    #[cfg(feature = "unix")]
    pub fn unix(path: impl Into<String>) -> TransportConfig {
        TransportConfigBuilder::new(TransportType::Unix)
            .custom("path", path.into())
            .build()
            .expect("Default Unix config should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = TransportConfigBuilder::new(TransportType::Stdio)
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(5))
            .enable_compression()
            .custom("test", "value")
            .build()
            .unwrap();

        assert_eq!(config.transport_type, TransportType::Stdio);
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert_eq!(config.read_timeout, Some(Duration::from_secs(5)));
        assert!(config.compression);
        assert_eq!(
            config.custom.get("test"),
            Some(&serde_json::Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_config_validation() {
        // Invalid timeout
        let result = TransportConfigBuilder::new(TransportType::Stdio)
            .connect_timeout(Duration::from_millis(50))
            .build();
        assert!(result.is_err());

        // Invalid max connections
        let result = TransportConfigBuilder::new(TransportType::Stdio)
            .max_connections(0)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_predefined_configs() {
        let stdio_config = Configs::stdio();
        assert_eq!(stdio_config.transport_type, TransportType::Stdio);

        let fast_stdio_config = Configs::stdio_fast();
        assert_eq!(fast_stdio_config.connect_timeout, Duration::from_secs(5));
    }
}
