use anyhow::{Context, Result};
use clap::Subcommand;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::{error, info};

use rh_cql::{
    compile, compile_to_json, elm::AccessModifier, validate, CompilerOptions, SignatureLevel,
};

#[derive(Subcommand)]
pub enum CqlCommands {
    /// Compile CQL source to ELM JSON
    Compile {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Output file path (defaults to stdout)
        #[clap(short, long)]
        output: Option<PathBuf>,

        /// Output compact JSON (no pretty-printing)
        #[clap(long)]
        compact: bool,

        /// Enable debug mode (annotations, locators, result types)
        #[clap(long)]
        debug: bool,

        /// Enable strict mode (disable implicit conversions)
        #[clap(long)]
        strict: bool,

        /// Include all signatures in output
        #[clap(long)]
        signatures: bool,
    },

    /// Validate CQL source without generating ELM
    Validate {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Show detailed error information
        #[clap(long)]
        verbose: bool,
    },

    /// Parse CQL and show library info
    Info {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,
    },

    /// Interactive REPL for CQL compilation
    Repl {
        /// Enable debug mode
        #[clap(long)]
        debug: bool,
    },
}

pub async fn handle_command(cmd: CqlCommands) -> Result<()> {
    match cmd {
        CqlCommands::Compile {
            input,
            output,
            compact,
            debug,
            strict,
            signatures,
        } => {
            compile_cql(
                &input,
                output.as_deref(),
                compact,
                debug,
                strict,
                signatures,
            )?;
        }
        CqlCommands::Validate { input, verbose } => {
            validate_cql(&input, verbose)?;
        }
        CqlCommands::Info { input } => {
            show_info(&input)?;
        }
        CqlCommands::Repl { debug } => {
            run_repl(debug).await?;
        }
    }

    Ok(())
}

/// Read CQL source from file or stdin
fn read_source(input: &str) -> Result<String> {
    if input == "-" {
        let mut source = String::new();
        io::stdin()
            .read_to_string(&mut source)
            .context("Failed to read from stdin")?;
        Ok(source)
    } else {
        fs::read_to_string(input).with_context(|| format!("Failed to read file: {input}"))
    }
}

/// Compile CQL source to ELM JSON
fn compile_cql(
    input: &str,
    output: Option<&Path>,
    compact: bool,
    debug: bool,
    strict: bool,
    signatures: bool,
) -> Result<()> {
    let source = read_source(input)?;

    // Build options
    let options = if debug {
        let mut opts = CompilerOptions::debug();
        if signatures {
            opts = opts.with_signature_level(SignatureLevel::All);
        }
        opts
    } else if strict {
        CompilerOptions::strict()
    } else {
        let mut opts = CompilerOptions::default();
        if signatures {
            opts = opts.with_signature_level(SignatureLevel::All);
        }
        opts
    };

    let json = compile_to_json(&source, Some(options), !compact)
        .context("Failed to compile CQL to ELM")?;

    // Write output
    if let Some(path) = output {
        fs::write(path, &json).with_context(|| format!("Failed to write to {}", path.display()))?;
        info!("✓ Compiled to {}", path.display());
    } else {
        println!("{json}");
    }

    Ok(())
}

/// Validate CQL source
fn validate_cql(input: &str, verbose: bool) -> Result<()> {
    let source = read_source(input)?;

    let result = validate(&source, None).context("Failed to validate CQL")?;

    if result.is_valid() {
        println!("✓ CQL is valid");

        if !result.warnings.is_empty() {
            println!("\nWarnings ({}):", result.warnings.len());
            for warning in &result.warnings {
                println!("  ⚠ {}", warning.message());
                if verbose {
                    if let Some(loc) = warning.locator() {
                        println!("    at line {}, column {}", loc.start_line, loc.start_char);
                    }
                }
            }
        }
    } else {
        println!("✗ CQL has errors\n");

        println!("Errors ({}):", result.errors.len());
        for err in &result.errors {
            println!("  ✗ {}", err.message());
            if verbose {
                if let Some(loc) = err.locator() {
                    println!("    at line {}, column {}", loc.start_line, loc.start_char);
                }
            }
        }

        if !result.warnings.is_empty() {
            println!("\nWarnings ({}):", result.warnings.len());
            for warning in &result.warnings {
                println!("  ⚠ {}", warning.message());
            }
        }

        std::process::exit(1);
    }

    Ok(())
}

