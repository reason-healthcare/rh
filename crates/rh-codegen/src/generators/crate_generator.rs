//! FHIR Crate Generation
//!
//! This module provides functionality to generate complete Rust crates from FHIR packages,
//! including Cargo.toml, lib.rs, README.md, and proper module structure.

use std::fs;
use std::path::Path;

use anyhow::Result;
use chrono::Local;

// use crate::quality::{run_quality_checks, QualityConfig};

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
    let resource_dir = src_dir.join("resources");
    let datatypes_dir = src_dir.join("datatypes");
    let extensions_dir = src_dir.join("extensions");
    let primitives_dir = src_dir.join("primitives");
    let traits_dir = src_dir.join("traits");
    let bindings_dir = src_dir.join("bindings");
    let profiles_dir = src_dir.join("profiles");

    fs::create_dir_all(&resource_dir)?;
    fs::create_dir_all(&datatypes_dir)?;
    fs::create_dir_all(&extensions_dir)?;
    fs::create_dir_all(&primitives_dir)?;
    fs::create_dir_all(&traits_dir)?;
    fs::create_dir_all(&bindings_dir)?;
    fs::create_dir_all(&profiles_dir)?;

    // Generate statistics by counting organized files (if any exist from organized generation)
    let stats = generate_crate_statistics_from_organized_dirs(
        &resource_dir,
        &datatypes_dir,
        &extensions_dir,
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
        &extensions_dir,
        &primitives_dir,
        &traits_dir,
        &bindings_dir,
        &profiles_dir,
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

    // Run quality checks as a final step
    //let quality_config = QualityConfig::default();
    //run_quality_checks(params.output, &quality_config)
    //   .map_err(|e| anyhow::anyhow!("Quality checks failed: {e}"))?;

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

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
phf = {{ version = "0.11", features = ["macros"] }}

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

// Allow clippy lint for derivable Default implementations
//
// TODO: Future optimization - derive Default when possible instead of manual impl
//
// Currently, we generate explicit Default implementations for all structs.
// Many of these could use #[derive(Default)] instead, which would be more idiomatic.
//
// Pros of deriving Default:
// - More idiomatic Rust code
// - Less generated code (no manual impl blocks)
// - Clearer intent (all fields use Default::default())
//
// Cons of current approach (manual impl):
// - Clippy warns about 1,100+ derivable implementations
// - More verbose generated code
//
// Pros of current approach:
// - Explicit and predictable behavior
// - Handles mixed initialization patterns consistently
// - Simpler code generation logic
//
// To implement derive-based approach would require:
// 1. Analyze all field types to ensure they implement Default
// 2. Detect required fields with non-Default initializations (String::new(), Vec::new(), etc.)
// 3. Add "Default" to struct derives only when ALL fields can use Default::default()
// 4. Skip manual impl generation for those structs
//
#![allow(clippy::derivable_impls)]

pub mod macros;
pub mod metadata;
pub mod primitives;
pub mod datatypes;
pub mod extensions;
pub mod resources;
pub mod profiles;
pub mod traits;
pub mod bindings;

pub use serde::{Deserialize, Serialize};
"#;

    Ok(lib_content.to_string())
}

/// Generate mod.rs files for each module directory
pub fn generate_module_files(
    resource_dir: &Path,
    datatypes_dir: &Path,
    extensions_dir: &Path,
    primitives_dir: &Path,
    traits_dir: &Path,
    bindings_dir: &Path,
    profiles_dir: &Path,
) -> Result<()> {
    // Generate resource/mod.rs
    let resource_mod_content = generate_mod_rs_for_directory(resource_dir, "FHIR resource types")?;
    fs::write(resource_dir.join("mod.rs"), resource_mod_content)?;

    // Generate datatypes/mod.rs
    let datatypes_mod_content = generate_mod_rs_for_directory(datatypes_dir, "FHIR data types")?;
    fs::write(datatypes_dir.join("mod.rs"), datatypes_mod_content)?;

    // Generate extensions/mod.rs
    let extensions_mod_content =
        generate_mod_rs_for_directory(extensions_dir, "FHIR extension types")?;
    fs::write(extensions_dir.join("mod.rs"), extensions_mod_content)?;

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

    // Generate profiles/mod.rs for FHIR profiles
    let profiles_mod_content =
        generate_mod_rs_for_directory(profiles_dir, "FHIR profiles derived from core resources")?;
    fs::write(profiles_dir.join("mod.rs"), profiles_mod_content)?;

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
    fn extensions(&self) -> &[crate::datatypes::extension::Extension];
}

