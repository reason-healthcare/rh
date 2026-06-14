use anyhow::Result;
use clap::{Args, Subcommand};
use rh_foundation::snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use std::path::{Path, PathBuf};
use tracing::{error, info};

use crate::output::{ExitCode, Format, OutputContext};

#[derive(Subcommand)]
pub enum SnapshotCommands {
    /// Generate snapshot for a StructureDefinition
    Generate(GenerateArgs),

    /// Display information about a snapshot
    Info(InfoArgs),

    /// Show differences between two snapshots (not yet implemented)
    Diff(DiffArgs),

    /// Validate a snapshot (not yet implemented)
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

pub async fn handle_command(cmd: SnapshotCommands, ctx: &OutputContext) -> Result<ExitCode> {
    match cmd {
        SnapshotCommands::Generate(args) => handle_generate(args, ctx).await,
        SnapshotCommands::Info(args) => handle_info(args, ctx).await,
        SnapshotCommands::Diff(args) => handle_diff(args, ctx).await,
        SnapshotCommands::Validate(args) => handle_validate(args, ctx).await,
    }
}

async fn handle_generate(args: GenerateArgs, ctx: &OutputContext) -> Result<ExitCode> {
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
            generator.load_structure_definition(sd);
        }
    }

    info!("Generating snapshot...");
    let snapshot = generator.generate_snapshot(&args.profile_url)?;

    let json = serde_json::to_string_pretty(&snapshot)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "profile_url": args.profile_url,
                "snapshot": snapshot,
            });
            if let Some(output_path) = args.output {
                std::fs::write(&output_path, serde_json::to_string_pretty(&result)?)?;
                info!("Snapshot written to: {}", output_path.display());
            } else {
                ctx.write_success(result)?;
            }
        }
        Format::Human => {
            if let Some(output_path) = args.output {
                std::fs::write(&output_path, &json)?;
                info!("Snapshot written to: {}", output_path.display());
            } else {
                println!("{json}");
            }
        }
    }

    Ok(ExitCode::Success)
}

async fn handle_info(args: InfoArgs, ctx: &OutputContext) -> Result<ExitCode> {
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
    let bindings_count = snapshot
        .element
        .iter()
        .filter(|e| e.binding.is_some())
        .count();
    let constraints_count: usize = snapshot
        .element
        .iter()
        .filter_map(|e| e.constraint.as_ref())
        .map(|c| c.len())
        .sum();

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "profile_url": profile_url,
                "elements": elements_count,
                "bindings": bindings_count,
                "constraints": constraints_count,
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!("Profile: {profile_url}");
            println!("Elements: {elements_count}");
            println!("Bindings: {bindings_count}");
            println!("Constraints: {constraints_count}");
        }
    }

    Ok(ExitCode::Success)
}

async fn handle_diff(_args: DiffArgs, ctx: &OutputContext) -> Result<ExitCode> {
    error!("Diff command not yet implemented");
    match ctx.format {
        Format::Json | Format::Ndjson => {
            let errors = vec![crate::output::ErrorInfo {
                code: "NotImplemented".to_string(),
                message: "Diff command not yet implemented".to_string(),
                span: None,
            }];
            ctx.write_errors(errors)?;
        }
        Format::Human => {
            eprintln!("Error: Diff command not yet implemented");
        }
    }
    Ok(ExitCode::OperationalError)
}

async fn handle_validate(_args: ValidateArgs, ctx: &OutputContext) -> Result<ExitCode> {
    error!("Validate command not yet implemented");
    match ctx.format {
        Format::Json | Format::Ndjson => {
            let errors = vec![crate::output::ErrorInfo {
                code: "NotImplemented".to_string(),
                message: "Validate command not yet implemented".to_string(),
                span: None,
            }];
            ctx.write_errors(errors)?;
        }
        Format::Human => {
            eprintln!("Error: Validate command not yet implemented");
        }
    }
    Ok(ExitCode::OperationalError)
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
