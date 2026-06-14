use anyhow::Result;
use clap::Subcommand;
use rh_fsh::{compile_fsh_files, FshParser, FshTank};
use std::io::Read;
use std::path::PathBuf;

use crate::output::{Format, OutputContext};

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
        /// Input FSH file, or - for stdin
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
        } => handle_compile(inputs, output, compact, ctx),
        FshCommands::Parse { input } => handle_parse(input, ctx),
        FshCommands::Tank { input } => handle_tank(input, ctx),
    }
}

fn handle_compile(
    inputs: Vec<String>,
    output: Option<PathBuf>,
    compact: bool,
    ctx: &OutputContext,
) -> Result<()> {
    let paths: Vec<PathBuf> = inputs.iter().map(PathBuf::from).collect();
    let package = compile_fsh_files(&paths)?;

    if !package.errors.is_empty() {
        for e in &package.errors {
            eprintln!("warning: {e}");
        }
    }

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let resources = package.resources.clone();
            let result = serde_json::json!({
                "resource_count": package.resources.len(),
                "warning_count": package.errors.len(),
                "resources": resources,
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
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
                eprintln!(
                    "Wrote {} resources to {}",
                    package.resources.len(),
                    dir.display()
                );
            } else {
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
    }

    Ok(())
}

fn handle_parse(input: String, ctx: &OutputContext) -> Result<()> {
    let content = if input == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(&input)?
    };

    let doc = FshParser::parse(&content, &input)?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let envelope =
                crate::output::OutputEnvelope::success(serde_json::to_value(&doc)?, "fsh parse");
            let json = if ctx.pretty {
                serde_json::to_string_pretty(&envelope)?
            } else {
                serde_json::to_string(&envelope)?
            };
            println!("{json}");
        }
        Format::Human => {
            println!("{}", serde_json::to_string_pretty(&doc)?);
        }
    }

    Ok(())
}

fn handle_tank(input: String, ctx: &OutputContext) -> Result<()> {
    let content = if input == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(&input)?
    };

    let doc = FshParser::parse(&content, &input)?;
    let mut tank = FshTank::new();
    tank.add_document(doc)
        .map_err(|errs| anyhow::anyhow!("{}", errs[0]))?;

    match ctx.format {
        Format::Json | Format::Ndjson => {
            let result = serde_json::json!({
                "profiles": tank.profiles.len(),
                "extensions": tank.extensions.len(),
                "instances": tank.instances.len(),
                "value_sets": tank.value_sets.len(),
                "code_systems": tank.code_systems.len(),
            });
            ctx.write_success(result)?;
        }
        Format::Human => {
            println!("Tank summary:");
            println!("  Profiles:     {}", tank.profiles.len());
            println!("  Extensions:   {}", tank.extensions.len());
            println!("  Instances:    {}", tank.instances.len());
            println!("  ValueSets:    {}", tank.value_sets.len());
            println!("  CodeSystems:  {}", tank.code_systems.len());
        }
    }

    Ok(())
}
