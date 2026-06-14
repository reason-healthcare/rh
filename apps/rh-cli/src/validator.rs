//! Validator CLI commands
//!
//! This module provides command-line interface for FHIR resource validation.

use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use glob::glob;
use rh_validator::{FhirValidator, Severity, TerminologyConfig, ValidationOptions};
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use crate::output::{ExitCode, Format, OutputContext};

#[derive(Subcommand)]
pub enum ValidatorCommands {
    /// Validate a single FHIR resource
    Resource(ResourceArgs),
    /// Validate multiple FHIR resources from NDJSON
    Batch(BatchArgs),
}

#[derive(Args)]
pub struct ResourceArgs {
    /// Input file path(s) or glob pattern(s) (reads from stdin if not provided)
    #[clap(short, long)]
    input: Vec<String>,

    /// Output format: human, json, operationoutcome
    #[clap(short, long, default_value = "human")]
    format: ValidatorFormat,

    /// Skip invariant validation (structural validation only)
    #[clap(long)]
    skip_invariants: bool,

    /// Skip binding validation
    #[clap(long)]
    skip_bindings: bool,

    /// Exit with non-zero code if validation fails (warnings count as failure)
    #[clap(long)]
    strict: bool,

    /// Enable strict HTML/script security checks (disabled by default)
    #[clap(long)]
    security_checks: bool,

    /// Optional terminology server endpoint (e.g. https://tx.fhir.org/r4)
    #[clap(long)]
    terminology_server: Option<String>,
}

#[derive(Args)]
pub struct BatchArgs {
    /// Input NDJSON file path(s) or glob pattern(s) (reads from stdin if not provided)
    #[clap(short, long)]
    input: Vec<String>,

    /// Output format: human, json, operationoutcome
    #[clap(short, long, default_value = "human")]
    format: ValidatorFormat,

    /// Number of threads for parallel validation
    #[clap(long, default_value = "4")]
    threads: usize,

    /// Skip invariant validation (structural validation only)
    #[clap(long)]
    skip_invariants: bool,

    /// Skip binding validation
    #[clap(long)]
    skip_bindings: bool,

    /// Show summary only (hide individual issues)
    #[clap(long)]
    summary_only: bool,

    /// Exit with non-zero code if validation fails (warnings count as failure)
    #[clap(long)]
    strict: bool,

    /// Enable strict HTML/script security checks (disabled by default)
    #[clap(long)]
    security_checks: bool,

    /// Optional terminology server endpoint (e.g. https://tx.fhir.org/r4)
    #[clap(long)]
    terminology_server: Option<String>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ValidatorFormat {
    #[clap(name = "human")]
    Human,
    #[clap(name = "json")]
    Json,
    #[clap(name = "operationoutcome")]
    OperationOutcome,
}

impl std::fmt::Display for ValidatorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidatorFormat::Human => write!(f, "human"),
            ValidatorFormat::Json => write!(f, "json"),
            ValidatorFormat::OperationOutcome => write!(f, "operationoutcome"),
        }
    }
}

struct CliCtx {
    format: Format,
    pretty: bool,
    use_symbols: bool,
}

impl CliCtx {
    fn write_success(&self, value: serde_json::Value) -> Result<()> {
        match self.format {
            Format::Json => {
                let envelope = crate::output::OutputEnvelope::success(value, "validate");
                let json = if self.pretty {
                    serde_json::to_string_pretty(&envelope)?
                } else {
                    serde_json::to_string(&envelope)?
                };
                println!("{json}");
            }
            Format::Ndjson => {
                let envelope = crate::output::OutputEnvelope::success(value, "validate");
                println!("{}", serde_json::to_string(&envelope)?);
            }
            Format::Human => {
                let json = if self.pretty {
                    serde_json::to_string_pretty(&value)?
                } else {
                    serde_json::to_string(&value)?
                };
                println!("{json}");
            }
        }
        Ok(())
    }
}

impl From<&OutputContext> for CliCtx {
    fn from(ctx: &OutputContext) -> Self {
        CliCtx {
            format: ctx.format,
            pretty: ctx.pretty,
            use_symbols: ctx.should_use_symbols(),
        }
    }
}

pub async fn handle_command(cmd: ValidatorCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        ValidatorCommands::Resource(args) => handle_resource_validation(args, ctx).await,
        ValidatorCommands::Batch(args) => handle_batch_validation(args, ctx).await,
    }
}

