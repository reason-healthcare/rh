use anyhow::Result;
use clap::Subcommand;
use rh_fsh::{compile_fsh_files_with_project_config, FshParser, FshTank};
use serde::Serialize;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::output::{Envelope, EnvelopeError, OutputContext, OutputFormat};

#[derive(Serialize)]
struct FshCompileOutput {
    resources: Vec<serde_json::Value>,
    diagnostics: Vec<FshDiagnostic>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    output_files: Vec<WrittenResource>,
}

#[derive(Serialize)]
struct FshDiagnostic {
    severity: &'static str,
    message: String,
}

#[derive(Serialize)]
struct WrittenResource {
    resource_type: String,
    id: String,
    file: String,
}

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
            let package = compile_fsh_files_with_project_config(&paths)?;
            let diagnostics: Vec<FshDiagnostic> = package
                .errors
                .iter()
                .map(|e| FshDiagnostic {
                    severity: "warning",
                    message: e.to_string(),
                })
                .collect();
            if !diagnostics.is_empty() {
                for e in &diagnostics {
                    warn!("{}", e.message);
                }
            }
            let mut output_files = Vec::new();
            if let Some(dir) = &output {
                std::fs::create_dir_all(dir)?;
                let mut used_names = std::collections::HashMap::new();
                for resource in &package.resources {
                    let id = resource
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let rt = resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Resource");
                    let file_name = deterministic_resource_file_name(rt, id, &mut used_names);
                    let filename = dir.join(&file_name);
                    let json = if compact {
                        serde_json::to_string(resource)?
                    } else {
                        serde_json::to_string_pretty(resource)?
                    };
                    std::fs::write(filename, json)?;
                    output_files.push(WrittenResource {
                        resource_type: rt.to_string(),
                        id: id.to_string(),
                        file: file_name,
                    });
                }
                info!(
                    "Wrote {} resources to {}",
                    package.resources.len(),
                    dir.display()
                );
            }

            if ctx.is_json() {
                print_envelope(
                    ctx,
                    &Envelope {
                        ok: true,
                        result: Some(FshCompileOutput {
                            resources: package.resources,
                            diagnostics,
                            output_files,
                        }),
                        errors: Vec::<EnvelopeError>::new(),
                        meta: crate::output::EnvelopeMeta {
                            version: env!("CARGO_PKG_VERSION").to_string(),
                            command: "fsh compile".to_string(),
                        },
                    },
                )?;
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

fn deterministic_resource_file_name(
    resource_type: &str,
    id: &str,
    used_names: &mut std::collections::HashMap<String, usize>,
) -> String {
    let base = format!(
        "{}-{}",
        sanitize_file_component(resource_type),
        sanitize_file_component(id)
    );
    let next = used_names.entry(base.clone()).or_insert(0);
    *next += 1;
    if *next == 1 {
        format!("{base}.json")
    } else {
        format!("{base}-{next}.json")
    }
}

fn sanitize_file_component(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deterministic_file_names_are_sanitized_and_disambiguated() {
        let mut used_names = HashMap::new();

        let first =
            deterministic_resource_file_name("http://example.org/Foo", "a/b", &mut used_names);
        let second =
            deterministic_resource_file_name("http://example.org/Foo", "a?b", &mut used_names);

        assert_eq!(first, "http___example.org_Foo-a_b.json");
        assert_eq!(second, "http___example.org_Foo-a_b-2.json");
    }
}
