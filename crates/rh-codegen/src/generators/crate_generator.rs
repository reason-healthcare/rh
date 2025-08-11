//! FHIR Crate Generation
//!
//! This module provides functionality to generate complete Rust crates from FHIR packages,
//! including Cargo.toml, lib.rs, README.md, and proper module structure.

use std::fs;
use std::path::Path;

use anyhow::Result;
use chrono::Local;

/// Parameters for crate generation
#[derive(Debug, Clone)]
pub struct CrateGenerationParams<'a> {
    /// Output directory for the crate
    pub output: &'a Path,
    /// Package name (e.g., "hl7.fhir.r4.core")
    pub package: &'a str,
    /// Package version (e.g., "4.0.1")
    pub version: &'a str,
    /// Canonical URL from package.json
    pub canonical_url: &'a str,
    /// Author from package.json
    pub author: &'a str,
    /// Description from package.json
    pub description: &'a str,
    /// Command that was invoked to generate this crate
    pub command_invoked: &'a str,
}

/// Statistics about generated crate content
#[derive(Debug, Clone)]
pub struct CrateStatistics {
    /// Number of generated structs
    pub num_structs: usize,
    /// Number of generated enums
    pub num_enums: usize,
    /// Total number of types
    pub total_types: usize,
    /// Canonical URL
    pub canonical_url: String,
}

/// Generate a complete Rust crate structure with idiomatic directory organization
pub fn generate_crate_structure(params: CrateGenerationParams) -> Result<()> {
    // Create the src directory structure
    let src_dir = params.output.join("src");
    fs::create_dir_all(&src_dir)?;

    // Create subdirectories
    let resource_dir = src_dir.join("resource");
    let datatypes_dir = src_dir.join("datatypes");
    let primitives_dir = src_dir.join("primitives");
    let traits_dir = src_dir.join("traits");
    let bindings_dir = src_dir.join("bindings");

    fs::create_dir_all(&resource_dir)?;
    fs::create_dir_all(&datatypes_dir)?;
    fs::create_dir_all(&primitives_dir)?;
    fs::create_dir_all(&traits_dir)?;
    fs::create_dir_all(&bindings_dir)?;

    // Generate statistics by counting organized files (if any exist from organized generation)
    let stats = generate_crate_statistics_from_organized_dirs(
        &resource_dir,
        &datatypes_dir,
        &primitives_dir,
    )?;

    // Generate Cargo.toml
    let cargo_toml_content = generate_cargo_toml(params.package, params.version);
    let cargo_toml_path = params.output.join("Cargo.toml");
    fs::write(&cargo_toml_path, cargo_toml_content)?;

    // Generate lib.rs with new structure
    let lib_rs_content = generate_lib_rs_idiomatic()?;
    let lib_rs_path = src_dir.join("lib.rs");
    fs::write(&lib_rs_path, lib_rs_content)?;

    // Generate macros.rs with FHIR primitive macros
    let macros_content = include_str!("../macros.rs");
    let macros_path = src_dir.join("macros.rs");
    fs::write(&macros_path, macros_content)?;

    // Generate mod.rs files for each module
    generate_module_files(
        &resource_dir,
        &datatypes_dir,
        &primitives_dir,
        &traits_dir,
        &bindings_dir,
    )?;

    // Generate README.md
    let readme_content = generate_readme_md(
        params.package,
        params.version,
        params.canonical_url,
        params.author,
        params.description,
        params.command_invoked,
        &stats,
    );
    let readme_path = params.output.join("README.md");
    fs::write(&readme_path, readme_content)?;

    Ok(())
}

/// Generate Cargo.toml content for the FHIR crate
fn generate_cargo_toml(package: &str, version: &str) -> String {
    // Convert FHIR package name to a valid Rust crate name
    let crate_name = package.replace(['.', '-'], "_");

    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"
description = "Generated FHIR types from {package} package version {version}"
authors = ["FHIR Code Generator"]
license = "MIT OR Apache-2.0"

[workspace]
# Empty workspace to exclude from parent workspace

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
paste = "1.0"

[lib]
name = "{crate_name}"
path = "src/lib.rs"
"#
    )
}

/// Generate lib.rs content with idiomatic module structure
fn generate_lib_rs_idiomatic() -> Result<String> {
    let lib_content = r#"//! Generated FHIR Rust bindings
//!
//! This crate contains Rust types and traits for FHIR resources and data types.
//! It includes macros for primitive field generation and maintains FHIR compliance.

pub mod macros;
pub mod primitives;
pub mod datatypes;
pub mod resource;
pub mod traits;
pub mod bindings;

// Re-export macros and serde traits for convenience
pub use macros::*;
pub use serde::{Deserialize, Serialize};
"#;

    Ok(lib_content.to_string())
}

