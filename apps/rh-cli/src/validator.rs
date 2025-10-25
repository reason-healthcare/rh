//! Validator CLI commands
//!
//! This module provides command-line interface for FHIR resource validation.

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use rh_foundation::cli;
use rh_validator::{extract_resource_type, FhirValidator, Severity, ValidatorConfig};
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
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!(
                "Invalid output format: {s}. Valid options: text, json"
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
    let config = ValidatorConfig::new()
        .with_skip_invariants(args.skip_invariants)
        .with_skip_bindings(args.skip_bindings);
    let validator = FhirValidator::with_config(config)?;

    let content = read_input(&args.input).context("Failed to read input")?;

    if content.trim().is_empty() {
        warn!("Input is empty");
        if args.strict {
            std::process::exit(1);
        }
        return Ok(());
    }

    let result = validator
        .validate_any_resource(&content)
        .context("Failed to validate FHIR resource")?;

    match args.format {
        OutputFormat::Text => print_single_result_text(&result, &content),
        OutputFormat::Json => print_single_result_json(&result, &content)?,
    }

    let has_errors = result.has_errors();
    let has_issues = args.strict && !result.issues.is_empty();

    if has_errors || has_issues {
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_batch_validation(args: BatchArgs) -> Result<()> {
    let config = ValidatorConfig::new()
        .with_skip_invariants(args.skip_invariants)
        .with_skip_bindings(args.skip_bindings);
    let validator = FhirValidator::with_config(config)?;

    let content = read_input(&args.input).context("Failed to read input")?;

    if content.trim().is_empty() {
        warn!("Input is empty");
        if args.strict {
            std::process::exit(1);
        }
        return Ok(());
    }

    let results = validator
        .validate_ndjson_any(&content)
        .context("Failed to validate NDJSON")?;

    match args.format {
        OutputFormat::Text => print_batch_results_text(&results, args.summary_only),
        OutputFormat::Json => print_batch_results_json(&results)?,
    }

    let has_errors = results.iter().any(|(_, r)| r.has_errors());
    let has_issues = args.strict && results.iter().any(|(_, r)| !r.issues.is_empty());

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

fn print_single_result_text(result: &rh_validator::ValidationResult, json: &str) {
    let errors = result.errors().count();
    let warnings = result.warnings().count();
    let info_count = result
        .issues
        .iter()
        .filter(|i| i.severity == Severity::Information)
        .count();

    let resource_type = extract_resource_type(json).unwrap_or_else(|_| "Unknown".to_string());

    if result.is_valid() {
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
            println!("  {}. {} {}", i + 1, icon, issue.details);
            if let Some(location) = &issue.location {
                println!("     Location: {location}");
            }
            if let Some(key) = &issue.invariant_key {
                println!("     Invariant: {key}");
            }
        }
    }
}

fn print_single_result_json(result: &rh_validator::ValidationResult, json: &str) -> Result<()> {
    let resource_type = extract_resource_type(json).unwrap_or_else(|_| "Unknown".to_string());

    let output = serde_json::json!({
        "resourceType": resource_type,
        "valid": result.is_valid(),
        "issues": result.issues.iter().map(|issue| {
            serde_json::json!({
                "severity": format!("{:?}", issue.severity).to_lowercase(),
                "code": format!("{:?}", issue.code),
                "details": issue.details,
                "location": issue.location,
                "expression": issue.expression,
                "invariant": issue.invariant_key,
            })
        }).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_batch_results_text(
    results: &[(usize, rh_validator::ValidationResult)],
    summary_only: bool,
) {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, r)| r.is_valid()).count();
    let invalid_count = total - valid_count;
    let total_errors: usize = results.iter().map(|(_, r)| r.errors().count()).sum();
    let total_warnings: usize = results.iter().map(|(_, r)| r.warnings().count()).sum();

    println!("📋 Batch Validation Summary:");
    println!("  Total resources: {total}");
    println!("  ✅ Valid: {valid_count}");
    println!("  ❌ Invalid: {invalid_count}");
    println!("  Total errors: {total_errors}");
    println!("  Total warnings: {total_warnings}");
    println!();

    if !summary_only && invalid_count > 0 {
        println!("❌ Invalid resources:");
        for (line_num, result) in results.iter() {
            if !result.is_valid() {
                let errors = result.errors().count();
                let warnings = result.warnings().count();
                println!(
                    "  Line {} ({}): {} error(s), {} warning(s)",
                    line_num + 1,
                    result.resource_type,
                    errors,
                    warnings
                );
                for issue in &result.issues {
                    if issue.severity == Severity::Error {
                        println!("    ❌ {}", issue.details);
                        if let Some(location) = &issue.location {
                            println!("       at {location}");
                        }
                    }
                }
            }
        }
    }
}

fn print_batch_results_json(results: &[(usize, rh_validator::ValidationResult)]) -> Result<()> {
    let total = results.len();
    let valid_count = results.iter().filter(|(_, r)| r.is_valid()).count();

    let json_results: Vec<_> = results
        .iter()
        .map(|(line_num, result)| {
            serde_json::json!({
                "line": line_num + 1,
                "resourceType": result.resource_type,
                "valid": result.is_valid(),
                "errors": result.errors().count(),
                "warnings": result.warnings().count(),
                "issues": result.issues.iter().map(|issue| {
                    serde_json::json!({
                        "severity": format!("{:?}", issue.severity).to_lowercase(),
                        "code": format!("{:?}", issue.code),
                        "details": issue.details,
                        "location": issue.location,
                        "expression": issue.expression,
                        "invariant": issue.invariant_key,
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
            "TEXT".parse::<OutputFormat>().unwrap(),
            OutputFormat::Text
        ));
        assert!(matches!(
            "JSON".parse::<OutputFormat>().unwrap(),
            OutputFormat::Json
        ));
        assert!("invalid".parse::<OutputFormat>().is_err());
    }
}
