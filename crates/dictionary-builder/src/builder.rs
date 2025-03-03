//! Dictionary building functionality.

use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::error::Error;
use crate::git;
use crate::manifest;
use crate::processing;
use crate::source::SourceFetcher;
use crate::types::{DictionaryDefinition, Manifest};

use glob::glob;
use tracing::{debug, info};

/// Configuration for the dictionary builder
#[derive(Debug, Clone)]
pub struct BuilderConfig {
    /// Directory containing dictionary definitions
    pub dictionaries_dir: PathBuf,
    /// Path where the manifest should be saved
    pub manifest_output_path: PathBuf,
    /// Directory for caching downloaded files
    pub cache_dir: Option<PathBuf>,
    /// Enable verbose output
    pub verbose: bool,
    /// Repository URL to include in the manifest
    pub repo_url: String,
}

/// Builder for dictionaries
pub struct DictionaryBuilder {
    pub config: BuilderConfig,
    source_fetcher: SourceFetcher,
}

impl DictionaryBuilder {
    /// Create a new dictionary builder
    pub fn new(config: BuilderConfig) -> Self {
        let source_fetcher = SourceFetcher::new(config.cache_dir.clone());
        Self {
            config,
            source_fetcher,
        }
    }

    /// Build all dictionaries
    pub fn build_all(&self) -> Result<()> {
        let definitions = self.load_dictionary_definitions()?;

        for def in &definitions {
            self.build_dictionary(def)?;
        }
        info!("Successfully built {} dictionaries", definitions.len());
        Ok(())
    }

    /// Update only dictionaries that have changed
    pub fn update_changed(&self) -> Result<()> {
        // For now, just build all dictionaries
        // In a real implementation, we would check which ones have changed
        self.build_all()
    }

    /// Only validate dictionary definitions without building
    pub fn validate_definitions(&self) -> Result<()> {
        let definitions = self.load_dictionary_definitions()?;

        for def in &definitions {
            self.validate_dictionary(def)?;
        }

        info!("All {} dictionary definitions are valid", definitions.len());
        Ok(())
    }

    /// Generate the manifest without building dictionaries
    pub fn generate_manifest(&self) -> Result<Manifest> {
        // If getting current commit hash fails, return "none"
        let git_hash = git::get_current_commit_hash(&self.config.dictionaries_dir)
            .unwrap_or("none".to_string());
        let manifest = manifest::create_manifest(
            &self.config.dictionaries_dir,
            &self.config.repo_url,
            &git_hash,
        )?;

        manifest::save_manifest(&manifest, &self.config.manifest_output_path)?;

        info!(
            "Generated manifest with {} dictionaries",
            manifest.dictionaries.len()
        );
        Ok(manifest)
    }

    // Private helper methods

    fn load_dictionary_definitions(&self) -> Result<Vec<DictionaryDefinition>> {
        let pattern = self.config.dictionaries_dir.join("*/dictionary.toml");
        let pattern_str = pattern.to_string_lossy();

        let mut definitions = Vec::new();

        for entry in glob(&pattern_str).map_err(|e| Error::General(e.to_string()))? {
            let path = entry.map_err(|e| Error::General(e.to_string()))?;
            debug!("Loading dictionary definition from {}", path.display());

            let content = fs::read_to_string(&path)?;
            let definition: DictionaryDefinition = toml::from_str(&content)?;

            definitions.push(definition);
        }

        Ok(definitions)
    }

    fn build_dictionary(&self, definition: &DictionaryDefinition) -> Result<()> {
        let dict_dir = self.config.dictionaries_dir.join(&definition.dictionary.id);
        let source_dir = dict_dir.join("source");
        let dict_output_dir = dict_dir.join("dict");

        // Create directories if they don't exist
        fs::create_dir_all(&source_dir)?;
        fs::create_dir_all(&dict_output_dir)?;

        // Process text sources
        let mut source_files = Vec::new();
        for text_source in &definition.sources.text_sources {
            let filename = Path::new(&text_source.path)
                .file_name()
                .ok_or_else(|| Error::General(format!("Invalid path: {}", text_source.path)))?;

            let source_file = source_dir.join(filename);

            self.source_fetcher.fetch_file(
                &text_source.repository,
                &text_source.path,
                &source_file,
            )?;

            source_files.push(source_file);
        }

        // Process the word list if there are text sources
        if !source_files.is_empty() {
            let wordlist_path = dict_output_dir.join("wordlist.txt");
            processing::process_word_list(&source_files, &wordlist_path)?;
        }

        // Process Hunspell dictionaries
        for (i, hunspell_source) in definition.sources.hunspell_sources.iter().enumerate() {
            let dic_filename = Path::new(&hunspell_source.dic_path)
                .file_name()
                .ok_or_else(|| {
                    Error::General(format!("Invalid path: {}", hunspell_source.dic_path))
                })?;
            let aff_filename = Path::new(&hunspell_source.aff_path)
                .file_name()
                .ok_or_else(|| {
                    Error::General(format!("Invalid path: {}", hunspell_source.aff_path))
                })?;

            let source_dic = source_dir.join(dic_filename);
            let source_aff = source_dir.join(aff_filename);

            // If there are multiple Hunspell dictionaries, prefix them with an index
            let dic_output_name = if definition.sources.hunspell_sources.len() > 1 {
                format!("{}-{}", i + 1, dic_filename.to_string_lossy())
            } else {
                dic_filename.to_string_lossy().to_string()
            };

            let aff_output_name = if definition.sources.hunspell_sources.len() > 1 {
                format!("{}-{}", i + 1, aff_filename.to_string_lossy())
            } else {
                aff_filename.to_string_lossy().to_string()
            };

            let dic_output = dict_output_dir.join(dic_output_name);
            let aff_output = dict_output_dir.join(aff_output_name);

            // Fetch the files
            self.source_fetcher.fetch_file(
                &hunspell_source.repository,
                &hunspell_source.dic_path,
                &source_dic,
            )?;

            self.source_fetcher.fetch_file(
                &hunspell_source.repository,
                &hunspell_source.aff_path,
                &source_aff,
            )?;

            // Copy to output directory
            fs::copy(&source_dic, &dic_output)?;
            fs::copy(&source_aff, &aff_output)?;

            // Validate the Hunspell dictionary
            processing::validate_hunspell_dictionary(&dic_output, &aff_output)?;
        }

        info!("Built dictionary: {}", definition.dictionary.id);
        Ok(())
    }

    fn validate_dictionary(&self, definition: &DictionaryDefinition) -> Result<()> {
        // Basic validation
        if definition.dictionary.id.is_empty() {
            return Err(Error::Validation(format!(
                "Dictionary ID is empty for '{}'",
                definition.dictionary.name
            )));
        }

        if definition.dictionary.name.is_empty() {
            return Err(Error::Validation(format!(
                "Dictionary name is empty for '{}'",
                definition.dictionary.id
            )));
        }

        if definition.sources.text_sources.is_empty()
            && definition.sources.hunspell_sources.is_empty()
        {
            return Err(Error::Validation(format!(
                "Dictionary '{}' has no sources",
                definition.dictionary.id
            )));
        }

        // Could add more validation here

        debug!(
            "Dictionary definition '{}' is valid",
            definition.dictionary.id
        );
        Ok(())
    }
}
