//! Core data structures used throughout the crate.

use serde::{Deserialize, Serialize};

/// Top-level manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// List of all dictionaries
    pub dictionaries: Vec<DictionaryInfo>,
    /// Timestamp when the manifest was generated
    pub generated_at: String,
    /// Git commit hash of the repository when the manifest was generated
    pub git_hash: String,
    /// URL of the repository
    pub repo_url: String,
}

/// Information about a single dictionary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryInfo {
    /// Unique identifier for the dictionary
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of the dictionary
    pub description: String,
    /// Optional list of language IDs this dictionary applies to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_ids: Option<Vec<String>>,
    /// Optional list of file extensions this dictionary applies to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extensions: Option<Vec<String>>,
    /// List of files included in this dictionary
    pub files: Vec<DictionaryFile>,
}

/// Information about a single file in a dictionary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DictionaryFile {
    /// A simple word list file
    #[serde(rename = "wordlist")]
    Wordlist {
        /// Path to the word list file
        path: String,
        /// SHA-256 checksum of the file
        checksum: String,
    },
    /// A Hunspell dictionary (pair of .dic and .aff files)
    #[serde(rename = "hunspell")]
    Hunspell {
        /// Path to the .dic file
        dic_path: String,
        /// Path to the .aff file
        aff_path: String,
        /// SHA-256 checksum of the .dic file
        dic_checksum: String,
        /// SHA-256 checksum of the .aff file
        aff_checksum: String,
    },
}

/// Dictionary definition from TOML
#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryDefinition {
    /// Dictionary metadata
    pub dictionary: DictionaryMetadata,
    /// Scope information
    #[serde(default)]
    pub scope: ScopeConfig,
    /// Source file configurations
    pub sources: SourcesConfig,
}

/// Dictionary metadata
#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryMetadata {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
}

/// Scope configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ScopeConfig {
    /// Language IDs this dictionary applies to
    #[serde(default)]
    pub language_ids: Vec<String>,
    /// File extensions this dictionary applies to
    #[serde(default)]
    pub file_extensions: Vec<String>,
}

/// Source file configurations
#[derive(Debug, Clone, Deserialize)]
pub struct SourcesConfig {
    /// Text source files
    #[serde(default)]
    pub text_sources: Vec<TextSource>,
    /// Hunspell dictionaries
    #[serde(default)]
    pub hunspell_sources: Vec<HunspellSource>,
}

/// Text source file configuration
#[derive(Debug, Clone, Deserialize)]
pub struct TextSource {
    /// Repository URL
    pub repository: String,
    /// Path within the repository
    pub path: String,
}

/// Hunspell dictionary source configuration
#[derive(Debug, Clone, Deserialize)]
pub struct HunspellSource {
    /// Repository URL
    pub repository: String,
    /// Path to the .aff file within the repository
    pub aff_path: String,
    /// Path to the .dic file within the repository
    pub dic_path: String,
}
