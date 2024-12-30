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

use super::{Compressor, CompressionError};
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression as Flate2Compression};
use std::fmt;
use std::io::{Read, Write};

/// Struct representing a Deflate compressor with configurable compression levels.
#[derive(Debug, Clone)]
pub struct DeflateCompressor {
    level: Flate2Compression,
    level_number: u32,
}

impl DeflateCompressor {
    /// Creates a new `DeflateCompressor` with the default compression level.
    ///
    /// The default compression level corresponds to `Flate2Compression::fast()` (level 1).
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::new();
    /// ```
    pub fn new() -> Self {
        let default_level = Flate2Compression::fast();
        DeflateCompressor {
            level: default_level.clone(),
            level_number: default_level.level(),
        }
    }

    /// Creates a new `DeflateCompressor` with a specified compression level.
    ///
    /// # Arguments
    ///
    /// * `level` - The compression level to use (`Flate2Compression::fast()`, `Flate2Compression::best()`, or `Flate2Compression::new(u32)`).
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
        DeflateCompressor {
            level: level.clone(),
            level_number: level.level(),
        }
    }

    /// Creates a new `DeflateCompressor` using predefined level names.
    ///
    /// Supported levels:
    /// - "fastest" -> `Flate2Compression::fast()` (level 1)
    /// - "default" -> `Flate2Compression::new(6)` (level 6)
    /// - "best" -> `Flate2Compression::best()` (level 9)
    ///
    /// # Arguments
    ///
    /// * `level` - A string slice representing the desired compression level ("fastest", "default", "best").
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::with_predefined_level("best").unwrap();
    /// ```
    pub fn with_predefined_level(level: &str) -> Result<Self, CompressionError> {
        let (compression, level_number) = match level.to_lowercase().as_str() {
            "fastest" => (Flate2Compression::fast(), 1),
            "default" => (Flate2Compression::new(6), 6),
            "best" => (Flate2Compression::best(), 9),
            _ => return Err(CompressionError::InvalidLevel(level.to_string())),
        };
        Ok(DeflateCompressor {
            level: compression,
            level_number,
        })
    }

    /// Creates a new `DeflateCompressor` with a specified compression level number.
    ///
    /// # Arguments
    ///
    /// * `level` - The compression level number to use (0-9).
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DeflateCompressor` or a `CompressionError` if the level is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::deflate::DeflateCompressor;
    ///
    /// let compressor = DeflateCompressor::with_level_number(7).unwrap();
    /// ```
    pub fn with_level_number(level: u32) -> Result<Self, CompressionError> {
        if level > 9 {
            return Err(CompressionError::InvalidLevel(format!("{}", level)));
        }
        Ok(DeflateCompressor {
            level: Flate2Compression::new(level),
            level_number: level,
        })
    }

    /// Retrieves the compression level number.
    ///
    /// # Returns
    ///
    /// The compression level as a `u32`.
    pub fn get_level(&self) -> u32 {
        self.level_number
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
        encoder.write_all(data).map_err(|e| CompressionError::Compression(e.to_string()))?;
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
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::Decompression(e.to_string()))?;
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
        assert_eq!(compressor.get_level(), 1);
    }

    #[test]
    fn test_deflate_compressor_custom_level() {
        let compressor = DeflateCompressor::with_level_number(7).unwrap();
        let data = b"Test data for custom compression level.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
        assert_eq!(compressor.get_level(), 7);
    }

    #[test]
    fn test_deflate_compressor_predefined_level() {
        let compressor = DeflateCompressor::with_predefined_level("best").unwrap();
        let data = b"Test data for predefined compression level.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
        assert_eq!(compressor.get_level(), 9);
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

    #[test]
    fn test_deflate_compressor_invalid_level() {
        let result = DeflateCompressor::with_predefined_level("superfast");
        assert!(result.is_err());
        if let Err(CompressionError::InvalidLevel(level)) = result {
            assert_eq!(level, "superfast");
        } else {
            panic!("Expected InvalidLevel error");
        }
    }
}
