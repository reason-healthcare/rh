use anyhow::{Context, Result};
use clap::Subcommand;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::{error, info};

use rh_cql::{
    compile, compile_to_elm_with_sourcemap, compile_to_json, elm::AccessModifier, evaluate_elm,
    evaluate_elm_with_trace, explain_compile, explain_parse, validate, CompilerOptions,
    CqlDateTime, Diagnostic, EvalContextBuilder, FixedClock, SignatureLevel,
};

#[derive(Subcommand)]
pub enum ExplainMode {
    /// Show the parse tree for CQL source
    Parse {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,
    },
    /// Show semantic analysis details (resolved types, overloads, conversions)
    Compile {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,
    },
}

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

        /// Also emit a source-map sidecar file alongside the ELM output
        #[clap(long)]
        source_map: bool,

        /// Path for source-map output (defaults to <output>.sourcemap.json or stderr)
        #[clap(long, value_name = "PATH")]
        source_map_output: Option<PathBuf>,
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

    /// Explain CQL parse tree or compilation details
    Explain {
        #[clap(subcommand)]
        mode: ExplainMode,
    },

    /// Evaluate a named expression in a compiled CQL library
    Eval {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Name of the expression definition to evaluate
        #[clap(short = 'e', long)]
        expression: String,

        /// Output a step-by-step evaluation trace
        #[clap(long)]
        trace: bool,
    },
}

// ---------------------------------------------------------------------------
// Command dispatcher
// ---------------------------------------------------------------------------