/// Show library information
fn show_info(input: &str) -> Result<()> {
    let source = read_source(input)?;

    let result = compile(&source, None).context("Failed to compile CQL")?;

    if !result.is_success() {
        println!("✗ CQL has errors:");
        for err in &result.errors {
            println!("  {}", err.message());
        }
        std::process::exit(1);
    }

    let lib = &result.library;

    // Library identity
    if let Some(id) = &lib.identifier {
        println!(
            "Library: {} v{}",
            id.id.as_deref().unwrap_or("(unnamed)"),
            id.version.as_deref().unwrap_or("(no version)")
        );
    } else {
        println!("Library: (unnamed)");
    }

    println!();

    // Using statements
    if let Some(usings) = &lib.usings {
        if !usings.defs.is_empty() {
            println!("Using:");
            for using in &usings.defs {
                let name = using.local_identifier.as_deref().unwrap_or("?");
                let uri = using.uri.as_deref().unwrap_or("?");
                let version = using
                    .version
                    .as_deref()
                    .map(|v| format!(" v{v}"))
                    .unwrap_or_default();
                println!("  {name}{version} ({uri})");
            }
            println!();
        }
    }

    // Includes
    if let Some(includes) = &lib.includes {
        if !includes.defs.is_empty() {
            println!("Includes:");
            for inc in &includes.defs {
                let alias = inc.local_identifier.as_deref().unwrap_or("?");
                let path = inc.path.as_deref().unwrap_or("?");
                let version = inc
                    .version
                    .as_deref()
                    .map(|v| format!(" v{v}"))
                    .unwrap_or_default();
                println!("  {alias}: {path}{version}");
            }
            println!();
        }
    }

    // Terminology
    let mut has_terminology = false;

    if let Some(cs) = &lib.code_systems {
        if !cs.defs.is_empty() {
            has_terminology = true;
            println!("Code Systems ({}):", cs.defs.len());
            for def in &cs.defs {
                let name = def.name.as_deref().unwrap_or("?");
                let id = def.id.as_deref().unwrap_or("?");
                println!("  {name}: {id}");
            }
        }
    }

    if let Some(vs) = &lib.value_sets {
        if !vs.defs.is_empty() {
            has_terminology = true;
            println!("Value Sets ({}):", vs.defs.len());
            for def in &vs.defs {
                let name = def.name.as_deref().unwrap_or("?");
                let id = def.id.as_deref().unwrap_or("?");
                println!("  {name}: {id}");
            }
        }
    }

    if let Some(codes) = &lib.codes {
        if !codes.defs.is_empty() {
            has_terminology = true;
            println!("Codes ({}):", codes.defs.len());
            for def in &codes.defs {
                let name = def.name.as_deref().unwrap_or("?");
                let id = def.id.as_deref().unwrap_or("?");
                println!("  {name}: {id}");
            }
        }
    }

    if let Some(concepts) = &lib.concepts {
        if !concepts.defs.is_empty() {
            has_terminology = true;
            println!("Concepts ({}):", concepts.defs.len());
            for def in &concepts.defs {
                let name = def.name.as_deref().unwrap_or("?");
                println!("  {name}");
            }
        }
    }

    if has_terminology {
        println!();
    }

    // Parameters
    if let Some(params) = &lib.parameters {
        if !params.defs.is_empty() {
            println!("Parameters ({}):", params.defs.len());
            for param in &params.defs {
                let name = param.name.as_deref().unwrap_or("?");
                let has_default = param.default_value.is_some();
                let default_str = if has_default { " (has default)" } else { "" };
                println!("  {name}{default_str}");
            }
            println!();
        }
    }

    // Contexts
    if let Some(contexts) = &lib.contexts {
        if !contexts.defs.is_empty() {
            println!("Contexts:");
            for ctx in &contexts.defs {
                let name = ctx.name.as_deref().unwrap_or("?");
                println!("  {name}");
            }
            println!();
        }
    }

    // Statements (definitions)
    if let Some(stmts) = &lib.statements {
        if !stmts.defs.is_empty() {
            println!("Definitions ({}):", stmts.defs.len());
            for def in &stmts.defs {
                let name = def.name.as_deref().unwrap_or("?");
                let is_private = matches!(def.access_level, Some(AccessModifier::Private));
                let ctx = def
                    .context
                    .as_deref()
                    .map(|c| format!(" [{c}]"))
                    .unwrap_or_default();
                let access_marker = if is_private { " (private)" } else { "" };
                println!("  {name}{ctx}{access_marker}");
            }
        }
    }

    Ok(())
}

