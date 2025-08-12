use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;
use tracing::{info, warn};

use rh_codegen::{
    generate_crate_structure, parse_package_metadata, CodeGenerator, CodegenConfig,
    CrateGenerationParams, QualityConfig,
};
use rh_codegen::quality::run_quality_checks;
use rh_loader::{LoaderConfig, PackageLoader};

/// Check if a directory exists and is not empty
fn is_directory_non_empty(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    if !path.is_dir() {
        return Ok(false);
    }

    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_some())
}

#[derive(Args)]
pub struct CodegenArgs {
    /// Package name (e.g., "hl7.fhir.r4.core")
    pub package: String,
    /// Package version (e.g., "4.0.1"). If not provided, uses the latest version
    pub version: Option<String>,

    /// Output directory for generated crate
    #[arg(short, long, default_value = "./generated")]
    pub output: String,

    /// Package directory for downloaded packages (defaults to ~/.fhir/packages)
    #[arg(long)]
    pub package_dir: Option<String>,

    /// Registry URL
    #[arg(long, default_value = "https://packages.fhir.org")]
    pub registry: String,

    /// Overwrite package if it already exists
    #[arg(long)]
    pub overwrite: bool,

    /// Force overwrite of non-empty output directory
    #[arg(long)]
    pub force: bool,
}

pub async fn handle_command(args: CodegenArgs) -> Result<()> {
    // Set up loader configuration first to resolve version if needed
    let token = std::env::var("RH_REGISTRY_TOKEN").ok();
    let loader_config = LoaderConfig {
        registry_url: args.registry.clone(),
        auth_token: token,
        timeout_seconds: 30,
        max_retries: 3,
        verify_checksums: false,
        overwrite_existing: args.overwrite,
    };

    let loader = PackageLoader::new(loader_config)?;

    // Resolve version - use provided version or get latest
    let version = match args.version {
        Some(v) => v,
        None => {
            info!(
                "No version specified, fetching latest version for {}",
                args.package
            );
            loader.get_latest_version(&args.package).await?
        }
    };

    info!(
        "Installing package {}@{} and generating organized crate",
        args.package, version
    );

    let output_path = Path::new(&args.output);

    // Check if output directory exists and is not empty before doing any work
    if is_directory_non_empty(output_path)? {
        if !args.force {
            return Err(anyhow::anyhow!(
                "Output directory '{}' is not empty. Use --force to overwrite existing files.",
                output_path.display()
            ));
        } else {
            warn!(
                "Will remove existing contents of output directory: {}",
                output_path.display()
            );
        }
    }

    // Set up package directory
    let pkg_dir = if let Some(dir) = args.package_dir {
        PathBuf::from(dir)
    } else {
        PackageLoader::get_default_packages_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get default packages directory: {}", e))?
    };

    // Download package if not already downloaded
    if !loader.is_package_downloaded(&args.package, &version, &pkg_dir)? {
        loader
            .download_package(&args.package, &version, &pkg_dir)
            .await?;
    }

    // Use default configuration
    let config = CodegenConfig::default();

    // Set up package directory
    let package_dir = pkg_dir
        .join(format!("{}#{}", args.package, version))
        .join("package");

    let effective_package_dir = if package_dir.exists() {
        &package_dir
    } else {
        &pkg_dir
    };

    // Create generator with ValueSet directory
    let mut generator = CodeGenerator::new_with_value_set_directory(config, effective_package_dir);

    // Remove existing output directory if --force was specified and directory was non-empty
    if args.force && is_directory_non_empty(output_path)? {
        info!(
            "Removing existing contents of output directory: {}",
            output_path.display()
        );
        if output_path.exists() {
            fs::remove_dir_all(output_path)?;
        }
    }

    // Create output directory
    fs::create_dir_all(output_path)?;

    // Process JSON files using organized structure
    process_json_files_organized(&mut generator, effective_package_dir, output_path)?;

    // Generate ValueSet enums in src/bindings directory
    let bindings_dir = output_path.join("src").join("bindings");
    generator
        .generate_enum_files(&bindings_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate enum files: {}", e))?;

    generator
        .generate_enums_mod_file(&bindings_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate enums mod file: {}", e))?;

    info!("Generated ValueSet enums to: {}", bindings_dir.display());

    // Generate complete crate structure
    let command_invoked = format!(
        "rh codegen {} {} --output {}{}{}{}",
        args.package,
        version,
        args.output,
        if args.registry != "https://packages.fhir.org" {
            format!(" --registry {}", args.registry)
        } else {
            String::new()
        },
        if args.overwrite { " --overwrite" } else { "" },
        if args.force { " --force" } else { "" }
    );

    let (canonical, author, description) =
        parse_package_metadata(&effective_package_dir.join("package.json"))
            .map_err(|e| anyhow::anyhow!("Failed to parse package metadata: {}", e))?;

    info!("Using canonical URL from package.json: {}", canonical);

    generate_crate_structure(CrateGenerationParams {
        output: output_path,
        package: &args.package,
        version: &version,
        canonical_url: &canonical,
        author: &author,
        description: &description,
        command_invoked: &command_invoked,
    })?;

    info!("Generated complete organized Rust crate structure");
    info!(
        "Successfully installed package and generated crate at: {}",
        output_path.display()
    );

    Ok(())
}

/// Process JSON files in a directory using organized structure
fn process_json_files_organized(
    generator: &mut CodeGenerator,
    input_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let entries = fs::read_dir(input_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            // Quick check for resourceType before attempting to load
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    warn!("Failed to read {}: {}", path.display(), e);
                    continue;
                }
            };

            // Parse JSON to check resourceType
            let json_value: serde_json::Value = match serde_json::from_str(&content) {
                Ok(value) => value,
                Err(e) => {
                    warn!("Skipping {}: Invalid JSON - {}", path.display(), e);
                    continue;
                }
            };

            // Skip if resourceType is not "StructureDefinition"
            if json_value.get("resourceType").and_then(|v| v.as_str())
                != Some("StructureDefinition")
            {
                continue;
            }

            // Try to load and process as StructureDefinition
            match generator.load_structure_definition(&path) {
                Ok(structure_def) => {
                    info!(
                        "Loaded StructureDefinition: {} ({})",
                        structure_def.name, structure_def.id
                    );

                    // Skip if status is retired
                    if structure_def.status == "retired" {
                        info!(
                            "Skipping retired StructureDefinition: {}",
                            structure_def.name
                        );
                        continue;
                    }

                    // Use the library function to generate structure and traits
                    match rh_codegen::generate_organized_directories_with_traits(
                        generator,
                        &structure_def,
                        output_dir,
                    ) {
                        Ok(()) => {
                            info!("Generated {} with traits", structure_def.name);
                        }
                        Err(e) => {
                            warn!("Failed to generate {}: {}", structure_def.name, e);
                        }
                    }
                }
                Err(e) => {
                    // This might not be a StructureDefinition file, so just skip it
                    warn!("Skipping {}: {}", path.display(), e);
                }
            }
        }
    }

    // Run quality checks on the generated crate
    info!("Running quality checks on generated crate...");
    let quality_config = QualityConfig::default();
    match run_quality_checks(output_dir, &quality_config) {
        Ok(()) => {
            info!("✅ Quality checks completed successfully");
        }
        Err(e) => {
            warn!("⚠️  Quality checks completed with issues: {}", e);
        }
    }

    Ok(())
}