pub async fn handle_command(cmd: CqlCommands) -> Result<()> {
    match cmd {
        CqlCommands::Compile {
            input,
            output,
            compact,
            debug,
            strict,
            signatures,
            source_map,
            source_map_output,
        } => {
            compile_cql(
                &input,
                output.as_deref(),
                compact,
                debug,
                strict,
                signatures,
                source_map,
                source_map_output.as_deref(),
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
        CqlCommands::Explain { mode } => {
            run_explain(mode)?;
        }
        CqlCommands::Eval {
            input,
            expression,
            trace,
        } => {
            eval_cql(&input, &expression, trace)?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Diagnostic formatting helpers
// ---------------------------------------------------------------------------

/// Print a list of compiler diagnostics to stderr.
///
/// `prefix` is the symbol shown before each item (e.g. "✗" or "⚠").
/// When `with_location` is true the source location line/column is appended
/// to the same line as the message when it is available.
fn print_diagnostic_list(items: &[Diagnostic], prefix: &str, with_location: bool) {
    for item in items {
        if with_location {
            if let Some(span) = &item.span {
                eprintln!(
                    "  {prefix} {} (line {}, col {})",
                    item.message, span.start.line, span.start.column
                );
                continue;
            }
        }
        eprintln!("  {prefix} {}", item.message);
    }
}

/// Print compile errors and warnings to stderr and return a formatted error.
///
/// Should be called when a compilation result is not successful. Returns an
/// `anyhow::Error` so callers can propagate with `return Err(...)` or `?`.
fn report_compile_failure(errors: &[Diagnostic], warnings: &[Diagnostic]) -> anyhow::Error {
    eprintln!("✗ Compilation failed with {} error(s):\n", errors.len());
    print_diagnostic_list(errors, "✗", true);
    if !warnings.is_empty() {
        eprintln!("\nWarnings ({}):", warnings.len());
        print_diagnostic_list(warnings, "⚠", false);
    }
    anyhow::anyhow!("CQL compilation failed")
}

// ---------------------------------------------------------------------------
// Source I/O
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Compile service
// ---------------------------------------------------------------------------

/// Compile CQL source to ELM JSON
#[allow(clippy::too_many_arguments)]
fn compile_cql(
    input: &str,
    output: Option<&Path>,
    compact: bool,
    debug: bool,
    strict: bool,
    signatures: bool,
    emit_source_map: bool,
    source_map_output: Option<&Path>,
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

    // Run the pipeline once.  When a source map is requested we use
    // compile_to_elm_with_sourcemap so that parse + analysis + emission happen
    // only once.
    let (json, sm_json_opt) = if emit_source_map {
        let result = match compile_to_elm_with_sourcemap(&source, Some(options), None) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("✗ {e}");
                anyhow::bail!("CQL compilation failed");
            }
        };

        if !result.is_success() {
            return Err(report_compile_failure(&result.errors, &result.warnings));
        }

        let json = if compact {
            result.to_compact_json()
        } else {
            result.to_json()
        }
        .context("Failed to serialize ELM to JSON")?;
        let sm_json = result
            .source_map_json()
            .context("Failed to serialize source map")?;
        (json, Some(sm_json))
    } else {
        let result = match compile(&source, Some(options)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("✗ {e}");
                anyhow::bail!("CQL compilation failed");
            }
        };

        if !result.is_success() {
            return Err(report_compile_failure(&result.errors, &result.warnings));
        }

        let json = if compact {
            result.to_compact_json()
        } else {
            result.to_json()
        }
        .context("Failed to serialize ELM to JSON")?;
        (json, None)
    };

    // Write ELM output
    if let Some(path) = output {
        fs::write(path, &json).with_context(|| format!("Failed to write to {}", path.display()))?;
        info!("✓ Compiled to {}", path.display());
    } else {
        println!("{json}");
    }

    // Write source-map output (only present when --source-map was requested)
    if let Some(sm_json) = sm_json_opt {
        match source_map_output {
            Some(path) => {
                fs::write(path, &sm_json)
                    .with_context(|| format!("Failed to write source map to {}", path.display()))?;
                info!("✓ Source map written to {}", path.display());
            }
            None => match output {
                Some(elm_path) => {
                    let sm_path = format!("{}.sourcemap.json", elm_path.display());
                    fs::write(&sm_path, &sm_json)
                        .with_context(|| format!("Failed to write source map to {sm_path}"))?;
                    info!("✓ Source map written to {sm_path}");
                }
                None => {
                    eprintln!("-- source map --");
                    eprintln!("{sm_json}");
                }
            },
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Explain service
// ---------------------------------------------------------------------------

/// Explain CQL parse tree or compilation
fn run_explain(mode: ExplainMode) -> Result<()> {
    match mode {
        ExplainMode::Parse { input } => {
            let source = read_source(&input)?;
            let text = explain_parse(&source).context("Failed to explain parse")?;
            println!("{text}");
        }
        ExplainMode::Compile { input } => {
            let source = read_source(&input)?;
            let text = explain_compile(&source, None).context("Failed to explain compile")?;
            println!("{text}");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Eval service
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Eval service
// ---------------------------------------------------------------------------

/// Evaluate a named expression in a CQL library
fn eval_cql(input: &str, expression: &str, show_trace: bool) -> Result<()> {
    let source = read_source(input)?;

    // Compile to ELM
    let result = compile(&source, None).context("Failed to compile CQL")?;
    if !result.is_success() {
        return Err(report_compile_failure(&result.errors, &result.warnings));
    }

    // Build a minimal EvalContext pinned to the current system time
    let now = {
        use chrono::{Datelike, Local, Timelike};
        let t = Local::now();
        CqlDateTime {
            year: t.year(),
            month: Some(t.month() as u8),
            day: Some(t.day() as u8),
            hour: Some(t.hour() as u8),
            minute: Some(t.minute() as u8),
            second: Some(t.second() as u8),
            millisecond: Some(t.timestamp_subsec_millis()),
            offset_seconds: Some(t.offset().local_minus_utc()),
        }
    };
    let ctx = EvalContextBuilder::new(FixedClock::new(now)).build();

    let library = &result.library;

    if show_trace {
        let (value, trace) = evaluate_elm_with_trace(library, expression, &ctx)
            .with_context(|| format!("Failed to evaluate expression '{expression}'"))?;
        println!("Result: {value}");
        println!();
        println!("Trace ({} events):", trace.len());
        for event in &trace {
            let node_id = event.elm_node_id.as_deref().unwrap_or("-");
            let children = if event.children.is_empty() {
                String::new()
            } else {
                format!(" children={:?}", event.children)
            };
            println!(
                "  [{}] op={} node={} inputs={:?} output={}{}",
                event.event_id, event.op, node_id, event.inputs, event.output, children
            );
        }
    } else {
        let value = evaluate_elm(library, expression, &ctx)
            .with_context(|| format!("Failed to evaluate expression '{expression}'"))?;
        println!("{value}");
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
                if verbose {
                    if let Some(span) = &warning.span {
                        println!(
                            "  ⚠ {} (line {}, col {})",
                            warning.message, span.start.line, span.start.column
                        );
                        continue;
                    }
                }
                println!("  ⚠ {}", warning.message);
            }
        }
    } else {
        println!("✗ CQL has errors\n");

        println!("Errors ({}):", result.errors.len());
        for err in &result.errors {
            if let Some(span) = &err.span {
                println!(
                    "  ✗ {} (line {}, col {})",
                    err.message, span.start.line, span.start.column
                );
            } else {
                println!("  ✗ {}", err.message);
            }
        }

        if !result.warnings.is_empty() {
            println!("\nWarnings ({}):", result.warnings.len());
            for warning in &result.warnings {
                if verbose {
                    if let Some(span) = &warning.span {
                        println!(
                            "  ⚠ {} (line {}, col {})",
                            warning.message, span.start.line, span.start.column
                        );
                        continue;
                    }
                }
                println!("  ⚠ {}", warning.message);
            }
        }

        anyhow::bail!("CQL validation failed");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Info service
// ---------------------------------------------------------------------------

/// Show library information
fn show_info(input: &str) -> Result<()> {
    let source = read_source(input)?;

    let result = compile(&source, None).context("Failed to compile CQL")?;

    if !result.is_success() {
        return Err(report_compile_failure(&result.errors, &result.warnings));
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
                match def {
                    rh_cql::elm::StatementDef::Expression(expr) => {
                        let name = expr.name.as_deref().unwrap_or("?");
                        let is_private = matches!(expr.access_level, Some(AccessModifier::Private));
                        let ctx = expr
                            .context
                            .as_deref()
                            .map(|c| format!(" [{c}]"))
                            .unwrap_or_default();
                        let access_marker = if is_private { " (private)" } else { "" };
                        println!("  {name}{ctx}{access_marker}");
                    }
                    rh_cql::elm::StatementDef::Function(func) => {
                        let name = func.name.as_deref().unwrap_or("?");
                        let is_private = matches!(func.access_level, Some(AccessModifier::Private));
                        let param_count = func.operand.len();
                        let access_marker = if is_private { " (private)" } else { "" };
                        println!("  {name}({param_count} params){access_marker}");
                    }
                }
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// REPL service
// ---------------------------------------------------------------------------

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
