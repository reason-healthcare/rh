use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::error;

mod codegen;
mod download;
mod fhirpath;
mod validator;

/// rh - Unified CLI for FHIR processing tools
///
/// A comprehensive toolkit for working with FHIR resources, including code generation,
/// FHIRPath expression evaluation, and package management.
#[derive(Parser)]
#[clap(name = "rh")]
#[clap(about = "Unified CLI for FHIR processing tools")]
#[clap(version)]
#[clap(
    long_about = "A comprehensive toolkit for working with FHIR resources, including:\n\
    • FHIR package downloading and management\n\
    • Code generation from FHIR StructureDefinitions\n\
    • FHIRPath expression parsing and evaluation\n\
    • JSON syntax and FHIR resource validation\n\
    • Type-safe Rust code generation"
)]
struct Cli {
    /// Enable verbose logging
    #[clap(short, long, global = true)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust types from FHIR StructureDefinitions
    #[clap(subcommand)]
    Codegen(codegen::CodegenCommands),

    /// Download FHIR packages from npm-style registries
    #[clap(subcommand)]
    Download(download::DownloadCommands),

    /// Parse and evaluate FHIRPath expressions
    #[clap(subcommand)]
    Fhirpath(fhirpath::FhirpathCommands),

    /// Validate JSON syntax and FHIR resources
    #[clap(subcommand)]
    Validate(validator::ValidatorCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let subscriber = if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .finish()
    };
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Codegen(cmd) => {
            if let Err(e) = codegen::handle_command(cmd).await {
                error!("Codegen error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Download(cmd) => {
            if let Err(e) = download::handle_command(cmd).await {
                error!("Download error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Fhirpath(cmd) => {
            if let Err(e) = fhirpath::handle_command(cmd).await {
                error!("FHIRPath error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Validate(cmd) => {
            if let Err(e) = validator::handle_command(cmd).await {
                error!("Validator error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
