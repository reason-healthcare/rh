use anyhow::{bail, Context, Result};
use clap::Subcommand;
use glob::glob;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing::{error, info};

use rh_cql::{
    compile, compile_to_elm_with_sourcemap, compile_to_json, compile_with_libraries,
    elm::AccessModifier, evaluate_elm_with_libraries, evaluate_elm_with_trace, explain_compile,
    explain_parse, validate, CompilationError, CompilerOptions, CqlDateTime, Diagnostic,
    EvalContextBuilder, EvalError, FileLibrarySourceProvider, FixedClock, InMemoryDataProvider,
    SignatureLevel, Value,
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
        /// Path(s) to CQL file(s) or glob pattern(s), or "-" to read from stdin
        #[clap(value_name = "FILE", num_args = 0..)]
        inputs: Vec<String>,

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.  The input file's directory is
        /// always searched automatically.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,

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

        /// Additional directory to search for included CQL libraries.
        /// May be specified multiple times.  The input file's directory is
        /// always searched automatically.
        #[clap(long, value_name = "DIR", num_args = 1)]
        lib_path: Vec<PathBuf>,

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
        CqlCommands::Validate {
            inputs,
            lib_path,
            verbose,
        } => {
            validate_cql_multi(&inputs, &lib_path, verbose)?;
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
            file,
            expression,
            data,
            lib_path,
            trace,
        } => {
            eval_cql(&file, &expression, data.as_deref(), &lib_path, trace)?;
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

/// Build the set of library search directories for an input file path plus
/// any extra `--lib-path` values, then call `compile_with_libraries`.
///
/// Returns a clear, actionable error when a required include is not found,
/// listing the searched directories so the user knows where to place the file.
fn compile_with_search_dirs(
    source: &str,
    input: &str,
    lib_paths: &[PathBuf],
) -> Result<rh_cql::CompileOutputWithLibs> {
    let mut search_dirs: Vec<PathBuf> = Vec::new();
    if input != "-" {
        if let Some(parent) = Path::new(input).parent() {
            if !parent.as_os_str().is_empty() {
                search_dirs.push(parent.to_path_buf());
            }
        }
    }
    search_dirs.extend_from_slice(lib_paths);

    if search_dirs.is_empty() {
        // No search dirs — fall back to single-library compile.
        let r = compile(source, None).context("Failed to compile CQL")?;
        return Ok(rh_cql::CompileOutputWithLibs {
            result: r,
            included: std::collections::HashMap::new(),
        });
    }

    use rh_cql::CompositeLibrarySourceProvider;
    let mut composite = CompositeLibrarySourceProvider::new();
    for dir in &search_dirs {
        composite = composite.add_provider(FileLibrarySourceProvider::new().with_path(dir));
    }

    compile_with_libraries(source, None, &composite).map_err(|e| match e {
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
    })
}

// ---------------------------------------------------------------------------
// Eval service
// ---------------------------------------------------------------------------

/// Evaluate a named expression in a CQL library
fn eval_cql(
    input: &str,
    expression: &str,
    data: Option<&str>,
    lib_paths: &[PathBuf],
    show_trace: bool,
) -> Result<()> {
    let source = read_source(input)?;

    // Compile to ELM, resolving any included libraries.
    let output = compile_with_search_dirs(&source, input, lib_paths)?;
    if !output.result.is_success() {
        return Err(report_compile_failure(
            &output.result.errors,
            &output.result.warnings,
        ));
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

    let mut builder = EvalContextBuilder::new(FixedClock::new(now));

    if let Some(data_path) = data {
        let (provider, context_value) = load_fhir_data(data_path)?;
        builder = builder.data_provider(provider);
        if let Some(cv) = context_value {
            builder = builder.context_value(cv);
        }
    }

    let ctx = builder.build();
    let library = &output.result.library;
    let included = &output.included;

    if show_trace {
        // Trace is only available for the main library; use evaluate_elm_with_trace
        // for the trace view (cross-library refs will still be resolved for result).
        let (value, trace) = evaluate_elm_with_trace(library, expression, &ctx)
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
/// `context Patient`-style expressions work out of the box.
fn load_fhir_data(path: &str) -> Result<(InMemoryDataProvider, Option<Value>)> {
    let content = read_source(path)?;
    let mut provider = InMemoryDataProvider::new();
    let mut single_context: Option<Value> = None;

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
                add_fhir_resource(&mut provider, resource);
            }
            return Ok((provider, None));
        }
    }

    // Fall back to single JSON document.
    let json: serde_json::Value =
        serde_json::from_str(trimmed).context("Failed to parse --data file as JSON")?;

    if json.get("resourceType").and_then(|v| v.as_str()) == Some("Bundle") {
        // FHIR Bundle — extract entries.
        if let Some(entries) = json.get("entry").and_then(|e| e.as_array()) {
            for entry in entries {
                if let Some(resource) = entry.get("resource") {
                    add_fhir_resource(&mut provider, resource.clone());
                }
            }
        }
    } else {
        // Single resource — also set it as context value.
        let value = json_to_cql_value(json.clone());
        add_fhir_resource(&mut provider, json);
        single_context = Some(value);
    }

    Ok((provider, single_context))
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

/// Validate CQL source
fn validate_cql(input: &str, lib_paths: &[PathBuf], verbose: bool) -> Result<()> {
    let source = read_source(input)?;

    // Use the bundled validate() path (which includes FHIRHelpers and other
    // standard FHIR support libraries) unless the caller explicitly provides
    // --lib-path directories.  This matches the behavior of `rh cql compile`
    // and avoids false "Could not resolve" errors for FHIR member access when
    // FHIRHelpers.cql is not present on disk next to the input file.
    let (errors, warnings, valid) = if lib_paths.is_empty() {
        let result = validate(&source, None).context("Failed to validate CQL")?;
        let valid = result.is_valid();
        (result.errors, result.warnings, valid)
    } else {
        let out = compile_with_search_dirs(&source, input, lib_paths)?;
        warn_unresolved_includes(&source, &out.included);
        let valid = out.result.is_success();
        (out.result.errors, out.result.warnings, valid)
    };

    if valid {
        println!("✓ CQL is valid");

        if !warnings.is_empty() {
            println!("\nWarnings ({}):", warnings.len());
            for warning in &warnings {
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

        println!("Errors ({}):", errors.len());
        for err in &errors {
            if let Some(span) = &err.span {
                println!(
                    "  ✗ {} (line {}, col {})",
                    err.message, span.start.line, span.start.column
                );
            } else {
                println!("  ✗ {}", err.message);
            }
        }

        if !warnings.is_empty() {
            println!("\nWarnings ({}):", warnings.len());
            for warning in &warnings {
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
