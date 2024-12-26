// src/compression/deflate.rs

//! Module implementing the Deflate compression algorithm.
//!
//! This module provides a `DeflateCompressor` struct that allows for lossless
//! compression and decompression of data using the Deflate algorithm.
//!
//! # Examples
//!
//! ```rust
//! use image_compression::compression::deflate::DeflateCompressor;
//!
//! let compressor = DeflateCompressor::new();
//! let data = b"Example data to compress";
//! let compressed = compressor.compress(data).unwrap();
//! let decompressed = compressor.decompress(&compressed).unwrap();
//! assert_eq!(data.to_vec(), decompressed);
//! ```

use crate::compression::Compressor;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression as Flate2Compression};
use std::fmt;
use std::io::{self, Read, Write};
use thiserror::Error;

/// Custom error type for compression and decompression operations.
#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("I/O error during compression/decompression: {0}")]
    Io(#[from] io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Decompression error: {0}")]
    Decompression(String),
}

/// Struct representing a Deflate compressor with configurable compression levels.
pub struct DeflateCompressor {
    level: Flate2Compression,
    level_number: u32,
}

impl DeflateCompressor {
    /// Creates a new `DeflateCompressor` with the default compression level.
    ///
    /// The default compression level corresponds to `Flate2Compression::default()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::new();
    /// ```
    pub fn new() -> Self {
        DeflateCompressor {
            level: Flate2Compression::default(),
            level_number: 6, // Default compression level
        }
    }

    /// Creates a new `DeflateCompressor` with a specified compression level.
    ///
    /// # Arguments
    ///
    /// * `level` - The compression level to use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use flate2::Compression;
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::with_level(Compression::new(9));
    /// ```
    pub fn with_level(level: Flate2Compression) -> Self {
        let level_number = match level {
            Flate2Compression::Fastest => 1,
            Flate2Compression::Default => 6,
            Flate2Compression::Best => 9,
            Flate2Compression::new(n) => n,
        };
        DeflateCompressor {
            level,
            level_number,
        }
    }
}

impl Compressor for DeflateCompressor {
    /// Compresses the given data using the Deflate algorithm.
    ///
    /// This method compresses the entire data and returns a `Vec<u8>` containing
    /// the compressed bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to compress.
    ///
    /// # Returns
    ///
    /// A `Result` containing the compressed data or a `CompressionError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::new();
    /// let data = b"Example data to compress";
    /// let compressed = compressor.compress(data).unwrap();
    /// ```
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = DeflateEncoder::new(Vec::new(), self.level);
        encoder.write_all(data)?;
        encoder.finish().map_err(|e| CompressionError::Compression(e.to_string()))
    }

    /// Decompresses the given data using the Deflate algorithm.
    ///
    /// This method decompresses the entire data and returns a `Vec<u8>` containing
    /// the original uncompressed bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to decompress.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decompressed data or a `CompressionError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::new();
    /// let data = b"Example data to compress";
    /// let compressed = compressor.compress(data).unwrap();
    /// let decompressed = compressor.decompress(&compressed).unwrap();
    /// assert_eq!(data.to_vec(), decompressed);
    /// ```
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}

/// Implement `fmt::Display` for `DeflateCompressor` for better readability.
impl fmt::Display for DeflateCompressor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DeflateCompressor (Compression Level: {})",
            self.level_number
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;

    #[test]
    fn test_deflate_compressor_default_level() {
        let compressor = DeflateCompressor::new();
        let data = b"Test data for default compression level.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_deflate_compressor_custom_level() {
        let compressor = DeflateCompressor::with_level(Compression::new(9));
        let data = b"Test data for custom compression level.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_deflate_compressor_empty_data() {
        let compressor = DeflateCompressor::new();
        let data: &[u8] = b"";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_deflate_compressor_invalid_decompress() {
        let compressor = DeflateCompressor::new();
        let invalid_data = b"Invalid compressed data";
        let result = compressor.decompress(invalid_data);
        assert!(result.is_err());
    }
}
