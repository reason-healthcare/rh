use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{error, info, warn};

use codegen::{CodeGenerator, CodegenConfig, PackageDownloader, PackageDownloadConfig};
use common::utils;

/// FHIR Code Generation CLI
///
/// Generate Rust types from FHIR StructureDefinition JSON files
#[derive(Parser)]
#[clap(name = "fhir-codegen")]
#[clap(about = "Generate Rust types from FHIR StructureDefinitions")]
#[clap(version)]
struct Cli {
    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new codegen configuration file
    Init {
        /// Path to the configuration file
        #[clap(short, long, default_value = "codegen.json")]
        config: PathBuf,
    },
    /// Generate Rust types from a single FHIR StructureDefinition
    Generate {
        /// Path to the FHIR StructureDefinition JSON file
        #[clap(short, long)]
        input: PathBuf,

        /// Output directory for generated Rust files
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Path to the configuration file
        #[clap(short, long, default_value = "codegen.json")]
        config: PathBuf,
    },
    /// Generate Rust types from multiple FHIR StructureDefinitions
    Batch {
        /// Directory containing FHIR StructureDefinition JSON files
        #[clap(short, long)]
        input_dir: PathBuf,

        /// Output directory for generated Rust files
        #[clap(short, long)]
        output_dir: PathBuf,

        /// Path to the configuration file
        #[clap(short, long, default_value = "codegen.json")]
        config: PathBuf,

        /// File pattern to match (e.g., "*.json")
        #[clap(short, long, default_value = "*.json")]
        pattern: String,
    },
    /// Download FHIR package from registry
    Download {
        /// Package name (e.g., "hl7.fhir.r4.core")
        package: String,

        /// Package version (e.g., "4.0.1")
        version: String,

        /// Output directory for downloaded package
        #[clap(short, long, default_value = "./packages")]
        output: PathBuf,
        
        /// Registry URL
        #[clap(long, default_value = "https://packages.fhir.org")]
        registry: String,
        
        /// Authentication token for private registries
        #[clap(long)]
        token: Option<String>,
    },
    /// Install FHIR package and generate types
    Install {
        /// Package name (e.g., "hl7.fhir.r4.core")
        package: String,

        /// Package version (e.g., "4.0.1")
        version: String,

        /// Output directory for generated Rust files
        #[clap(short, long, default_value = "./generated")]
        output: PathBuf,

        /// Path to the configuration file
        #[clap(short, long, default_value = "codegen.json")]
        config: PathBuf,
        
        /// Registry URL
        #[clap(long, default_value = "https://packages.fhir.org")]
        registry: String,
        
        /// Authentication token for private registries
        #[clap(long)]
        token: Option<String>,
    },
}

fn main() -> Result<()> {
    tokio::runtime::Runtime::new()?.block_on(async_main())
}

async fn async_main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let subscriber = if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .finish()
    };
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Init { config } => {
            init_config(&config)?;
        }
        Commands::Generate {
            input,
            output,
            config,
        } => {
            generate_single(&input, output.as_deref(), &config)?;
        }
        Commands::Batch {
            input_dir,
            output_dir,
            config,
            pattern,
        } => {
            generate_batch(&input_dir, &output_dir, &config, &pattern)?;
        }
        Commands::Download {
            package,
            version,
            output,
            registry,
            token,
        } => {
            download_package(&package, &version, &output, &registry, token.as_deref()).await?;
        }
        Commands::Install {
            package,
            version,
            output,
            config,
            registry,
            token,
        } => {
            install_package(&package, &version, &output, &config, &registry, token.as_deref()).await?;
        }
    }

    Ok(())
}

/// Initialize a new codegen configuration file
fn init_config(config_path: &Path) -> Result<()> {
    info!(
        "Initializing codegen configuration at: {}",
        config_path.display()
    );

    let default_config = CodegenConfig::default();
    utils::save_config_to_file(&default_config, &config_path.to_string_lossy())
        .map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))?;

    info!("Configuration initialized successfully!");
    info!("Edit the configuration file to customize code generation settings.");

    Ok(())
}

