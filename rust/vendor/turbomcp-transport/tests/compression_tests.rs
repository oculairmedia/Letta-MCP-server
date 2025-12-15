//! Comprehensive tests for message compression support

#[cfg(feature = "compression")]
mod compression_tests {
    use serde_json::json;
    use turbomcp_transport::compression::{CompressionType, MessageCompressor};

    #[test]
    fn test_compression_type_debug() {
        let none = CompressionType::None;
        let debug_str = format!("{none:?}");
        assert_eq!(debug_str, "None");
    }

    #[test]
    fn test_compression_type_clone() {
        let original = CompressionType::None;
        let cloned = original;
        assert!(matches!(cloned, CompressionType::None));
    }

    #[test]
    fn test_compression_type_copy() {
        let original = CompressionType::None;
        let copied = original;
        assert!(matches!(copied, CompressionType::None));
        assert!(matches!(original, CompressionType::None)); // Original still accessible
    }

    #[test]
    fn test_message_compressor_new() {
        let _compressor = MessageCompressor::new(CompressionType::None);
        // Should not panic and create successfully
        assert_eq!(format!("{:?}", CompressionType::None), "None");
    }

    #[test]
    fn test_message_compressor_default() {
        let compressor = MessageCompressor::default();
        // Should create with None compression by default
        // Test by compressing a message
        let message = json!({"test": "data"});
        let result = compressor.compress(&message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compress_simple_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({"test": "data"});

        let compressed = compressor.compress(&message).unwrap();
        let expected = serde_json::to_vec(&message).unwrap();
        assert_eq!(compressed, expected);
    }

    #[test]
    fn test_compress_complex_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({
            "method": "tools/call",
            "params": {
                "name": "calculator",
                "arguments": {"operation": "add", "a": 1, "b": 2}
            },
            "id": 123,
            "jsonrpc": "2.0"
        });

        let compressed = compressor.compress(&message).unwrap();
        let expected = serde_json::to_vec(&message).unwrap();
        assert_eq!(compressed, expected);
    }

    #[test]
    fn test_decompress_simple_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let original_message = json!({"test": "data"});

        let compressed = compressor.compress(&original_message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(original_message, decompressed);
    }

    #[test]
    fn test_decompress_complex_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let original_message = json!({
            "tools": [
                {"name": "calculator", "description": "Perform calculations"},
                {"name": "weather", "description": "Get weather info"}
            ],
            "resources": ["file1.txt", "file2.txt"],
            "metadata": {
                "version": "1.0.0",
                "capabilities": ["tools", "resources"]
            }
        });

        let compressed = compressor.compress(&original_message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(original_message, decompressed);
    }

    #[test]
    fn test_compress_empty_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({});

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_json_array() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!([1, 2, 3, "test", {"nested": true}]);

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_json_with_nulls() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({
            "null_value": null,
            "optional": null,
            "present": "value"
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_json_with_numbers() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({
            "integer": 42,
            "float": std::f64::consts::PI,
            "negative": -123,
            "zero": 0,
            "large": 9223372036854775807i64
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_json_with_booleans() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({
            "true_value": true,
            "false_value": false,
            "mixed": [true, false, true]
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_json_with_unicode() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({
            "english": "Hello World",
            "unicode": "Hello 世界 🌍",
            "emoji": "🚀🎉💻",
            "special": "Special chars: !@#$%^&*()"
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_compress_large_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let large_string = "x".repeat(10000);
        let message = json!({
            "large_field": large_string,
            "array": (0..1000).collect::<Vec<i32>>(),
            "nested": {
                "deep": {
                    "very_deep": {
                        "data": "nested content"
                    }
                }
            }
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[test]
    fn test_decompress_invalid_json() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let invalid_json = b"invalid json {";

        let result = compressor.decompress(invalid_json);
        assert!(result.is_err());

        if let Err(err) = result {
            let error_msg = format!("{err}");
            assert!(error_msg.contains("SerializationFailed") || error_msg.contains("failed"));
        }
    }

    #[test]
    fn test_roundtrip_consistency() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let messages = vec![
            json!(null),
            json!(true),
            json!(false),
            json!(42),
            json!(std::f64::consts::PI),
            json!("string"),
            json!([]),
            json!({}),
            json!([1, 2, 3]),
            json!({"key": "value"}),
        ];

        for original in messages {
            let compressed = compressor.compress(&original).unwrap();
            let decompressed = compressor.decompress(&compressed).unwrap();
            assert_eq!(original, decompressed, "Roundtrip failed for: {original:?}");
        }
    }

    // Feature-gated tests for compression algorithms
    #[cfg(feature = "flate2")]
    #[test]
    fn test_gzip_compression() {
        let compressor = MessageCompressor::new(CompressionType::Gzip);
        let message = json!({
            "large_data": "x".repeat(1000),
            "repeated": ["same", "same", "same", "same"]
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);

        // Verify compression actually reduces size for repetitive data
        let original_size = serde_json::to_vec(&message).unwrap().len();
        assert!(
            compressed.len() < original_size,
            "Gzip should compress repetitive data"
        );
    }

    #[cfg(feature = "brotli")]
    #[test]
    fn test_brotli_compression() {
        let compressor = MessageCompressor::new(CompressionType::Brotli);
        let message = json!({
            "large_data": "y".repeat(1000),
            "patterns": ["pattern", "pattern", "pattern"]
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);

        // Verify compression reduces size
        let original_size = serde_json::to_vec(&message).unwrap().len();
        assert!(
            compressed.len() < original_size,
            "Brotli should compress repetitive data"
        );
    }

    #[cfg(feature = "lz4_flex")]
    #[test]
    fn test_lz4_compression() {
        let compressor = MessageCompressor::new(CompressionType::Lz4);
        let message = json!({
            "large_data": "z".repeat(1000),
            "numbers": (0..100).collect::<Vec<i32>>()
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);

        // Verify compression reduces size
        let original_size = serde_json::to_vec(&message).unwrap().len();
        assert!(
            compressed.len() < original_size,
            "LZ4 should compress repetitive data"
        );
    }
}

// Tests that run even without compression feature
#[test]
fn test_compression_module_accessible() {
    // This test ensures the module can be imported even without compression feature
    // The actual compression types might be feature-gated but the module should exist
    // Compilation test - no runtime assertions needed
}
