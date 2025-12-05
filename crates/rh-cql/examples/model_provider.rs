//! Example: Using ModelInfo Providers
//!
//! This example demonstrates how to use ModelInfo providers to resolve
//! CQL types and access model information for FHIR resources.
//!
//! Run with: `cargo run --example model_provider`

use rh_cql::modelinfo::{ClassInfo, ClassInfoElement, ModelInfo, SimpleTypeInfo, TypeInfo};
use rh_cql::{fhir_r4_model_info, fhir_r4_provider, MemoryModelInfoProvider, ModelInfoProvider};

fn main() -> anyhow::Result<()> {
    println!("=== ModelInfo Provider Example ===\n");

    // Example 1: Using the built-in FHIR R4 provider
    println!("1. Using built-in FHIR R4 provider:");
    let provider = fhir_r4_provider();

    // Check what models are available
    println!("   Available models: {:?}", provider.list_models());
    println!("   Model count: {}", provider.model_count());

    // Get the FHIR model
    if let Some(model) = provider.get_model("FHIR", Some("4.0.1")) {
        println!("   FHIR model name: {:?}", model.name);
        println!("   FHIR model version: {:?}", model.version);
        println!("   Patient class name: {:?}", model.patient_class_name);
        println!("   Type count: {}", model.type_info.len());
    }

    // Example 2: Resolving types
    println!("\n2. Resolving types:");

    // Resolve Patient class
    if let Some(patient) = provider.resolve_class("FHIR", Some("4.0.1"), "Patient") {
        println!("   Found Patient class:");
        println!("      Name: {:?}", patient.name);
        println!("      Namespace: {:?}", patient.namespace);
        println!("      Base type: {:?}", patient.base_type);
        println!("      Retrievable: {:?}", patient.retrievable);
        println!(
            "      Elements: {:?}",
            patient
                .element
                .iter()
                .map(|e| e.name.clone())
                .collect::<Vec<_>>()
        );
    }

    // Resolve Observation with primary code path
    if let Some(obs) = provider.resolve_class("FHIR", Some("4.0.1"), "Observation") {
        println!("\n   Found Observation class:");
        println!("      Primary code path: {:?}", obs.primary_code_path);
    }

    // Resolve by qualified name
    if provider
        .resolve_type("FHIR", Some("4.0.1"), "FHIR.Patient")
        .is_some()
    {
        println!("\n   Resolved 'FHIR.Patient' by qualified name");
    }

    // Example 3: Getting patient class via helper
    println!("\n3. Getting patient class:");
    if let Some(patient_class) = provider.get_patient_class("FHIR", Some("4.0.1")) {
        println!(
            "   Patient class has {} elements",
            patient_class.element.len()
        );
    }

    // Example 4: Creating a custom model
    println!("\n4. Creating a custom model:");
    let custom_provider = MemoryModelInfoProvider::new();

    // Create a simple custom model
    let custom_model = ModelInfo {
        name: Some("MyModel".to_string()),
        version: Some("1.0.0".to_string()),
        url: Some("http://example.org/mymodel".to_string()),
        patient_class_name: Some("Person".to_string()),
        type_info: vec![
            TypeInfo::SimpleTypeInfo(SimpleTypeInfo {
                namespace: Some("MyModel".to_string()),
                name: Some("Identifier".to_string()),
                base_type: Some("System.String".to_string()),
                ..Default::default()
            }),
            TypeInfo::ClassInfo(ClassInfo {
                namespace: Some("MyModel".to_string()),
                name: Some("Person".to_string()),
                retrievable: Some(true),
                element: vec![
                    ClassInfoElement {
                        name: Some("id".to_string()),
                        element_type: Some("MyModel.Identifier".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("name".to_string()),
                        element_type: Some("System.String".to_string()),
                        ..Default::default()
                    },
                    ClassInfoElement {
                        name: Some("birthDate".to_string()),
                        element_type: Some("System.Date".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    };

    custom_provider.register_model(custom_model);
    println!("   Registered custom model 'MyModel'");

    if let Some(person) = custom_provider.resolve_class("MyModel", Some("1.0.0"), "Person") {
        println!(
            "   Resolved Person class with {} elements",
            person.element.len()
        );
    }

    // Example 5: Provider with statistics
    println!("\n5. Provider with statistics tracking:");
    let stats_provider = MemoryModelInfoProvider::with_stats();
    stats_provider.register_model(fhir_r4_model_info());

    // Make some lookups
    stats_provider.get_model("FHIR", Some("4.0.1"));
    stats_provider.resolve_type("FHIR", Some("4.0.1"), "Patient");
    stats_provider.resolve_type("FHIR", Some("4.0.1"), "Observation");
    stats_provider.get_model("NonExistent", None); // miss

    let stats = stats_provider.stats();
    println!("   Cache hits: {}", stats.hits);
    println!("   Cache misses: {}", stats.misses);
    println!("   Hit rate: {:.1}%", stats.hit_rate() * 100.0);

    // Example 6: Multiple model versions
    println!("\n6. Multiple model versions:");
    let multi_provider = MemoryModelInfoProvider::new();

    multi_provider.register_model(ModelInfo {
        name: Some("FHIR".to_string()),
        version: Some("3.0.2".to_string()),
        url: Some("http://hl7.org/fhir/STU3".to_string()),
        ..Default::default()
    });

    multi_provider.register_model(ModelInfo {
        name: Some("FHIR".to_string()),
        version: Some("4.0.1".to_string()),
        url: Some("http://hl7.org/fhir/R4".to_string()),
        ..Default::default()
    });

    println!("   Models: {:?}", multi_provider.list_models());
    println!(
        "   Has FHIR 3.0.2: {}",
        multi_provider.has_model("FHIR", Some("3.0.2"))
    );
    println!(
        "   Has FHIR 4.0.1: {}",
        multi_provider.has_model("FHIR", Some("4.0.1"))
    );
    println!(
        "   Has FHIR (any): {}",
        multi_provider.has_model("FHIR", None)
    );

    println!("\n=== Example Complete ===");
    Ok(())
}
