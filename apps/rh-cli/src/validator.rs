//! Validator CLI commands
//!
//! This module provides command-line interface for JSON validation functionality.

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use rh_foundation::cli;
use rh_validator::{FhirValidator, JsonValidator, ValidationResult};
use std::path::PathBuf;
use tracing::{info, warn};

#[derive(Subcommand)]
pub enum ValidatorCommands {
    /// Validate JSON syntax in a file or from stdin
    Json(JsonArgs),
    /// Validate FHIR resource type against FHIR specifications
    Fhir(FhirArgs),
}

#[derive(Args)]
pub struct JsonArgs {
    /// Input file path (reads from stdin if not provided)
    #[clap(short, long)]
    input: Option<PathBuf>,

    /// Output format (text, json)
    #[clap(short, long, default_value = "text")]
    format: OutputFormat,

    /// Maximum nesting depth allowed
    #[clap(long, default_value = "100")]
    max_depth: usize,

    /// Validate multiple JSON documents (NDJSON format)
    #[clap(long)]
    multiple: bool,

    /// Show detailed statistics for valid JSON
    #[clap(long)]
    stats: bool,

    /// Exit with non-zero code if validation fails
    #[clap(long)]
    strict: bool,
}

#[derive(Args)]
pub struct FhirArgs {
    /// Input file path (reads from stdin if not provided)
    #[clap(short, long)]
    input: Option<PathBuf>,

    /// FHIR version to validate against
    #[clap(long, default_value = "4.0.1")]
    version: String,

    /// Output format (text, json)
    #[clap(short, long, default_value = "text")]
    format: OutputFormat,

    /// Validate multiple FHIR resources (NDJSON format)
    #[clap(long)]
    multiple: bool,

    /// Show detailed statistics for valid FHIR resources
    #[clap(long)]
    stats: bool,

    /// Exit with non-zero code if validation fails
    #[clap(long)]
    strict: bool,

    /// Custom package directory for FHIR packages
    #[clap(long)]
    package_dir: Option<PathBuf>,

    /// Force regeneration of Rust crate even if it already exists
    #[clap(long)]
    regenerate: bool,
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
        ValidatorCommands::Json(args) => handle_json_validation(args).await,
        ValidatorCommands::Fhir(args) => handle_fhir_validation(args).await,
    }
}

