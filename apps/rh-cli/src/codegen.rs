use std::fs;
use std::path::{Path, PathBuf};

use chrono::Local;

use anyhow::Result;
use clap::Subcommand;
use tracing::{error, info, warn};

use rh_codegen::{CodeGenerator, CodegenConfig};
use rh_common::utils;
use rh_loader::PackageLoader;

use crate::download;

#[derive(Subcommand)]
pub enum CodegenCommands {
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

        /// Directory containing ValueSet JSON files for dynamic enum generation
        #[clap(long)]
        value_set_dir: Option<PathBuf>,
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

        /// Directory containing ValueSet JSON files for dynamic enum generation
        #[clap(long)]
        value_set_dir: Option<PathBuf>,
    },

    /// Install FHIR package and generate types
    Install {
        /// Package name (e.g., "hl7.fhir.r4.core")
        package: String,
        /// Package version (e.g., "4.0.1")
        version: String,

        /// Package directory for downloaded packages (defaults to ~/.fhir/packages)
        #[clap(long)]
        package_dir: Option<PathBuf>,

        /// Output directory for generated types
        #[clap(short, long, default_value = "./generated")]
        output: PathBuf,

        /// Configuration file path
        #[clap(short, long, default_value = "codegen.json")]
        config: PathBuf,

        /// Directory containing value set definitions
        #[clap(long)]
        value_set_dir: Option<PathBuf>,

        /// Generate a complete Rust crate with Cargo.toml
        #[clap(long)]
        generate_crate: bool,

        /// Registry URL
        #[clap(long, default_value = "https://packages.fhir.org")]
        registry: String,

        /// Overwrite package if it already exists
        #[clap(long)]
        overwrite: bool,
    },
}

