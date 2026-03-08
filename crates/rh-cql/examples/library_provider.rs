//! Example: Using Library Source Providers
//!
//! This example demonstrates how to use library source providers
//! for CQL library dependency resolution.
//!
//! Run with: `cargo run --example library_provider`

use rh_cql::library::{
    CompositeLibrarySourceProvider, FileLibrarySourceProvider, LibraryIdentifier, LibrarySource,
    LibrarySourceProvider, MemoryLibrarySourceProvider,
};

fn main() -> anyhow::Result<()> {
    println!("=== Library Source Provider Example ===\n");

    // Example 1: Using MemoryLibrarySourceProvider
    println!("1. Using MemoryLibrarySourceProvider:");

    let memory_provider = MemoryLibrarySourceProvider::new();

    // Register some CQL libraries
    let fhir_helpers = r#"
library FHIRHelpers version '4.0.1'

using FHIR version '4.0.1'

// Conversion functions for FHIR types
define function ToQuantity(value FHIR.Quantity):
  System.Quantity { value: value.value, unit: value.unit.value }

define function ToInterval(value FHIR.Period):
  Interval[value.start, value.end]

define function ToCode(value FHIR.Coding):
  Code { code: value.code, system: value.system, display: value.display }
"#;

    let common_lib = r#"
library Common version '1.0.0'

using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'

context Patient

define "Active Conditions":
  [Condition] C
    where C.clinicalStatus ~ 'active'
"#;

    // Register libraries with identifiers
    memory_provider.register_source(
        LibraryIdentifier::new("FHIRHelpers", Some("4.0.1")),
        fhir_helpers.to_string(),
    );
    memory_provider.register_source(
        LibraryIdentifier::new("Common", Some("1.0.0")),
        common_lib.to_string(),
    );

    println!(
        "   Registered {} libraries",
        memory_provider.library_count()
    );

    // List all libraries
    println!("   Available libraries:");
    for lib_id in memory_provider.list_libraries() {
        println!("      - {lib_id}");
    }

    // Look up by exact version
    let fhir_id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
    if let Some(source) = memory_provider.get_source(&fhir_id) {
        println!("\n   Found FHIRHelpers:");
        println!("      Identifier: {}", source.identifier);
        println!("      Source length: {} bytes", source.source.len());
        println!(
            "      First line: {}",
            source.source.lines().next().unwrap_or("")
        );
    }

    // Look up without version (finds any matching library)
    let common_unversioned = LibraryIdentifier::unversioned("Common");
    if let Some(source) = memory_provider.get_source(&common_unversioned) {
        println!("\n   Found Common (any version):");
        println!("      Resolved to: {}", source.identifier);
    }

    // Example 2: LibraryIdentifier operations
    println!("\n2. LibraryIdentifier operations:");

    let id1 = LibraryIdentifier::new("MyLibrary", Some("2.0.0"));
    let id2 = LibraryIdentifier::unversioned("MyLibrary");

    println!("   id1 (versioned): {id1}");
    println!("   id1.to_key(): {}", id1.to_key());
    println!("   id2 (unversioned): {id2}");
    println!("   id2.to_key(): {}", id2.to_key());

    // Matching logic
    println!("\n   Matching:");
    println!(
        "      id2.matches(&id1): {} (unversioned matches any version)",
        id2.matches(&id1)
    );
    println!(
        "      id1.matches(&id2): {} (versioned doesn't match unversioned)",
        id1.matches(&id2)
    );

    // Example 3: FileLibrarySourceProvider
    println!("\n3. FileLibrarySourceProvider:");

    let file_provider = FileLibrarySourceProvider::new()
        .with_path("./cql")
        .with_path("./libs")
        .with_extension("cql");

    println!("   Search paths: {:?}", file_provider.paths());

    // Show how it generates filenames
    let versioned_id = LibraryIdentifier::new("MeasureLib", Some("1.2.3"));
    let filenames = file_provider.possible_filenames(&versioned_id);
    println!("   For '{versioned_id}', searches: {filenames:?}");

    // Example 4: CompositeLibrarySourceProvider
    println!("\n4. CompositeLibrarySourceProvider:");

    // Create a layered provider: memory first, then filesystem
    let composite = CompositeLibrarySourceProvider::new()
        .add_provider(memory_provider.clone())
        .add_provider(file_provider);

    println!("   Created composite with memory + file providers");
    println!("   Libraries from composite:");
    for lib_id in composite.list_libraries() {
        println!("      - {lib_id}");
    }

    // Lookup will check memory first
    if composite.has_library(&fhir_id) {
        println!("\n   Found FHIRHelpers in composite (from memory)");
    }

    // Example 5: Using LibrarySource directly
    println!("\n5. LibrarySource with location metadata:");

    let source_with_location = LibrarySource::new(
        LibraryIdentifier::new("TrackedLib", Some("1.0")),
        "library TrackedLib version '1.0'",
        Some("/project/cql/TrackedLib-1.0.cql"),
    );

    println!("   Library: {}", source_with_location.identifier);
    println!(
        "   Location: {:?}",
        source_with_location.location.as_deref().unwrap_or("(none)")
    );

    // Register with location
    let tracked_provider = MemoryLibrarySourceProvider::new();
    tracked_provider.register(source_with_location);

    let retrieved = tracked_provider
        .get_source(&LibraryIdentifier::new("TrackedLib", Some("1.0")))
        .unwrap();
    println!(
        "   Retrieved location: {:?}",
        retrieved.location.as_deref().unwrap_or("(none)")
    );

    // Example 6: Finding multiple versions
    println!("\n6. Finding multiple versions:");

    let multi_version_provider = MemoryLibrarySourceProvider::new();
    multi_version_provider.register_source(
        LibraryIdentifier::new("Versioned", Some("1.0")),
        "library Versioned version '1.0'".to_string(),
    );
    multi_version_provider.register_source(
        LibraryIdentifier::new("Versioned", Some("2.0")),
        "library Versioned version '2.0'".to_string(),
    );
    multi_version_provider.register_source(
        LibraryIdentifier::new("Versioned", Some("3.0-beta")),
        "library Versioned version '3.0-beta'".to_string(),
    );

    let versions = multi_version_provider.find_by_name("Versioned");
    println!("   Found {} versions of 'Versioned':", versions.len());
    for v in versions {
        println!("      - {v}");
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
