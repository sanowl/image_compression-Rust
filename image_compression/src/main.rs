// src/main.rs

use clap::{Arg, Command};
use image_compression::compression::deflate::DeflateCompressor;
use image_compression::io::reader::read_image;
use image_compression::io::writer::write_image;

fn main() {
    let matches = Command::new("Image Compression Tool")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Compresses images losslessly")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .takes_value(true)
            .required(true)
            .help("Input image file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .takes_value(true)
            .required(true)
            .help("Output compressed file"))
        .get_matches();

    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    // Read the image
    let image = read_image(input_path).expect("Failed to read image");

    // Convert image to raw bytes (assuming RGB)
    let image_bytes = image.to_rgb8().to_vec();

    // Compress the image
    let compressor = DeflateCompressor::new();
    let compressed_data = compressor.compress(&image_bytes).expect("Compression failed");

    // Write the compressed data
    write_image(output_path, &compressed_data).expect("Failed to write compressed image");

    println!("Image compressed successfully!");
}
