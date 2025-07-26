//! Array Indexing Demo
//!
//! This example demonstrates FHIRPath array indexing functionality, 
//! showing how to access elements from collections using index notation.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 FHIRPath Array Indexing Demo");
    println!("================================\n");

    // Create a sample FHIR Patient with multiple name entries
    let patient = json!({
        "resourceType": "Patient",
        "id": "demo-patient",
        "name": [
            {
                "use": "official",
                "family": "Smith",
                "given": ["John", "William"]
            },
            {
                "use": "usual",
                "family": "Smith", 
                "given": ["Johnny"]
            },
            {
                "use": "maiden",
                "family": "Johnson",
                "given": ["Jane"]
            }
        ],
        "birthDate": "1985-06-15"
    });

    println!("📄 Sample Patient Data:");
    println!("{}\n", serde_json::to_string_pretty(&patient)?);

    // Set up the FHIRPath parser and evaluator
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(patient);

    // Test cases for array indexing
    let test_cases = vec![
        ("name[0]", "Access first name entry"),
        ("name[1]", "Access second name entry"),
        ("name[2]", "Access third name entry"),
        ("name[10]", "Out of bounds access (should be empty)"),
        ("name[0].given[0]", "Access first given name from first entry"),
        ("name[0].given[1]", "Access second given name from first entry"),
        ("name[1].given[0]", "Access first given name from second entry"),
        ("name[0].family", "Access family name from first entry"),
    ];

    for (expression, description) in test_cases {
        println!("🧪 Testing: {expression}");
        println!("   Description: {description}");
        
        match parser.parse(expression) {
            Ok(expr) => {
                match evaluator.evaluate(&expr, &context) {
                    Ok(result) => {
                        println!("   ✅ Result: {result:?}");
                    }
                    Err(e) => {
                        println!("   ❌ Evaluation Error: {e:?}");
                    }
                }
            }
            Err(e) => {
                println!("   ❌ Parsing Error: {e:?}");
            }
        }
        println!();
    }

    println!("🎉 Array indexing demo completed!");
    Ok(())
}