async fn handle_json_validation(args: JsonArgs) -> Result<()> {
    let validator = JsonValidator::with_max_depth(args.max_depth);

    // Read input content
    let content = read_input(&args.input).context("Failed to read input")?;

    if content.trim().is_empty() {
        warn!("Input is empty");
        if args.strict {
            std::process::exit(1);
        }
        return Ok(());
    }

    let has_errors = if args.multiple {
        // Validate multiple JSON documents (NDJSON)
        let results = validator.validate_multiple(&content);

        match args.format {
            OutputFormat::Text => {
                print_multiple_results_text(&results, args.stats);
            }
            OutputFormat::Json => {
                print_multiple_results_json(&results)?;
            }
        }

        results.iter().any(|(_, result)| !result.is_valid())
    } else {
        // Validate single JSON document
        let result = validator.validate(&content);

        match args.format {
            OutputFormat::Text => {
                print_single_result_text(&result, args.stats);
            }
            OutputFormat::Json => {
                print_single_result_json(&result)?;
            }
        }

        !result.is_valid()
    };

    if has_errors && args.strict {
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_fhir_validation(args: FhirArgs) -> Result<()> {
    // Create FHIR validator with custom settings
    let validator = if let Some(package_dir) = &args.package_dir {
        FhirValidator::with_package_dir(package_dir.clone())
            .context("Failed to create FHIR validator with package dir")?
    } else {
        FhirValidator::new().context("Failed to create FHIR validator")?
    };

    // Note: with_default_version will be added in Phase 1
    // validator = validator.with_default_version(&args.version);

    // Read input content
    let content = read_input(&args.input).context("Failed to read input")?;

    if content.trim().is_empty() {
        warn!("Input is empty");
        if args.strict {
            std::process::exit(1);
        }
        return Ok(());
    }

    let has_errors = if args.multiple {
        // Validate multiple FHIR resources (NDJSON)
        let results = validator
            .validate_multiple(&content, Some(&args.version))
            .context("Failed to validate multiple FHIR resources")?;

        match args.format {
            OutputFormat::Text => {
                print_multiple_results_text(&results, args.stats);
            }
            OutputFormat::Json => {
                print_multiple_results_json(&results)?;
            }
        }

        results.iter().any(|(_, result)| !result.is_valid())
    } else {
        // Validate single FHIR resource
        let result = validator
            .validate_with_version(&content, &args.version)
            .context("Failed to validate FHIR resource")?;

        match args.format {
            OutputFormat::Text => {
                print_single_result_text(&result, args.stats);
            }
            OutputFormat::Json => {
                print_single_result_json(&result)?;
            }
        }

        !result.is_valid()
    };

    if has_errors && args.strict {
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

fn print_single_result_text(result: &ValidationResult, show_stats: bool) {
    if result.is_valid() {
        println!("✅ JSON is valid");
        if show_stats {
            println!("📊 Statistics: JSON structure is well-formed");
        }
    } else {
        println!(
            "❌ JSON validation failed with {} error(s):",
            result.error_count()
        );
        for (i, issue) in result.errors().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
    }
}

fn print_single_result_json(result: &ValidationResult) -> Result<()> {
    let output = if result.is_valid() {
        serde_json::json!({
            "valid": true,
            "errors": []
        })
    } else {
        let error_strings: Vec<String> = result.errors().map(|e| e.to_string()).collect();
        serde_json::json!({
            "valid": false,
            "errors": error_strings
        })
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_multiple_results_text(results: &[(usize, ValidationResult)], show_stats: bool) {
    let total = results.len();
    let valid_count = results
        .iter()
        .filter(|(_, result)| result.is_valid())
        .count();
    let invalid_count = total - valid_count;

    println!("📋 Validation Summary:");
    println!("  Total documents: {total}");
    println!("  ✅ Valid: {valid_count}");
    println!("  ❌ Invalid: {invalid_count}");
    println!();

    if invalid_count > 0 {
        println!("❌ Invalid documents:");
        for (line_number, result) in results {
            if !result.is_valid() {
                println!("  Line {}: {} error(s)", line_number, result.error_count());
                for error in result.errors() {
                    println!("    - {error}");
                }
            }
        }
    }

    if show_stats && valid_count > 0 {
        println!();
        println!("📊 Statistics:");
        println!("  Valid JSON documents processed successfully");
    }
}

fn print_multiple_results_json(results: &[(usize, ValidationResult)]) -> Result<()> {
    let mut json_results = Vec::new();

    for (line_number, result) in results {
        let json_result = if result.is_valid() {
            serde_json::json!({
                "line": line_number,
                "valid": true,
                "errors": []
            })
        } else {
            let error_strings: Vec<String> = result.errors().map(|e| e.to_string()).collect();
            serde_json::json!({
                "line": line_number,
                "valid": false,
                "errors": error_strings
            })
        };
        json_results.push(json_result);
    }

    let total = results.len();
    let valid_count = results
        .iter()
        .filter(|(_, result)| result.is_valid())
        .count();

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

    #[test]
    fn test_single_result_formatting() {
        use rh_validator::{IssueCode, Severity, ValidationIssue};

        let valid_result = ValidationResult::new("Patient");
        print_single_result_text(&valid_result, false);
        print_single_result_text(&valid_result, true);

        let mut invalid_result = ValidationResult::new("Patient");
        invalid_result.add_issue(ValidationIssue::new(
            Severity::Error,
            IssueCode::Structure,
            "Test error at line 1, column 5",
        ));
        print_single_result_text(&invalid_result, false);

        assert!(print_single_result_json(&valid_result).is_ok());
        assert!(print_single_result_json(&invalid_result).is_ok());
    }

    #[test]
    fn test_multiple_results_formatting() {
        use rh_validator::{IssueCode, Severity, ValidationIssue};

        let mut invalid_result = ValidationResult::new("Patient");
        invalid_result.add_issue(ValidationIssue::new(
            Severity::Error,
            IssueCode::Structure,
            "Test error at line 2, column 1",
        ));

        let results = vec![
            (1, ValidationResult::new("Patient")),
            (2, invalid_result),
            (3, ValidationResult::new("Patient")),
        ];

        print_multiple_results_text(&results, false);
        print_multiple_results_text(&results, true);
        assert!(print_multiple_results_json(&results).is_ok());
    }
}
