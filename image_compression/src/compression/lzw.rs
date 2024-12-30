// src/compression/lzw.rs

use super::{Compressor, CompressionError};
use std::collections::HashMap;
use std::fmt;

/// A compressor that uses the LZW algorithm.
#[derive(Debug, Clone)]
pub struct LzwCompressor {
    max_table_size: usize,
}



impl LzwCompressor {
    /// Creates a new `LzwCompressor` with a specified maximum table size.
    ///
    /// # Arguments
    ///
    /// * `max_table_size` - The maximum number of entries in the compression table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use image_compression::compression::lzw::LzwCompressor;
    ///
    /// let compressor = LzwCompressor::new(4096); // Example max table size
    /// ```
    pub fn new(max_table_size: usize) -> Self {
        LzwCompressor { max_table_size }
    }
}

impl Compressor for LzwCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut dictionary: HashMap<Vec<u8>, usize> = HashMap::new();
        for i in 0..=255 {
            dictionary.insert(vec![i as u8], i);
        }

        let mut w: Vec<u8> = Vec::new();
        let mut result: Vec<u8> = Vec::new();
        let mut next_code = 256;

        for &k in data {
            let mut wk = w.clone();
            wk.push(k);
            if dictionary.contains_key(&wk) {
                w = wk;
            } else {
                if let Some(&code) = dictionary.get(&w) {
                    result.extend(&code.to_be_bytes());
                } else {
                    return Err(CompressionError::Compression("Failed to retrieve code from dictionary".to_string()));
                }
                if next_code < self.max_table_size {
                    dictionary.insert(wk, next_code);
                    next_code += 1;
                }
                w = vec![k];
            }
        }

        if !w.is_empty() {
            if let Some(&code) = dictionary.get(&w) {
                result.extend(&code.to_be_bytes());
            } else {
                return Err(CompressionError::Compression("Failed to retrieve final code from dictionary".to_string()));
            }
        }

        Ok(result)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        let mut dictionary: Vec<Vec<u8>> = Vec::with_capacity(self.max_table_size);
        for i in 0..=255 {
            dictionary.push(vec![i as u8]);
        }

        let mut result: Vec<u8> = Vec::new();

        let mut iter = data.chunks(2); // Assuming codes are 16-bit
        let first_code = match iter.next() {
            Some(chunk) if chunk.len() == 2 => u16::from_be_bytes([chunk[0], chunk[1]]) as usize,
            _ => return Err(CompressionError::Decompression("Invalid compressed data".to_string())),
        };

        let mut w = match dictionary.get(first_code) {
            Some(bytes) => bytes.clone(),
            None => return Err(CompressionError::Decompression("Invalid compressed code".to_string())),
        };
        result.extend(&w);

        for chunk in iter {
            if chunk.len() != 2 {
                return Err(CompressionError::Decompression("Invalid compressed data".to_string()));
            }
            let k = u16::from_be_bytes([chunk[0], chunk[1]]) as usize;
            let entry = if let Some(bytes) = dictionary.get(k) {
                bytes.clone()
            } else if k == dictionary.len() {
                let mut new_entry = w.clone();
                new_entry.push(w[0]);
                new_entry
            } else {
                return Err(CompressionError::Decompression("Invalid compressed code".to_string()));
            };
            result.extend(&entry);
            if dictionary.len() < self.max_table_size {
                let mut new_entry = w.clone();
                new_entry.push(entry[0]);
                dictionary.push(new_entry);
            }
            w = entry;
        }

        Ok(result)
    }
}

/// Implement `fmt::Display` for `LzwCompressor` for better readability.
impl fmt::Display for LzwCompressor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LzwCompressor (Max Table Size: {})", self.max_table_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lzw_compressor_compression_decompression() {
        let compressor = LzwCompressor::new(4096);
        let data = b"Test data for LZW compression.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_lzw_compressor_empty_data() {
        let compressor = LzwCompressor::new(4096);
        let data: &[u8] = b"";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_lzw_compressor_invalid_decompress() {
        let compressor = LzwCompressor::new(4096);
        let invalid_data = b"Invalid compressed data";
        let result = compressor.decompress(invalid_data);
        assert!(result.is_err());
        if let Err(CompressionError::Decompression(msg)) = result {
            assert_eq!(msg, "Invalid compressed data".to_string());
        } else {
            panic!("Expected CompressionError::Decompression");
        }
    }
}
