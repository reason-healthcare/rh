//! Example to test README generation with the new procedural example
//!
//! This example demonstrates generating a crate structure with README.md
//! to verify the new procedural resource creation example is included.

use anyhow::Result;
use rh_codegen::{generate_crate_structure, CrateGenerationParams};
use std::path::Path;

fn main() -> Result<()> {
    let output_path = Path::new("./test_readme_output");

    // Clean up any existing output
    if output_path.exists() {
        std::fs::remove_dir_all(output_path)?;
    }

    // Generate crate structure with a test README
    generate_crate_structure(CrateGenerationParams {
        output: output_path,
        package: "test.package.core",
        version: "1.0.0",
        canonical_url: "http://test.org/fhir",
        author: "Test Author",
        description: "Test FHIR package for README verification",
        command_invoked: "rh codegen test.package.core 1.0.0",
    })?;

    println!(
        "✅ Generated test crate with README at: {}",
        output_path.display()
    );
    println!("📄 Check the README.md file to verify the procedural example was added");

    Ok(())
}
