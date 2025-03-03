//! Manifest generation and handling.

use std::fs;
use std::path::Path;

use chrono::Utc;
use glob::glob;
use tracing::{debug, info};

use crate::Result;
use crate::checksum;
use crate::error::Error;
use crate::types::{DictionaryDefinition, DictionaryFile, DictionaryInfo, Manifest};

/// Create a manifest from dictionaries in the specified directory
pub fn create_manifest(
    dictionaries_dir: &Path,
    repo_url: &str,
    git_hash: &str,
) -> Result<Manifest> {
    let pattern = dictionaries_dir.join("*/dictionary.toml");
    let pattern_str = pattern.to_string_lossy();

    let mut dictionaries = Vec::new();

    for entry in glob(&pattern_str).map_err(|e| Error::General(e.to_string()))? {
        let path = entry.map_err(|e| Error::General(e.to_string()))?;
        debug!("Processing dictionary at {}", path.display());

        let dict_info = process_dictionary_definition(&path, dictionaries_dir)?;
        dictionaries.push(dict_info);
    }

    // Sort dictionaries by ID for consistent output
    dictionaries.sort_by(|a, b| a.id.cmp(&b.id));

    let manifest = Manifest {
        dictionaries,
        generated_at: Utc::now().to_rfc3339(),
        git_hash: git_hash.to_string(),
        repo_url: repo_url.to_string(),
    };

    Ok(manifest)
}

/// Save a manifest to a file
pub fn save_manifest(manifest: &Manifest, path: &Path) -> Result<()> {
    let content = serde_json::to_string_pretty(manifest)?;
    fs::write(path, content)?;
    info!("Saved manifest to {}", path.display());
    Ok(())
}

// Helper function to process a single dictionary definition
fn process_dictionary_definition(toml_path: &Path, base_dir: &Path) -> Result<DictionaryInfo> {
    let content = fs::read_to_string(toml_path)?;
    let definition: DictionaryDefinition = toml::from_str(&content)?;

    let dict_dir = toml_path.parent().ok_or_else(|| {
        Error::General(format!(
            "Failed to get parent directory of {}",
            toml_path.display()
        ))
    })?;

    let dict_output_dir = dict_dir.join("dict");
    let mut files = Vec::new();

    // Process wordlist.txt if it exists
    let wordlist_path = dict_output_dir.join("wordlist.txt");
    if wordlist_path.exists() {
        let rel_path = make_relative_path(&wordlist_path, base_dir)?;
        let checksum = checksum::calculate_sha256(&wordlist_path)?;

        files.push(DictionaryFile::Wordlist {
            path: rel_path,
            checksum,
        });
    }

    // Process Hunspell dictionaries
    let dic_pattern = dict_output_dir.join("*.dic");
    let dic_pattern_str = dic_pattern.to_string_lossy();

    for dic_entry in glob(&dic_pattern_str).map_err(|e| Error::General(e.to_string()))? {
        let dic_path = dic_entry.map_err(|e| Error::General(e.to_string()))?;
        let _dic_stem = dic_path.file_stem().ok_or_else(|| {
            Error::General(format!("Failed to get file stem of {}", dic_path.display()))
        })?;

        let aff_path = dic_path.with_extension("aff");
        if !aff_path.exists() {
            return Err(Error::Manifest(format!(
                "Missing .aff file for {}",
                dic_path.display()
            )));
        }

        let rel_dic_path = make_relative_path(&dic_path, base_dir)?;
        let rel_aff_path = make_relative_path(&aff_path, base_dir)?;

        let dic_checksum = checksum::calculate_sha256(&dic_path)?;
        let aff_checksum = checksum::calculate_sha256(&aff_path)?;

        files.push(DictionaryFile::Hunspell {
            dic_path: rel_dic_path,
            aff_path: rel_aff_path,
            dic_checksum,
            aff_checksum,
        });
    }

    if files.is_empty() {
        return Err(Error::Manifest(format!(
            "No dictionary files found for {}",
            definition.dictionary.id
        )));
    }

    Ok(DictionaryInfo {
        id: definition.dictionary.id,
        name: definition.dictionary.name,
        description: definition.dictionary.description,
        language_ids: if definition.scope.language_ids.is_empty() {
            None
        } else {
            Some(definition.scope.language_ids)
        },
        file_extensions: if definition.scope.file_extensions.is_empty() {
            None
        } else {
            Some(definition.scope.file_extensions)
        },
        files,
    })
}

// Helper function to make a path relative to a base directory
fn make_relative_path(path: &Path, base_dir: &Path) -> Result<String> {
    let rel_path = path.strip_prefix(base_dir).map_err(|_| {
        Error::General(format!(
            "Failed to make {} relative to {}",
            path.display(),
            base_dir.display()
        ))
    })?;

    Ok(rel_path.to_string_lossy().to_string())
}
