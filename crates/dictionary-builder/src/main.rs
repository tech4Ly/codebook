//! Command-line interface for the dictionary builder.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

use dictionary_builder::{BuilderConfig, DictionaryBuilder, Error};

/// CLI options
#[derive(Parser)]
#[command(
    name = "dictionary-builder",
    about = "Tool for building and managing spell-checking dictionaries",
    version
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Directory containing dictionary definitions
    #[arg(long, default_value = "dictionaries")]
    config_dir: PathBuf,

    /// Path to save the output manifest
    #[arg(long, default_value = "manifest.json")]
    manifest_output: PathBuf,

    /// Directory for caching downloaded files
    #[arg(long)]
    cache_dir: Option<PathBuf>,

    /// Repository URL to include in manifest
    #[arg(long, default_value = "https://github.com/blopker/codebook")]
    repo_url: String,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands
#[derive(Subcommand)]
enum Commands {
    /// Build all dictionaries
    Build,

    /// Update only changed dictionaries
    Update,

    /// Validate dictionary definitions without building
    Validate,

    /// Only generate the manifest file
    GenerateManifest,

    /// Remove cached files
    Clean,
}

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up logging
    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");

    // Run the appropriate command
    if let Err(err) = run(cli) {
        error!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Error> {
    // Create builder configuration
    let config = BuilderConfig {
        dictionaries_dir: cli.config_dir,
        manifest_output_path: cli.manifest_output,
        cache_dir: cli.cache_dir,
        verbose: cli.verbose,
        repo_url: cli.repo_url,
    };

    // Create dictionary builder
    let builder = DictionaryBuilder::new(config);

    // Execute the selected command
    match cli.command {
        Commands::Build => {
            info!("Building all dictionaries...");
            builder.build_all()?;
        }
        Commands::Update => {
            info!("Updating changed dictionaries...");
            builder.update_changed()?;
        }
        Commands::Validate => {
            info!("Validating dictionary definitions...");
            builder.validate_definitions()?;
        }
        Commands::GenerateManifest => {
            info!("Generating manifest...");
            builder.generate_manifest()?;
        }
        Commands::Clean => {
            info!("Cleaning cache...");
            let source_fetcher =
                dictionary_builder::source::SourceFetcher::new(builder.config.cache_dir.clone());
            source_fetcher.clean_cache()?;
        }
    }

    info!("Done!");
    Ok(())
}