/// Run interactive REPL
async fn run_repl(debug: bool) -> Result<()> {
    println!("CQL Compiler REPL");
    println!("Enter CQL source (multi-line supported, end with blank line)");
    println!("Commands: :quit, :help, :debug, :compact");
    println!();

    let mut rl = DefaultEditor::new()?;
    let mut compact_output = false;
    let mut debug_mode = debug;

    loop {
        // Collect multi-line input
        let mut source = String::new();
        let mut line_num = 1;

        loop {
            let prompt = if source.is_empty() {
                "cql> ".to_string()
            } else {
                format!("{line_num:3}> ")
            };

            match rl.readline(&prompt) {
                Ok(line) => {
                    // Check for commands on first line
                    if source.is_empty() && line.starts_with(':') {
                        match line.trim() {
                            ":quit" | ":q" | ":exit" => {
                                println!("Goodbye!");
                                return Ok(());
                            }
                            ":help" | ":h" => {
                                print_repl_help();
                                break;
                            }
                            ":debug" => {
                                debug_mode = !debug_mode;
                                println!("Debug mode: {}", if debug_mode { "ON" } else { "OFF" });
                                break;
                            }
                            ":compact" => {
                                compact_output = !compact_output;
                                println!(
                                    "Compact output: {}",
                                    if compact_output { "ON" } else { "OFF" }
                                );
                                break;
                            }
                            _ => {
                                println!("Unknown command. Type :help for help.");
                                break;
                            }
                        }
                    }

                    // Empty line ends input
                    if line.is_empty() && !source.is_empty() {
                        break;
                    }

                    if !line.is_empty() {
                        if !source.is_empty() {
                            source.push('\n');
                        }
                        source.push_str(&line);
                        line_num += 1;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    source.clear();
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye!");
                    return Ok(());
                }
                Err(err) => {
                    error!("Error: {err}");
                    return Err(err.into());
                }
            }
        }

        // Skip if empty or was a command
        if source.is_empty() {
            continue;
        }

        // Compile and display
        let options = if debug_mode {
            CompilerOptions::debug()
        } else {
            CompilerOptions::default()
        };

        match compile_to_json(&source, Some(options), !compact_output) {
            Ok(json) => {
                println!("\n{json}\n");
            }
            Err(e) => {
                println!("\n✗ Error: {e}\n");
            }
        }
    }
}

fn print_repl_help() {
    println!("CQL REPL Commands:");
    println!("  :quit, :q, :exit  Exit the REPL");
    println!("  :help, :h         Show this help");
    println!("  :debug            Toggle debug mode (annotations, locators)");
    println!("  :compact          Toggle compact JSON output");
    println!();
    println!("Enter CQL source code (can be multi-line).");
    println!("Press Enter twice to compile.");
    println!();
}
