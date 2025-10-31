//! Validator CLI commands
//!
//! This module provides command-line interface for FHIR resource validation.

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use rh_foundation::cli;
use rh_validator::{FhirValidator, Severity};
use std::path::PathBuf;
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
    /// Input file path (reads from stdin if not provided)
    #[clap(short, long)]
    input: Option<PathBuf>,

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
}

#[derive(Args)]
pub struct BatchArgs {
    /// Input NDJSON file path (reads from stdin if not provided)
    #[clap(short, long)]
    input: Option<PathBuf>,

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
    // Note: skip_invariants and skip_bindings are not yet supported in the new API
    if args.skip_invariants {
        warn!("--skip-invariants flag is not yet implemented");
    }
    if args.skip_bindings {
        warn!("--skip-bindings flag is not yet implemented");
    }

    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    let content = read_input(&args.input).context("Failed to read input")?;

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
}

async fn handle_batch_validation(args: BatchArgs) -> Result<()> {
    // Note: skip_invariants and skip_bindings are not yet supported in the new API
    if args.skip_invariants {
        warn!("--skip-invariants flag is not yet implemented");
    }
    if args.skip_bindings {
        warn!("--skip-bindings flag is not yet implemented");
    }

    let validator = FhirValidator::new(rh_validator::FhirVersion::R4, None)?;

    let content = read_input(&args.input).context("Failed to read input")?;

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
}

fn read_input(path: &Option<PathBuf>) -> Result<String> {
    if path.is_some() {
        info!("Reading from file: {}", path.as_ref().unwrap().display());
    } else {
        info!("Reading from stdin");
    }
    cli::read_input_from_path(path).map_err(Into::into)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