/// Generate Rust types from a single FHIR StructureDefinition
fn generate_single(
    input_path: &Path,
    output_path: Option<&Path>,
    config_path: &Path,
) -> Result<()> {
    info!("Loading configuration from: {}", config_path.display());

    let config: CodegenConfig = utils::load_config_from_file(&config_path.to_string_lossy())
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

    info!("Generating Rust types from: {}", input_path.display());

    let mut generator = CodeGenerator::new(config.clone());

    // Load the StructureDefinition
    let structure_def = generator
        .load_structure_definition(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to load StructureDefinition: {}", e))?;

    info!(
        "Loaded StructureDefinition: {} ({})",
        structure_def.name, structure_def.id
    );

    // Determine output path
    let output_file_path = if let Some(output) = output_path {
        output.to_path_buf()
    } else {
        let output_dir = Path::new(&config.output_dir);
        let file_name = format!("{}.rs", structure_def.name.to_lowercase());
        output_dir.join(file_name)
    };

    // Generate the code
    generator
        .generate_to_file(&structure_def, &output_file_path)
        .map_err(|e| anyhow::anyhow!("Failed to generate code: {}", e))?;

    info!("Generated Rust types to: {}", output_file_path.display());

    Ok(())
}

/// Generate Rust types from multiple FHIR StructureDefinitions
fn generate_batch(
    input_dir: &Path,
    output_dir: &Path,
    config_path: &Path,
    pattern: &str,
) -> Result<()> {
    info!("Loading configuration from: {}", config_path.display());

    let config: CodegenConfig = utils::load_config_from_file(&config_path.to_string_lossy())
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

    info!(
        "Processing FHIR StructureDefinitions from: {}",
        input_dir.display()
    );
    info!("Output directory: {}", output_dir.display());
    info!("File pattern: {}", pattern);

    let mut generator = CodeGenerator::new(config);

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Find all matching files
    let mut processed_count = 0;
    let mut error_count = 0;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a file or doesn't match pattern
        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy();
        if !matches_pattern(&file_name, pattern) {
            continue;
        }

        info!("Processing: {}", path.display());

        match process_single_file(&mut generator, &path, output_dir) {
            Ok(output_path) => {
                info!("Generated: {}", output_path.display());
                processed_count += 1;
            }
            Err(e) => {
                error!("Failed to process {}: {}", path.display(), e);
                error_count += 1;
            }
        }
    }

    info!("Batch processing complete!");
    info!("Processed: {} files", processed_count);
    if error_count > 0 {
        warn!("Errors: {} files", error_count);
    }

    Ok(())
}

/// Process a single file in batch mode
fn process_single_file(
    generator: &mut CodeGenerator,
    input_path: &Path,
    output_dir: &Path,
) -> Result<PathBuf> {
    // Load the StructureDefinition
    let structure_def = generator
        .load_structure_definition(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to load StructureDefinition: {}", e))?;

    // Determine output path
    let file_name = format!("{}.rs", structure_def.name.to_lowercase());
    let output_path = output_dir.join(file_name);

    // Generate the code
    generator
        .generate_to_file(&structure_def, &output_path)
        .map_err(|e| anyhow::anyhow!("Failed to generate code: {}", e))?;

    Ok(output_path)
}

/// Simple pattern matching (supports * wildcard)
fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if pattern.starts_with("*.") {
        let extension = &pattern[2..];
        return filename.ends_with(extension);
    }

    if pattern.ends_with("*") {
        let prefix = &pattern[..pattern.len() - 1];
        return filename.starts_with(prefix);
    }

    filename == pattern
}

/// Download FHIR package from registry
async fn download_package(
    package: &str,
    version: &str,
    output: &Path,
    registry: &str,
    token: Option<&str>,
) -> Result<()> {
    info!("Downloading package {}@{} from {}", package, version, registry);

    let download_config = PackageDownloadConfig {
        registry_url: registry.to_string(),
        auth_token: token.map(|t| t.to_string()),
        timeout_seconds: 30,
    };

    let downloader = PackageDownloader::new(download_config)?;
    downloader.download_package(package, version, output).await?;

    info!("Successfully downloaded package to {:?}", output);
    Ok(())
}

/// Install FHIR package and generate types  
async fn install_package(
    package: &str,
    version: &str,
    output: &Path,
    config_path: &Path,
    registry: &str,
    token: Option<&str>,
) -> Result<()> {
    info!("Installing package {}@{} and generating types", package, version);

    // First download the package to a temporary directory
    let temp_dir = std::env::temp_dir().join(format!("fhir-package-{}-{}", package, version));
    download_package(package, version, &temp_dir, registry, token).await?;

    // Load the codegen configuration
    let config = if config_path.exists() {
        let config_content = fs::read_to_string(config_path)?;
        serde_json::from_str(&config_content)?
    } else {
        warn!("Configuration file not found, using default settings");
        CodegenConfig::default()
    };

    // Create the generator
    let mut generator = CodeGenerator::new(config);

    // Find all JSON files in the downloaded package
    let entries = fs::read_dir(&temp_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            match process_single_file(&mut generator, &path, output) {
                Ok(output_path) => {
                    info!("Generated: {:?}", output_path);
                }
                Err(e) => {
                    warn!("Failed to process {:?}: {}", path, e);
                }
            }
        }
    }

    // Clean up temporary directory
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    info!("Successfully installed package and generated types");
    Ok(())
}
