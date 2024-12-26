// tests/compression_tests.rs

use image_compression::compression::deflate::DeflateCompressor;
use image_compression::io::reader::read_image;
use image_compression::io::writer::write_image;

#[test]
fn test_deflate_compression_decompression() {
    let compressor = DeflateCompressor::new();
    
    // Read a sample image (ensure "tests/sample.png" exists)
    let image = read_image("tests/sample.png").expect("Failed to read sample image");
    let image_bytes = image.to_rgb8().to_vec();

    // Compress the image
    let compressed = compressor.compress(&image_bytes).expect("Compression failed");

    // Decompress the image
    let decompressed = compressor.decompress(&compressed).expect("Decompression failed");

    // Assert that decompressed data matches original
    assert_eq!(image_bytes, decompressed);
}
