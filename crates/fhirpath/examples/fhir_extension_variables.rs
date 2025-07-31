//! FHIR Extension Variables Examples
//!
//! This example demonstrates how to use FHIR-specific extension variables
//! like %resource, %ucum, %sct, and %loinc in FHIRPath expressions.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 FHIR Extension Variables Examples");
    println!("====================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Example FHIR Observation resource
    let observation = json!({
        "resourceType": "Observation",
        "id": "example-vital-signs",
        "status": "final",
        "category": [{
            "coding": [{
                "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                "code": "vital-signs",
                "display": "Vital Signs"
            }]
        }],
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "8867-4",
                "display": "Heart rate"
            }]
        },
        "subject": {
            "reference": "Patient/example"
        },
        "valueQuantity": {
            "value": 72,
            "unit": "beats/min",
            "system": "http://unitsofmeasure.org",
            "code": "/min"
        }
    });

    let context = EvaluationContext::new(observation);

    println!("📋 Testing with Observation resource\n");

    // Example 1: %resource variable - access to current resource
    println!("🏥 Example 1: %resource Variable");
    println!("--------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.resourceType",
        "Get the resource type using %resource",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.id",
        "Get the resource ID using %resource",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.status",
        "Get the observation status using %resource",
    )?;

    // Example 2: Using %resource in complex expressions
    println!("\n🔍 Example 2: %resource in Complex Expressions");
    println!("----------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.code.coding.where(system = %loinc)",
        "Find LOINC codings using %resource and %loinc",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.valueQuantity.where(system = %ucum)",
        "Find UCUM quantities using %resource and %ucum",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.category.coding.exists(system.contains('hl7.org'))",
        "Check for HL7 category systems using %resource",
    )?;

    // Example 3: Terminology system variables
    println!("\n📚 Example 3: Terminology System Variables");
    println!("-----------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%ucum",
        "UCUM units of measure system URL",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%loinc",
        "LOINC laboratory coding system URL",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%sct",
        "SNOMED CT clinical terminology system URL",
    )?;

    // Example 4: Comparing with terminology systems
    println!("\n🔬 Example 4: Comparing with Terminology Systems");
    println!("------------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "code.coding.where(system = %loinc).exists()",
        "Check if observation has LOINC coding",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "valueQuantity.system = %ucum",
        "Check if quantity uses UCUM system",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "code.coding.where(system = %sct).exists()",
        "Check if observation has SNOMED CT coding",
    )?;

    // Example 5: Building dynamic queries
    println!("\n⚡ Example 5: Dynamic Queries with Variables");
    println!("-------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "code.coding.where(system in (%loinc | %sct)).code",
        "Get codes from standard terminologies",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.descendants().where($this.system in (%ucum | %loinc | %sct))",
        "Find all elements using standard systems",
    )?;

    // Example 6: Practical healthcare scenarios
    println!("\n🏥 Example 6: Practical Healthcare Scenarios");
    println!("-------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.code.coding.where(system = %loinc and code = '8867-4').exists()",
        "Check for specific LOINC heart rate code",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.valueQuantity.where(system = %ucum).value",
        "Get numeric value from UCUM quantity",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.category.coding.where(system.startsWith('http://terminology.hl7.org')).code",
        "Get HL7 category codes",
    )?;

    println!("\n✅ FHIR Extension Variables Examples Complete!");
    println!("These examples show how to:");
    println!("• Use %resource to access the current FHIR resource");
    println!("• Reference standard terminology systems with %ucum, %loinc, %sct");
    println!("• Build dynamic queries using extension variables");
    println!("• Create practical healthcare data validation expressions");
    println!("• Combine variables with standard FHIRPath functions");

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
