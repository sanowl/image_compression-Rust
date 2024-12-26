// src/compression/lzw.rs

use crate::compression::Compressor;
use std::collections::HashMap;

pub struct LZWCompressor {
    max_table_size: usize,
}

impl LZWCompressor {
    pub fn new(max_table_size: usize) -> Self {
        LZWCompressor { max_table_size }
    }
}

impl Compressor for LZWCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
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
            }
        }

        Ok(result)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut dictionary: Vec<Vec<u8>> = Vec::with_capacity(self.max_table_size);
        for i in 0..=255 {
            dictionary.push(vec![i as u8]);
        }

        let mut result: Vec<u8> = Vec::new();

        let mut iter = data.chunks(2); // Assuming codes are 16-bit
        let first_code = match iter.next() {
            Some(chunk) if chunk.len() == 2 => u16::from_be_bytes([chunk[0], chunk[1]]) as usize,
            _ => return Err("Invalid compressed data".to_string()),
        };

        let mut w = match dictionary.get(first_code) {
            Some(bytes) => bytes.clone(),
            None => return Err("Invalid compressed code".to_string()),
        };
        result.extend(&w);

        for chunk in iter {
            if chunk.len() != 2 {
                return Err("Invalid compressed data".to_string());
            }
            let k = u16::from_be_bytes([chunk[0], chunk[1]]) as usize;
            let entry = if let Some(bytes) = dictionary.get(k) {
                bytes.clone()
            } else if k == dictionary.len() {
                let mut new_entry = w.clone();
                new_entry.push(w[0]);
                new_entry
            } else {
                return Err("Invalid compressed code".to_string());
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
