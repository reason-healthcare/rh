//! FHIR Extension Function Examples
//!
//! This example demonstrates how to use FHIR-specific extension functions
//! like extension() and hasValue() to work with FHIR extensions.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏥 FHIR Extension Function Examples");
    println!("===================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Example 1: Patient with multiple extensions
    let patient_with_extensions = json!({
        "resourceType": "Patient",
        "id": "example-001",
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2106-3",
                            "display": "White"
                        }
                    }
                ]
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2186-5",
                            "display": "Not Hispanic or Latino"
                        }
                    }
                ]
            },
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-nickname",
                "valueString": "Johnny"
            },
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-birthCountry",
                "valueString": "United States"
            }
        ],
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["John", "Edward"]
        }]
    });

    let context = EvaluationContext::new(patient_with_extensions);

    println!("📋 Testing with Patient resource with multiple extensions\n");

    // Example 1: Find all extensions
    println!("🔍 Example 1: Finding Extensions by URL");
    println!("--------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-nickname')",
        "Find patient nickname extension",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')",
        "Find US Core race extension",
    )?;

    // Example 2: Extract extension values
    println!("\n💎 Example 2: Extracting Extension Values");
    println!("----------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-nickname').valueString",
        "Get nickname value",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-birthCountry').valueString",
        "Get birth country value",
    )?;

    // Example 3: Check if extensions have values
    println!("\n✅ Example 3: Checking Extension Values with hasValue()");
    println!("----------------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-nickname')[0].hasValue()",
        "Check if nickname extension has a value",
    )?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-birthCountry')[0].hasValue()",
        "Check if birth country extension has a value")?;

    // Example 4: Working with complex extensions
    println!("\n🔬 Example 4: Complex Extensions (nested extensions)");
    println!("--------------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension",
        "Get nested extensions from race extension",
    )?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'ombCategory').valueCoding.display",
        "Get race display value from nested extension")?;

    // Example 5: Extensions that don't exist
    println!("\n❌ Example 5: Non-existent Extensions");
    println!("------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/nonexistent')",
        "Try to find non-existent extension",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension('http://example.org/nonexistent').count()",
        "Count non-existent extensions",
    )?;

    println!("\n✅ FHIR Extension Function Examples Complete!");
    println!("These examples show how to:");
    println!("• Find extensions by URL using extension() function");
    println!("• Extract values from extensions");
    println!("• Check if extensions have values using hasValue()");
    println!("• Work with complex nested extensions");
    println!("• Handle non-existent extensions gracefully");

    Ok(())
}

/// Helper function to test and display FHIRPath expressions
fn test_expression(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Expression: {expression}");
    println!("  Purpose: {description}");

    match parser.parse(expression) {
        Ok(parsed) => match evaluator.evaluate(&parsed, context) {
            Ok(result) => {
                println!("  Result: {result:?}\n");
            }
            Err(e) => {
                println!("  Error: {e:?}\n");
            }
        },
        Err(e) => {
            println!("  Parse Error: {e:?}\n");
        }
    }

    Ok(())
}
