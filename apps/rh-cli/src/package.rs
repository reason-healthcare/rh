use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::output::{Format, OutputContext};

#[derive(Subcommand)]
pub enum PackageCommands {
    /// Scaffold a new FHIR Package source directory
    Init(InitArgs),

    /// Build a FHIR Package from a source directory
    Build(BuildArgs),

    /// Resolve canonical references and write/update fhir-lock.json
    Lock(LockArgs),

    /// Report canonical reference pinning status (pinned vs. unpinned)
    LockCheck(LockCheckArgs),

    /// Validate source before building (sync check, lock check)
    Check(CheckArgs),

    /// Pack an expanded output directory into a .tgz tarball
    Pack(PackArgs),
}

#[derive(Args)]
pub struct InitArgs {
    /// Target directory for the new package (defaults to current directory)
    pub dir: Option<PathBuf>,

    /// Package name (e.g. hl7.fhir.us.core). Inferred from --canonical when absent.
    #[clap(long, short = 'n')]
    pub name: Option<String>,

    /// Canonical URL base for resources (e.g. https://example.org/fhir)
    #[clap(long, short = 'c')]
    pub canonical: String,

    /// Package version [default: 0.1.0]
    #[clap(long, default_value = "0.1.0")]
    pub version: String,

    /// FHIR version [default: 4.0.1]
    #[clap(long, default_value = "4.0.1")]
    pub fhir_version: String,

    /// Human-readable title (defaults to PascalCase of name)
    #[clap(long)]
    pub title: Option<String>,

    /// Package description
    #[clap(long)]
    pub description: Option<String>,

    /// Author or publisher name
    #[clap(long)]
    pub author: Option<String>,

    /// SPDX license identifier [default: CC0-1.0]
    #[clap(long, default_value = "CC0-1.0")]
    pub license: String,

    /// IG resource status (draft|active|retired) [default: draft]
    #[clap(long, default_value = "draft")]
    pub status: String,
}

#[derive(Args)]
pub struct BuildArgs {
    /// Path to the source directory containing packager.toml and FHIR resources
    pub dir: PathBuf,

    /// Output directory for the expanded package (default: <dir>/output)
    #[clap(short, long)]
    pub out: Option<PathBuf>,
}

#[derive(Args)]
pub struct LockArgs {
    /// Path to the source directory
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct LockCheckArgs {
    /// Path to the source directory
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Path to the source directory
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct PackArgs {
    /// Path to the expanded output directory to pack
    pub dir: PathBuf,

    /// Output path for the resulting .tgz file (default: <dir>/../<name>-<version>.tgz)
    #[clap(short, long)]
    pub out: Option<PathBuf>,
}

pub async fn handle_command(cmd: PackageCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        PackageCommands::Init(args) => handle_init(args, ctx),
        PackageCommands::Build(args) => handle_build(args, ctx),
        PackageCommands::Lock(args) => handle_lock(args, ctx),
        PackageCommands::LockCheck(args) => handle_lock_check(args, ctx),
        PackageCommands::Check(args) => handle_check(args, ctx),
        PackageCommands::Pack(args) => handle_pack(args, ctx),
    }
}

fn handle_init(args: InitArgs, ctx: &OutputContext) -> Result<()> {
    let dir = args
        .dir
        .unwrap_or_else(|| std::env::current_dir().expect("cannot determine current dir"));

    let name = match args.name {
        Some(n) => n,
        None => {
            let derived = rh_packager::name_from_canonical(&args.canonical).ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not derive a package name from '{}'. \
                     Provide --name explicitly (e.g. --name hl7.fhir.us.core).",
                    &args.canonical
                )
            })?;
            if matches!(ctx.format, Format::Human) {
                eprintln!("  Name derived from canonical: {derived}");
            }
            derived
        }
    };

    let opts = rh_packager::InitOptions {
        name,
        canonical: args.canonical,
        version: args.version,
        fhir_version: args.fhir_version,
        title: args.title,
        description: args.description,
        author: args.author,
        license: args.license,
        status: args.status,
    };
    let created = rh_packager::init_package(&dir, opts)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "directory": dir.display().to_string(),
                "created_files": created.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            for path in &created {
                println!("  Created: {}", path.display());
            }
            println!("\nPackage initialised at {}", dir.display());
            println!(
                "Next: edit packager.toml, then run `rh package build {}`",
                dir.display()
            );
        }
    }

    Ok(())
}

fn handle_build(args: BuildArgs, ctx: &OutputContext) -> Result<()> {
    let output_dir = args.out.unwrap_or_else(|| args.dir.join("output"));
    rh_packager::build(&args.dir, &output_dir)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "source": args.dir.display().to_string(),
                "output": output_dir.display().to_string(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!(
                "Built package from {} to {}",
                args.dir.display(),
                output_dir.display()
            );
        }
    }

    Ok(())
}

fn handle_lock(args: LockArgs, ctx: &OutputContext) -> Result<()> {
    let output_dir = args.dir.join("output");
    rh_packager::lock_package(&args.dir, &output_dir)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "source": args.dir.display().to_string(),
                "output": output_dir.display().to_string(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!("Locked package at {}", args.dir.display());
        }
    }

    Ok(())
}

fn handle_lock_check(args: LockCheckArgs, ctx: &OutputContext) -> Result<()> {
    let report = rh_packager::check_lock(&args.dir)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "pinned": report.pinned.iter().map(|r| serde_json::json!({
                    "url": r.url,
                    "pinned_version": r.pinned_version,
                    "resource_key": r.resource_key,
                    "field_path": r.field_path,
                })).collect::<Vec<_>>(),
                "unpinned": report.unpinned.iter().map(|r| serde_json::json!({
                    "url": r.url,
                    "resource_key": r.resource_key,
                    "field_path": r.field_path,
                })).collect::<Vec<_>>(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            let sym_pinned = crate::output::symbol_success(ctx);
            let sym_unpinned = crate::output::symbol_warn(ctx);
            println!("PINNED ({}):", report.pinned.len());
            for r in &report.pinned {
                println!(
                    "  {} {}|{} (in: {}, field: {})",
                    sym_pinned,
                    r.url,
                    r.pinned_version.as_deref().unwrap_or("?"),
                    r.resource_key,
                    r.field_path
                );
            }

            println!("\nUNPINNED ({}):", report.unpinned.len());
            for r in &report.unpinned {
                println!(
                    "  {} {} (in: {}, field: {})",
                    sym_unpinned, r.url, r.resource_key, r.field_path
                );
            }

            if !report.unpinned.is_empty() {
                println!(
                    "\nRun `rh package lock {}` to pin all unversioned references.",
                    args.dir.display()
                );
            }
        }
    }

    Ok(())
}

fn handle_check(args: CheckArgs, ctx: &OutputContext) -> Result<()> {
    rh_packager::check(&args.dir)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "directory": args.dir.display().to_string(),
                "valid": true,
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            let sym = crate::output::symbol_success(ctx);
            println!("{sym} Package check passed for {}", args.dir.display());
        }
    }

    Ok(())
}

fn handle_pack(args: PackArgs, ctx: &OutputContext) -> Result<()> {
    let tgz = rh_packager::pack_dir(&args.dir)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "output": tgz.display().to_string(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!("Packed: {}", tgz.display());
        }
    }

    Ok(())
}
