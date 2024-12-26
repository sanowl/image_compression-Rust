// src/config/mod.rs

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub compression_algorithm: String,
    pub compression_level: Option<u32>,
    // Add other configuration fields as needed
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(path.as_ref().to_str().unwrap()))?;
        settings.try_into()
    }
}
