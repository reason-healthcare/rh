use anyhow::Result;
use clap::Subcommand;
use rh_fsh::{compile_fsh_files, FshParser, FshTank};
use serde::Serialize;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::output::{Envelope, OutputContext, OutputFormat};

#[derive(Serialize)]
struct TankSummary {
    profiles: usize,
    extensions: usize,
    instances: usize,
    value_sets: usize,
    code_systems: usize,
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
pub enum FshCommands {
    /// Compile FSH source to FHIR JSON
    Compile {
        /// Input FSH files
        inputs: Vec<String>,
        /// Output directory (defaults to stdout)
        #[clap(short, long)]
        output: Option<PathBuf>,
        /// Output compact JSON (no pretty-printing)
        #[clap(long)]
        compact: bool,
    },
    /// Parse FSH and show the AST
    Parse {
        /// Input FSH file
        input: String,
    },
    /// Load FSH into the tank and show summary
    Tank {
        /// Input FSH file
        input: String,
    },
}

pub async fn handle_command(cmd: FshCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        FshCommands::Compile {
            inputs,
            output,
            compact,
        } => {
            let paths: Vec<PathBuf> = inputs.iter().map(PathBuf::from).collect();
            let package = compile_fsh_files(&paths)?;
            if !package.errors.is_empty() {
                for e in &package.errors {
                    warn!("{e}");
                }
            }
            if let Some(dir) = &output {
                std::fs::create_dir_all(dir)?;
                for resource in &package.resources {
                    let id = resource
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let rt = resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Resource");
                    let filename = dir.join(format!("{rt}-{id}.json"));
                    let json = if compact {
                        serde_json::to_string(resource)?
                    } else {
                        serde_json::to_string_pretty(resource)?
                    };
                    std::fs::write(filename, json)?;
                }
                info!(
                    "Wrote {} resources to {}",
                    package.resources.len(),
                    dir.display()
                );
            }

            if ctx.is_json() {
                print_envelope(ctx, &Envelope::ok(package.resources, "fsh compile"))?;
            } else if output.is_none() {
                for resource in &package.resources {
                    let json = if compact {
                        serde_json::to_string(resource)?
                    } else {
                        serde_json::to_string_pretty(resource)?
                    };
                    println!("{json}");
                }
            }
        }
        FshCommands::Parse { input } => {
            let content = std::fs::read_to_string(&input)?;
            let doc = FshParser::parse(&content, &input)?;
            if ctx.is_json() {
                print_envelope(ctx, &Envelope::ok(doc, "fsh parse"))?;
            } else {
                println!("{}", serde_json::to_string_pretty(&doc)?);
            }
        }
        FshCommands::Tank { input } => {
            let content = std::fs::read_to_string(&input)?;
            let doc = FshParser::parse(&content, &input)?;
            let mut tank = FshTank::new();
            tank.add_document(doc)
                .map_err(|errs| anyhow::anyhow!("{}", errs[0]))?;
            let summary = TankSummary {
                profiles: tank.profiles.len(),
                extensions: tank.extensions.len(),
                instances: tank.instances.len(),
                value_sets: tank.value_sets.len(),
                code_systems: tank.code_systems.len(),
            };
            if ctx.is_json() {
                print_envelope(ctx, &Envelope::ok(summary, "fsh tank"))?;
            } else {
                info!("Tank summary:");
                println!("  Profiles:     {}", summary.profiles);
                println!("  Extensions:   {}", summary.extensions);
                println!("  Instances:    {}", summary.instances);
                println!("  ValueSets:    {}", summary.value_sets);
                println!("  CodeSystems:  {}", summary.code_systems);
            }
        }
    }
    Ok(())
}
