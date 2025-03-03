//! Error types used throughout the crate.

use thiserror::Error;

/// Main error type for the dictionary-builder crate
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error parsing TOML
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Error serializing or deserializing JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error with Git operations
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Error fetching source files
    #[error("Source fetch error: {0}")]
    Fetch(String),

    /// Error processing files
    #[error("Processing error: {0}")]
    Processing(String),

    /// Error generating manifest
    #[error("Manifest error: {0}")]
    Manifest(String),

    /// Error validating dictionary definitions
    #[error("Validation error: {0}")]
    Validation(String),

    /// Error calculating checksums
    #[error("Checksum error: {0}")]
    Checksum(String),

    /// Error with network requests
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// General error
    #[error("{0}")]
    General(String),
}
