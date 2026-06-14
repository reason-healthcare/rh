use anyhow::Result;
use clap::{Parser, Subcommand};

mod codegen;
mod cql;
mod download;
mod fhirpath;
mod fsh;
mod output;
mod package;
mod snapshot;
mod validator;
mod vcl;

use output::{error_envelope, ColorMode, EnvelopeError, ExitCode, OutputContext, OutputFormat};

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
    /// Output format: human (default), json, ndjson
    #[clap(
        long,
        global = true,
        default_value = "human",
        value_name = "FORMAT"
    )]
    format: String,

    /// Suppress informational output (stderr)
    #[clap(short, long, global = true)]
    quiet: bool,

    /// Increase verbosity (-v info, -vv debug, -vvv trace)
    #[clap(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Color output policy: auto (default), always, never
    #[clap(
        long,
        global = true,
        default_value = "auto",
        value_name = "WHEN"
    )]
    color: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate organized Rust crates from FHIR Packages
    Codegen(codegen::CodegenArgs),

    /// Compile CQL (Clinical Quality Language) to ELM
    #[clap(subcommand)]
    Cql(cql::CqlCommands),

    /// Download and install FHIR packages from npm-style registries
    #[clap(subcommand)]
    Download(download::DownloadCommands),

    /// Parse and evaluate FHIRPath expressions
    #[clap(subcommand)]
    Fhirpath(fhirpath::FhirpathCommands),

    /// Parse and translate VCL (ValueSet Compose Language) expressions
    #[clap(subcommand)]
    Vcl(vcl::VclCommands),

    /// Compile and work with FHIR Shorthand (FSH) files
    #[clap(subcommand)]
    Fsh(fsh::FshCommands),

    /// Generate and manage StructureDefinition snapshots
    #[clap(subcommand)]
    Snapshot(snapshot::SnapshotCommands),

    /// Assemble a conformant FHIR Package from a source directory
    #[clap(subcommand)]
    Package(package::PackageCommands),

    /// Validate FHIR resources
    #[clap(subcommand)]
    Validate(validator::ValidatorCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let format: OutputFormat = cli
        .format
        .parse()
        .map_err(|e: String| anyhow::anyhow!(e))?;
    let color: ColorMode = cli
        .color
        .parse()
        .map_err(|e: String| anyhow::anyhow!(e))?;

    // Build output context from global flags
    let output_ctx = OutputContext::new(format, cli.quiet, color);

    // Initialize tracing subscriber
    // verbose: 0=INFO (or WARN when quiet), 1=DEBUG, 2+=TRACE
    let max_level = if cli.quiet {
        tracing::Level::WARN
    } else {
        match cli.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        }
    };
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(max_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let result = match cli.command {
        Commands::Codegen(cmd) => codegen::handle_command(cmd, &output_ctx).await,
        Commands::Cql(cmd) => cql::handle_command(cmd, &output_ctx).await,
        Commands::Download(cmd) => download::handle_command(cmd, &output_ctx).await,
        Commands::Fhirpath(cmd) => fhirpath::handle_command(cmd, &output_ctx).await,
        Commands::Vcl(cmd) => vcl::handle_command(cmd, &output_ctx).await,
        Commands::Fsh(cmd) => fsh::handle_command(cmd, &output_ctx).await,
        Commands::Snapshot(cmd) => snapshot::handle_command(cmd, &output_ctx).await,
        Commands::Validate(cmd) => validator::handle_command(cmd, &output_ctx).await,
        Commands::Package(cmd) => package::handle_command(cmd, &output_ctx).await,
    };

    if let Err(e) = result {
        // Print `error: <message>` to stderr (no tracing timestamp/level prefix)
        eprintln!("error: {e}");
        // When JSON format is active, also emit an error envelope to stdout
        // so machine consumers always get a parseable response.
        if output_ctx.is_json() {
            let env = error_envelope(
                vec![EnvelopeError::new("operational_error", e.to_string())],
                "rh",
            );
            if let Ok(json) = serde_json::to_string_pretty(&env) {
                println!("{json}");
            }
        }
        ExitCode::OperationalError.exit();
    }

    Ok(())
}