pub async fn handle_command(cmd: CodegenCommands) -> Result<()> {
    match cmd {
        CodegenCommands::Init { config } => {
            init_config(&config)?;
        }
        CodegenCommands::Generate {
            input,
            output,
            config,
            value_set_dir,
        } => {
            generate_single(&input, output.as_deref(), &config, value_set_dir.as_deref())?;
        }
        CodegenCommands::Batch {
            input_dir,
            output_dir,
            config,
            pattern,
            value_set_dir,
        } => {
            generate_batch(
                &input_dir,
                &output_dir,
                &config,
                &pattern,
                value_set_dir.as_deref(),
            )?;
        }

        CodegenCommands::Install {
            package,
            version,
            package_dir,
            output,
            config,
            value_set_dir,
            generate_crate,
            registry,
            overwrite,
        } => {
            let pkg_dir = match package_dir {
                Some(dir) => dir,
                None => PackageLoader::get_default_packages_dir().map_err(|e| {
                    anyhow::anyhow!("Failed to get default packages directory: {}", e)
                })?,
            };
            let token = std::env::var("RH_REGISTRY_TOKEN").ok();
            install_package(InstallParams {
                package: &package,
                version: &version,
                package_dir: &pkg_dir,
                output: &output,
                config_path: &config,
                value_set_dir: value_set_dir.as_deref(),
                generate_crate,
                registry: &registry,
                token: token.as_deref(),
                overwrite,
            })
            .await?;
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
    value_set_dir: Option<&Path>,
) -> Result<()> {
    info!("Loading configuration from: {}", config_path.display());

    let config: CodegenConfig = utils::load_config_from_file(&config_path.to_string_lossy())
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

    info!("Generating Rust types from: {}", input_path.display());

    let mut generator = if let Some(vs_dir) = value_set_dir {
        info!("Using ValueSet directory: {}", vs_dir.display());
        CodeGenerator::new_with_value_set_directory(config.clone(), vs_dir)
    } else {
        CodeGenerator::new(config.clone())
    };

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
        if output.is_dir() {
            // If output is a directory, create the filename within it
            let file_name = generator.to_filename(&structure_def);
            output.join(file_name)
        } else {
            // If output is a file path, use it directly
            output.to_path_buf()
        }
    } else {
        let output_dir = Path::new(&config.output_dir);
        let file_name = generator.to_filename(&structure_def);
        output_dir.join(file_name)
    };

    // Generate the code
    match generator.generate_to_file(&structure_def, &output_file_path) {
        Ok(()) => {
            info!("Generated Rust types to: {}", output_file_path.display());

            // If we have ValueSet enums, generate them to an enums directory
            let parent_dir = output_file_path.parent().unwrap_or_else(|| Path::new("."));
            let enums_dir = parent_dir.join("enums");

            if generator.has_cached_enums() {
                if let Err(e) = generator.generate_enum_files(&enums_dir) {
                    warn!("Failed to generate enum files: {}", e);
                } else if let Err(e) = generator.generate_enums_mod_file(&enums_dir) {
                    warn!("Failed to generate enums mod file: {}", e);
                } else {
                    info!("Generated ValueSet enums to: {}", enums_dir.display());
                }
            }
        }
        Err(rh_codegen::CodegenError::Generation { message })
            if message.contains("Skipping LogicalModel") =>
        {
            warn!("{}", message);
            return Ok(());
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to generate code: {}", e));
        }
    }

    Ok(())
}

/// Generate Rust types from multiple FHIR StructureDefinitions
fn generate_batch(
    input_dir: &Path,
    output_dir: &Path,
    config_path: &Path,
    pattern: &str,
    value_set_dir: Option<&Path>,
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

    let mut generator = if let Some(vs_dir) = value_set_dir {
        info!("Using ValueSet directory: {}", vs_dir.display());
        CodeGenerator::new_with_value_set_directory(config, vs_dir)
    } else {
        CodeGenerator::new(config)
    };

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
            Err(e) if e.to_string().contains("Skipped LogicalModel") => {
                // LogicalModels are intentionally skipped, don't count as errors
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
    let file_name = generator.to_filename(&structure_def);
    let output_path = output_dir.join(file_name);

    // Generate the code
    match generator.generate_to_file(&structure_def, &output_path) {
        Ok(()) => Ok(output_path),
        Err(rh_codegen::CodegenError::Generation { message })
            if message.contains("Skipping LogicalModel") =>
        {
            // LogicalModels are skipped, but this isn't an error for batch processing
            warn!("{}", message);
            Err(anyhow::anyhow!("Skipped LogicalModel"))
        }
        Err(e) => Err(anyhow::anyhow!("Failed to generate code: {}", e)),
    }
}

/// Process all JSON files in a directory for FHIR StructureDefinitions
fn process_json_files(
    generator: &mut CodeGenerator,
    scan_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let entries = fs::read_dir(scan_dir)?;
    let mut processed_count = 0;
    let mut generated_count = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            processed_count += 1;

            // Only process StructureDefinition files for type generation
            if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                if filename.starts_with("StructureDefinition-") {
                    match process_single_file(generator, &path, output_dir) {
                        Ok(output_path) => {
                            generated_count += 1;
                            info!("Generated: {:?} from {:?}", output_path, filename);
                        }
                        Err(e) => {
                            warn!("Failed to process {:?}: {}", filename, e);
                        }
                    }
                }
            }
        }
    }

    info!(
        "Processed {} JSON files, generated {} Rust files",
        processed_count, generated_count
    );
    Ok(())
}

/// Simple pattern matching (supports * wildcard)
fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if let Some(extension) = pattern.strip_prefix("*.") {
        return filename.ends_with(extension);
    }

    if let Some(prefix) = pattern.strip_suffix("*") {
        return filename.starts_with(prefix);
    }

    filename == pattern
}

/// Parameters for crate generation
struct CrateGenerationParams<'a> {
    output: &'a Path,
    types_dir: &'a Path,
    package: &'a str,
    version: &'a str,
    canonical_url: &'a str,
    author: &'a str,
    description: &'a str,
    command_invoked: &'a str,
}

