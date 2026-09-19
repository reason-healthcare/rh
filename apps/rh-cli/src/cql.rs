use anyhow::{bail, Context, Result};
use clap::Subcommand;
use glob::glob;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::{error, info};

use crate::output::{
    error_envelope, Envelope, EnvelopeError, ExitCode, OutputContext, OutputFormat,
};

use rh_cpg::context::{parse_cql_datetime, MeasurementPeriod};
use rh_cpg::fhir_to_cql::fhir_to_cql_value;
use rh_cql::analytics::{
    data_requirements, emit_measure_runtime_manifest, emit_sql_query_library, emit_sql_text,
    emit_view_definitions, format_data_requirements, format_dependencies, format_elm_inspection,
    format_lower_check, format_relational_plan, inspect_elm, lower_check, relational_plan,
    MeasureRuntimeManifest, MeasureRuntimeResultDefinition, SqlQueryLibraryArtifact,
    ViewDefinitionArtifact,
};
use rh_cql::options::CompilerOption;
use rh_cql::{
    compile, compile_to_elm_with_sourcemap_and_libraries, compile_with_libraries,
    elm::AccessModifier, evaluate_elm_with_libraries, evaluate_elm_with_libraries_and_trace,
    explain_compile, explain_parse, get_default_packages_dir, CompilationError, CompilationResult,
    CompilerOptions, CqlCode, CqlDateTime, Diagnostic, EvalContextBuilder, EvalError,
    FileLibrarySourceProvider, FixedClock, InMemoryDataProvider, InMemoryTerminologyProvider,
    PackageLibrarySourceProvider, SignatureLevel, SourceMapCompilationResult, Value,
};