async fn handle_resource_validation(args: ResourceArgs, ctx: &OutputContext) -> Result<()> {
    let cli = CliCtx::from(ctx);
    tokio::task::spawn_blocking(move || {
        if args.skip_invariants {
            warn!("--skip-invariants flag is not yet implemented");
        }
        if args.skip_bindings {
            warn!("--skip-bindings flag is not yet implemented");
        }

        let validator = build_validator(args.security_checks, args.terminology_server.as_deref())?;
        report_effective_runtime_options(args.security_checks, args.terminology_server.as_deref());

        let input_paths = resolve_input_paths(&args.input)?;

        if !input_paths.is_empty() {
            let mut results = Vec::new();
            for path in &input_paths {
                let content = read_input_from_file(path)
                    .with_context(|| format!("Failed to read input file: {}", path.display()))?;

                if content.trim().is_empty() {
                    warn!("Input is empty: {}", path.display());
                    continue;
                }

                let resource: serde_json::Value = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

                let result = validator.validate_auto(&resource).with_context(|| {
                    format!("Failed to validate FHIR resource in {}", path.display())
                })?;

                results.push((path.display().to_string(), resource, result));
            }

            if results.is_empty() {
                warn!("Input is empty");
                if args.strict {
                    return Err(anyhow::anyhow!("Empty input with --strict"));
                }
                return Ok(());
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            output_labeled_batch_results(&results, &args.format, &cli)?;

            if has_errors || has_issues {
                std::process::exit(i32::from(ExitCode::ValidationError));
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                return Err(anyhow::anyhow!("Empty input with --strict"));
            }
            return Ok(());
        }

        let resource: serde_json::Value =
            serde_json::from_str(&content).context("Failed to parse JSON")?;

        let result = validator
            .validate_auto(&resource)
            .context("Failed to validate FHIR resource")?;

        let has_errors = !result.valid;
        let has_issues = args.strict && !result.issues.is_empty();

        output_single_result(&result, &resource, &args.format, &cli)?;

        if has_errors || has_issues {
            std::process::exit(i32::from(ExitCode::ValidationError));
        }

        Ok(())
    })
    .await
    .context("Resource validation task failed")?
}

async fn handle_batch_validation(args: BatchArgs, ctx: &OutputContext) -> Result<()> {
    let cli = CliCtx::from(ctx);
    tokio::task::spawn_blocking(move || {
        if args.skip_invariants {
            warn!("--skip-invariants flag is not yet implemented");
        }
        if args.skip_bindings {
            warn!("--skip-bindings flag is not yet implemented");
        }

        let validator = build_validator(args.security_checks, args.terminology_server.as_deref())?;
        report_effective_runtime_options(args.security_checks, args.terminology_server.as_deref());

        let input_paths = resolve_input_paths(&args.input)?;

        if !input_paths.is_empty() {
            let mut results = Vec::new();
            for path in &input_paths {
                let content = read_input_from_file(path)
                    .with_context(|| format!("Failed to read input file: {}", path.display()))?;

                if content.trim().is_empty() {
                    warn!("Input is empty: {}", path.display());
                    continue;
                }

                for (line_num, line) in content.lines().enumerate() {
                    if line.trim().is_empty() {
                        continue;
                    }

                    let resource: serde_json::Value =
                        serde_json::from_str(line).with_context(|| {
                            format!(
                                "Failed to parse JSON in {} at line {}",
                                path.display(),
                                line_num + 1
                            )
                        })?;

                    let result = validator.validate_auto(&resource).with_context(|| {
                        format!(
                            "Failed to validate resource in {} at line {}",
                            path.display(),
                            line_num + 1
                        )
                    })?;

                    results.push((
                        format!("{}:{}", path.display(), line_num + 1),
                        resource,
                        result,
                    ));
                }
            }

            if results.is_empty() {
                warn!("Input is empty");
                if args.strict {
                    return Err(anyhow::anyhow!("Empty input with --strict"));
                }
                return Ok(());
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            output_labeled_batch_results(&results, &args.format, &cli)?;

            if has_errors || has_issues {
                std::process::exit(i32::from(ExitCode::ValidationError));
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                return Err(anyhow::anyhow!("Empty input with --strict"));
            }
            return Ok(());
        }

        let mut results = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let resource: serde_json::Value = serde_json::from_str(line)
                .with_context(|| format!("Failed to parse JSON at line {}", line_num + 1))?;
            let result = validator
                .validate_auto(&resource)
                .with_context(|| format!("Failed to validate resource at line {}", line_num + 1))?;
            results.push((line_num, resource, result));
        }

        let has_errors = results.iter().any(|(_, _, r)| !r.valid);
        let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

        output_batch_results(&results, &args.format, args.summary_only, &cli)?;

        if has_errors || has_issues {
            std::process::exit(i32::from(ExitCode::ValidationError));
        }

        Ok(())
    })
    .await
    .context("Batch validation task failed")?
}

fn symbol_ok(cli: &CliCtx) -> &'static str {
    if cli.use_symbols {
        "\u{2705}"
    } else {
        "OK"
    }
}

