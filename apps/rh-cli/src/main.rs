use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::error;

mod codegen;
mod download;
mod fhirpath;
mod snapshot;
mod validator;
mod vcl;

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

    /// Parse and translate VCL (ValueSet Compose Language) expressions
    #[clap(subcommand)]
    Vcl(vcl::VclCommands),

    /// Generate and manage StructureDefinition snapshots
    #[clap(subcommand)]
    Snapshot(snapshot::SnapshotCommands),

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
            .with_writer(std::io::stderr)
            .with_max_level(tracing::Level::DEBUG)
            .finish()
    } else {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
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
        Commands::Vcl(cmd) => {
            if let Err(e) = vcl::handle_command(cmd).await {
                error!("VCL error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Snapshot(cmd) => {
            if let Err(e) = snapshot::handle_command(cmd).await {
                error!("Snapshot error: {}", e);
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