/// Trait for FHIR resources
pub trait Resource {
    /// Get the resource type name
    fn resource_type(&self) -> &'static str;
    
    /// Get the logical id of this resource
    fn id(&self) -> Option<&str>;
    
    /// Get the metadata about this resource
    fn meta(&self) -> Option<&crate::datatypes::meta::Meta>;
}

/// Trait for domain resources (resources that can have narrative)
pub trait DomainResource: Resource + HasExtensions {
    /// Get the narrative text for this domain resource
    fn narrative(&self) -> Option<&crate::datatypes::narrative::Narrative>;
}
"#;

    Ok(content.to_string())
}

/// Generate statistics from organized directories
fn generate_crate_statistics_from_organized_dirs(
    resource_dir: &Path,
    datatypes_dir: &Path,
    extensions_dir: &Path,
    primitives_dir: &Path,
) -> Result<CrateStatistics> {
    let mut num_structs = 0;
    let num_enums = 0; // Enums would be handled separately

    // Count files in each directory
    for dir in [resource_dir, datatypes_dir, extensions_dir, primitives_dir] {
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

    content.push_str("## Important Notice\n\n");
    content
        .push_str("**This crate was automatically generated using the RH codegen CLI tool.**\n\n");
    content.push_str(&format!(
        "- **Generator command**:\n```bash\n{command_invoked}\n```\n\n"
    ));
    content.push_str(&format!("- **Generation timestamp**: {}\n\n", Local::now()));

    content.push_str("## Package Information\n\n");

    content.push_str(&format!("* **Package Name** {package}\n"));
    content.push_str(&format!("* **Package Author** {author}\n"));
    content.push_str(&format!("* **Version** {version}\n"));
    content.push_str(&format!("* **Canonical URL** `{canonical_url}`\n\n"));

    content.push_str(&format!(
        "**Statistics: {} structs, {} enums, {} total types**\n\n",
        stats.num_structs, stats.num_enums, stats.total_types
    ));

    content.push_str(&format!("## Description\n\n{description}"));

    content.push_str("\n\n");

    content.push_str("## Features\n\n");
    content.push_str(
        "- **Complete FHIR type definitions** - All resources, datatypes, and primitives\n",
    );
    content.push_str(
        "- **Serde serialization** - Built-in JSON serialization/deserialization support\n",
    );
    content.push_str(
        "- **Type metadata** - Compile-time metadata for field types and path resolution\n",
    );
    content.push_str(
        "- **Idiomatic Rust** - Clean, organized module structure with proper naming conventions\n",
    );
    content.push_str("- **Zero-cost abstractions** - PHF (perfect hash function) maps for O(1) metadata lookups\n\n");

    content.push_str("## Usage\n\n");
    content.push_str("Add this crate to your `Cargo.toml`:\n\n");
    content.push_str("```toml\n");
    content.push_str("[dependencies]\n");
    content.push_str(&format!("{crate_name} = \"0.1.0\"\n"));
    content.push_str("```\n\n");

    content.push_str("### Deserializing FHIR Resources\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::resources::patient::Patient;\n"));
    content.push_str("use serde_json;\n\n");
    content.push_str("let json_data = r#\"{\\\"resourceType\\\": \\\"Patient\\\", \\\"id\\\": \\\"example\\\"}\"#;\n");
    content.push_str("let patient: Patient = serde_json::from_str(json_data)?;\n\n");
    content.push_str("println!(\"Patient ID: {}\", patient.id.unwrap_or_default());\n");
    content.push_str("```\n\n");

    content.push_str("### Creating Resources Programmatically\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::resources::patient::Patient;\n"));
    content.push_str(&format!(
        "use {crate_name}::datatypes::human_name::HumanName;\n"
    ));
    content.push_str(&format!(
        "use {crate_name}::bindings::administrative_gender::AdministrativeGender;\n"
    ));
    content.push_str(&format!(
        "use {crate_name}::primitives::date::DateType;\n\n"
    ));
    content.push_str("let patient = Patient {\n");
    content.push_str("    id: Some(\"patient-123\".to_string()),\n");
    content.push_str("    active: Some(true),\n");
    content.push_str("    name: vec![HumanName {\n");
    content.push_str("        family: Some(\"Doe\".to_string()),\n");
    content.push_str("        given: vec![\"John\".to_string()],\n");
    content.push_str("        ..Default::default()\n");
    content.push_str("    }],\n");
    content.push_str("    gender: Some(AdministrativeGender::Male),\n");
    content.push_str("    birth_date: Some(\"1990-01-15\".to_string()),\n");
    content.push_str("    ..Default::default()\n");
    content.push_str("};\n");
    content.push_str("```\n\n");

    content.push_str("### Using Type Metadata\n\n");
    content.push_str("This crate includes compile-time metadata for all FHIR types, enabling runtime type introspection and path resolution:\n\n");
    content.push_str("```rust\n");
    content.push_str(&format!("use {crate_name}::metadata::{{resolve_path, get_field_info, FhirFieldType, FhirPrimitiveType}};\n\n"));
    content.push_str("// Resolve nested paths to their FHIR types\n");
    content.push_str("if let Some(field_type) = resolve_path(\"Patient.birthDate\") {\n");
    content.push_str("    match field_type {\n");
    content.push_str("        FhirFieldType::Primitive(FhirPrimitiveType::Date) => {\n");
    content.push_str("            println!(\"birthDate is a FHIR date type\");\n");
    content.push_str("        }\n");
    content.push_str("        _ => {}\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");
    content.push_str("// Resolve complex nested paths\n");
    content.push_str("if let Some(field_type) = resolve_path(\"Patient.name.given\") {\n");
    content.push_str("    match field_type {\n");
    content.push_str("        FhirFieldType::Primitive(FhirPrimitiveType::String) => {\n");
    content.push_str("            println!(\"name.given is a string array\");\n");
    content.push_str("        }\n");
    content.push_str("        _ => {}\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");
    content.push_str("// Get field information directly\n");
    content.push_str("if let Some(field_info) = get_field_info(\"Patient\", \"active\") {\n");
    content.push_str("    println!(\"Min cardinality: {}\", field_info.min);\n");
    content.push_str("    println!(\"Max cardinality: {:?}\", field_info.max);\n");
    content.push_str("    println!(\"Is choice type: {}\", field_info.is_choice_type);\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("The metadata system enables:\n");
    content.push_str("- **Path resolution** - Navigate nested paths like `Patient.name.given`\n");
    content.push_str("- **Type introspection** - Determine field types at runtime\n");
    content.push_str("- **Cardinality information** - Min/max occurrence constraints\n");
    content.push_str("- **Choice type detection** - Identify polymorphic fields\n");
    content
        .push_str("- **Zero runtime cost** - All lookups use compile-time perfect hash maps\n\n");

    content.push_str("## Structure\n\n");
    content.push_str("This crate organizes FHIR types into logical modules:\n\n");
    content.push_str("- **resources/** - All FHIR resources (Patient, Observation, etc.)\n");
    content.push_str(
        "- **datatypes/** - Complex and primitive datatypes (HumanName, Address, etc.)\n",
    );
    content.push_str("- **bindings/** - ValueSet enumerations (AdministrativeGender, etc.)\n");
    content.push_str("- **primitives/** - Base primitive types (DateType, DateTimeType, etc.)\n");
    content.push_str("- **metadata.rs** - Type metadata and path resolution functions\n\n");

    content.push_str("## Regenerating This Crate\n\n");
    content.push_str("To regenerate this crate with updated FHIR definitions:\n\n");
    content.push_str("```bash\n");
    content.push_str(command_invoked);
    content.push_str("\n```\n\n");

    content.push_str("## License\n\n");
    content.push_str("This generated crate is provided under MIT OR Apache-2.0 license.\n\n");

    content.push_str("## Related Links\n\n");
    content.push_str("- [FHIR Specification](https://hl7.org/fhir/)\n");
    content.push_str("- [FHIR Package Registry](https://packages.fhir.org/)\n");
    content.push_str("- [RH Project](https://github.com/reasonhealth/rh)\n\n");
    content.push_str("---\n\n");
    content.push_str(&format!(
        "*Generated by RH codegen tool at {}*\n",
        Local::now()
    ));

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
