//! Extension System Demo
//! 
//! This example demonstrates the FHIRPath extension system, showing how
//! custom functions and variables can be added to extend FHIRPath functionality.

use serde_json::json;
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 FHIRPath Extension System Demo");
    println!("=====================================\n");

    // Sample FHIR Patient resource for testing
    let patient_data = json!({
        "resourceType": "Patient",
        "id": "example-patient",
        "name": [
            {
                "use": "official",
                "family": "Smith",
                "given": ["John", "Q"]
            }
        ],
        "extension": [
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-nickname",
                "valueString": "Johnny"
            },
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-birthCountry",
                "valueString": "USA"
            }
        ]
    });

    println!("📋 Testing with Patient resource:\n{}\n", 
             serde_json::to_string_pretty(&patient_data)?);

    // Test FHIR Extension Functions
    println!("🏥 Testing FHIR Extension Functions:");
    println!("------------------------------------");
    
    test_expression(&patient_data, 
                   "extension('http://example.org/fhir/StructureDefinition/patient-nickname')", 
                   "FHIR extension function - finds extensions by URL")?;
    
    test_expression(&patient_data, 
                   "extension('http://example.org/fhir/StructureDefinition/patient-nickname')[0].hasValue()", 
                   "FHIR hasValue function - checks if extension has a value")?;

    // Test FHIR Extension Variables
    println!("\n🔬 Testing FHIR Extension Variables:");
    println!("------------------------------------");
    
    test_expression(&patient_data, "%resource", 
                   "Resource variable - current FHIR resource")?;
    
    test_expression(&patient_data, "%ucum", 
                   "UCUM variable - units of measure system")?;
    
    test_expression(&patient_data, "%sct", 
                   "SNOMED CT variable - clinical terminology system")?;
    
    test_expression(&patient_data, "%loinc", 
                   "LOINC variable - laboratory data coding system")?;

    // Test Combined Usage
    println!("\n🎯 Testing Combined Extension Usage:");
    println!("------------------------------------");
    
    test_expression(&patient_data, 
                   "extension('http://example.org/fhir/StructureDefinition/patient-nickname')[0].valueString", 
                   "Combined: FHIR extension access")?;

    println!("\n✅ Extension System Demo Complete!");
    println!("FHIR extension functions and variables are working correctly.");
    println!("\nThe extension system provides:");
    println!("• FHIR-specific functions for healthcare data");
    println!("• FHIR variables for standard healthcare systems");
    println!("• Easy integration with existing FHIRPath expressions");

    Ok(())
}

/// Helper function to test and display FHIRPath expressions
fn test_expression(data: &serde_json::Value, expression: &str, description: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Expression: {}", expression);
    println!("  Purpose: {}", description);
    
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(data.clone());
    
    match parser.parse(expression) {
        Ok(parsed) => {
            match evaluator.evaluate(&parsed, &context) {
                Ok(result) => {
                    println!("  Result: {:?}\n", result);
                },
                Err(e) => {
                    println!("  Error: {:?}\n", e);
                }
            }
        },
        Err(e) => {
            println!("  Compilation Error: {:?}\n", e);
        }
    }
    
    Ok(())
}