/// Generate a complete Rust crate structure with Cargo.toml, lib.rs, and README.md
fn generate_crate_structure(params: CrateGenerationParams) -> Result<()> {
    // Generate statistics by counting generated files
    let stats = generate_crate_statistics(params.types_dir)?;

    // Generate Cargo.toml
    let cargo_toml_content = generate_cargo_toml(params.package, params.version);
    let cargo_toml_path = params.output.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;
    info!("Generated: {}", cargo_toml_path.display());

    // Generate lib.rs
    let lib_rs_content = generate_lib_rs(params.types_dir)?;
    let lib_rs_path = params.output.join("lib.rs");
    fs::write(&lib_rs_path, lib_rs_content)?;
    info!("Generated: {}", lib_rs_path.display());

    // Generate README.md
    let readme_content = generate_readme_md(
        params.package,
        params.version,
        params.canonical_url,
        params.author,
        params.description,
        params.command_invoked,
        &stats,
    );
    let readme_path = params.output.join("README.md");
    fs::write(&readme_path, readme_content)?;
    info!("Generated: {}", readme_path.display());

    // Generate mod.rs in the types directory
    let mod_rs_content = generate_types_mod_rs(params.types_dir)?;
    let mod_rs_path = params.types_dir.join("mod.rs");
    fs::write(&mod_rs_path, mod_rs_content)?;
    info!("Generated: {}", mod_rs_path.display());

    Ok(())
}

/// Generate Cargo.toml content for the FHIR crate
fn generate_cargo_toml(package: &str, version: &str) -> String {
    // Convert FHIR package name to a valid Rust crate name
    let crate_name = package.replace(['.', '-'], "_");

    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"
description = "Generated FHIR types from {package} package version {version}"
authors = ["FHIR Code Generator"]
license = "MIT OR Apache-2.0"

[workspace]
# Empty workspace to exclude from parent workspace

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

[lib]
name = "{crate_name}"
path = "lib.rs"
"#
    )
}

/// Generate lib.rs content that exports all modules
fn generate_lib_rs(_types_dir: &Path) -> Result<String> {
    let mut lib_content = String::new();

    // Add crate-level documentation
    lib_content.push_str(
        r#"//! Generated FHIR types
//!
//! This crate contains generated Rust types for FHIR resources.
//! All types include serde serialization support.

pub use serde::{Deserialize, Serialize};

"#,
    );

    // Add types module
    lib_content.push_str("pub mod types;\n");
    lib_content.push_str("pub use types::*;\n");

    Ok(lib_content)
}

/// Generate types/mod.rs that declares all generated modules
fn generate_types_mod_rs(types_dir: &Path) -> Result<String> {
    let mut content = String::new();
    content.push_str("// Generated FHIR types\n\n");

    // Get all .rs files in the types directory
    let mut rs_files = Vec::new();
    if types_dir.exists() {
        for entry in fs::read_dir(types_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "mod" {
                        rs_files.push(stem.to_string());
                    }
                }
            }
        }
    }

    // Sort module names for consistency
    rs_files.sort();

    // Generate module declarations
    for module_name in &rs_files {
        content.push_str(&format!("pub mod {module_name};\n"));
    }

    // Add re-exports
    if !rs_files.is_empty() {
        content.push('\n');
        for module_name in &rs_files {
            content.push_str(&format!("pub use {module_name}::*;\n"));
        }
    }

    Ok(content)
}

/// Statistics about generated crate content
#[derive(Debug)]
struct CrateStatistics {
    num_structs: usize,
    num_enums: usize,
    total_types: usize,
    #[allow(dead_code)]
    canonical_url: String,
}

/// Generate statistics by counting generated files and analyzing content
fn generate_crate_statistics(types_dir: &Path) -> Result<CrateStatistics> {
    let mut num_structs = 0;
    let mut num_enums = 0;

    // Count .rs files in types directory (each represents roughly one main struct)
    if types_dir.exists() {
        for entry in fs::read_dir(types_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "mod" {
                        // Count each .rs file as a struct (main type)
                        num_structs += 1;

                        // Try to count additional structs within the file (nested types)
                        if let Ok(content) = fs::read_to_string(&path) {
                            num_structs += content.matches("pub struct ").count().saturating_sub(1);
                        }
                    }
                }
            }
        }
    }

    // Count enums in enums directory
    let enums_dir = types_dir.join("enums");
    if enums_dir.exists() {
        for entry in fs::read_dir(&enums_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "mod" {
                        num_enums += 1;
                    }
                }
            }
        }
    }

    let total_types = num_structs + num_enums;

    Ok(CrateStatistics {
        num_structs,
        num_enums,
        total_types,
        canonical_url: "Unknown".to_string(), // Will be updated by caller
    })
}

