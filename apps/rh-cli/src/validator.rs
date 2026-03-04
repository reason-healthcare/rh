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

    /// Output format (text, json)
    #[clap(short, long, default_value = "text")]
    format: OutputFormat,

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

    /// Output format (text, json)
    #[clap(short, long, default_value = "text")]
    format: OutputFormat,

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

#[derive(Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
    OperationOutcome,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "operationoutcome" => Ok(OutputFormat::OperationOutcome),
            _ => Err(format!(
                "Invalid output format: {s}. Valid options: text, json, operationoutcome"
            )),
        }
    }
}

/// Handle validator commands
pub async fn handle_command(cmd: ValidatorCommands) -> Result<()> {
    match cmd {
        ValidatorCommands::Resource(args) => handle_resource_validation(args).await,
        ValidatorCommands::Batch(args) => handle_batch_validation(args).await,
    }
}

async fn handle_resource_validation(args: ResourceArgs) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        // Note: skip_invariants and skip_bindings are not yet supported in the new API
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
                    std::process::exit(1);
                }
                return Ok(());
            }

            if results.len() == 1 {
                let (_, resource, result) = &results[0];
                match args.format {
                    OutputFormat::Text => print_single_result_text(result, resource),
                    OutputFormat::Json => print_single_result_json(result, resource)?,
                    OutputFormat::OperationOutcome => print_operation_outcome(result)?,
                }
            } else {
                match args.format {
                    OutputFormat::Text => print_labeled_batch_results_text(&results, false),
                    OutputFormat::Json => print_labeled_batch_results_json(&results)?,
                    OutputFormat::OperationOutcome => {
                        print_labeled_batch_operation_outcomes(&results)?
                    }
                }
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            if has_errors || has_issues {
                std::process::exit(1);
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                std::process::exit(1);
            }
            return Ok(());
        }

        let resource: serde_json::Value =
            serde_json::from_str(&content).context("Failed to parse JSON")?;

        let result = validator
            .validate_auto(&resource)
            .context("Failed to validate FHIR resource")?;

        match args.format {
            OutputFormat::Text => print_single_result_text(&result, &resource),
            OutputFormat::Json => print_single_result_json(&result, &resource)?,
            OutputFormat::OperationOutcome => print_operation_outcome(&result)?,
        }

        let has_errors = !result.valid;
        let has_issues = args.strict && !result.issues.is_empty();

        if has_errors || has_issues {
            std::process::exit(1);
        }

        Ok(())
    })
    .await
    .context("Resource validation task failed")?
}

async fn handle_batch_validation(args: BatchArgs) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        // Note: skip_invariants and skip_bindings are not yet supported in the new API
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
                    std::process::exit(1);
                }
                return Ok(());
            }

            match args.format {
                OutputFormat::Text => print_labeled_batch_results_text(&results, args.summary_only),
                OutputFormat::Json => print_labeled_batch_results_json(&results)?,
                OutputFormat::OperationOutcome => print_labeled_batch_operation_outcomes(&results)?,
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            if has_errors || has_issues {
                std::process::exit(1);
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                std::process::exit(1);
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

        match args.format {
            OutputFormat::Text => print_batch_results_text(&results, args.summary_only),
            OutputFormat::Json => print_batch_results_json(&results)?,
            OutputFormat::OperationOutcome => print_batch_operation_outcomes(&results)?,
        }

        let has_errors = results.iter().any(|(_, _, r)| !r.valid);
        let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

        if has_errors || has_issues {
            std::process::exit(1);
        }

        Ok(())
    })
    .await
    .context("Batch validation task failed")?
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

