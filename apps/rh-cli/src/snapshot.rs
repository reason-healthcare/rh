use anyhow::Result;
use clap::{Args, Subcommand};
use rh_foundation::snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use std::path::{Path, PathBuf};
use tracing::{error, info};

#[derive(Subcommand)]
pub enum SnapshotCommands {
    /// Generate snapshot for a StructureDefinition
    Generate(GenerateArgs),

    /// Display information about a snapshot
    Info(InfoArgs),

    /// Show differences between two snapshots
    Diff(DiffArgs),

    /// Validate a snapshot
    Validate(ValidateArgs),
}

#[derive(Args)]
pub struct GenerateArgs {
    /// URL of the StructureDefinition profile to generate snapshot for
    #[clap(value_name = "URL")]
    pub profile_url: String,

    /// FHIR package name to load definitions from (can be specified multiple times)
    #[clap(short, long = "package", value_name = "PACKAGE@VERSION")]
    pub packages: Vec<String>,

    /// Directory containing FHIR package definitions
    #[clap(
        long,
        value_name = "DIR",
        default_value = "~/.fhir/packages",
        help = "Directory containing downloaded FHIR packages"
    )]
    pub packages_dir: PathBuf,

    /// Output file path (defaults to stdout)
    #[clap(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Enable verbose output
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct InfoArgs {
    /// URL of the StructureDefinition to inspect
    #[clap(value_name = "URL")]
    pub profile_url: String,

    /// FHIR package name (can be specified multiple times)
    #[clap(short, long = "package", value_name = "PACKAGE@VERSION")]
    pub packages: Vec<String>,

    /// Directory containing FHIR package definitions
    #[clap(
        long,
        value_name = "DIR",
        default_value = "~/.fhir/packages",
        help = "Directory containing downloaded FHIR packages"
    )]
    pub packages_dir: PathBuf,
}

#[derive(Args)]
pub struct DiffArgs {
    /// URL of first StructureDefinition
    #[clap(value_name = "URL1")]
    pub url1: String,

    /// URL of second StructureDefinition
    #[clap(value_name = "URL2")]
    pub url2: String,

    /// FHIR package name (can be specified multiple times)
    #[clap(short, long = "package", value_name = "PACKAGE@VERSION")]
    pub packages: Vec<String>,

    /// Directory containing FHIR package definitions
    #[clap(
        long,
        value_name = "DIR",
        default_value = "~/.fhir/packages",
        help = "Directory containing downloaded FHIR packages"
    )]
    pub packages_dir: PathBuf,
}

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to snapshot JSON file
    #[clap(value_name = "FILE")]
    pub file: PathBuf,
}

pub async fn handle_command(cmd: SnapshotCommands) -> Result<()> {
    match cmd {
        SnapshotCommands::Generate(args) => handle_generate(args).await,
        SnapshotCommands::Info(args) => handle_info(args).await,
        SnapshotCommands::Diff(args) => handle_diff(args).await,
        SnapshotCommands::Validate(args) => handle_validate(args).await,
    }
}

async fn handle_generate(args: GenerateArgs) -> Result<()> {
    info!("Generating snapshot for profile: {}", args.profile_url);

    let packages_dir = expand_home_dir(&args.packages_dir)?;

    let mut generator = SnapshotGenerator::new();

    for package_spec in &args.packages {
        let (package_name, version) = parse_package_spec(package_spec)?;
        info!("Loading package: {}@{}", package_name, version);

        let structure_defs =
            StructureDefinitionLoader::load_from_package(&package_name, &version, &packages_dir)?;

        info!(
            "Loaded {} StructureDefinitions from package",
            structure_defs.len()
        );

        for sd in structure_defs {
            if args.verbose {
                info!("  - {} ({})", sd.name, sd.url);
            }
            generator.load_structure_definition(sd);
        }
    }

    info!("Generating snapshot...");
    let snapshot = generator.generate_snapshot(&args.profile_url)?;

    let output = serde_json::to_string_pretty(&*snapshot)?;

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, output)?;
        info!("Snapshot written to: {}", output_path.display());
    } else {
        println!("{output}");
    }

    Ok(())
}

async fn handle_info(args: InfoArgs) -> Result<()> {
    info!("Getting info for profile: {}", args.profile_url);

    let packages_dir = expand_home_dir(&args.packages_dir)?;

    let mut generator = SnapshotGenerator::new();

    for package_spec in &args.packages {
        let (package_name, version) = parse_package_spec(package_spec)?;
        info!("Loading package: {}@{}", package_name, version);

        let structure_defs =
            StructureDefinitionLoader::load_from_package(&package_name, &version, &packages_dir)?;

        for sd in structure_defs {
            generator.load_structure_definition(sd);
        }
    }

    let snapshot = generator.generate_snapshot(&args.profile_url)?;

    let profile_url = &args.profile_url;
    let elements_count = snapshot.element.len();
    println!("Profile: {profile_url}");
    println!("Elements: {elements_count}");

    let bindings_count = snapshot
        .element
        .iter()
        .filter(|e| e.binding.is_some())
        .count();
    println!("Bindings: {bindings_count}");

    let constraints_count: usize = snapshot
        .element
        .iter()
        .filter_map(|e| e.constraint.as_ref())
        .map(|c| c.len())
        .sum();
    println!("Constraints: {constraints_count}");

    Ok(())
}

async fn handle_diff(_args: DiffArgs) -> Result<()> {
    error!("Diff command not yet implemented");
    Ok(())
}

async fn handle_validate(_args: ValidateArgs) -> Result<()> {
    error!("Validate command not yet implemented");
    Ok(())
}

fn parse_package_spec(spec: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = spec.split('@').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid package specification: {spec}. Expected format: package@version");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn expand_home_dir(path: &Path) -> Result<PathBuf> {
    let path_str = path.to_string_lossy();
    if let Some(stripped) = path_str.strip_prefix("~/") {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
        Ok(PathBuf::from(home).join(stripped))
    } else {
        Ok(path.to_path_buf())
    }
}
