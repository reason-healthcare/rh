use clap::{CommandFactory, Parser, Subcommand};
use tracing::error;

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

use output::{ColorOpt, ExitCode, Format, OutputContext};

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
        long = "output-format",
        global = true,
        default_value = "human",
        value_name = "FORMAT"
    )]
    output_format: Format,

    /// Pretty-print JSON output
    #[clap(long, global = true)]
    pretty: bool,

    /// Suppress all non-error output on stderr
    #[clap(short = 'q', long = "quiet", global = true)]
    quiet: bool,

    /// Enable verbose logging
    #[clap(short, long, global = true)]
    verbose: bool,

    /// Enable debug-level logging (overrides --verbose)
    #[clap(long, global = true)]
    debug: bool,

    /// Color output: auto (default), always, never
    #[clap(long, global = true, default_value = "auto")]
    color: ColorOpt,

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

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },
}

fn setup_tracing(cli: &Cli) {
    let level = if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let no_color = std::env::var("NO_COLOR").is_ok();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(level)
        .with_ansi(!no_color)
        .finish();

    tracing::subscriber::set_global_default(subscriber).ok();
}

fn run(cli: Cli) -> ExitCode {
    setup_tracing(&cli);

    let format = cli.output_format;
    let quiet = cli.quiet;
    let verbose = if cli.debug {
        2u8
    } else if cli.verbose {
        1u8
    } else {
        0u8
    };
    let color = cli.color;
    let pretty = cli.pretty;

    let result: anyhow::Result<ExitCode> = match cli.command {
        Commands::Codegen(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "codegen");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(codegen::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Cql(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "cql");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(cql::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Download(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "download");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(download::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Fhirpath(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "fhirpath");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(fhirpath::handle_command(cmd, &ctx))
        }
        Commands::Vcl(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "vcl");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(vcl::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Fsh(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "fsh");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(fsh::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Snapshot(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "snapshot");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(snapshot::handle_command(cmd, &ctx))
        }
        Commands::Package(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "package");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(package::handle_command(cmd, &ctx))
                .map(|()| ExitCode::Success)
        }
        Commands::Validate(cmd) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "validate");
            ctx.pretty = pretty;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(validator::handle_command(cmd, &ctx))
        }
        Commands::Completions { shell } => {
            let mut app = Cli::command();
            let bin_name = "rh".to_string();
            clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
            Ok(ExitCode::Success)
        }
    };

    match result {
        Ok(code) => code,
        Err(e) => {
            let mut ctx = OutputContext::new(format, quiet, verbose, color, "rh");
            ctx.pretty = pretty;
            let code = output::classify_error(&e);
            if let Err(write_err) = ctx.write_error(code, &e.to_string()) {
                error!("Failed to write error output: {write_err}");
            }
            code
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let exit_code = run(cli);
    std::process::exit(i32::from(exit_code));
}
