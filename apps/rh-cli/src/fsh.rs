use anyhow::Result;
use clap::Subcommand;
use rh_fsh::{compile_fsh_files, FshParser, FshTank};
use std::path::PathBuf;

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

pub async fn handle_command(cmd: FshCommands) -> Result<()> {
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
                    eprintln!("Warning: {e}");
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
        FshCommands::Parse { input } => {
            let content = std::fs::read_to_string(&input)?;
            let doc = FshParser::parse(&content, &input)?;
            println!("{}", serde_json::to_string_pretty(&doc)?);
        }
        FshCommands::Tank { input } => {
            let content = std::fs::read_to_string(&input)?;
            let doc = FshParser::parse(&content, &input)?;
            let mut tank = FshTank::new();
            tank.add_document(doc)
                .map_err(|errs| anyhow::anyhow!("{}", errs[0]))?;
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
