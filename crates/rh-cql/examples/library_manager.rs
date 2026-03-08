//! Example: Using LibraryManager
//!
//! This example demonstrates how to use LibraryManager to resolve
//! and manage CQL library dependencies.
//!
//! Run with: `cargo run --example library_manager`

use rh_cql::elm::{IncludeDef, IncludeDefs, Library, VersionedIdentifier};
use rh_cql::library::{
    CompiledLibrary, LibraryIdentifier, LibraryManager, MemoryLibrarySourceProvider,
};

fn main() -> anyhow::Result<()> {
    println!("=== LibraryManager Example ===\n");

    // Create a source provider with library source code
    let provider = MemoryLibrarySourceProvider::new();

    // Register some CQL libraries
    provider.register_source(
        LibraryIdentifier::new("FHIRHelpers", Some("4.0.1")),
        r#"
library FHIRHelpers version '4.0.1'

using FHIR version '4.0.1'

define function ToQuantity(value FHIR.Quantity): System.Quantity {
  System.Quantity { value: value.value, unit: value.unit }
}
"#
        .to_string(),
    );

    provider.register_source(
        LibraryIdentifier::new("CommonLogic", Some("1.0.0")),
        r#"
library CommonLogic version '1.0.0'

using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'

define function IsAdult(birthDate Date): Boolean {
  CalculateAgeInYearsAt(birthDate, Today()) >= 18
}
"#
        .to_string(),
    );

    provider.register_source(
        LibraryIdentifier::new("MyMeasure", Some("1.0.0")),
        r#"
library MyMeasure version '1.0.0'

using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'
include CommonLogic version '1.0.0'

parameter MeasurementPeriod Interval<DateTime>

context Patient

define "In Initial Population":
  IsAdult(Patient.birthDate)
"#
        .to_string(),
    );

    // Create library manager
    let manager = LibraryManager::new(provider);

    // Example 1: Check source availability
    println!("1. Source Availability:");
    let helpers_id = LibraryIdentifier::new("FHIRHelpers", Some("4.0.1"));
    let measure_id = LibraryIdentifier::new("MyMeasure", Some("1.0.0"));
    let nonexistent_id = LibraryIdentifier::new("NonExistent", Some("1.0"));

    println!(
        "   FHIRHelpers available: {}",
        manager.has_source(&helpers_id)
    );
    println!(
        "   MyMeasure available: {}",
        manager.has_source(&measure_id)
    );
    println!(
        "   NonExistent available: {}",
        manager.has_source(&nonexistent_id)
    );

    // Example 2: Resolve a library
    println!("\n2. Resolving Libraries:");
    match manager.resolve(&helpers_id) {
        Ok(library) => {
            println!(
                "   Resolved: {} v{:?}",
                library.name().unwrap_or("?"),
                library.version()
            );
        }
        Err(e) => println!("   Error: {e}"),
    }

    // Example 3: Check cache
    println!("\n3. Cache Status:");
    println!("   FHIRHelpers cached: {}", manager.is_cached(&helpers_id));

    let cached = manager.cached_libraries();
    println!("   Cached libraries: {}", cached.len());
    for lib in &cached {
        println!("      - {lib}");
    }

    // Example 4: Resolve with error handling
    println!("\n4. Error Handling:");
    match manager.resolve(&nonexistent_id) {
        Ok(_) => println!("   Found (unexpected!)"),
        Err(e) => println!("   Expected error: {e}"),
    }

    // Example 5: Dependency graph (using pre-cached libraries with includes)
    println!("\n5. Dependency Graph:");

    // Pre-cache libraries with their include information
    let fhir_helpers = CompiledLibrary::new(Library {
        identifier: Some(VersionedIdentifier {
            id: Some("FHIRHelpers".to_string()),
            version: Some("4.0.1".to_string()),
            system: None,
        }),
        ..Default::default()
    });
    manager.cache_library(&helpers_id, fhir_helpers);

    let common_id = LibraryIdentifier::new("CommonLogic", Some("1.0.0"));
    let common_logic = CompiledLibrary::new(Library {
        identifier: Some(VersionedIdentifier {
            id: Some("CommonLogic".to_string()),
            version: Some("1.0.0".to_string()),
            system: None,
        }),
        includes: Some(IncludeDefs {
            defs: vec![IncludeDef {
                local_identifier: Some("FHIRHelpers".to_string()),
                path: Some("FHIRHelpers".to_string()),
                version: Some("4.0.1".to_string()),
            }],
        }),
        ..Default::default()
    });
    manager.cache_library(&common_id, common_logic);

    let my_measure = CompiledLibrary::new(Library {
        identifier: Some(VersionedIdentifier {
            id: Some("MyMeasure".to_string()),
            version: Some("1.0.0".to_string()),
            system: None,
        }),
        includes: Some(IncludeDefs {
            defs: vec![
                IncludeDef {
                    local_identifier: Some("FHIRHelpers".to_string()),
                    path: Some("FHIRHelpers".to_string()),
                    version: Some("4.0.1".to_string()),
                },
                IncludeDef {
                    local_identifier: Some("Common".to_string()),
                    path: Some("CommonLogic".to_string()),
                    version: Some("1.0.0".to_string()),
                },
            ],
        }),
        ..Default::default()
    });
    manager.cache_library(&measure_id, my_measure);

    let graph = manager.dependency_graph(&measure_id);
    println!("   Graph for MyMeasure:");
    for (lib, deps) in &graph {
        let dep_names: Vec<_> = deps.iter().map(|d| d.to_string()).collect();
        if dep_names.is_empty() {
            println!("      {lib} -> (none)");
        } else {
            println!("      {lib} -> [{}]", dep_names.join(", "));
        }
    }

    // Example 6: Topological sort
    println!("\n6. Topological Sort (compilation order):");
    match manager.topological_sort(&measure_id) {
        Ok(sorted) => {
            for (i, lib) in sorted.iter().enumerate() {
                println!("      {}. {lib}", i + 1);
            }
        }
        Err(e) => println!("   Error: {e}"),
    }

    // Example 7: Cycle detection
    println!("\n7. Cycle Detection:");
    match manager.detect_cycle(&measure_id) {
        Some(cycle) => {
            let cycle_str: Vec<_> = cycle.iter().map(|id| id.to_string()).collect();
            println!("   Cycle found: {}", cycle_str.join(" -> "));
        }
        None => println!("   No cycles detected"),
    }

    // Example 8: Create a cycle and detect it
    println!("\n8. Detecting Circular Dependencies:");

    // Create two libraries that depend on each other
    let cycle_a_id = LibraryIdentifier::new("CycleA", Some("1.0"));
    let cycle_b_id = LibraryIdentifier::new("CycleB", Some("1.0"));

    manager.cache_library(
        &cycle_a_id,
        CompiledLibrary::new(Library {
            identifier: Some(VersionedIdentifier {
                id: Some("CycleA".to_string()),
                version: Some("1.0".to_string()),
                system: None,
            }),
            includes: Some(IncludeDefs {
                defs: vec![IncludeDef {
                    local_identifier: Some("CycleB".to_string()),
                    path: Some("CycleB".to_string()),
                    version: Some("1.0".to_string()),
                }],
            }),
            ..Default::default()
        }),
    );

    manager.cache_library(
        &cycle_b_id,
        CompiledLibrary::new(Library {
            identifier: Some(VersionedIdentifier {
                id: Some("CycleB".to_string()),
                version: Some("1.0".to_string()),
                system: None,
            }),
            includes: Some(IncludeDefs {
                defs: vec![IncludeDef {
                    local_identifier: Some("CycleA".to_string()),
                    path: Some("CycleA".to_string()),
                    version: Some("1.0".to_string()),
                }],
            }),
            ..Default::default()
        }),
    );

    match manager.detect_cycle(&cycle_a_id) {
        Some(cycle) => {
            let cycle_str: Vec<_> = cycle.iter().map(|id| id.to_string()).collect();
            println!("   Cycle detected: {}", cycle_str.join(" -> "));
        }
        None => println!("   No cycle (unexpected!)"),
    }

    // Example 9: Cache statistics
    println!("\n9. Cache Statistics:");
    let stats = manager.cache_stats();
    println!("   Insertions: {}", stats.insertions);
    println!("   Hits: {}", stats.hits);
    println!("   Misses: {}", stats.misses);
    println!("   Cached count: {}", manager.cached_libraries().len());

    // Example 10: Cache operations
    println!("\n10. Cache Operations:");
    println!(
        "   Before invalidate - FHIRHelpers cached: {}",
        manager.is_cached(&helpers_id)
    );
    manager.invalidate(&helpers_id);
    println!(
        "   After invalidate - FHIRHelpers cached: {}",
        manager.is_cached(&helpers_id)
    );

    println!(
        "   Before clear - cached count: {}",
        manager.cached_libraries().len()
    );
    manager.clear_cache();
    println!(
        "   After clear - cached count: {}",
        manager.cached_libraries().len()
    );

    println!("\n=== Example Complete ===");
    Ok(())
}
