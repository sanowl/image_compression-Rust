// src/config/mod.rs

use serde::Deserialize;
use std::path::Path;
use config::{Config as ConfigLoader, ConfigError, File};
use crate::compression::CompressionError;
use std::convert::TryInto;
use log::{info, error};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub compression_algorithm: String,
    pub compression_level: Option<u32>,
    // Add other configuration fields as needed
}

impl AppConfig {
    /// Loads configuration from the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path reference to the configuration file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AppConfig` or a `ConfigError`.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut settings = ConfigLoader::default();

        // Convert the path to a string, handling potential errors
        let path_str = path.as_ref()
            .to_str()
            .ok_or_else(|| ConfigError::Message("Invalid path".to_string()))?;

        info!("Loading configuration from {}", path_str);

        // Merge the configuration file into the settings
        settings.merge(File::with_path(path_str)).map_err(|e| {
            error!("Failed to merge config file '{}': {}", path_str, e);
            e
        })?;

        // Attempt to deserialize the settings into `AppConfig`
        settings.try_into::<AppConfig>().map_err(|e| {
            error!("Failed to deserialize config into AppConfig: {}", e);
            e
        })
    }

    /// Creates a `DeflateCompressor` based on the loaded configuration.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DeflateCompressor` or a `CompressionError`.
    pub fn create_compressor(&self) -> Result<crate::compression::deflate::DeflateCompressor, CompressionError> {
        let compression_level = self.compression_level.unwrap_or(6); // Default level 6

        if compression_level > 9 {
            return Err(CompressionError::InvalidLevel(compression_level.to_string()));
        }

        let compression = flate2::Compression::new(compression_level);
        Ok(crate::compression::deflate::DeflateCompressor::with_level(compression))
    }

    /// Validates the configuration fields.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `ConfigError` if validation fails.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.compression_level.unwrap_or(6) > 9 {
            return Err(ConfigError::Message(format!(
                "Compression level {} is invalid. Must be between 0 and 9.",
                self.compression_level.unwrap_or(6)
            )));
        }
        // Add more validation as needed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_from_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.toml");

        // Write a sample configuration file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "compression_algorithm = 'deflate'\ncompression_level = 6").unwrap();

        // Load the configuration
        let config = AppConfig::load_from_file(&file_path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.compression_algorithm, "deflate");
        assert_eq!(config.compression_level, Some(6));

        // Validate the configuration
        assert!(config.validate().is_ok());

        // Clean up
        dir.close().unwrap();
    }
}
