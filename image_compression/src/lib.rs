// src/lib.rs

pub mod compression;
pub mod io;
pub mod config;

// Re-exporting for easier access
pub use compression::*;
pub use io::*;
pub use config::*;
