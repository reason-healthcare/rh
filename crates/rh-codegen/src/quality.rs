//! Quality assurance module for generated FHIR crates
//!
//! This module provides functionality to run quality checks on generated Rust crates,
//! including formatting with rustfmt and compilation checks with cargo check.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

use crate::CodegenError;

/// Configuration for quality checks
#[derive(Debug, Clone)]
pub struct QualityConfig {
    /// Whether to run cargo fmt on the generated crate
    pub run_format: bool,
    /// Whether to run cargo check on the generated crate
    pub run_compile_check: bool,
    /// Whether to run cargo clippy on the generated crate
    pub run_clippy: bool,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            run_format: true,
            run_compile_check: true,
            run_clippy: false, // Disabled by default as it might be too strict for generated code
        }
    }
}

/// Run all configured quality checks on a generated crate
pub fn run_quality_checks(crate_path: &Path, config: &QualityConfig) -> Result<()> {
    println!("🔍 Running quality checks on generated crate...");

    if config.run_format {
        run_format_check(crate_path)?;
    }

    if config.run_compile_check {
        run_compile_check(crate_path)?;
    }

    if config.run_clippy {
        run_clippy_check(crate_path)?;
    }

    println!("✅ All quality checks passed");
    Ok(())
}

/// Format the generated crate using rustfmt
pub fn run_format_check(crate_path: &Path) -> Result<()> {
    println!("🎨 Formatting generated crate...");

    // Try cargo fmt first, fallback to rustfmt directly if it fails
    let output = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .current_dir(crate_path)
        .output()
        .with_context(|| format!("Failed to execute cargo fmt in {}", crate_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if should_fallback_to_rustfmt(&stderr) {
            println!(
                "⚠️  cargo fmt could not format the generated crate, trying direct rustfmt..."
            );
            run_rustfmt_direct(crate_path)?;
            println!("✅ Formatting completed successfully (using rustfmt directly)");
            return Ok(());
        }

        return Err(anyhow::anyhow!("cargo fmt failed: {stderr}"));
    }

    println!("✅ Formatting completed successfully");
    Ok(())
}

fn should_fallback_to_rustfmt(stderr: &str) -> bool {
    stderr.contains("Failed to find targets")
        || stderr.contains("no targets")
        || stderr.contains("failed to find a workspace root")
}

fn run_rustfmt_direct(crate_path: &Path) -> Result<()> {
    let src_dir = crate_path.join("src");
    let mut rust_files = Vec::new();
    collect_rust_files(&src_dir, &mut rust_files)?;

    if rust_files.is_empty() {
        return Err(anyhow::anyhow!(
            "rustfmt fallback found no Rust files under {}",
            src_dir.display()
        ));
    }

    let rustfmt_output = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .args(&rust_files)
        .output()
        .with_context(|| "Failed to execute rustfmt directly")?;

    if !rustfmt_output.status.success() {
        let rustfmt_stderr = String::from_utf8_lossy(&rustfmt_output.stderr);
        return Err(anyhow::anyhow!("rustfmt failed: {rustfmt_stderr}"));
    }

    Ok(())
}

fn collect_rust_files(dir: &Path, rust_files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry.with_context(|| format!("Failed to read entry in {}", dir.display()))?;
        let path = entry.path();

        if path.is_dir() {
            collect_rust_files(&path, rust_files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            rust_files.push(path);
        }
    }

    Ok(())
}

/// Run cargo check on the generated crate as a compilation quality gate
pub fn run_compile_check(crate_path: &Path) -> Result<()> {
    println!("🔧 Running cargo check on generated crate...");

    let output = Command::new("cargo")
        .arg("check")
        .current_dir(crate_path)
        .output()
        .with_context(|| format!("Failed to execute cargo check in {}", crate_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!("❌ Cargo check failed for generated crate:");
        if !stdout.is_empty() {
            eprintln!("stdout: {stdout}");
        }
        if !stderr.is_empty() {
            eprintln!("stderr: {stderr}");
        }

        return Err(anyhow::anyhow!(
            "Generated crate failed cargo check. See output above for details."
        ));
    }

    println!("✅ Cargo check passed for generated crate");
    Ok(())
}

/// Run cargo clippy on the generated crate for additional linting
pub fn run_clippy_check(crate_path: &Path) -> Result<()> {
    println!("📎 Running cargo clippy on generated crate...");

    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .current_dir(crate_path)
        .output()
        .with_context(|| format!("Failed to execute cargo clippy in {}", crate_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!("❌ Cargo clippy failed for generated crate:");
        if !stdout.is_empty() {
            eprintln!("stdout: {stdout}");
        }
        if !stderr.is_empty() {
            eprintln!("stderr: {stderr}");
        }

        return Err(anyhow::anyhow!(
            "Generated crate failed cargo clippy. See output above for details."
        ));
    }

    println!("✅ Cargo clippy passed for generated crate");
    Ok(())
}

/// Legacy wrapper for backward compatibility
pub fn format_generated_crate<P: AsRef<Path>>(crate_path: P) -> Result<(), CodegenError> {
    run_format_check(crate_path.as_ref()).map_err(|e| CodegenError::Generation {
        message: e.to_string(),
    })
}