#[derive(Serialize)]
struct CqlDiagnostic {
    file: String,
    valid: bool,
    errors: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmitViewsOutput {
    count: usize,
    files: Vec<String>,
    views: Vec<ViewDefinitionArtifact>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmitSqlOutput {
    sql: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    library: Option<SqlQueryLibraryArtifact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmitRuntimeOutput {
    manifest: MeasureRuntimeManifest,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
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

fn format_diagnostic_message(item: &Diagnostic, with_location: bool) -> String {
    if with_location {
        if let Some(span) = &item.span {
            return format!(
                "{} (line {}, col {})",
                item.message, span.start.line, span.start.column
            );
        }
    }
    item.message.clone()
}

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
pub enum ElmCommands {
    /// Inspect compiled ELM structure
    Inspect {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },
    /// Show expression, parameter, value set, code, and function dependencies
    Deps {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
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

        /// Include result type metadata in output
        #[clap(long)]
        result_types: bool,

        /// Enable strict mode (disable implicit conversions)
        #[clap(long)]
        strict: bool,

        /// Include all signatures in output
        #[clap(long)]
        signatures: bool,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times. The input file's directory is
        /// always searched automatically.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,

        /// Also emit a source-map sidecar file alongside the ELM output
        #[clap(long)]
        source_map: bool,

        /// Path for source-map output (defaults to <output>.sourcemap.json or stderr)
        #[clap(long, value_name = "PATH")]
        source_map_output: Option<PathBuf>,
    },

    /// Validate CQL source without generating ELM
    Validate {
        /// Path(s) to CQL file(s) or glob pattern(s), or "-" to read from stdin
        #[clap(value_name = "FILE", num_args = 0..)]
        inputs: Vec<String>,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.  The input file's directory is
        /// always searched automatically.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,

        /// Show detailed error information (error locations and annotations)
        #[clap(long)]
        details: bool,
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

    /// Inspect compiled ELM output
    #[clap(subcommand)]
    Elm(ElmCommands),

    /// Extract CQL data requirements from compiled ELM
    DataRequirements {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Build an inspectable relational plan from compiled ELM
    Plan {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Planning target label
        #[clap(long, default_value = "relational")]
        target: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Report whether compiled ELM can lower to a target
    LowerCheck {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Lowering target label
        #[clap(long, default_value = "sql-on-fhir")]
        target: String,

        /// Display format: pretty, json
        #[clap(long = "display-format", default_value = "pretty")]
        display_format: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Emit SQL-on-FHIR ViewDefinition JSON artifacts from CQL retrieves
    EmitViews {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// Output directory for generated ViewDefinition JSON files
        #[clap(long, value_name = "DIR")]
        out: PathBuf,

        /// Canonical base URL for generated ViewDefinition artifacts
        #[clap(long, default_value = "https://reason.health/rh/generated/sql-on-fhir")]
        canonical_base: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Emit a SQL-on-FHIR SQLQuery Library or raw SQL from CQL
    EmitSql {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// ViewDefinition JSON file or directory. May be specified multiple times.
        /// If omitted, views are generated in memory from the CQL retrieves.
        #[clap(long, value_name = "PATH", num_args = 1)]
        views: Vec<PathBuf>,

        /// Output file path. Defaults to stdout.
        #[clap(long, value_name = "PATH")]
        out: Option<PathBuf>,

        /// Emit only SQL text instead of a SQLQuery Library JSON artifact
        #[clap(long)]
        sql_only: bool,

        /// Canonical base URL for generated in-memory ViewDefinition artifacts
        #[clap(long, default_value = "https://reason.health/rh/generated/sql-on-fhir")]
        canonical_base: String,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Emit a ReasonHealth measure runtime manifest from generated artifacts
    EmitRuntime {
        /// Path to CQL file, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        input: String,

        /// SQLQuery Library JSON artifact path to reference from the manifest
        #[clap(long, value_name = "PATH")]
        query: PathBuf,

        /// ViewDefinition JSON file or directory. May be specified multiple times.
        #[clap(long, value_name = "PATH", num_args = 1, required = true)]
        views: Vec<PathBuf>,

        /// Output file path. Defaults to stdout.
        #[clap(long, value_name = "PATH")]
        out: Option<PathBuf>,

        /// Result mapping as name=column. May be specified multiple times.
        #[clap(long = "result", value_name = "NAME=COLUMN")]
        result: Vec<String>,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,
    },

    /// Evaluate a named expression in a compiled CQL library
    Eval {
        /// CQL file to evaluate, or "-" to read from stdin
        #[clap(value_name = "FILE")]
        file: String,

        /// Expression definition name to evaluate
        #[clap(value_name = "EXPRESSION")]
        expression: String,

        /// FHIR data file (Bundle JSON, NDJSON, or single resource) to use for
        /// Retrieve operations. Use "-" to read from stdin.
        #[clap(long, value_name = "FILE")]
        data: Option<String>,

        /// ValueSet JSON or Bundle JSON containing complete ValueSet expansions.
        /// May be specified multiple times. The input is used only for CQL
        /// terminology membership; patient data remains supplied by --data.
        #[clap(long, value_name = "FILE", num_args = 1, conflicts_with = "valuesets")]
        terminology: Vec<PathBuf>,

        /// Subject reference used to select the CQL context resource from --data.
        #[clap(long)]
        subject: Option<String>,

        /// RFC 3339 date-time or FHIR date used for deterministic CQL evaluation.
        #[clap(long)]
        evaluation_date: Option<String>,

        /// Inclusive Measurement Period start as an RFC 3339 date-time or FHIR date.
        #[clap(long, requires = "measurement_period_end")]
        measurement_period_start: Option<String>,

        /// Inclusive Measurement Period end as an RFC 3339 date-time or FHIR date.
        #[clap(long, requires = "measurement_period_start")]
        measurement_period_end: Option<String>,

        /// Additional CQL parameter as NAME=JSON. May be specified multiple times.
        #[clap(long, value_name = "NAME=JSON")]
        parameter: Vec<String>,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.  The input file's directory is
        /// always searched automatically.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,

        /// JSON file mapping value set URLs to member code lists, for
        /// terminology membership checks during evaluation.
        /// Format: {"<valueset-url>": {"codes": [{"system": "...", "code": "..."}, ...]}}
        #[clap(long, value_name = "FILE")]
        valuesets: Option<String>,

        /// Output a step-by-step evaluation trace
        #[clap(long)]
        trace: bool,
    },
}

// ---------------------------------------------------------------------------
// Command dispatcher
// ---------------------------------------------------------------------------

pub async fn handle_command(cmd: CqlCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        CqlCommands::Compile {
            input,
            output,
            compact,
            debug,
            strict,
            result_types,
            signatures,
            lib_path,
            source_map,
            source_map_output,
        } => {
            compile_cql(
                &CompileRequest {
                    input,
                    output,
                    compact,
                    debug,
                    strict,
                    result_types,
                    signatures,
                    lib_path,
                    source_map,
                    source_map_output,
                },
                ctx,
            )?;
        }
        CqlCommands::Validate {
            inputs,
            lib_path,
            details,
        } => {
            if ctx.is_json() {
                let diagnostics = validate_cql_multi_json(&inputs, &lib_path, details)?;
                let has_errors = diagnostics.iter().any(|diagnostic| !diagnostic.valid);
                print_envelope(ctx, &Envelope::ok(diagnostics, "cql validate"))?;
                if has_errors {
                    ExitCode::ValidationFailure.exit();
                }
            } else if let Err(e) = validate_cql_multi(&inputs, &lib_path, details) {
                error!("{e}");
                ExitCode::ValidationFailure.exit();
            }
        }
        CqlCommands::Info { input } => {
            show_info(&input)?;
        }
        CqlCommands::Repl { debug } => {
            rh_cql::repl::run_repl(debug)?;
        }
        CqlCommands::Explain { mode } => {
            run_explain(mode)?;
        }
        CqlCommands::Elm(mode) => {
            run_elm(mode, ctx)?;
        }
        CqlCommands::DataRequirements {
            input,
            display_format,
            lib_path,
        } => {
            run_data_requirements(&input, &display_format, &lib_path, ctx)?;
        }
        CqlCommands::Plan {
            input,
            target,
            display_format,
            lib_path,
        } => {
            run_plan(&input, &target, &display_format, &lib_path, ctx)?;
        }
        CqlCommands::LowerCheck {
            input,
            target,
            display_format,
            lib_path,
        } => {
            run_lower_check(&input, &target, &display_format, &lib_path, ctx)?;
        }
        CqlCommands::EmitViews {
            input,
            out,
            canonical_base,
            lib_path,
        } => {
            run_emit_views(&input, &out, &canonical_base, &lib_path, ctx)?;
        }
        CqlCommands::EmitSql {
            input,
            views,
            out,
            sql_only,
            canonical_base,
            lib_path,
        } => {
            run_emit_sql(
                &input,
                &views,
                out.as_deref(),
                sql_only,
                &canonical_base,
                &lib_path,
                ctx,
            )?;
        }
        CqlCommands::EmitRuntime {
            input,
            query,
            views,
            out,
            result,
            lib_path,
        } => {
            run_emit_runtime(
                &input,
                &query,
                &views,
                out.as_deref(),
                &result,
                &lib_path,
                ctx,
            )?;
        }
        CqlCommands::Eval {
            file,
            expression,
            data,
            terminology,
            subject,
            evaluation_date,
            measurement_period_start,
            measurement_period_end,
            parameter,
            lib_path,
            valuesets,
            trace,
        } => {
            eval_cql(
                &file,
                &expression,
                EvalCqlOptions {
                    data: data.as_deref(),
                    terminology_paths: &terminology,
                    subject: subject.as_deref(),
                    evaluation_date: evaluation_date.as_deref(),
                    measurement_period: measurement_period_start.zip(measurement_period_end),
                    parameters: &parameter,
                    lib_paths: &lib_path,
                    valuesets_path: valuesets.as_deref(),
                    show_trace: trace,
                },
            )?;
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

/// Expand a list of file paths / glob patterns into a sorted, deduplicated list
/// of concrete file paths.  Each entry is tried as a literal path first; if it
/// does not exist on disk it is treated as a glob pattern.
fn resolve_cql_paths(inputs: &[String]) -> Result<Vec<PathBuf>> {
    let mut resolved = BTreeSet::new();
    let mut unmatched = Vec::new();

    for input in inputs {
        let path = PathBuf::from(input);
        if path.exists() {
            resolved.insert(path);
            continue;
        }

        let mut matched_any = false;
        let entries = glob(input).with_context(|| format!("Invalid glob pattern: '{input}'"))?;
        for entry in entries {
            let entry = entry.with_context(|| format!("Invalid path for pattern: '{input}'"))?;
            if entry.is_file() {
                matched_any = true;
                resolved.insert(entry);
            }
        }

        if !matched_any {
            unmatched.push(input.clone());
        }
    }

    if !unmatched.is_empty() {
        let joined = unmatched
            .iter()
            .map(|p| format!("'{p}'"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("Input pattern matched no files: {joined}");
    }

    Ok(resolved.into_iter().collect())
}

/// Validate one or more CQL files (or stdin) specified as paths / glob patterns.
///
/// When multiple files match, each file is validated independently and a
/// summary failure is returned after all files have been processed.
fn validate_cql_multi(inputs: &[String], lib_paths: &[PathBuf], verbose: bool) -> Result<()> {
    // No inputs, or the explicit stdin sentinel "-" → validate from stdin.
    if inputs.is_empty() || (inputs.len() == 1 && inputs[0] == "-") {
        return validate_cql(
            inputs.first().map_or("-", |s| s.as_str()),
            lib_paths,
            verbose,
        );
    }

    let paths = resolve_cql_paths(inputs)?;
    let multiple = paths.len() > 1;
    let mut failure_count = 0usize;

    for path in &paths {
        let display = path.display().to_string();
        if multiple {
            println!("[{display}]");
        }
        if validate_cql(&display, lib_paths, verbose).is_err() {
            failure_count += 1;
        }
        if multiple {
            println!();
        }
    }

    if failure_count > 0 {
        bail!("{failure_count} file(s) failed CQL validation");
    }
    Ok(())
}

fn validate_cql_multi_json(
    inputs: &[String],
    lib_paths: &[PathBuf],
    verbose: bool,
) -> Result<Vec<CqlDiagnostic>> {
    if inputs.is_empty() || (inputs.len() == 1 && inputs[0] == "-") {
        return Ok(vec![validate_cql_collect(
            inputs.first().map_or("-", |s| s.as_str()),
            lib_paths,
            verbose,
        )?]);
    }

    let paths = resolve_cql_paths(inputs)?;
    paths
        .iter()
        .map(|path| validate_cql_collect(&path.display().to_string(), lib_paths, verbose))
        .collect()
}

fn validate_cql_collect(
    input: &str,
    lib_paths: &[PathBuf],
    verbose: bool,
) -> Result<CqlDiagnostic> {
    let source = read_source(input)?;
    let out = compile_with_search_dirs(&source, input, lib_paths, None)?;
    let valid = out.result.is_success();
    let (errors, _warnings) = (out.result.errors, out.result.warnings);

    Ok(CqlDiagnostic {
        file: input.to_string(),
        valid,
        errors: errors
            .iter()
            .map(|item| format_diagnostic_message(item, verbose))
            .collect(),
    })
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

// ---------------------------------------------------------------------------
// Compile service
// ---------------------------------------------------------------------------

/// Parsed flags for `cql compile`, grouped to keep [`compile_cql`] focused on
/// orchestration rather than a long parameter list.
struct CompileRequest {
    input: String,
    output: Option<PathBuf>,
    compact: bool,
    debug: bool,
    strict: bool,
    result_types: bool,
    signatures: bool,
    lib_path: Vec<PathBuf>,
    source_map: bool,
    source_map_output: Option<PathBuf>,
}

/// Read-only access to the ELM-serializable portion of a compilation outcome.
///
/// Implemented for both [`CompilationResult`] and [`SourceMapCompilationResult`]
/// so the compile pipeline can serialize either through a single code path.
trait ElmOutcome {
    fn errors(&self) -> &[Diagnostic];
    fn warnings(&self) -> &[Diagnostic];
    fn to_json(&self) -> std::result::Result<String, CompilationError>;
    fn to_compact_json(&self) -> std::result::Result<String, CompilationError>;
}

impl ElmOutcome for CompilationResult {
    fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
    fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
    }
    fn to_json(&self) -> std::result::Result<String, CompilationError> {
        CompilationResult::to_json(self)
    }
    fn to_compact_json(&self) -> std::result::Result<String, CompilationError> {
        CompilationResult::to_compact_json(self)
    }
}

impl ElmOutcome for SourceMapCompilationResult {
    fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
    fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
    }
    fn to_json(&self) -> std::result::Result<String, CompilationError> {
        SourceMapCompilationResult::to_json(self)
    }
    fn to_compact_json(&self) -> std::result::Result<String, CompilationError> {
        SourceMapCompilationResult::to_compact_json(self)
    }
}

/// Resolve the [`CompilerOptions`] for a `cql compile` run from its flags.
///
/// `debug` mode already enables result types, so `--result-types` is only
/// honored outside of debug mode to match the previous per-branch behavior.
fn build_compiler_options(
    debug: bool,
    strict: bool,
    result_types: bool,
    signatures: bool,
) -> CompilerOptions {
    let mut opts = if debug {
        CompilerOptions::debug()
    } else if strict {
        CompilerOptions::strict()
    } else {
        CompilerOptions::default()
    };
    if result_types && !debug {
        opts = opts.with_option(CompilerOption::EnableResultTypes);
    }
    if signatures {
        opts = opts.with_signature_level(SignatureLevel::All);
    }
    opts
}

fn parse_error_location(message: &str) -> Option<(usize, usize)> {
    let (_, location) = message.split_once("line ")?;
    let (line, location) = location.split_once(", column ")?;
    let column = location
        .split(|character: char| !character.is_ascii_digit())
        .next()?;
    Some((line.parse().ok()?, column.parse().ok()?))
}

fn compile_envelope_error(
    code: &'static str,
    message: impl Into<String>,
    location: Option<(usize, usize)>,
) -> EnvelopeError {
    let error = EnvelopeError::new(code, message);
    match location {
        Some((line, column)) => error.with_span(format!("{line}:{column}")),
        None => error,
    }
}

fn exit_with_compile_envelope(
    ctx: &OutputContext,
    errors: Vec<EnvelopeError>,
    human_report: impl FnOnce(),
) -> ! {
    if ctx.is_json() {
        let envelope = error_envelope(errors, "cql compile");
        if let Err(error) = print_envelope(ctx, &envelope) {
            eprintln!("error: {error}");
        }
    } else {
        human_report();
    }
    ExitCode::ParseError.exit();
}

/// Print a fatal pipeline error and exit with the parse-error code.
fn exit_on_compile_error(e: impl std::fmt::Display, ctx: &OutputContext) -> ! {
    let message = e.to_string();
    let error = compile_envelope_error("parse_error", &message, parse_error_location(&message));
    exit_with_compile_envelope(ctx, vec![error], || eprintln!("✗ {message}"));
}

/// Report a failed (non-success) compilation outcome and exit.
fn exit_on_compile_failure(outcome: &impl ElmOutcome, ctx: &OutputContext) -> ! {
    let errors = outcome
        .errors()
        .iter()
        .map(|diagnostic| {
            let location = diagnostic
                .span
                .as_ref()
                .map(|span| (span.start.line, span.start.column));
            compile_envelope_error("compilation_error", &diagnostic.message, location)
        })
        .collect();
    exit_with_compile_envelope(ctx, errors, || {
        let err = report_compile_failure(outcome.errors(), outcome.warnings());
        error!("{err}");
    });
}

/// Serialize an ELM outcome to JSON, honoring compact mode.
fn serialize_elm(outcome: &impl ElmOutcome, compact: bool) -> Result<String> {
    let json = if compact {
        outcome.to_compact_json()
    } else {
        outcome.to_json()
    }
    .context("Failed to serialize ELM to JSON")?;
    Ok(json)
}

/// Compile CQL source to ELM JSON
fn compile_cql(req: &CompileRequest, ctx: &OutputContext) -> Result<()> {
    let source = read_source(&req.input)?;
    let options = build_compiler_options(req.debug, req.strict, req.result_types, req.signatures);

    // Run the pipeline once. Source-map compilation still uses the
    // library-aware path so `--lib-path` and package-cache includes behave the
    // same as ordinary ELM compilation.
    let (json, sm_json_opt) = if req.source_map {
        let result = match compile_with_search_dirs_and_sourcemap(
            &source,
            &req.input,
            &req.lib_path,
            Some(options),
            None,
        ) {
            Ok(r) => r,
            Err(e) => exit_on_compile_error(e, ctx),
        };
        if !result.is_success() {
            exit_on_compile_failure(&result, ctx);
        }
        let json = serialize_elm(&result, req.compact)?;
        let sm_json = result
            .source_map_json()
            .context("Failed to serialize source map")?;
        (json, Some(sm_json))
    } else {
        let result =
            match compile_with_search_dirs(&source, &req.input, &req.lib_path, Some(options)) {
                Ok(r) => r,
                Err(e) => exit_on_compile_error(e, ctx),
            }
            .result;
        if !result.is_success() {
            exit_on_compile_failure(&result, ctx);
        }
        let json = serialize_elm(&result, req.compact)?;
        (json, None)
    };

    // Write ELM output
    if let Some(path) = req.output.as_deref() {
        fs::write(path, &json).with_context(|| format!("Failed to write to {}", path.display()))?;
        info!("✓ Compiled to {}", path.display());
    }
    if ctx.is_json() {
        let elm_value = serde_json::from_str::<serde_json::Value>(&json)?;
        print_envelope(ctx, &Envelope::ok(elm_value, "cql compile"))?;
    } else if req.output.is_none() {
        println!("{json}");
    }

    // Write source-map output (only present when --source-map was requested)
    if let Some(sm_json) = sm_json_opt {
        match req.source_map_output.as_deref() {
            Some(path) => {
                fs::write(path, &sm_json)
                    .with_context(|| format!("Failed to write source map to {}", path.display()))?;
                info!("✓ Source map written to {}", path.display());
            }
            None => match req.output.as_deref() {
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
// ELM analytics services
// ---------------------------------------------------------------------------

fn compile_library_for_analysis(
    input: &str,
    lib_paths: &[PathBuf],
) -> Result<rh_cql::elm::Library> {
    let source = read_source(input)?;
    let output = compile_with_search_dirs(&source, input, lib_paths, None)?;
    if !output.result.is_success() {
        return Err(report_compile_failure(
            &output.result.errors,
            &output.result.warnings,
        ));
    }
    Ok(output.result.library)
}

fn print_analysis<T: Serialize>(
    ctx: &OutputContext,
    command: &str,
    display_format: &str,
    value: &T,
    pretty: String,
) -> Result<()> {
    if ctx.is_json() {
        return print_envelope(ctx, &Envelope::ok(value, command));
    }

    match display_format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        "pretty" => {
            print!("{pretty}");
        }
        other => bail!("Unknown display format '{other}'. Valid values: pretty, json"),
    }

    Ok(())
}

fn run_elm(mode: ElmCommands, ctx: &OutputContext) -> Result<()> {
    match mode {
        ElmCommands::Inspect {
            input,
            display_format,
            lib_path,
        } => {
            let library = compile_library_for_analysis(&input, &lib_path)?;
            let inspection = inspect_elm(&library);
            print_analysis(
                ctx,
                "cql elm inspect",
                &display_format,
                &inspection,
                format_elm_inspection(&inspection),
            )?;
        }
        ElmCommands::Deps {
            input,
            display_format,
            lib_path,
        } => {
            let library = compile_library_for_analysis(&input, &lib_path)?;
            let inspection = inspect_elm(&library);
            print_analysis(
                ctx,
                "cql elm deps",
                &display_format,
                &inspection.dependencies,
                format_dependencies(&inspection),
            )?;
        }
    }

    Ok(())
}

fn run_data_requirements(
    input: &str,
    display_format: &str,
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let requirements = data_requirements(&library);
    print_analysis(
        ctx,
        "cql data-requirements",
        display_format,
        &requirements,
        format_data_requirements(&requirements),
    )
}

fn run_plan(
    input: &str,
    target: &str,
    display_format: &str,
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let plan = relational_plan(&library, target);
    print_analysis(
        ctx,
        "cql plan",
        display_format,
        &plan,
        format_relational_plan(&plan),
    )
}

fn run_lower_check(
    input: &str,
    target: &str,
    display_format: &str,
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let report = lower_check(&library, target);
    print_analysis(
        ctx,
        "cql lower-check",
        display_format,
        &report,
        format_lower_check(&report),
    )
}

fn run_emit_views(
    input: &str,
    out: &Path,
    canonical_base: &str,
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let generation = emit_view_definitions(&library, canonical_base);
    fs::create_dir_all(out)
        .with_context(|| format!("Failed to create output directory {}", out.display()))?;

    let mut files = Vec::new();
    for view in &generation.views {
        let path = out.join(format!("{}.json", view.name));
        let json = serde_json::to_string_pretty(view)?;
        fs::write(&path, json)
            .with_context(|| format!("Failed to write ViewDefinition to {}", path.display()))?;
        files.push(path.display().to_string());
    }

    let result = EmitViewsOutput {
        count: generation.views.len(),
        files,
        views: generation.views,
    };

    if ctx.is_json() {
        print_envelope(ctx, &Envelope::ok(result, "cql emit-views"))?;
    } else {
        println!(
            "Wrote {} ViewDefinition file(s) to {}",
            result.count,
            out.display()
        );
        for file in &result.files {
            println!("  - {file}");
        }
    }

    Ok(())
}

fn run_emit_sql(
    input: &str,
    view_paths: &[PathBuf],
    out: Option<&Path>,
    sql_only: bool,
    canonical_base: &str,
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let views = if view_paths.is_empty() {
        emit_view_definitions(&library, canonical_base).views
    } else {
        load_view_definitions(view_paths)?
    };

    if sql_only {
        let sql = emit_sql_text(&library, &views);
        if let Some(path) = out {
            write_output_file(path, &sql)?;
        }
        let result = EmitSqlOutput {
            sql: sql.clone(),
            library: None,
            output: out.map(|path| path.display().to_string()),
        };
        if ctx.is_json() {
            print_envelope(ctx, &Envelope::ok(result, "cql emit-sql"))?;
        } else if out.is_none() {
            print!("{sql}");
        } else if let Some(path) = out {
            println!("Wrote SQL to {}", path.display());
        }
        return Ok(());
    }

    let generation = emit_sql_query_library(&library, &views, canonical_base);
    let json = serde_json::to_string_pretty(&generation.library)?;
    if let Some(path) = out {
        write_output_file(path, &json)?;
    }

    let result = EmitSqlOutput {
        sql: generation.sql,
        library: Some(generation.library),
        output: out.map(|path| path.display().to_string()),
    };
    if ctx.is_json() {
        print_envelope(ctx, &Envelope::ok(result, "cql emit-sql"))?;
    } else if out.is_none() {
        println!("{json}");
    } else if let Some(path) = out {
        println!("Wrote SQLQuery Library to {}", path.display());
    }

    Ok(())
}

fn run_emit_runtime(
    input: &str,
    query_path: &Path,
    view_paths: &[PathBuf],
    out: Option<&Path>,
    result_specs: &[String],
    lib_paths: &[PathBuf],
    ctx: &OutputContext,
) -> Result<()> {
    let library = compile_library_for_analysis(input, lib_paths)?;
    let view_files = collect_view_definition_files(view_paths)?;
    let query_reference = manifest_path_reference(out, query_path)?;
    let view_references = view_files
        .iter()
        .map(|path| manifest_path_reference(out, path))
        .collect::<Result<Vec<_>>>()?;
    let results = parse_runtime_results(result_specs)?;
    let manifest =
        emit_measure_runtime_manifest(&library, query_reference, view_references, results);
    let json = serde_json::to_string_pretty(&manifest)?;

    if let Some(path) = out {
        write_output_file(path, &json)?;
    }

    let result = EmitRuntimeOutput {
        manifest,
        output: out.map(|path| path.display().to_string()),
    };

    if ctx.is_json() {
        print_envelope(ctx, &Envelope::ok(result, "cql emit-runtime"))?;
    } else if out.is_none() {
        println!("{json}");
    } else if let Some(path) = out {
        println!("Wrote measure runtime manifest to {}", path.display());
    }

    Ok(())
}

fn load_view_definitions(paths: &[PathBuf]) -> Result<Vec<ViewDefinitionArtifact>> {
    let files = collect_view_definition_files(paths)?;
    let mut views = Vec::new();
    for file in files {
        let contents = fs::read_to_string(&file)
            .with_context(|| format!("Failed to read ViewDefinition {}", file.display()))?;
        let view = serde_json::from_str::<ViewDefinitionArtifact>(&contents)
            .with_context(|| format!("Failed to parse ViewDefinition {}", file.display()))?;
        views.push(view);
    }

    if views.is_empty() {
        bail!("No ViewDefinition JSON files were found");
    }
    Ok(views)
}

fn collect_view_definition_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_dir() {
            let mut entries = fs::read_dir(path)
                .with_context(|| format!("Failed to read view directory {}", path.display()))?
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<std::result::Result<Vec<_>, _>>()
                .with_context(|| format!("Failed to read view directory {}", path.display()))?;
            entries.sort();
            files.extend(
                entries
                    .into_iter()
                    .filter(|entry| entry.extension().is_some_and(|ext| ext == "json")),
            );
        } else {
            files.push(path.clone());
        }
    }
    files.sort();
    files.dedup();

    if files.is_empty() {
        bail!("No ViewDefinition JSON files were found");
    }
    Ok(files)
}

fn parse_runtime_results(specs: &[String]) -> Result<Vec<MeasureRuntimeResultDefinition>> {
    let specs = if specs.is_empty() {
        vec!["initialPopulation=patient_id".to_string()]
    } else {
        specs.to_vec()
    };

    specs
        .iter()
        .map(|spec| {
            let (name, column) = spec
                .split_once('=')
                .with_context(|| format!("Invalid result mapping {spec}; expected name=column"))?;
            if name.is_empty() || column.is_empty() {
                bail!("Invalid result mapping {spec}; name and column are required");
            }
            Ok(MeasureRuntimeResultDefinition {
                name: name.to_string(),
                kind: Some("population".to_string()),
                source: "query".to_string(),
                column: column.to_string(),
            })
        })
        .collect()
}

fn manifest_path_reference(out: Option<&Path>, artifact_path: &Path) -> Result<String> {
    let cwd = std::env::current_dir().context("Failed to read current directory")?;
    let artifact = if artifact_path.is_absolute() {
        artifact_path.to_path_buf()
    } else {
        cwd.join(artifact_path)
    };

    let Some(out) = out else {
        return Ok(path_to_slash_string(artifact_path));
    };
    let out_dir = out.parent().unwrap_or_else(|| Path::new("."));
    let base = if out_dir.is_absolute() {
        out_dir.to_path_buf()
    } else {
        cwd.join(out_dir)
    };

    if let Ok(relative) = artifact.strip_prefix(&base) {
        Ok(path_to_slash_string(relative))
    } else {
        Ok(path_to_slash_string(&artifact))
    }
}

fn path_to_slash_string(path: &Path) -> String {
    path.to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

fn write_output_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create output directory {}", parent.display())
            })?;
        }
    }
    fs::write(path, contents).with_context(|| format!("Failed to write {}", path.display()))
}

// ---------------------------------------------------------------------------
// CQL include parsing helpers
// ---------------------------------------------------------------------------

/// A parsed `include` directive from CQL source.
#[derive(Debug)]
struct IncludeDirective {
    /// The raw library name/path (e.g. `fhir.cqf.common.FHIRHelpers` or `FHIRHelpers`).
    name: String,
    /// The version string if present (e.g. `4.0.1`).
    version: Option<String>,
    /// The local alias (`called <alias>`).
    alias: String,
}

impl IncludeDirective {
    /// If the name is package-qualified (last segment starts with uppercase),
    /// return the package portion (e.g. `"fhir.cqf.common"` for
    /// `"fhir.cqf.common.FHIRHelpers"`).
    fn package_name(&self) -> Option<&str> {
        let pos = self.name.rfind('.')?;
        if self.name[pos + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_uppercase())
        {
            Some(&self.name[..pos])
        } else {
            None
        }
    }
}

/// Parse all `include` directives from CQL source.
///
/// Handles both forms:
/// - `include <name> version '<ver>' called <alias>`
/// - `include <name> called <alias>`
fn parse_includes(source: &str) -> Vec<IncludeDirective> {
    let mut directives = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }
        let rest = match trimmed
            .strip_prefix("include ")
            .or_else(|| trimmed.strip_prefix("include\t"))
        {
            Some(r) => r.trim_start(),
            None => continue,
        };

        let (name, rest) = split_cql_identifier(rest);
        if name.is_empty() {
            continue;
        }
        let rest = rest.trim_start();

        let (version, rest) = if let Some(after_kw) = rest.strip_prefix("version") {
            let after_kw = after_kw.trim_start();
            if let Some(inner) = after_kw.strip_prefix('\'') {
                if let Some(end) = inner.find('\'') {
                    (Some(inner[..end].to_string()), &inner[end + 1..])
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            (None, rest)
        };

        let rest = rest.trim_start();
        let rest = match rest
            .strip_prefix("called")
            .map(str::trim_start)
            .filter(|r| {
                r.chars()
                    .next()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_')
            }) {
            Some(r) => r,
            None => continue,
        };

        let (alias, _) = split_cql_identifier(rest);
        if alias.is_empty() {
            continue;
        }

        directives.push(IncludeDirective {
            name: name.to_string(),
            version,
            alias: alias.to_string(),
        });
    }
    directives
}

/// Split a CQL identifier (alphanumeric, `.`, `_`) from the start of `s`.
fn split_cql_identifier(s: &str) -> (&str, &str) {
    let end = s
        .find(|c: char| !c.is_alphanumeric() && c != '.' && c != '_')
        .unwrap_or(s.len());
    (&s[..end], &s[end..])
}

/// Emit a warning for every declared include that was not resolved during
/// compilation.  Unresolved includes typically cause cascading false-positive
/// member-access errors, so each warning notes that downstream errors may not
/// reflect real problems in the CQL.
fn warn_unresolved_includes(
    source: &str,
    included: &std::collections::HashMap<String, rh_cql::elm::Library>,
) {
    for inc in parse_includes(source) {
        if included.contains_key(&inc.alias) {
            continue;
        }

        let install_hint = if let Some(pkg) = inc.package_name() {
            let ver = inc.version.as_deref().unwrap_or("latest");
            format!(
                "\n  Try installing the FHIR package:\n    \
                 rh download package {pkg} {ver}"
            )
        } else if let Some(ver) = &inc.version {
            format!(
                "\n  Could not find a FHIR package providing '{}' version '{}'.\n  \
                 Try: rh download package <package-name> {}",
                inc.name, ver, ver
            )
        } else {
            format!(
                "\n  Could not find a FHIR package providing '{}'.\n  \
                 Try: rh download package <package-name> <version>",
                inc.name
            )
        };

        eprintln!(
            "⚠ Warning: Library '{}' (alias '{}') could not be resolved.{}\n  \
             Member-access errors on expressions using '{}' may be false positives.",
            inc.name, inc.alias, install_hint, inc.alias
        );
    }
}

// ---------------------------------------------------------------------------
// Library-aware compile helper
// ---------------------------------------------------------------------------

fn build_library_provider(
    input: &str,
    lib_paths: &[PathBuf],
) -> (rh_cql::CompositeLibrarySourceProvider, Vec<PathBuf>) {
    let mut search_dirs: Vec<PathBuf> = Vec::new();
    if input != "-" {
        if let Some(parent) = Path::new(input).parent() {
            if !parent.as_os_str().is_empty() {
                search_dirs.push(parent.to_path_buf());
            }
        }
    }
    for path in lib_paths {
        if !search_dirs.iter().any(|existing| existing == path) {
            search_dirs.push(path.clone());
        }
    }

    let mut composite = rh_cql::CompositeLibrarySourceProvider::new();
    for dir in &search_dirs {
        composite = composite.add_provider(FileLibrarySourceProvider::new().with_path(dir));
    }
    // Always add the FHIR package cache so that includes like
    // `include fhir.cqf.common.FHIRHelpers` resolve from
    // ~/.fhir/packages without requiring --lib-path.
    if let Ok(pkg_dir) = get_default_packages_dir() {
        composite = composite.add_provider(PackageLibrarySourceProvider::new(pkg_dir));
    }

    (composite, search_dirs)
}

fn format_library_resolution_error(
    error: CompilationError,
    search_dirs: &[PathBuf],
) -> anyhow::Error {
    match error {
        CompilationError::LibraryNotFound {
            name,
            searched_paths,
        } => {
            let path_list = if !searched_paths.is_empty() {
                searched_paths
                    .iter()
                    .map(|p| format!("  - {p}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                search_dirs
                    .iter()
                    .map(|d| format!("  - {}", d.display()))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            anyhow::anyhow!(
                "Library '{}' not found.\nSearched paths:\n{}",
                name,
                path_list
            )
        }
        other => anyhow::anyhow!("{other}"),
    }
}

/// Build the set of library search directories for an input file path plus
/// any extra `--lib-path` values, then call `compile_with_libraries`.
///
/// Returns a clear, actionable error when a required include is not found,
/// listing the searched directories so the user knows where to place the file.
fn compile_with_search_dirs(
    source: &str,
    input: &str,
    lib_paths: &[PathBuf],
    options: Option<CompilerOptions>,
) -> Result<rh_cql::CompileOutputWithLibs> {
    let (provider, search_dirs) = build_library_provider(input, lib_paths);
    compile_with_libraries(source, options, &provider)
        .map_err(|e| format_library_resolution_error(e, &search_dirs))
}

fn compile_with_search_dirs_and_sourcemap(
    source: &str,
    input: &str,
    lib_paths: &[PathBuf],
    options: Option<CompilerOptions>,
    library_uri: Option<&str>,
) -> Result<rh_cql::SourceMapCompilationResult> {
    let (provider, search_dirs) = build_library_provider(input, lib_paths);
    compile_to_elm_with_sourcemap_and_libraries(source, options, library_uri, &provider)
        .map_err(|e| format_library_resolution_error(e, &search_dirs))
}

// ---------------------------------------------------------------------------
// Eval service
// ---------------------------------------------------------------------------

struct EvalCqlOptions<'a> {
    data: Option<&'a str>,
    terminology_paths: &'a [PathBuf],
    subject: Option<&'a str>,
    evaluation_date: Option<&'a str>,
    measurement_period: Option<(String, String)>,
    parameters: &'a [String],
    lib_paths: &'a [PathBuf],
    valuesets_path: Option<&'a str>,
    show_trace: bool,
}

/// Evaluate a named expression in a CQL library.
fn eval_cql(input: &str, expression: &str, options: EvalCqlOptions<'_>) -> Result<()> {
    let EvalCqlOptions {
        data,
        terminology_paths,
        subject,
        evaluation_date,
        measurement_period,
        parameters,
        lib_paths,
        valuesets_path,
        show_trace,
    } = options;
    let source = read_source(input)?;

    // Compile to ELM, resolving any included libraries.
    let output = compile_with_search_dirs(&source, input, lib_paths, None)?;
    if !output.result.is_success() {
        return Err(report_compile_failure(
            &output.result.errors,
            &output.result.warnings,
        ));
    }

    // Keep the historical current-time default while allowing a reproducible
    // clock to be supplied by fixture and automated execution callers.
    let now = evaluation_date
        .map(parse_cql_datetime)
        .transpose()?
        .unwrap_or_else(|| {
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
        });

    let mut builder = EvalContextBuilder::new(FixedClock::new(now));

    for parameter in parameters {
        let (name, json) = parameter
            .split_once('=')
            .with_context(|| format!("invalid --parameter '{parameter}': expected NAME=JSON"))?;
        if name.is_empty() {
            bail!("invalid --parameter '{parameter}': parameter name is required");
        }
        let value = serde_json::from_str(json).with_context(|| {
            format!("invalid --parameter '{parameter}': value must be valid JSON")
        })?;
        builder = builder.parameter(name, fhir_to_cql_value(&value));
    }
    if let Some((start, end)) = measurement_period {
        let period = MeasurementPeriod {
            start,
            end,
            start_inclusive: true,
            end_inclusive: true,
        };
        builder = builder.parameter("Measurement Period", period.as_cql_interval()?);
    }

    if let Some(data_path) = data {
        // Determine the declared context type (e.g. "Patient") so that
        // load_fhir_data can pick the matching resource from a Bundle and
        // set it as the context value.
        let context_type = output
            .result
            .library
            .contexts
            .as_ref()
            .and_then(|c| c.defs.first())
            .and_then(|d| d.name.as_deref());
        let (provider, context_value) = load_fhir_data(data_path, context_type, subject)?;
        builder = builder.data_provider(provider);
        if let Some(cv) = context_value {
            builder = builder.context_value(cv);
        }
    }

    if !terminology_paths.is_empty() {
        builder = builder.terminology_provider(load_terminology(terminology_paths)?);
    } else if let Some(vs_path) = valuesets_path {
        // Retain the historical compact URL-to-code map input when the newer
        // FHIR ValueSet/Bundle terminology input is not used.
        let vs_content = read_source(vs_path)?;
        let vs_json: serde_json::Value = serde_json::from_str(&vs_content)
            .context("Failed to parse value set expansions JSON")?;

        let mut term_provider = InMemoryTerminologyProvider::new();
        if let Some(obj) = vs_json.as_object() {
            for (url, entry) in obj {
                if let Some(codes) = entry.get("codes").and_then(|c| c.as_array()) {
                    for code in codes {
                        let system = code.get("system").and_then(|s| s.as_str()).unwrap_or("");
                        let code_val = code.get("code").and_then(|c| c.as_str()).unwrap_or("");
                        if !system.is_empty() && !code_val.is_empty() {
                            term_provider.add_code(
                                url,
                                rh_cql::CqlCode {
                                    code: code_val.to_string(),
                                    system: system.to_string(),
                                    display: None,
                                    version: None,
                                },
                            );
                        }
                    }
                }
            }
        }
        builder = builder.terminology_provider(term_provider);
    }

    let ctx = builder.build();
    let library = &output.result.library;
    let included = &output.included;

    if show_trace {
        // Trace events remain scoped to the main library, but evaluation must use
        // the same included-library closure as the non-trace path.
        let (value, trace) =
            evaluate_elm_with_libraries_and_trace(library, included, expression, &ctx)
                .map_err(|e| enrich_eval_error(e, expression, library))?;
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
        let value = evaluate_elm_with_libraries(library, included, expression, &ctx)
            .map_err(|e| enrich_eval_error(e, expression, library))?;
        println!("{value}");
    }

    Ok(())
}

/// Load complete FHIR ValueSet expansions for native CQL evaluation.
///
/// This mirrors the package-content terminology contract used by rh-cpg while
/// keeping clinical data (`--data`) separate from knowledge content
/// (`--terminology`). Versioned aliases are always registered. An unversioned
/// alias is available only if exactly one version of the canonical was loaded.
fn load_terminology(paths: &[PathBuf]) -> Result<InMemoryTerminologyProvider> {
    type ValueSetVersions = Vec<(Option<String>, Vec<CqlCode>)>;

    let mut valuesets: BTreeMap<String, ValueSetVersions> = BTreeMap::new();
    let mut loaded_valuesets = 0usize;

    for path in paths {
        let source = fs::read_to_string(path)
            .with_context(|| format!("Failed to read terminology input {}", path.display()))?;
        let document: serde_json::Value = serde_json::from_str(&source)
            .with_context(|| format!("Failed to parse terminology JSON {}", path.display()))?;
        let resources: Vec<&serde_json::Value> = if document
            .get("resourceType")
            .and_then(serde_json::Value::as_str)
            == Some("Bundle")
        {
            document
                .get("entry")
                .and_then(serde_json::Value::as_array)
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| entry.get("resource"))
                        .collect()
                })
                .unwrap_or_default()
        } else {
            vec![&document]
        };

        for value_set in resources {
            if value_set
                .get("resourceType")
                .and_then(serde_json::Value::as_str)
                != Some("ValueSet")
            {
                continue;
            }
            let url = value_set
                .get("url")
                .and_then(serde_json::Value::as_str)
                .with_context(|| format!("ValueSet in {} is missing url", path.display()))?;
            let expansion = value_set.get("expansion").with_context(|| {
                format!(
                    "ValueSet '{}' in {} has no complete expansion",
                    url,
                    path.display()
                )
            })?;
            if expansion.get("offset").is_some() {
                bail!(
                    "ValueSet '{}' in {} is a paged expansion; --terminology accepts only complete unfiltered expansions",
                    url,
                    path.display()
                );
            }
            // `used-codesystem` records provenance for a complete expansion;
            // it is not an expansion filter. Permit exactly one well-formed
            // canonical|version declaration and reject all other parameters.
            let used_codesystem = match expansion.get("parameter") {
                None => None,
                Some(parameters) => {
                    let parameters = parameters.as_array().with_context(|| {
                        format!(
                            "ValueSet '{}' in {} has non-array expansion.parameter",
                            url,
                            path.display()
                        )
                    })?;
                    let parameter = parameters.first().and_then(serde_json::Value::as_object);
                    if parameters.len() != 1
                        || parameter.is_none_or(|object| {
                            object.len() != 2
                                || object.get("name").and_then(serde_json::Value::as_str)
                                    != Some("used-codesystem")
                                || !object.contains_key("valueUri")
                        })
                    {
                        bail!(
                            "ValueSet '{}' in {} has filtered or unsupported expansion.parameter; --terminology accepts only a single used-codesystem provenance parameter with name and valueUri",
                            url,
                            path.display()
                        );
                    }
                    let value = parameters[0]
                        .get("valueUri")
                        .and_then(serde_json::Value::as_str)
                        .with_context(|| {
                            format!(
                                "ValueSet '{}' in {} has used-codesystem without valueUri",
                                url,
                                path.display()
                            )
                        })?;
                    let (system, version) = value
                        .split_once('|')
                        .filter(|(system, version)| {
                            !system.is_empty() && !version.is_empty() && !version.contains('|')
                        })
                        .with_context(|| {
                            format!(
                                "ValueSet '{}' in {} has used-codesystem without canonical|version",
                                url,
                                path.display()
                            )
                        })?;
                    Some((system.to_string(), version.to_string()))
                }
            };
            let contains = value_set
                .pointer("/expansion/contains")
                .and_then(serde_json::Value::as_array)
                .with_context(|| {
                    format!(
                        "ValueSet '{}' in {} has no complete expansion.contains",
                        url,
                        path.display()
                    )
                })?;
            if expansion
                .get("total")
                .and_then(serde_json::Value::as_u64)
                .is_some_and(|total| total != contains.len() as u64)
            {
                bail!(
                    "ValueSet '{}' in {} has an incomplete expansion: total does not equal contains length",
                    url,
                    path.display()
                );
            }
            let mut codes = Vec::with_capacity(contains.len());
            for concept in contains {
                if concept.get("contains").is_some() {
                    bail!(
                        "ValueSet '{}' in {} has a hierarchical expansion; --terminology accepts only flat complete expansions",
                        url,
                        path.display()
                    );
                }
                let code = concept
                    .get("code")
                    .and_then(serde_json::Value::as_str)
                    .with_context(|| {
                        format!(
                            "ValueSet '{}' in {} has expansion.contains without code",
                            url,
                            path.display()
                        )
                    })?;
                let system = concept
                    .get("system")
                    .and_then(serde_json::Value::as_str)
                    .with_context(|| {
                        format!(
                            "ValueSet '{}' in {} has expansion.contains without system",
                            url,
                            path.display()
                        )
                    })?;
                codes.push(CqlCode {
                    code: code.to_string(),
                    system: system.to_string(),
                    display: concept
                        .get("display")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string),
                    version: concept
                        .get("version")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string),
                });
            }
            if let Some((used_system, used_version)) = &used_codesystem {
                if codes.iter().any(|code| {
                    code.system != *used_system || code.version.as_deref() != Some(used_version)
                }) {
                    bail!(
                        "ValueSet '{}' in {} has used-codesystem provenance that does not match every expansion concept system/version",
                        url,
                        path.display()
                    );
                }
            }
            valuesets.entry(url.to_string()).or_default().push((
                value_set
                    .get("version")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string),
                codes,
            ));
            loaded_valuesets += 1;
        }
    }

    if loaded_valuesets == 0 {
        bail!("--terminology input did not contain any ValueSet resources");
    }

    let mut provider = InMemoryTerminologyProvider::new();
    for (url, versions) in valuesets {
        let mut seen_versions = BTreeSet::new();
        for (version, codes) in &versions {
            if !seen_versions.insert(version.clone()) {
                bail!(
                    "duplicate ValueSet canonical/version '{}|{}'",
                    url,
                    version.as_deref().unwrap_or("")
                );
            }
            if let Some(version) = version {
                provider.register_valueset(format!("{url}|{version}"), codes.clone());
            }
        }
        if versions.len() == 1 {
            provider.register_valueset(url, versions[0].1.clone());
        }
    }
    Ok(provider)
}

/// Convert an `EvalError` into an `anyhow::Error` with a helpful message.
///
/// When the error is "expression not found", the available expression names
/// are appended so the user knows what they can evaluate.
fn enrich_eval_error(
    err: EvalError,
    requested: &str,
    library: &rh_cql::elm::Library,
) -> anyhow::Error {
    let msg = err.to_string();
    if msg.contains("not found in library") {
        // Check whether the top-level requested expression actually exists in
        // the library.  When it does exist, the "not found in library" came
        // from evaluating a sub-expression (e.g. a cross-library reference
        // like `Global."Inpatient Encounter"`) — in that case we pass the
        // original error through unchanged so the user sees the real cause.
        let top_level_exists = library
            .statements
            .as_ref()
            .map(|s| {
                s.defs.iter().any(|d| match d {
                    rh_cql::elm::StatementDef::Expression(e) => {
                        e.name.as_deref() == Some(requested)
                    }
                    _ => false,
                })
            })
            .unwrap_or(false);

        if top_level_exists {
            return anyhow::anyhow!("Failed to evaluate '{}': {}", requested, msg);
        }

        // The top-level expression itself was not found — list alternatives.
        let names: Vec<String> = library
            .statements
            .as_ref()
            .map(|s| {
                s.defs
                    .iter()
                    .filter_map(|d| match d {
                        rh_cql::elm::StatementDef::Expression(e) => e.name.clone(),
                        rh_cql::elm::StatementDef::Function(_) => None,
                    })
                    .collect()
            })
            .unwrap_or_default();

        if names.is_empty() {
            anyhow::anyhow!(
                "Expression '{}' not found (library defines no expressions)",
                requested
            )
        } else {
            anyhow::anyhow!(
                "Expression '{}' not found.\n\nAvailable expressions:\n{}",
                requested,
                names
                    .iter()
                    .map(|n| format!("  - {n}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    } else {
        anyhow::anyhow!("Failed to evaluate '{}': {}", requested, msg)
    }
}

/// Load FHIR resources from a file path (or "-" for stdin) into an
/// `InMemoryDataProvider` keyed by `resourceType`.  When the input is a
/// single non-Bundle resource it is also returned as a context value so that
/// `context Patient`-style expressions work out of the box.  When the input
/// is a Bundle and `context_type` is provided (e.g. `"Patient"`), the first
/// matching resource is also returned as the context value.
fn load_fhir_data(
    path: &str,
    context_type: Option<&str>,
    subject: Option<&str>,
) -> Result<(InMemoryDataProvider, Option<Value>)> {
    let content = read_source(path)?;
    let mut provider = InMemoryDataProvider::new();
    let mut single_context: Option<Value> = None;
    let mut subject_found = subject.is_none();

    // Try NDJSON: multiple non-empty lines each being a JSON object.
    let trimmed = content.trim();
    if trimmed.contains('\n') {
        let lines: Vec<&str> = trimmed.lines().filter(|l| !l.trim().is_empty()).collect();
        let parsed: Vec<serde_json::Value> = lines
            .iter()
            .map(|l| serde_json::from_str(l))
            .collect::<std::result::Result<_, _>>()
            .ok()
            .unwrap_or_default();
        if parsed.len() == lines.len() {
            for resource in parsed {
                if subject.is_some_and(|reference| resource_matches_reference(&resource, reference))
                {
                    single_context = Some(json_to_cql_value(resource.clone()));
                    subject_found = true;
                }
                add_fhir_resource(&mut provider, resource);
            }
            if !subject_found {
                bail!(
                    "--subject '{}' was not found in --data",
                    subject.unwrap_or_default()
                );
            }
            return Ok((provider, single_context));
        }
    }

    // Fall back to single JSON document.
    let json: serde_json::Value =
        serde_json::from_str(trimmed).context("Failed to parse --data file as JSON")?;

    if json.get("resourceType").and_then(|v| v.as_str()) == Some("Bundle") {
        // FHIR Bundle — extract entries and optionally pick the context resource.
        if let Some(entries) = json.get("entry").and_then(|e| e.as_array()) {
            for entry in entries {
                if let Some(resource) = entry.get("resource") {
                    // Set context_value to the first resource whose resourceType
                    // matches the library's declared context (e.g. "Patient").
                    if subject
                        .is_some_and(|reference| resource_matches_reference(resource, reference))
                    {
                        single_context = Some(json_to_cql_value(resource.clone()));
                        subject_found = true;
                    } else if single_context.is_none() {
                        if let Some(ct) = context_type {
                            if resource.get("resourceType").and_then(|v| v.as_str()) == Some(ct) {
                                single_context = Some(json_to_cql_value(resource.clone()));
                            }
                        }
                    }
                    add_fhir_resource(&mut provider, resource.clone());
                }
            }
        }
        if !subject_found {
            bail!(
                "--subject '{}' was not found in --data",
                subject.unwrap_or_default()
            );
        }
    } else {
        // Single resource — also set it as context value.
        if subject.is_some_and(|reference| !resource_matches_reference(&json, reference)) {
            bail!(
                "--subject '{}' was not found in --data",
                subject.unwrap_or_default()
            );
        }
        let value = json_to_cql_value(json.clone());
        add_fhir_resource(&mut provider, json);
        single_context = Some(value);
    }

    Ok((provider, single_context))
}

fn resource_matches_reference(resource: &serde_json::Value, reference: &str) -> bool {
    let Some((resource_type, id)) = reference.split_once('/') else {
        return false;
    };
    resource
        .get("resourceType")
        .and_then(serde_json::Value::as_str)
        == Some(resource_type)
        && resource.get("id").and_then(serde_json::Value::as_str) == Some(id)
}

/// Recursively convert a `serde_json::Value` to a CQL `Value`.
fn json_to_cql_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else {
                Value::Decimal(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::List(arr.into_iter().map(json_to_cql_value).collect())
        }
        serde_json::Value::Object(map) => Value::Tuple(
            map.into_iter()
                .map(|(k, v)| (k, json_to_cql_value(v)))
                .collect::<BTreeMap<_, _>>(),
        ),
    }
}

/// Add a FHIR JSON resource to `provider`, keyed by its `resourceType` field.
/// Resources without a `resourceType` are silently skipped.
fn add_fhir_resource(provider: &mut InMemoryDataProvider, resource: serde_json::Value) {
    if let Some(rt) = resource.get("resourceType").and_then(|v| v.as_str()) {
        let rt = rt.to_string();
        provider.add_resource(rt, json_to_cql_value(resource));
    }
}

/// Print validation warnings to stdout. Each warning is annotated with its
/// source location only when `verbose` is true.
fn print_validation_warnings(warnings: &[Diagnostic], verbose: bool) {
    if warnings.is_empty() {
        return;
    }
    println!("\nWarnings ({}):", warnings.len());
    for warning in warnings {
        println!("  ⚠ {}", format_diagnostic_message(warning, verbose));
    }
}

/// Validate CQL source
fn validate_cql(input: &str, lib_paths: &[PathBuf], verbose: bool) -> Result<()> {
    let source = read_source(input)?;

    let out = compile_with_search_dirs(&source, input, lib_paths, None)?;
    warn_unresolved_includes(&source, &out.included);
    let valid = out.result.is_success();
    let (errors, warnings) = (out.result.errors, out.result.warnings);

    if valid {
        println!("✓ CQL is valid");
        print_validation_warnings(&warnings, verbose);
    } else {
        println!("✗ CQL has errors\n");

        println!("Errors ({}):", errors.len());
        for err in &errors {
            println!("  ✗ {}", format_diagnostic_message(err, true));
        }

        print_validation_warnings(&warnings, verbose);

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
// (REPL moved to rh-cql::repl; see crates/rh-cql/src/repl.rs)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::load_terminology;
    use rh_cql::eval::{CqlCode, TerminologyProvider};
    use std::fs;

    fn write_terminology(document: serde_json::Value) -> tempfile::NamedTempFile {
        let file = tempfile::NamedTempFile::new().expect("temporary terminology file");
        fs::write(
            file.path(),
            serde_json::to_vec(&document).expect("serialize terminology"),
        )
        .expect("write terminology");
        file
    }

    #[test]
    fn terminology_loader_rejects_partial_expansions() {
        let file = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "expansion": {
                "total": 2,
                "contains": [{ "system": "http://example.org/system", "code": "one" }]
            }
        }));

        let error = load_terminology(&[file.path().to_path_buf()])
            .expect_err("partial expansion must be rejected");
        assert!(error.to_string().contains("incomplete expansion"));
    }

    #[test]
    fn terminology_loader_rejects_inputs_without_valuesets() {
        let file = write_terminology(serde_json::json!({
            "resourceType": "Bundle",
            "type": "collection",
            "entry": [{ "resource": { "resourceType": "Patient", "id": "patient" } }]
        }));

        let error = load_terminology(&[file.path().to_path_buf()])
            .expect_err("terminology input without ValueSets must fail");
        assert!(error.to_string().contains("did not contain any ValueSet"));
    }

    #[test]
    fn terminology_loader_accepts_complete_used_codesystem_provenance() {
        let file = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "version": "2.0.0",
            "expansion": {
                "total": 1,
                "parameter": [{
                    "name": "used-codesystem",
                    "valueUri": "http://example.org/system|1.0.0"
                }],
                "contains": [{
                    "system": "http://example.org/system",
                    "version": "1.0.0",
                    "code": "one"
                }]
            }
        }));
        let provider =
            load_terminology(&[file.path().to_path_buf()]).expect("provenance expansion must load");
        assert!(provider
            .in_valueset(
                &CqlCode {
                    system: "http://example.org/system".to_string(),
                    version: Some("1.0.0".to_string()),
                    code: "one".to_string(),
                    display: None,
                },
                "http://example.org/ValueSet/test|2.0.0"
            )
            .expect("exact version is registered"));
    }

    #[test]
    fn terminology_loader_rejects_filtered_and_wrong_used_codesystem_provenance() {
        let filtered = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "expansion": {
                "parameter": [{ "name": "filter", "valueString": "active" }],
                "contains": [{ "system": "http://example.org/system", "code": "one" }]
            }
        }));
        let error = load_terminology(&[filtered.path().to_path_buf()])
            .expect_err("filter parameter must be rejected");
        assert!(error.to_string().contains("filtered or unsupported"));

        let wrong_version = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "expansion": {
                "parameter": [{
                    "name": "used-codesystem",
                    "valueUri": "http://example.org/system|2.0.0"
                }],
                "contains": [{
                    "system": "http://example.org/system",
                    "version": "1.0.0",
                    "code": "one"
                }]
            }
        }));
        let error = load_terminology(&[wrong_version.path().to_path_buf()])
            .expect_err("mismatched used-codesystem version must be rejected");
        assert!(error
            .to_string()
            .contains("does not match every expansion concept"));

        let malformed_shape = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "expansion": {
                "parameter": [{
                    "name": "used-codesystem",
                    "valueUri": "http://example.org/system|1.0.0",
                    "valueString": "must-not-be-ignored"
                }],
                "contains": [{
                    "system": "http://example.org/system",
                    "version": "1.0.0",
                    "code": "one"
                }]
            }
        }));
        let error = load_terminology(&[malformed_shape.path().to_path_buf()])
            .expect_err("used-codesystem must not contain a second value[x]");
        assert!(error.to_string().contains("filtered or unsupported"));
    }

    #[test]
    fn terminology_loader_registers_exact_versioned_valueset() {
        let file = write_terminology(serde_json::json!({
            "resourceType": "ValueSet",
            "url": "http://example.org/ValueSet/test",
            "version": "2.0.0",
            "expansion": {
                "contains": [{ "system": "http://example.org/system", "code": "one" }]
            }
        }));
        let provider =
            load_terminology(&[file.path().to_path_buf()]).expect("complete expansion must load");
        let code = CqlCode {
            system: "http://example.org/system".to_string(),
            code: "one".to_string(),
            display: None,
            version: None,
        };

        assert!(provider
            .in_valueset(&code, "http://example.org/ValueSet/test|2.0.0")
            .expect("exact version is registered"));
        assert!(provider
            .in_valueset(&code, "http://example.org/ValueSet/test|3.0.0")
            .is_err());
    }
}