fn symbol_fail(cli: &CliCtx) -> &'static str {
    if cli.use_symbols {
        "\u{274C}"
    } else {
        "FAIL"
    }
}

fn symbol_warn(cli: &CliCtx) -> &'static str {
    if cli.use_symbols {
        "\u{26A0}\u{FE0F}"
    } else {
        "WARN"
    }
}

fn output_single_result(
    result: &rh_validator::ValidationResult,
    resource: &serde_json::Value,
    format: &ValidatorFormat,
    cli: &CliCtx,
) -> Result<()> {
    let resource_type = resource
        .get("resourceType")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    match format {
        ValidatorFormat::OperationOutcome => {
            let oo = result.to_operation_outcome();
            match cli.format {
                Format::Json => {
                    let envelope = crate::output::OutputEnvelope::success(
                        serde_json::to_value(&oo)?,
                        "validate resource",
                    );
                    let json = if cli.pretty {
                        serde_json::to_string_pretty(&envelope)?
                    } else {
                        serde_json::to_string(&envelope)?
                    };
                    println!("{json}");
                }
                Format::Ndjson => {
                    let envelope = crate::output::OutputEnvelope::success(
                        serde_json::to_value(&oo)?,
                        "validate resource",
                    );
                    println!("{}", serde_json::to_string(&envelope)?);
                }
                Format::Human => {
                    println!("{}", serde_json::to_string_pretty(&oo)?);
                }
            }
        }
        ValidatorFormat::Json => {
            let issues: Vec<serde_json::Value> = result
                .issues
                .iter()
                .map(|issue| {
                    serde_json::json!({
                        "severity": issue.severity.to_string(),
                        "code": issue.code.to_string(),
                        "message": issue.message,
                        "path": issue.path,
                        "location": issue.location,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "resourceType": resource_type,
                "valid": result.valid,
                "errors": result.error_count(),
                "warnings": result.warning_count(),
                "issues": issues,
            });

            match cli.format {
                Format::Json | Format::Ndjson => {
                    cli.write_success(output)?;
                }
                Format::Human => {
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        }
        ValidatorFormat::Human => {
            let sym_ok = symbol_ok(cli);
            let sym_fail = symbol_fail(cli);
            let sym_warn = symbol_warn(cli);

            if result.valid {
                println!("{sym_ok} FHIR resource is valid ({resource_type})");
                let warnings = result.warning_count();
                let info_count = result.info_count();
                if warnings > 0 {
                    println!("{sym_warn}  {warnings} warning(s)");
                }
                if info_count > 0 {
                    println!("{sym_warn}  {info_count} informational message(s)");
                }
            } else {
                println!("{sym_fail} FHIR validation failed ({resource_type})");
                println!("  Errors: {}", result.error_count());
                let warnings = result.warning_count();
                if warnings > 0 {
                    println!("  Warnings: {warnings}");
                }
                let info_count = result.info_count();
                if info_count > 0 {
                    println!("  Info: {info_count}");
                }
            }

            if !result.issues.is_empty() {
                eprintln!();
                eprintln!("Issues:");
                for (i, issue) in result.issues.iter().enumerate() {
                    let icon = match issue.severity {
                        Severity::Error => "ERROR",
                        Severity::Warning => "WARN",
                        Severity::Information => "INFO",
                    };
                    eprintln!("  {}. [{}] {}", i + 1, icon, issue.message);
                    if let Some(path) = &issue.path {
                        eprintln!("     Path: {path}");
                    }
                    if let Some(location) = &issue.location {
                        eprintln!(
                            "     Location: line {}, column {}",
                            location.line, location.column
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn output_labeled_batch_results(
    results: &[(String, serde_json::Value, rh_validator::ValidationResult)],
    format: &ValidatorFormat,
    cli: &CliCtx,
) -> Result<()> {
    match format {
        ValidatorFormat::OperationOutcome => {
            for (_, _, result) in results {
                let oo = result.to_operation_outcome();
                match cli.format {
                    Format::Json | Format::Ndjson => {
                        let envelope = crate::output::OutputEnvelope::success(
                            serde_json::to_value(&oo)?,
                            "validate batch",
                        );
                        println!("{}", serde_json::to_string(&envelope)?);
                    }
                    Format::Human => {
                        println!("{}", serde_json::to_string(&oo)?);
                    }
                }
            }
        }
        ValidatorFormat::Json => {
            let total = results.len();
            let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();

            let json_results: Vec<serde_json::Value> = results
                .iter()
                .map(|(source, resource, result)| {
                    let resource_type = resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let issues: Vec<serde_json::Value> = result
                        .issues
                        .iter()
                        .map(|issue| {
                            serde_json::json!({
                                "severity": issue.severity.to_string(),
                                "code": issue.code.to_string(),
                                "message": issue.message,
                                "path": issue.path,
                                "location": issue.location,
                            })
                        })
                        .collect();
                    serde_json::json!({
                        "source": source,
                        "resourceType": resource_type,
                        "valid": result.valid,
                        "errors": result.error_count(),
                        "warnings": result.warning_count(),
                        "issues": issues,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "summary": {
                    "total": total,
                    "valid": valid_count,
                    "invalid": total - valid_count
                },
                "results": json_results
            });

            match cli.format {
                Format::Json | Format::Ndjson => {
                    cli.write_success(output)?;
                }
                Format::Human => {
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        }
        ValidatorFormat::Human => {
            let sym_ok = symbol_ok(cli);
            let sym_fail = symbol_fail(cli);

            let total = results.len();
            let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();
            let invalid_count = total - valid_count;
            let total_errors: usize = results.iter().map(|(_, _, r)| r.error_count()).sum();
            let total_warnings: usize = results.iter().map(|(_, _, r)| r.warning_count()).sum();

            println!("Batch Validation Summary:");
            println!("  Total resources: {total}");
            println!("  {sym_ok} Valid: {valid_count}");
            println!("  {sym_fail} Invalid: {invalid_count}");
            println!("  Total errors: {total_errors}");
            println!("  Total warnings: {total_warnings}");
            for (source, resource, result) in results.iter() {
                let resource_type = resource
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                if result.valid {
                    println!("  {sym_ok} {source} ({resource_type})");
                } else {
                    println!(
                        "  {sym_fail} {source} ({resource_type}): {} error(s), {} warning(s)",
                        result.error_count(),
                        result.warning_count()
                    );
                    for issue in &result.issues {
                        if issue.severity == Severity::Error {
                            eprintln!("    ERROR: {}", issue.message);
                            if let Some(path) = &issue.path {
                                eprintln!("       at {path}");
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn output_batch_results(
    results: &[(usize, serde_json::Value, rh_validator::ValidationResult)],
    format: &ValidatorFormat,
    summary_only: bool,
    cli: &CliCtx,
) -> Result<()> {
    match format {
        ValidatorFormat::OperationOutcome => {
            for (_, _, result) in results {
                let oo = result.to_operation_outcome();
                match cli.format {
                    Format::Json | Format::Ndjson => {
                        let envelope = crate::output::OutputEnvelope::success(
                            serde_json::to_value(&oo)?,
                            "validate batch",
                        );
                        println!("{}", serde_json::to_string(&envelope)?);
                    }
                    Format::Human => {
                        println!("{}", serde_json::to_string(&oo)?);
                    }
                }
            }
        }
        ValidatorFormat::Json => {
            let total = results.len();
            let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();

            let json_results: Vec<serde_json::Value> = results
                .iter()
                .map(|(line_num, resource, result)| {
                    let resource_type = resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let issues: Vec<serde_json::Value> = result
                        .issues
                        .iter()
                        .map(|issue| {
                            serde_json::json!({
                                "severity": issue.severity.to_string(),
                                "code": issue.code.to_string(),
                                "message": issue.message,
                                "path": issue.path,
                                "location": issue.location,
                            })
                        })
                        .collect();
                    serde_json::json!({
                        "line": line_num + 1,
                        "resourceType": resource_type,
                        "valid": result.valid,
                        "errors": result.error_count(),
                        "warnings": result.warning_count(),
                        "issues": issues,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "summary": {
                    "total": total,
                    "valid": valid_count,
                    "invalid": total - valid_count
                },
                "results": json_results
            });

            match cli.format {
                Format::Json | Format::Ndjson => {
                    cli.write_success(output)?;
                }
                Format::Human => {
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        }
        ValidatorFormat::Human => {
            let sym_ok = symbol_ok(cli);
            let sym_fail = symbol_fail(cli);

            let total = results.len();
            let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();
            let invalid_count = total - valid_count;
            let total_errors: usize = results.iter().map(|(_, _, r)| r.error_count()).sum();
            let total_warnings: usize = results.iter().map(|(_, _, r)| r.warning_count()).sum();

            println!("Batch Validation Summary:");
            println!("  Total resources: {total}");
            println!("  {sym_ok} Valid: {valid_count}");
            println!("  {sym_fail} Invalid: {invalid_count}");
            println!("  Total errors: {total_errors}");
            println!("  Total warnings: {total_warnings}");
            println!();

            if !summary_only {
                for (line_num, resource, result) in results.iter() {
                    let resource_type = resource
                        .get("resourceType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    if result.valid {
                        println!("  Line {} ({}): valid", line_num + 1, resource_type);
                    } else {
                        let errors = result.error_count();
                        let warnings = result.warning_count();
                        println!(
                            "  Line {} ({}): {} error(s), {} warning(s)",
                            line_num + 1,
                            resource_type,
                            errors,
                            warnings
                        );
                        for issue in &result.issues {
                            if issue.severity == Severity::Error {
                                eprintln!("    ERROR: {}", issue.message);
                                if let Some(path) = &issue.path {
                                    eprintln!("       at {path}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn build_validator(
    security_checks: bool,
    terminology_server: Option<&str>,
) -> Result<FhirValidator> {
    let options = ValidationOptions { security_checks };
    let terminology_config = terminology_server.map(TerminologyConfig::with_server);

    FhirValidator::with_options(
        rh_validator::FhirVersion::R4,
        None,
        terminology_config,
        options,
    )
}

fn report_effective_runtime_options(security_checks: bool, terminology_server: Option<&str>) {
    info!(
        security_checks,
        terminology_configured = terminology_server.is_some(),
        terminology_server = terminology_server.unwrap_or("<none>"),
        "Validator runtime options"
    );
}

fn read_input_from_stdin() -> Result<String> {
    info!("Reading from stdin");
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;
    Ok(buffer)
}

fn read_input_from_file(path: &Path) -> Result<String> {
    info!("Reading from file: {}", path.display());
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
}

fn resolve_input_paths(inputs: &[String]) -> Result<Vec<PathBuf>> {
    if inputs.is_empty() {
        return Ok(Vec::new());
    }

    let mut resolved = BTreeSet::new();
    let mut unmatched_patterns = Vec::new();

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
            unmatched_patterns.push(input.clone());
        }
    }

    if !unmatched_patterns.is_empty() {
        let joined = unmatched_patterns
            .iter()
            .map(|pattern| format!("'{pattern}'"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("Input pattern matched no files: {joined}");
    }

    Ok(resolved.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_format_display() {
        assert_eq!(ValidatorFormat::Human.to_string(), "human");
        assert_eq!(ValidatorFormat::Json.to_string(), "json");
        assert_eq!(
            ValidatorFormat::OperationOutcome.to_string(),
            "operationoutcome"
        );
    }

    #[test]
    fn test_validator_format_clone() {
        let fmt = ValidatorFormat::Json;
        let fmt2 = fmt.clone();
        assert_eq!(fmt.to_string(), fmt2.to_string());
    }

    #[test]
    fn test_resolve_input_paths_empty() {
        let paths = resolve_input_paths(&[]).unwrap();
        assert!(paths.is_empty());
    }

    #[test]
    fn test_resolve_input_paths_nonexistent() {
        let result = resolve_input_paths(&["/nonexistent/path/to/file.json".to_string()]);
        assert!(result.is_err());
    }
}