/// Generate README.md content with package information and statistics
fn generate_readme_md(
    package: &str,
    version: &str,
    canonical_url: &str,
    author: &str,
    description: &str,
    command_invoked: &str,
    stats: &CrateStatistics,
) -> String {
    let crate_name = package.replace(['.', '-'], "_");
    let mut content = String::new();

    content.push_str(&format!("# {crate_name}\n\n"));
    content.push_str(&format!("**Generated FHIR Types for {package}**\n\n"));
    content.push_str(&format!("This crate contains automatically generated Rust types for FHIR (Fast Healthcare Interoperability Resources) based on the `{package}` package.\n\n"));

    content.push_str("## ⚠️ Important Notice\n\n");
    content
        .push_str("**This crate was automatically generated using the RH codegen CLI tool.**\n\n");
    content.push_str(&format!(
        "- **Generator command**:\n```bash\n{command_invoked}\n```\n\n"
    ));
    content.push_str(&format!("- **Generation timestamp**: {}\n\n", Local::now()));

    content.push_str("## 📦 Package Information\n\n");

    content.push_str(&format!("* **Package Name** {package}\n"));
    content.push_str(&format!("* **Package Author** {author}\n"));
    content.push_str(&format!("* **Version** {version}\n"));
    content.push_str(&format!("* **Canonical URL** `{canonical_url}`\n\n"));

    content.push_str(&format!(
        "**📊 There are {} structs, {} enums, with a total of {} types**\n\n",
        stats.num_structs, stats.num_enums, stats.total_types
    ));

    content.push_str(&format!("## 📚 Description\n\n{description}"));

    content.push_str("\n\n");

    content.push_str("## 🚀 Usage\n\n");
    content.push_str("Add this crate to your `Cargo.toml`:\n\n");
    content.push_str("```toml\n");
    content.push_str("[dependencies]\n");
    content.push_str(&format!("{crate_name} = \"0.1.0\"\n"));
    content.push_str("```\n\n");

    content.push_str("Example usage:\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::*;\n"));
    content.push_str("use serde_json;\n\n");
    content.push_str("// All types include serde serialization support\n");
    content.push_str("let json_data = r#\"{\\\"resourceType\\\": \\\"Patient\\\", \\\"id\\\": \\\"example\\\"}\"#;\n");
    content.push_str("let patient: Patient = serde_json::from_str(json_data)?;\n\n");
    content.push_str("println!(\"Patient ID: {}\", patient.id.unwrap_or_default());\n");
    content.push_str("```\n\n");

    content.push_str("## 🏗️ Structure\n\n");
    content
        .push_str("- **`types/`** - Contains all generated FHIR resource and data type structs\n");
    content.push_str("- **`types/enums/`** - Contains generated enums for FHIR value sets\n");
    content.push_str("- **`lib.rs`** - Main library file that re-exports all types\n\n");

    content.push_str("## 🔄 Regeneration\n\n");
    content.push_str("To regenerate this crate with updated FHIR definitions, run:\n\n");
    content.push_str("```bash\n");
    content.push_str(command_invoked);
    content.push_str("\n```\n\n");

    content.push_str("## ⚖️ License\n\n");
    content.push_str("This generated crate is provided under MIT OR Apache-2.0 license.\n\n");

    content.push_str("## 🔗 Related Links\n\n");
    content.push_str("- [FHIR Specification](https://hl7.org/fhir/)\n");
    content.push_str("- [RH Codegen Tool](https://github.com/reason-healthcare/rh)\n");

    content
}

/// Parameters for installing FHIR packages and generating types
struct InstallParams<'a> {
    package: &'a str,
    version: &'a str,
    package_dir: &'a Path,
    output: &'a Path,
    config_path: &'a Path,
    value_set_dir: Option<&'a Path>,
    generate_crate: bool,
    registry: &'a str,
    token: Option<&'a str>,
    overwrite: bool,
}

