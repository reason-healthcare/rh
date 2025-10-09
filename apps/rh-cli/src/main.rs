use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::error;

mod codegen;
mod download;
mod ffq;
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
#[clap(long_about = "RH CLI - A comprehensive command-line toolkit for working with FHIR")]
struct Cli {
    /// Enable verbose logging
    #[clap(short, long, global = true)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate organized Rust crates from FHIR Packages
    Codegen(codegen::CodegenArgs),

    /// Download and install FHIR packages from npm-style registries
    #[clap(subcommand)]
    Download(download::DownloadCommands),

    /// Parse and evaluate FHIRPath expressions
    #[clap(subcommand)]
    Fhirpath(fhirpath::FhirpathCommands),

    /// Parse and translate FFQ (FHIR Filter Query) expressions
    #[clap(subcommand)]
    Ffq(ffq::FfqCommands),

    /// Validate FHIR resources
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
        Commands::Ffq(cmd) => {
            if let Err(e) = ffq::handle_command(ffq::FfqArgs { command: cmd }).await {
                error!("FFQ error: {}", e);
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
