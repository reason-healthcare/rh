//! Validator CLI commands
//!
//! This module provides command-line interface for FHIR resource validation.

use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use glob::glob;
use rh_validator::{report, FhirValidator, Severity, TerminologyConfig, ValidationOptions};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use crate::output::{
    Envelope, ExitCode, OutputContext, OutputFormat as GlobalOutputFormat,
};

#[derive(Serialize)]
struct ValidationIssue {
    severity: String,
    message: String,
    path: Option<String>,
}

#[derive(Serialize)]
struct ValidationResult {
    resource: Option<String>,
    valid: bool,
    issues: Vec<ValidationIssue>,
}

fn print_envelope<T: Serialize>(ctx: &OutputContext, envelope: &Envelope<T>) -> Result<()> {
    let json = if matches!(ctx.format, GlobalOutputFormat::Json) {
        serde_json::to_string_pretty(envelope)?
    } else {
        serde_json::to_string(envelope)?
    };
    println!("{json}");
    Ok(())
}

fn to_validation_result(
    resource: &serde_json::Value,
    result: &rh_validator::ValidationResult,
) -> ValidationResult {
    ValidationResult {
        resource: resource
            .get("resourceType")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        valid: result.valid,
        issues: result
            .issues
            .iter()
            .map(|issue| ValidationIssue {
                severity: issue.severity.to_string(),
                message: issue.message.clone(),
                path: issue.path.clone(),
            })
            .collect(),
    }
}

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

    /// Output format for validation report (text, json, operationoutcome)
    #[clap(long = "report-format", default_value = "text")]
    report_format: OutputFormat,

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

    /// Output format for validation report (text, json, operationoutcome)
    #[clap(long = "report-format", default_value = "text")]
    report_format: OutputFormat,

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
pub async fn handle_command(cmd: ValidatorCommands, ctx: &OutputContext) -> Result<()> {
    match cmd {
        ValidatorCommands::Resource(args) => handle_resource_validation(args, ctx.clone()).await,
        ValidatorCommands::Batch(args) => handle_batch_validation(args, ctx.clone()).await,
    }
}

async fn handle_resource_validation(args: ResourceArgs, ctx: OutputContext) -> Result<()> {
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
                    ExitCode::ValidationFailure.exit();
                }
                return Ok(());
            }

            if ctx.is_json() {
                let payload = results
                    .iter()
                    .map(|(_, resource, result)| to_validation_result(resource, result))
                    .collect::<Vec<_>>();
                print_envelope(&ctx, &Envelope::ok(payload, "validate resource"))?;
            } else if results.len() == 1 {
                let (_, resource, result) = &results[0];
                match args.report_format {
                    OutputFormat::Text => report::print_single_result_text(result, resource),
                    OutputFormat::Json => report::print_single_result_json(result, resource)?,
                    OutputFormat::OperationOutcome => report::print_operation_outcome(result)?,
                }
            } else {
                match args.report_format {
                    OutputFormat::Text => report::print_labeled_batch_results_text(&results, false),
                    OutputFormat::Json => report::print_labeled_batch_results_json(&results)?,
                    OutputFormat::OperationOutcome => {
                        report::print_labeled_batch_operation_outcomes(&results)?
                    }
                }
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            if has_errors || has_issues {
                ExitCode::ValidationFailure.exit();
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                ExitCode::ValidationFailure.exit();
            }
            return Ok(());
        }

        let resource: serde_json::Value =
            serde_json::from_str(&content).context("Failed to parse JSON")?;

        let result = validator
            .validate_auto(&resource)
            .context("Failed to validate FHIR resource")?;

        if ctx.is_json() {
            let payload = vec![to_validation_result(&resource, &result)];
            print_envelope(&ctx, &Envelope::ok(payload, "validate resource"))?;
        } else {
            match args.report_format {
                OutputFormat::Text => report::print_single_result_text(&result, &resource),
                OutputFormat::Json => report::print_single_result_json(&result, &resource)?,
                OutputFormat::OperationOutcome => report::print_operation_outcome(&result)?,
            }
        }

        let has_errors = !result.valid;
        let has_issues = args.strict && !result.issues.is_empty();

        if has_errors || has_issues {
            ExitCode::ValidationFailure.exit();
        }

        Ok(())
    })
    .await
    .context("Resource validation task failed")?
}

async fn handle_batch_validation(args: BatchArgs, ctx: OutputContext) -> Result<()> {
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
                    ExitCode::ValidationFailure.exit();
                }
                return Ok(());
            }

            if ctx.is_json() {
                let payload = results
                    .iter()
                    .map(|(_, resource, result)| to_validation_result(resource, result))
                    .collect::<Vec<_>>();
                print_envelope(&ctx, &Envelope::ok(payload, "validate batch"))?;
            } else {
                match args.report_format {
                    OutputFormat::Text => {
                        report::print_labeled_batch_results_text(&results, args.summary_only)
                    }
                    OutputFormat::Json => report::print_labeled_batch_results_json(&results)?,
                    OutputFormat::OperationOutcome => {
                        report::print_labeled_batch_operation_outcomes(&results)?
                    }
                }
            }

            let has_errors = results.iter().any(|(_, _, r)| !r.valid);
            let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

            if has_errors || has_issues {
                ExitCode::ValidationFailure.exit();
            }

            return Ok(());
        }

        let content = read_input_from_stdin().context("Failed to read input")?;

        if content.trim().is_empty() {
            warn!("Input is empty");
            if args.strict {
                ExitCode::ValidationFailure.exit();
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

        if ctx.is_json() {
            let payload = results
                .iter()
                .map(|(_, resource, result)| to_validation_result(resource, result))
                .collect::<Vec<_>>();
            print_envelope(&ctx, &Envelope::ok(payload, "validate batch"))?;
        } else {
            match args.report_format {
                OutputFormat::Text => report::print_batch_results_text(&results, args.summary_only),
                OutputFormat::Json => report::print_batch_results_json(&results)?,
                OutputFormat::OperationOutcome => report::print_batch_operation_outcomes(&results)?,
            }
        }

        let has_errors = results.iter().any(|(_, _, r)| !r.valid);
        let has_issues = args.strict && results.iter().any(|(_, _, r)| !r.issues.is_empty());

        if has_errors || has_issues {
            ExitCode::ValidationFailure.exit();
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
