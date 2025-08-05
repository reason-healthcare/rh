//! Test example for choice type generation
//!
//! This example tests that FHIR choice types like effective[x] and value[x]
//! are properly expanded into multiple typed fields.

use anyhow::Result;
use rh_codegen::{CodeGenerator, CodegenConfig, TokenGenerator};

fn main() -> Result<()> {
    println!("Testing choice type generation...");
    
    // Create a basic codegen configuration
    let config = CodegenConfig::default();
    let mut generator = CodeGenerator::new(config);

    // Create a test structure definition with choice types
    let test_structure = create_observation_structure_definition();

    println!("Generating struct for Observation with choice types...");
    
    // Generate the struct
    let rust_struct = generator.generate_struct(&test_structure)?;
    
    // Generate tokens and pretty-print
    let token_generator = TokenGenerator::new();
    let tokens = token_generator.generate_struct(&rust_struct);
    let formatted_code = prettyplease::unparse(&syn::parse2(tokens)?);
    
    println!("Generated code:\n{}", formatted_code);
    
    // Check that choice type fields were generated
    let field_names: Vec<&str> = rust_struct.fields.iter().map(|f| f.name.as_str()).collect();
    
    println!("\nGenerated fields:");
    for field_name in &field_names {
        println!("  - {}", field_name);
    }
    
    // Verify that choice type fields exist
    let has_effective_date_time = field_names.contains(&"effective_date_time");
    let has_effective_period = field_names.contains(&"effective_period");
    let has_effective_timing = field_names.contains(&"effective_timing");
    let has_effective_instant = field_names.contains(&"effective_instant");
    
    let has_value_quantity = field_names.contains(&"value_quantity");
    let has_value_codeable_concept = field_names.contains(&"value_codeable_concept");
    let has_value_string = field_names.contains(&"value_string");
    
    println!("\nChoice type verification:");
    println!("  effective[x] fields:");
    println!("    effective_date_time: {}", has_effective_date_time);
    println!("    effective_period: {}", has_effective_period);
    println!("    effective_timing: {}", has_effective_timing);
    println!("    effective_instant: {}", has_effective_instant);
    
    println!("  value[x] fields:");
    println!("    value_quantity: {}", has_value_quantity);
    println!("    value_codeable_concept: {}", has_value_codeable_concept);
    println!("    value_string: {}", has_value_string);

    if has_effective_date_time && has_effective_period && has_value_quantity && has_value_string {
        println!("\n✅ Choice type generation working correctly!");
    } else {
        println!("\n❌ Choice type generation not working as expected");
    }

    Ok(())
}

fn create_observation_structure_definition() -> rh_codegen::StructureDefinition {
    use serde_json::json;
    
    let json_data = json!({
        "resourceType": "StructureDefinition",
        "id": "Observation",
        "url": "http://hl7.org/fhir/StructureDefinition/Observation",
        "version": "4.0.1",
        "name": "Observation",
        "title": "Observation",
        "status": "active",
        "description": "Measurements and simple assertions made about a patient, device or other subject.",
        "kind": "resource",
        "abstract": false,
        "type": "Observation",
        "baseDefinition": "http://hl7.org/fhir/StructureDefinition/DomainResource",
        "snapshot": {
            "element": [
                {
                    "path": "Observation",
                    "short": "Measurements and simple assertions",
                    "definition": "Measurements and simple assertions made about a patient, device or other subject.",
                    "min": 0,
                    "max": "*",
                    "type": [{"code": "DomainResource"}]
                },
                {
                    "path": "Observation.status",
                    "short": "registered | preliminary | final | amended +",
                    "definition": "The status of the result value.",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "code"}]
                },
                {
                    "path": "Observation.code",
                    "short": "Type of observation (code / type)",
                    "definition": "Describes what was observed. Sometimes this is called the observation 'name'.",
                    "min": 1,
                    "max": "1",
                    "type": [{"code": "CodeableConcept"}]
                },
                {
                    "path": "Observation.effective[x]",
                    "short": "Clinically relevant time/time-period for observation",
                    "definition": "The time or time-period the observed value is asserted as being true.",
                    "min": 0,
                    "max": "1",
                    "type": [
                        {"code": "dateTime"},
                        {"code": "Period"},
                        {"code": "Timing"},
                        {"code": "instant"}
                    ]
                },
                {
                    "path": "Observation.value[x]",
                    "short": "Actual result",
                    "definition": "The information determined as a result of making the observation, if the information has a simple value.",
                    "min": 0,
                    "max": "1",
                    "type": [
                        {"code": "Quantity"},
                        {"code": "CodeableConcept"},
                        {"code": "string"},
                        {"code": "boolean"},
                        {"code": "integer"},
                        {"code": "Range"},
                        {"code": "Ratio"},
                        {"code": "SampledData"},
                        {"code": "time"},
                        {"code": "dateTime"},
                        {"code": "Period"}
                    ]
                }
            ]
        }
    });
    
    serde_json::from_value(json_data).expect("Failed to parse StructureDefinition")
}
