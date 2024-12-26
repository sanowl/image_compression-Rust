// src/io/writer.rs

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_image<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}
