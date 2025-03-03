//! Library for building and managing spell-checking dictionaries.
//!
//! This crate provides functionality for processing dictionary definitions,
//! retrieving source files, processing them, and generating a manifest.

pub mod builder;
pub mod checksum;
pub mod error;
pub mod git;
pub mod manifest;
pub mod processing;
pub mod source;
pub mod types;

// Re-export commonly used types and functions
pub use builder::{BuilderConfig, DictionaryBuilder};
pub use error::Error;
pub use manifest::{create_manifest, save_manifest};
pub use types::Manifest;

/// Result type used throughout the crate
pub type Result<T> = std::result::Result<T, error::Error>;
