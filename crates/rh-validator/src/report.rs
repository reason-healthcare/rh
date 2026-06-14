//! Validation report renderers.
//!
//! Provides human-readable text, JSON, and OperationOutcome formatters for
//! [`ValidationResult`] so that the CLI does not contain formatting business logic.

use anyhow::Result;
use serde_json::Value;

use crate::{Severity, ValidationResult};

// ---------------------------------------------------------------------------
// Single-resource renderers
// ---------------------------------------------------------------------------

/// Print a single-resource validation result in human-readable text.
pub fn print_single_result_text(result: &ValidationResult, resource: &Value) {
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

/// Print a single-resource validation result as JSON.
pub fn print_single_result_json(result: &ValidationResult, resource: &Value) -> Result<()> {
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

/// Print a single-resource result as a FHIR OperationOutcome.
pub fn print_operation_outcome(result: &ValidationResult) -> Result<()> {
    let operation_outcome = result.to_operation_outcome();
    println!("{}", serde_json::to_string_pretty(&operation_outcome)?);
    Ok(())
}

// ---------------------------------------------------------------------------
// Batch renderers (line-number keyed)
// ---------------------------------------------------------------------------

/// Print batch results (keyed by line number) in human-readable text.
pub fn print_batch_results_text(
    results: &[(usize, Value, ValidationResult)],
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

/// Print batch results (keyed by line number) as JSON.
pub fn print_batch_results_json(results: &[(usize, Value, ValidationResult)]) -> Result<()> {
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
        "summary": { "total": total, "valid": valid_count, "invalid": total - valid_count },
        "results": json_results
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

/// Print batch results (keyed by line number) as FHIR OperationOutcomes.
pub fn print_batch_operation_outcomes(results: &[(usize, Value, ValidationResult)]) -> Result<()> {
    for (_, _, result) in results {
        let oo = result.to_operation_outcome();
        println!("{}", serde_json::to_string(&oo)?);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Labeled batch renderers (source-label keyed)
// ---------------------------------------------------------------------------

/// Print labeled batch results (keyed by source label) in human-readable text.
pub fn print_labeled_batch_results_text(
    results: &[(String, Value, ValidationResult)],
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

/// Print labeled batch results (keyed by source label) as JSON.
pub fn print_labeled_batch_results_json(
    results: &[(String, Value, ValidationResult)],
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
        "summary": { "total": total, "valid": valid_count, "invalid": total - valid_count },
        "results": json_results
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

/// Print labeled batch results as FHIR OperationOutcomes.
pub fn print_labeled_batch_operation_outcomes(
    results: &[(String, Value, ValidationResult)],
) -> Result<()> {
    for (_, _, result) in results {
        let oo = result.to_operation_outcome();
        println!("{}", serde_json::to_string(&oo)?);
    }
    Ok(())
}
