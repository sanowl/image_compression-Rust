// src/compression/error.rs

//! Module defining errors related to compression operations.

use std::fmt;

/// Enum representing possible compression errors.
#[derive(Debug)]
pub enum CompressionError {
    /// Represents errors that occur during compression.
    Compression(String),
    /// Represents errors that occur during decompression.
    Decompression(String),
    /// Represents an invalid compression level error.
    InvalidLevel(String),
    /// Represents configuration-related errors.
    Configuration(String),
    // Add other variants as needed.
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::Compression(msg) => write!(f, "Compression error: {}", msg),
            CompressionError::Decompression(msg) => write!(f, "Decompression error: {}", msg),
            CompressionError::InvalidLevel(level) => write!(f, "Invalid compression level: {}", level),
            CompressionError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            // Handle other variants accordingly.
        }
    }
}

impl std::error::Error for CompressionError {}
