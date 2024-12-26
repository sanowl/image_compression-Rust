pub mod lzw;
pub mod deflate;
pub mod utils;

pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String>;
}
