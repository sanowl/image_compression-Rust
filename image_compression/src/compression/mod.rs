// src/compression/mod.rs

use thiserror::Error;

/// Defines the various errors that can occur during compression and decompression.
#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Invalid compression level: {0}")]
    InvalidLevel(String),

    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Unknown compression algorithm: {0}")]
    UnknownAlgorithm(String),

    // Add other compression-related errors as needed
}

/// The `Compressor` trait defines the essential methods for compression algorithms.
pub trait Compressor {
    /// Compresses the input data and returns the compressed byte vector.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice of the data to compress.
    ///
    /// # Returns
    ///
    /// A `Result` containing the compressed data or a `CompressionError` if compression fails.
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;

    /// Decompresses the input data and returns the original byte vector.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice of the data to decompress.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decompressed data or a `CompressionError` if decompression fails.
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
}

pub mod lzw;
pub mod deflate;
pub mod utils;

/// Enum representing the supported compression algorithms.
pub enum CompressionAlgorithmType {
    Deflate(deflate::DeflateCompressor),
    Lzw(lzw::LzwCompressor),
    // Add other algorithms as needed
}

impl CompressionAlgorithmType {
    /// Factory method to create a compressor based on the configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the application's configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the appropriate `CompressionAlgorithmType` or a `CompressionError`.
    pub fn new(config: &crate::config::AppConfig) -> Result<Self, CompressionError> {
        match config.compression_algorithm.as_str() {
            "deflate" => {
                let level = config.compression_level.unwrap_or(6);
                let compressor = deflate::DeflateCompressor::new(level)?;
                Ok(CompressionAlgorithmType::Deflate(compressor))
            },
            "lzw" => {
                let compressor = lzw::LzwCompressor::new();
                Ok(CompressionAlgorithmType::Lzw(compressor))
            },
            other => Err(CompressionError::UnknownAlgorithm(other.to_string())),
        }
    }
}

impl Compressor for CompressionAlgorithmType {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match self {
            CompressionAlgorithmType::Deflate(c) => c.compress(data),
            CompressionAlgorithmType::Lzw(c) => c.compress(data),
            // Handle other algorithms
        }
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match self {
            CompressionAlgorithmType::Deflate(c) => c.decompress(data),
            CompressionAlgorithmType::Lzw(c) => c.decompress(data),
            // Handle other algorithms
        }
    }
}
