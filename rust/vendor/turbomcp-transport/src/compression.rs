//! Message compression support

use std::io::{Read, Write};

use crate::core::{TransportError, TransportResult};
use serde_json::Value;

/// Compression algorithm
#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    /// No compression
    None,
    /// Gzip compression
    #[cfg(feature = "flate2")]
    Gzip,
    /// Brotli compression  
    #[cfg(feature = "brotli")]
    Brotli,
    /// LZ4 compression
    #[cfg(feature = "lz4_flex")]
    Lz4,
}

/// Message compressor/decompressor
#[derive(Debug)]
pub struct MessageCompressor {
    compression_type: CompressionType,
}

impl MessageCompressor {
    /// Create a new message compressor
    #[must_use]
    pub const fn new(compression_type: CompressionType) -> Self {
        Self { compression_type }
    }

    /// Compress a JSON message
    pub fn compress(&self, message: &Value) -> TransportResult<Vec<u8>> {
        let json_bytes = serde_json::to_vec(message)
            .map_err(|e| TransportError::SerializationFailed(e.to_string()))?;

        match self.compression_type {
            CompressionType::None => Ok(json_bytes),

            #[cfg(feature = "flate2")]
            CompressionType::Gzip => {
                use flate2::{Compression, write::GzEncoder};

                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(&json_bytes)
                    .map_err(|e| TransportError::Internal(e.to_string()))?;
                encoder
                    .finish()
                    .map_err(|e| TransportError::Internal(e.to_string()))
            }

            #[cfg(feature = "brotli")]
            CompressionType::Brotli => {
                use brotli::enc::BrotliEncoderParams;

                let params = BrotliEncoderParams::default();
                let mut compressed = Vec::new();
                brotli::BrotliCompress(&mut json_bytes.as_slice(), &mut compressed, &params)
                    .map_err(|e| {
                        TransportError::Internal(format!("Brotli compression failed: {e}"))
                    })?;
                Ok(compressed)
            }

            #[cfg(feature = "lz4_flex")]
            CompressionType::Lz4 => {
                use lz4_flex::compress_prepend_size;
                Ok(compress_prepend_size(&json_bytes))
            }
        }
    }

    /// Decompress a message back to JSON
    pub fn decompress(&self, compressed: &[u8]) -> TransportResult<Value> {
        let json_bytes = match self.compression_type {
            CompressionType::None => compressed.to_vec(),

            #[cfg(feature = "flate2")]
            CompressionType::Gzip => {
                use flate2::read::GzDecoder;

                let mut decoder = GzDecoder::new(compressed);
                let mut decompressed = Vec::new();
                decoder
                    .read_to_end(&mut decompressed)
                    .map_err(|e| TransportError::Internal(e.to_string()))?;
                decompressed
            }

            #[cfg(feature = "brotli")]
            CompressionType::Brotli => {
                let mut decompressed = Vec::new();
                brotli::BrotliDecompress(&mut &compressed[..], &mut decompressed).map_err(|e| {
                    TransportError::Internal(format!("Brotli decompression failed: {e}"))
                })?;
                decompressed
            }

            #[cfg(feature = "lz4_flex")]
            CompressionType::Lz4 => {
                use lz4_flex::decompress_size_prepended;
                decompress_size_prepended(compressed)
                    .map_err(|e| TransportError::Internal(e.to_string()))?
            }
        };

        serde_json::from_slice(&json_bytes)
            .map_err(|e| TransportError::SerializationFailed(e.to_string()))
    }
}

impl Default for MessageCompressor {
    fn default() -> Self {
        Self::new(CompressionType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_no_compression() {
        let compressor = MessageCompressor::new(CompressionType::None);
        let message = json!({"test": "data", "number": 42});

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);
    }

    #[cfg(feature = "lz4_flex")]
    #[test]
    fn test_lz4_compression() {
        let compressor = MessageCompressor::new(CompressionType::Lz4);
        let message = json!({
            "large_data": "x".repeat(1000),
            "numbers": (0..100).collect::<Vec<i32>>()
        });

        let compressed = compressor.compress(&message).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(message, decompressed);

        // Verify compression actually reduces size for large data
        let original_size = serde_json::to_vec(&message).unwrap().len();
        assert!(compressed.len() < original_size);
    }
}
