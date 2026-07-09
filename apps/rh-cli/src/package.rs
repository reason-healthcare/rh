use anyhow::Result;
use clap::{Args, Subcommand};
use serde::Serialize;
use std::path::PathBuf;
use tracing::info;

use crate::output::{Envelope, OutputContext, OutputFormat};

#[derive(Serialize)]
struct InitResult {
    dir: String,
    files_created: Vec<String>,
}

#[derive(Serialize)]
struct LockCheckResult {
    pinned: Vec<String>,
    unpinned: Vec<String>,
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
    /// Convention: <sld>.<path-segments> — http://hl7.org/fhir/us/core → hl7.fhir.us.core
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
        PackageCommands::Init(args) => {
            let dir = args
                .dir
                .unwrap_or_else(|| std::env::current_dir().expect("cannot determine current dir"));

            let name = match args.name {
                Some(n) => n,
                None => {
                    let derived =
                        rh_packager::name_from_canonical(&args.canonical).ok_or_else(|| {
                            anyhow::anyhow!(
                                "Could not derive a package name from '{}'. \
                             Provide --name explicitly (e.g. --name hl7.fhir.us.core).",
                                args.canonical
                            )
                        })?;
                    info!("Name derived from canonical: {derived}");
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
            if ctx.is_json() {
                let result = InitResult {
                    dir: dir.display().to_string(),
                    files_created: created
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect(),
                };
                print_envelope(ctx, &Envelope::ok(result, "package init"))?;
            } else {
                for path in &created {
                    println!("  Created: {}", path.display());
                }
                println!("\nPackage initialised at {}", dir.display());
                println!(
                    "Next: edit packager.toml, then run `rh package build {}`",
                    dir.display()
                );
            }
            Ok(())
        }
        PackageCommands::Build(args) => {
            let output_dir = args.out.unwrap_or_else(|| args.dir.join("output"));
            rh_packager::build(&args.dir, &output_dir)?;
            if ctx.is_json() {
                print_envelope(
                    ctx,
                    &Envelope::ok(serde_json::json!({ "ok": true }), "package build"),
                )?;
            }
            Ok(())
        }
        PackageCommands::Lock(args) => {
            let output_dir = args.dir.join("output");
            rh_packager::lock_package(&args.dir, &output_dir)?;
            Ok(())
        }
        PackageCommands::LockCheck(args) => {
            let report = rh_packager::check_lock(&args.dir)?;
            if ctx.is_json() {
                let result = LockCheckResult {
                    pinned: report
                        .pinned
                        .iter()
                        .map(|r| {
                            format!(
                                "{}|{} (in: {}, field: {})",
                                r.url,
                                r.pinned_version.as_deref().unwrap_or("?"),
                                r.resource_key,
                                r.field_path
                            )
                        })
                        .collect(),
                    unpinned: report
                        .unpinned
                        .iter()
                        .map(|r| {
                            format!(
                                "{} (in: {}, field: {})",
                                r.url, r.resource_key, r.field_path
                            )
                        })
                        .collect(),
                };
                print_envelope(ctx, &Envelope::ok(result, "package lock-check"))?;
            } else {
                println!("PINNED ({}):", report.pinned.len());
                for r in &report.pinned {
                    println!(
                        "  {}|{} (in: {}, field: {})",
                        r.url,
                        r.pinned_version.as_deref().unwrap_or("?"),
                        r.resource_key,
                        r.field_path
                    );
                }

                println!("\nUNPINNED ({}):", report.unpinned.len());
                for r in &report.unpinned {
                    println!(
                        "  {} (in: {}, field: {})",
                        r.url, r.resource_key, r.field_path
                    );
                }

                if !report.unpinned.is_empty() {
                    println!(
                        "\nRun `rh package lock {}` to pin all unversioned references.",
                        args.dir.display()
                    );
                }
            }
            Ok(())
        }
        PackageCommands::Check(args) => {
            rh_packager::check(&args.dir)?;
            Ok(())
        }
        PackageCommands::Pack(args) => {
            let tgz = rh_packager::pack_dir(&args.dir)?;
            if ctx.is_json() {
                print_envelope(
                    ctx,
                    &Envelope::ok(
                        serde_json::json!({ "path": tgz.to_string_lossy() }),
                        "package pack",
                    ),
                )?;
            } else {
                println!("Packed: {}", tgz.display());
            }
            Ok(())
        }
    }
}