/// Generate mod.rs files for each module directory
fn generate_module_files(
    resource_dir: &Path,
    datatypes_dir: &Path,
    primitives_dir: &Path,
    traits_dir: &Path,
    bindings_dir: &Path,
) -> Result<()> {
    // Generate resource/mod.rs
    let resource_mod_content = generate_mod_rs_for_directory(resource_dir, "FHIR resource types")?;
    fs::write(resource_dir.join("mod.rs"), resource_mod_content)?;

    // Generate datatypes/mod.rs
    let datatypes_mod_content = generate_mod_rs_for_directory(datatypes_dir, "FHIR data types")?;
    fs::write(datatypes_dir.join("mod.rs"), datatypes_mod_content)?;

    // Generate primitives/mod.rs
    let primitives_mod_content =
        generate_mod_rs_for_directory(primitives_dir, "FHIR primitive types")?;
    fs::write(primitives_dir.join("mod.rs"), primitives_mod_content)?;

    // Generate traits/mod.rs (placeholder for now, but don't overwrite existing content)
    let traits_mod_path = traits_dir.join("mod.rs");
    if !traits_mod_path.exists() || fs::read_to_string(&traits_mod_path)?.len() < 500 {
        // If file is small, it's likely just placeholder
        let traits_mod_content = generate_traits_mod_rs()?;
        fs::write(traits_mod_path, traits_mod_content)?;
    }

    // Generate bindings/mod.rs for ValueSet enums
    let bindings_mod_content =
        generate_mod_rs_for_directory(bindings_dir, "FHIR ValueSet bindings and enums")?;
    fs::write(bindings_dir.join("mod.rs"), bindings_mod_content)?;

    Ok(())
}

/// Generate mod.rs content for a specific directory
fn generate_mod_rs_for_directory(dir: &Path, description: &str) -> Result<String> {
    let mut content = String::new();
    content.push_str(&format!("//! {description}\n\n"));

    // Get all .rs files in the directory
    let mut rs_files = Vec::new();
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "mod" {
                        rs_files.push(stem.to_string());
                    }
                }
            }
        }
    }

    // Sort module names for consistency
    rs_files.sort();

    // Generate module declarations
    for module_name in &rs_files {
        content.push_str(&format!("pub mod {module_name};\n"));
    }

    // Note: No glob re-exports to avoid ambiguous re-export warnings
    // Individual types can be imported explicitly when needed

    Ok(content)
}

/// Generate traits/mod.rs with placeholder content
fn generate_traits_mod_rs() -> Result<String> {
    let content = r#"//! FHIR traits for common functionality
//!
//! This module contains traits that define common interfaces for FHIR types.

// Placeholder traits - these would be generated based on FHIR structure definitions

/// Trait for types that have extensions
pub trait HasExtensions {
    /// Get the extensions for this type
    fn extensions(&self) -> &[crate::datatypes::Extension];
}

/// Trait for FHIR resources
pub trait Resource {
    /// Get the resource type name
    fn resource_type(&self) -> &'static str;
    
    /// Get the logical id of this resource
    fn id(&self) -> Option<&str>;
    
    /// Get the metadata about this resource
    fn meta(&self) -> Option<&crate::datatypes::Meta>;
}

/// Trait for domain resources (resources that can have narrative)
pub trait DomainResource: Resource + HasExtensions {
    /// Get the narrative text for this domain resource
    fn narrative(&self) -> Option<&crate::datatypes::Narrative>;
}
"#;

    Ok(content.to_string())
}

/// Generate statistics from organized directories
fn generate_crate_statistics_from_organized_dirs(
    resource_dir: &Path,
    datatypes_dir: &Path,
    primitives_dir: &Path,
) -> Result<CrateStatistics> {
    let mut num_structs = 0;
    let num_enums = 0; // Enums would be handled separately

    // Count files in each directory
    for dir in [resource_dir, datatypes_dir, primitives_dir] {
        if dir.exists() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem != "mod" {
                            num_structs += 1;

                            // Count additional structs within the file (nested types)
                            if let Ok(content) = fs::read_to_string(&path) {
                                num_structs +=
                                    content.matches("pub struct ").count().saturating_sub(1);
                            }
                        }
                    }
                }
            }
        }
    }

    let total_types = num_structs + num_enums;

    Ok(CrateStatistics {
        num_structs,
        num_enums,
        total_types,
        canonical_url: "Unknown".to_string(),
    })
}

