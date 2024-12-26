// src/io/reader.rs

use image::DynamicImage;
use image::io::Reader as ImageReader;
use std::path::Path;

pub fn read_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, image::ImageError> {
    ImageReader::open(path)?.decode()
}
