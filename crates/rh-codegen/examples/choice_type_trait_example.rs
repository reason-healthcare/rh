//! Example demonstrating choice type trait methods for FHIR resources
//!
//! This example shows how the trait generator creates convenient methods for
//! accessing FHIR choice types (like value[x] and effective[x]) through traits.

use rh_codegen::fhir_types::{
    ElementDefinition, ElementType, StructureDefinition, StructureDefinitionDifferential,
};
use rh_codegen::{CodeGenerator, CodegenConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Create an Observation StructureDefinition with choice types
    let observation_structure = StructureDefinition {
        resource_type: "StructureDefinition".to_string(),
        id: "Observation".to_string(),
        url: "http://hl7.org/fhir/StructureDefinition/Observation".to_string(),
        name: "Observation".to_string(),
        title: Some("Observation".to_string()),
        status: "active".to_string(),
        kind: "resource".to_string(),
        is_abstract: false,
        base_type: "Observation".to_string(),
        base_definition: None,
        version: Some("4.0.1".to_string()),
        description: Some("Measurements and simple assertions made about a patient.".to_string()),
        purpose: None,
        differential: Some(StructureDefinitionDifferential {
            element: vec![
                // Observation.value[x] - can be string, boolean, dateTime, or Period
                ElementDefinition {
                    id: Some("Observation.value[x]".to_string()),
                    path: "Observation.value[x]".to_string(),
                    element_type: Some(vec![
                        ElementType {
                            code: Some("string".to_string()),
                            target_profile: None,
                        },
                        ElementType {
                            code: Some("boolean".to_string()),
                            target_profile: None,
                        },
                        ElementType {
                            code: Some("dateTime".to_string()),
                            target_profile: None,
                        },
                        ElementType {
                            code: Some("Period".to_string()),
                            target_profile: None,
                        },
                    ]),
                    min: Some(0),
                    max: Some("1".to_string()),
                    definition: Some(
                        "The information determined as a result of making the observation."
                            .to_string(),
                    ),
                    short: Some("Actual result".to_string()),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
                // Observation.effective[x] - can be dateTime, Period, or Timing
                ElementDefinition {
                    id: Some("Observation.effective[x]".to_string()),
                    path: "Observation.effective[x]".to_string(),
                    element_type: Some(vec![
                        ElementType {
                            code: Some("dateTime".to_string()),
                            target_profile: None,
                        },
                        ElementType {
                            code: Some("Period".to_string()),
                            target_profile: None,
                        },
                        ElementType {
                            code: Some("Timing".to_string()),
                            target_profile: None,
                        },
                    ]),
                    min: Some(0),
                    max: Some("1".to_string()),
                    definition: Some(
                        "The time or time-period the observed value is asserted as being true."
                            .to_string(),
                    ),
                    short: Some("Clinically relevant time/time-period for observation".to_string()),
                    fixed: None,
                    pattern: None,
                    binding: None,
                },
            ],
        }),
        snapshot: None,
    };

    // Generate the trait
    println!("Generating Observation trait with choice type methods...");
    let observation_trait = generator.generate_trait(&observation_structure)?;

    // Display information about the generated trait
    println!("Generated trait: {}", observation_trait.name);
    println!("Number of methods: {}", observation_trait.methods.len());
    println!();

    // List all the methods in the trait
    println!("Choice type trait methods:");
    for method in &observation_trait.methods {
        if method.name.contains("value") || method.name.contains("effective") {
            println!("- {}", method.name);
            if let Some(doc) = &method.doc_comment {
                println!("  Documentation: {doc}");
            }
            if let Some(return_type) = &method.return_type {
                println!("  Return type: {return_type:?}");
            }
            if method.default_body.is_some() {
                println!("  Has default implementation: Yes");
            }
            println!();
        }
    }

    // Generate the trait to a file
    let output_path = "target/observation_choice_trait_example.rs";
    generator.generate_trait_to_file(&observation_structure, output_path)?;
    println!("Generated trait written to: {output_path}");

    // Read and display the generated choice type methods
    let generated_code = std::fs::read_to_string(output_path)?;

    // Extract just the choice type methods for display
    let lines: Vec<&str> = generated_code.lines().collect();
    let mut in_choice_method = false;
    let mut choice_methods = Vec::new();

    for line in lines {
        if line.contains("fn value")
            || line.contains("fn has_value")
            || line.contains("fn effective")
            || line.contains("fn has_effective")
        {
            in_choice_method = true;
        }

        if in_choice_method {
            choice_methods.push(line);
            if line.trim() == "}" && !line.contains("if") && !line.contains("else") {
                in_choice_method = false;
                choice_methods.push(""); // Add blank line
            }
        }
    }

    println!("\nGenerated choice type methods:");
    println!("{}", "=".repeat(80));
    for line in choice_methods {
        println!("{line}");
    }
    println!("{}", "=".repeat(80));

    println!("\nSummary:");
    println!(
        "✅ Generated {} choice type methods for 'value[x]' field",
        observation_trait
            .methods
            .iter()
            .filter(|m| m.name.contains("value"))
            .count()
    );
    println!(
        "✅ Generated {} choice type methods for 'effective[x]' field",
        observation_trait
            .methods
            .iter()
            .filter(|m| m.name.contains("effective"))
            .count()
    );
    println!("✅ Each choice type gets:");
    println!("   - Generic getter method (returns formatted string)");
    println!("   - Type checker method (has_* returns bool)");
    println!("   - Type-specific getters for each variant");
    println!("✅ All methods have default implementations in the trait");

    Ok(())
}