fn print_single_result_text(result: &rh_validator::ValidationResult, resource: &serde_json::Value) {
    let errors = result.error_count();
    let warnings = result.warning_count();
    let info_count = result.info_count();

    let resource_type = resource
        .get("resourceType")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    if result.valid {
        println!("✅ FHIR resource is valid ({resource_type})");
        if warnings > 0 {
            println!("⚠️  {warnings} warning(s)");
        }
        if info_count > 0 {
            println!("ℹ️  {info_count} informational message(s)");
        }
    } else {
        println!("❌ FHIR validation failed ({resource_type})");
        println!("  Errors: {errors}");
        if warnings > 0 {
            println!("  Warnings: {warnings}");
        }
        if info_count > 0 {
            println!("  Info: {info_count}");
        }
    }

    if !result.issues.is_empty() {
        println!();
        println!("Issues:");
        for (i, issue) in result.issues.iter().enumerate() {
            let icon = match issue.severity {
                Severity::Error => "❌",
                Severity::Warning => "⚠️ ",
                Severity::Information => "ℹ️ ",
            };
            println!("  {}. {} {}", i + 1, icon, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
            if let Some(location) = &issue.location {
                println!(
                    "     Location: line {}, column {}",
                    location.line, location.column
                );
            }
        }
    }
}

fn print_single_result_json(
    result: &rh_validator::ValidationResult,
    resource: &serde_json::Value,
) -> Result<()> {
    let resource_type = resource
        .get("resourceType")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    let output = serde_json::json!({
        "resourceType": resource_type,
        "valid": result.valid,
        "errors": result.error_count(),
        "warnings": result.warning_count(),
        "issues": result.issues.iter().map(|issue| {
            serde_json::json!({
                "severity": issue.severity.to_string(),
                "code": issue.code.to_string(),
                "message": issue.message,
                "path": issue.path,
                "location": issue.location,
            })
        }).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_batch_results_text(
    results: &[(usize, serde_json::Value, rh_validator::ValidationResult)],
    summary_only: bool,
) {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();
    let invalid_count = total - valid_count;
    let total_errors: usize = results.iter().map(|(_, _, r)| r.error_count()).sum();
    let total_warnings: usize = results.iter().map(|(_, _, r)| r.warning_count()).sum();

    println!("📋 Batch Validation Summary:");
    println!("  Total resources: {total}");
    println!("  ✅ Valid: {valid_count}");
    println!("  ❌ Invalid: {invalid_count}");
    println!("  Total errors: {total_errors}");
    println!("  Total warnings: {total_warnings}");
    println!();

    if !summary_only && invalid_count > 0 {
        println!("❌ Invalid resources:");
        for (line_num, resource, result) in results.iter() {
            if !result.valid {
                let resource_type = resource
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
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
                        println!("    ❌ {}", issue.message);
                        if let Some(path) = &issue.path {
                            println!("       at {path}");
                        }
                    }
                }
            }
        }
    }
}

fn print_batch_results_json(
    results: &[(usize, serde_json::Value, rh_validator::ValidationResult)],
) -> Result<()> {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();

    let json_results: Vec<_> = results
        .iter()
        .map(|(line_num, resource, result)| {
            let resource_type = resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            serde_json::json!({
                "line": line_num + 1,
                "resourceType": resource_type,
                "valid": result.valid,
                "errors": result.error_count(),
                "warnings": result.warning_count(),
                "issues": result.issues.iter().map(|issue| {
                    serde_json::json!({
                        "severity": issue.severity.to_string(),
                        "code": issue.code.to_string(),
                        "message": issue.message,
                        "path": issue.path,
                        "location": issue.location,
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect();

    let summary = serde_json::json!({
        "summary": {
            "total": total,
            "valid": valid_count,
            "invalid": total - valid_count
        },
        "results": json_results
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn print_labeled_batch_results_text(
    results: &[(String, serde_json::Value, rh_validator::ValidationResult)],
    summary_only: bool,
) {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();
    let invalid_count = total - valid_count;
    let total_errors: usize = results.iter().map(|(_, _, r)| r.error_count()).sum();
    let total_warnings: usize = results.iter().map(|(_, _, r)| r.warning_count()).sum();

    println!("📋 Batch Validation Summary:");
    println!("  Total resources: {total}");
    println!("  ✅ Valid: {valid_count}");
    println!("  ❌ Invalid: {invalid_count}");
    println!("  Total errors: {total_errors}");
    println!("  Total warnings: {total_warnings}");
    println!();

    if !summary_only && invalid_count > 0 {
        println!("❌ Invalid resources:");
        for (source, resource, result) in results.iter() {
            if !result.valid {
                let resource_type = resource
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let errors = result.error_count();
                let warnings = result.warning_count();
                println!(
                    "  {} ({}): {} error(s), {} warning(s)",
                    source, resource_type, errors, warnings
                );
                for issue in &result.issues {
                    if issue.severity == Severity::Error {
                        println!("    ❌ {}", issue.message);
                        if let Some(path) = &issue.path {
                            println!("       at {path}");
                        }
                    }
                }
            }
        }
    }
}

fn print_labeled_batch_results_json(
    results: &[(String, serde_json::Value, rh_validator::ValidationResult)],
) -> Result<()> {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, _, r)| r.valid).count();

    let json_results: Vec<_> = results
        .iter()
        .map(|(source, resource, result)| {
            let resource_type = resource
                .get("resourceType")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            serde_json::json!({
                "source": source,
                "resourceType": resource_type,
                "valid": result.valid,
                "errors": result.error_count(),
                "warnings": result.warning_count(),
                "issues": result.issues.iter().map(|issue| {
                    serde_json::json!({
                        "severity": issue.severity.to_string(),
                        "code": issue.code.to_string(),
                        "message": issue.message,
                        "path": issue.path,
                        "location": issue.location,
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect();

    let summary = serde_json::json!({
        "summary": {
            "total": total,
            "valid": valid_count,
            "invalid": total - valid_count
        },
        "results": json_results
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn print_operation_outcome(result: &rh_validator::ValidationResult) -> Result<()> {
    let operation_outcome = result.to_operation_outcome();
    println!("{}", serde_json::to_string_pretty(&operation_outcome)?);
    Ok(())
}

fn print_batch_operation_outcomes(
    results: &[(usize, serde_json::Value, rh_validator::ValidationResult)],
) -> Result<()> {
    for (_, _, result) in results {
        let operation_outcome = result.to_operation_outcome();
        println!("{}", serde_json::to_string(&operation_outcome)?);
    }
    Ok(())
}

fn print_labeled_batch_operation_outcomes(
    results: &[(String, serde_json::Value, rh_validator::ValidationResult)],
) -> Result<()> {
    for (_, _, result) in results {
        let operation_outcome = result.to_operation_outcome();
        println!("{}", serde_json::to_string(&operation_outcome)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_output_format_parsing() {
        assert!(matches!(
            "text".parse::<OutputFormat>().unwrap(),
            OutputFormat::Text
        ));
        assert!(matches!(
            "json".parse::<OutputFormat>().unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            "operationoutcome".parse::<OutputFormat>().unwrap(),
            OutputFormat::OperationOutcome
        ));
        assert!(matches!(
            "TEXT".parse::<OutputFormat>().unwrap(),
            OutputFormat::Text
        ));
        assert!(matches!(
            "JSON".parse::<OutputFormat>().unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            "OPERATIONOUTCOME".parse::<OutputFormat>().unwrap(),
            OutputFormat::OperationOutcome
        ));
        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_resolve_input_paths_explicit_path() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("patient.json");
        fs::write(&file_path, "{}").unwrap();

        let resolved = resolve_input_paths(&[file_path.to_string_lossy().into_owned()]).unwrap();

        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0], file_path);
    }

    #[test]
    fn test_resolve_input_paths_glob_matches() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("a.json"), "{}").unwrap();
        fs::write(temp_dir.path().join("b.json"), "{}").unwrap();

        let pattern = format!("{}/*.json", temp_dir.path().display());
        let resolved = resolve_input_paths(&[pattern]).unwrap();

        assert_eq!(resolved.len(), 2);
    }

    #[test]
    fn test_resolve_input_paths_recursive_glob() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("a.json"), "{}").unwrap();

        let pattern = format!("{}/**/*.json", temp_dir.path().display());
        let resolved = resolve_input_paths(&[pattern]).unwrap();

        assert_eq!(resolved.len(), 1);
    }

    #[test]
    fn test_resolve_input_paths_mixed_and_deduped() {
        let temp_dir = TempDir::new().unwrap();
        let explicit = temp_dir.path().join("a.json");
        fs::write(&explicit, "{}").unwrap();
        fs::write(temp_dir.path().join("b.json"), "{}").unwrap();

        let pattern = format!("{}/*.json", temp_dir.path().display());
        let resolved =
            resolve_input_paths(&[explicit.to_string_lossy().into_owned(), pattern]).unwrap();

        assert_eq!(resolved.len(), 2);
    }

    #[test]
    fn test_resolve_input_paths_no_match_error() {
        let temp_dir = TempDir::new().unwrap();
        let pattern = format!("{}/*.missing", temp_dir.path().display());

        let err = resolve_input_paths(std::slice::from_ref(&pattern)).unwrap_err();
        let message = err.to_string();

        assert!(message.contains("Input pattern matched no files"));
        assert!(message.contains(&pattern));
    }
}
