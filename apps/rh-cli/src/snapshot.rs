use anyhow::Result;
use clap::{Args, Subcommand};
use rh_foundation::cli::{expand_home_dir, parse_package_spec};
use rh_foundation::snapshot::{SnapshotGenerator, StructureDefinitionLoader};
use serde::Serialize;
use std::path::PathBuf;
use tracing::info;

use crate::output::{Envelope, OutputContext, OutputFormat};

#[derive(Serialize)]
struct SnapshotInfo {
    profile: String,
    elements: usize,
    bindings: usize,
    constraints: usize,
}

fn print_envelope<T: Serialize>(ctx: &OutputContext, envelope: &Envelope<T>) -> Result<()> {
    let json = if matches!(ctx.format, OutputFormat::Json) {
        serde_json::to_string_pretty(envelope)?
    } else {
        serde_json::to_string(envelope)?
    };
    println!("{json}");
    Ok(())
}

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

    /// Show detailed output
    #[clap(short, long)]
    pub details: bool,
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

pub async fn handle_command(cmd: SnapshotCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        SnapshotCommands::Generate(args) => handle_generate(args, ctx).await,
        SnapshotCommands::Info(args) => handle_info(args, ctx).await,
        SnapshotCommands::Diff(args) => handle_diff(args).await,
        SnapshotCommands::Validate(args) => handle_validate(args).await,
    }
}

async fn handle_generate(args: GenerateArgs, ctx: &OutputContext) -> Result<()> {
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
            if args.details {
                info!("  - {} ({})", sd.name, sd.url);
            }
            generator.load_structure_definition(sd);
        }
    }

    info!("Generating snapshot...");
    let snapshot = generator.generate_snapshot(&args.profile_url)?;

    let output = serde_json::to_string_pretty(&*snapshot)?;

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output)?;
        info!("Snapshot written to: {}", output_path.display());
    }

    if ctx.is_json() {
        print_envelope(
            ctx,
            &Envelope::ok(snapshot.as_ref().clone(), "snapshot generate"),
        )?;
    } else if args.output.is_none() {
        println!("{output}");
    }

    Ok(())
}

async fn handle_info(args: InfoArgs, ctx: &OutputContext) -> Result<()> {
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

    let info = SnapshotInfo {
        profile: args.profile_url.clone(),
        elements: snapshot.element.len(),
        bindings: snapshot
            .element
            .iter()
            .filter(|e| e.binding.is_some())
            .count(),
        constraints: snapshot
            .element
            .iter()
            .filter_map(|e| e.constraint.as_ref())
            .map(|c| c.len())
            .sum(),
    };

    if ctx.is_json() {
        print_envelope(ctx, &Envelope::ok(info, "snapshot info"))?;
    } else {
        println!("Profile: {}", info.profile);
        println!("Elements: {}", info.elements);
        println!("Bindings: {}", info.bindings);
        println!("Constraints: {}", info.constraints);
    }

    Ok(())
}

async fn handle_diff(_args: DiffArgs) -> Result<()> {
    anyhow::bail!("not yet implemented")
}

async fn handle_validate(_args: ValidateArgs) -> Result<()> {
    anyhow::bail!("not yet implemented")
}