/// Generate README.md content with package information and statistics
fn generate_readme_md(
    package: &str,
    version: &str,
    canonical_url: &str,
    author: &str,
    description: &str,
    command_invoked: &str,
    stats: &CrateStatistics,
) -> String {
    let crate_name = package.replace(['.', '-'], "_");
    let mut content = String::new();

    content.push_str(&format!("# {crate_name}\n\n"));
    content.push_str(&format!("**Generated FHIR Types for {package}**\n\n"));
    content.push_str(&format!("This crate contains automatically generated Rust types for FHIR (Fast Healthcare Interoperability Resources) based on the `{package}` package.\n\n"));

    content.push_str("## ⚠️ Important Notice\n\n");
    content
        .push_str("**This crate was automatically generated using the RH codegen CLI tool.**\n\n");
    content.push_str(&format!(
        "- **Generator command**:\n```bash\n{command_invoked}\n```\n\n"
    ));
    content.push_str(&format!("- **Generation timestamp**: {}\n\n", Local::now()));

    content.push_str("## 📦 Package Information\n\n");

    content.push_str(&format!("* **Package Name** {package}\n"));
    content.push_str(&format!("* **Package Author** {author}\n"));
    content.push_str(&format!("* **Version** {version}\n"));
    content.push_str(&format!("* **Canonical URL** `{canonical_url}`\n\n"));

    content.push_str(&format!(
        "**📊 There are {} structs, {} enums, with a total of {} types**\n\n",
        stats.num_structs, stats.num_enums, stats.total_types
    ));

    content.push_str(&format!("## 📚 Description\n\n{description}"));

    content.push_str("\n\n");

    content.push_str("## 🚀 Usage\n\n");
    content.push_str("Add this crate to your `Cargo.toml`:\n\n");
    content.push_str("```toml\n");
    content.push_str("[dependencies]\n");
    content.push_str(&format!("{crate_name} = \"0.1.0\"\n"));
    content.push_str("```\n\n");

    content.push_str("Example usage:\n\n");
    content.push_str("### Deserializing from JSON\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::*;\n"));
    content.push_str("use serde_json;\n\n");
    content.push_str("// All types include serde serialization support\n");
    content.push_str("let json_data = r#\"{\\\"resourceType\\\": \\\"Patient\\\", \\\"id\\\": \\\"example\\\"}\"#;\n");
    content.push_str("let patient: Patient = serde_json::from_str(json_data)?;\n\n");
    content.push_str("println!(\"Patient ID: {}\", patient.id.unwrap_or_default());\n");
    content.push_str("```\n\n");

    content.push_str("### Creating resources procedurally\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::*;\n"));
    content.push_str("use serde_json;\n\n");
    content.push_str("// Create a new Patient resource\n");
    content.push_str("let patient = Patient {\n");
    content.push_str("    id: Some(\"patient-123\".to_string()),\n");
    content.push_str("    meta: None,\n");
    content.push_str("    implicit_rules: None,\n");
    content.push_str("    language: None,\n");
    content.push_str("    text: None,\n");
    content.push_str("    contained: vec![],\n");
    content.push_str("    extension: vec![],\n");
    content.push_str("    modifier_extension: vec![],\n");
    content.push_str("    active: Some(true),\n");
    content.push_str("    name: vec![HumanName {\n");
    content.push_str("        family: Some(\"Doe\".to_string()),\n");
    content.push_str("        given: vec![\"John\".to_string()],\n");
    content.push_str("        ..Default::default()\n");
    content.push_str("    }],\n");
    content.push_str("    gender: Some(AdministrativeGender::Male),\n");
    content.push_str("    ..Default::default()\n");
    content.push_str("};\n\n");
    content.push_str("// Serialize to JSON\n");
    content.push_str("let json = serde_json::to_string_pretty(&patient)?;\n");
    content.push_str("println!(\"Patient JSON: {}\", json);\n");
    content.push_str("```\n\n");

    content.push_str("## 🏗️ Structure\n\n");
    content.push_str(
        "This crate uses an idiomatic Rust module structure organized by FHIR type category:\n\n",
    );
    content.push_str(
        "- **`src/resource/`** - FHIR resource types (Patient, Observation, Bundle, etc.)\n",
    );
    content.push_str(
        "- **`src/datatypes/`** - FHIR data types (Narrative, Extension, Coding, etc.)\n",
    );
    content.push_str(
        "- **`src/primitives/`** - FHIR primitive types (string, boolean, dateTime, etc.)\n",
    );
    content.push_str("- **`src/traits/`** - Common trait interfaces for FHIR functionality\n");
    content.push_str("- **`src/lib.rs`** - Main library file that re-exports all modules\n\n");

    content.push_str("## 🔄 Regeneration\n\n");
    content.push_str("To regenerate this crate with updated FHIR definitions, run:\n\n");
    content.push_str("```bash\n");
    content.push_str(command_invoked);
    content.push_str("\n```\n\n");

    content.push_str("## ⚖️ License\n\n");
    content.push_str("This generated crate is provided under MIT OR Apache-2.0 license.\n\n");

    content.push_str("## 🔗 Related Links\n\n");
    content.push_str("- [FHIR Specification](https://hl7.org/fhir/)\n");
    content.push_str("- [RH Codegen Tool](https://github.com/reason-healthcare/rh)\n");

    content
}

/// Parse package.json to extract metadata for crate generation
pub fn parse_package_metadata(package_json_path: &Path) -> Result<(String, String, String)> {
    let package_json_content = fs::read_to_string(package_json_path)?;
    let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

    let canonical = package_json
        .get("canonical")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let author = package_json
        .get("author")
        .and_then(|v| v.as_str())
        .unwrap_or("FHIR Code Generator")
        .to_string();

    let description = package_json
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("Generated FHIR types crate.")
        .to_string();

    Ok((canonical, author, description))
}
