// tests/io_tests.rs

use image_compression::io::reader::read_image;
use image_compression::io::writer::write_image;
use std::fs;

#[test]
fn test_image_read_write() {
    let input_path = "tests/sample.png";
    let output_path = "tests/output.bin";

    // Read the image
    let image = read_image(input_path).expect("Failed to read image");

    // Convert image to raw bytes
    let image_bytes = image.to_rgb8().to_vec();

    // Write the compressed data
    write_image(output_path, &image_bytes).expect("Failed to write compressed image");

    // Read back the written data
    let written_data = fs::read(output_path).expect("Failed to read written file");

    // Assert that written data matches original
    assert_eq!(image_bytes, written_data);

    // Clean up
    fs::remove_file(output_path).expect("Failed to remove output file");
}