/// Install FHIR package and generate types
async fn install_package(params: InstallParams<'_>) -> Result<()> {
    info!(
        "Installing package {}@{} and generating types",
        params.package, params.version
    );

    // Download the package to the package directory
    download::download_package_to_dir(
        params.package,
        params.version,
        params.package_dir,
        params.registry,
        params.overwrite,
    )
    .await?;

    // Load the codegen configuration
    let config = if params.config_path.exists() {
        let config_content = fs::read_to_string(params.config_path)?;
        serde_json::from_str(&config_content)?
    } else {
        warn!("Configuration file not found, using default settings");
        CodegenConfig::default()
    };

    // Determine the ValueSet directory - default to package directory if not specified
    let package_dir = params
        .package_dir
        .join(format!("{}#{}", params.package, params.version))
        .join("package");
    let effective_value_set_dir = if let Some(value_set_dir) = params.value_set_dir {
        value_set_dir
    } else {
        // Default to using the package directory for ValueSets
        if package_dir.exists() {
            info!(
                "Using package directory for ValueSets: {}",
                package_dir.display()
            );
            &package_dir
        } else {
            info!(
                "Using package directory for ValueSets: {}",
                params.package_dir.display()
            );
            params.package_dir
        }
    };

    // Create the generator with ValueSet directory
    let mut generator =
        CodeGenerator::new_with_value_set_directory(config, effective_value_set_dir);

    // Create output directory
    fs::create_dir_all(params.output)?;

    // Determine the actual output directory for types
    let types_output = if params.generate_crate {
        let types_dir = params.output.join("types");
        fs::create_dir_all(&types_dir)?;
        types_dir
    } else {
        params.output.to_path_buf()
    };

    // Find all StructureDefinition JSON files in the package directory
    if !package_dir.exists() {
        warn!("Package directory not found, scanning temp directory directly");
        process_json_files(&mut generator, params.package_dir, &types_output)?;
    } else {
        process_json_files(&mut generator, &package_dir, &types_output)?;
    }

    // Generate ValueSet enums in a separate enums directory
    let enums_dir = if params.generate_crate {
        types_output.join("enums")
    } else {
        params.output.join("enums")
    };
    generator
        .generate_enum_files(&enums_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate enum files: {}", e))?;

    generator
        .generate_enums_mod_file(&enums_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate enums mod file: {}", e))?;

    info!("Generated ValueSet enums to: {}", enums_dir.display());

    // Generate crate structure if requested
    if params.generate_crate {
        // Build the command string for documentation
        let mut command_parts = vec![
            "cargo".to_string(),
            "run".to_string(),
            "--bin".to_string(),
            "rh".to_string(),
            "codegen".to_string(),
            "install".to_string(),
            params.package.to_string(),
            params.version.to_string(),
            "--output".to_string(),
            params.output.to_string_lossy().to_string(),
            "--generate-crate".to_string(),
        ];

        if params.registry != "https://packages.fhir.org" {
            command_parts.push("--registry".to_string());
            command_parts.push(params.registry.to_string());
        }

        if params.token.is_some() {
            command_parts.push("--token".to_string());
            command_parts.push("<TOKEN>".to_string());
        }

        let command_invoked = command_parts.join(" ");

        let package_json_content = fs::read_to_string(package_dir.join("package.json"))
            .map_err(|e| anyhow::anyhow!("Failed to read package.json: {}", e))?;

        let package_json: serde_json::Value = serde_json::from_str(&package_json_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse package.json: {}", e))?;

        let canonical = package_json
            .get("canonical")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let author = package_json
            .get("author")
            .and_then(|v| v.as_str())
            .unwrap_or("FHIR Code Generator");

        let description = package_json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("Generated FHIR types crate.");

        info!("Using canonical URL from package.json: {}", canonical);

        generate_crate_structure(CrateGenerationParams {
            output: params.output,
            types_dir: &types_output,
            package: params.package,
            version: params.version,
            canonical_url: canonical,
            author,
            description,
            command_invoked: &command_invoked,
        })?;
        info!("Generated complete Rust crate structure");
    }

    // Package remains in persistent directory for future use

    info!("Successfully installed package and generated types");
    Ok(())
}
