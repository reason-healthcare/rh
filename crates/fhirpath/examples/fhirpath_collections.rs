/// FHIRPath - Collection Operations Example
///
/// This example demonstrates how to work with collections and arrays
/// in FHIRPath expressions.
use anyhow::Result;
use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("📋 FHIRPath Collection Operations Examples");
    println!("==========================================\n");

    // Create test data with collections
    let test_data = json!({
        "numbers": [1, 2, 3, 4, 5],
        "names": ["Alice", "Bob", "Charlie", "Diana"],
        "scores": [85, 92, 78, 95, 88, 91],
        "patients": [
            {
                "id": "patient1",
                "name": "John Doe",
                "age": 30,
                "active": true
            },
            {
                "id": "patient2",
                "name": "Jane Smith",
                "age": 25,
                "active": false
            },
            {
                "id": "patient3",
                "name": "Bob Johnson",
                "age": 45,
                "active": true
            }
        ],
        "observations": [
            {
                "code": "blood-pressure",
                "value": 120,
                "unit": "mmHg"
            },
            {
                "code": "heart-rate",
                "value": 72,
                "unit": "bpm"
            },
            {
                "code": "temperature",
                "value": 98.6,
                "unit": "F"
            }
        ]
    });

    let context = EvaluationContext::new(test_data);

    // Example 1: Basic collection access
    println!("1️⃣ Basic Collection Access:");
    let expressions = vec![
        "numbers", // Access entire collection
        "names",   // Access string collection
        "scores",  // Access number collection
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 2: Collection size and existence
    println!("\n2️⃣ Collection Size and Existence:");
    let expressions = vec![
        "numbers.count()", // Count items in collection
        "names.exists()",  // Check if collection exists and has items
        "scores.empty()",  // Check if collection is empty
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 3: Collection filtering with where()
    println!("\n3️⃣ Collection Filtering:");
    let expressions = vec![
        // Note: These expressions depend on the FHIRPath implementation
        // Supporting collection operations and filtering
        "patients.where(active = true)", // Filter active patients
        "patients.where(age > 30)",      // Filter by age
        "scores.where($ > 90)",          // Filter high scores ($ = current item)
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 4: Collection field access
    println!("\n4️⃣ Collection Field Access:");
    let expressions = vec![
        "patients.name",      // Get all patient names
        "patients.age",       // Get all patient ages
        "patients.id",        // Get all patient IDs
        "observations.code",  // Get all observation codes
        "observations.value", // Get all observation values
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 5: Mathematical operations on collections
    println!("\n5️⃣ Mathematical Operations on Collections:");

    // Create a simple context for mathematical operations
    let math_context = EvaluationContext::new(json!({
        "values": [10, 20, 30, 40, 50]
    }));

    let expressions = vec![
        // Note: These may require specific FHIRPath collection function support
        "values", // Show the collection
    ];

    for expr_str in expressions {
        let expr = parser.parse(expr_str)?;
        let result = evaluator.evaluate(&expr, &math_context)?;
        println!("   {} = {:?}", expr_str, result);
    }

    // Example 6: String operations on collections
    println!("\n6️⃣ String Operations on Collections:");
    let expressions = vec![
        "names", // Show all names
                // These would require FHIRPath string function support on collections
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 7: Boolean operations on collections
    println!("\n7️⃣ Boolean Operations on Collections:");
    let expressions = vec![
        "patients.active", // Get all active status values
                           // Boolean collection operations would need specific FHIRPath support
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    // Example 8: Practical FHIR collection scenarios
    println!("\n8️⃣ Practical FHIR Collection Scenarios:");

    let fhir_bundle = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient1",
                    "active": true
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient2",
                    "active": false
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final"
                }
            }
        ]
    });

    let bundle_context = EvaluationContext::new(fhir_bundle);

    let expressions = vec![
        "entry",                       // Get all bundle entries
        "entry.resource",              // Get all resources
        "entry.resource.resourceType", // Get all resource types
    ];

    for expr_str in expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &bundle_context) {
                Ok(result) => println!("   {} = {:?}", expr_str, result),
                Err(e) => println!("   {} = Error: {}", expr_str, e),
            },
            Err(e) => println!("   {} = Parse Error: {}", expr_str, e),
        }
    }

    println!("\n✅ All collection operation examples completed!");
    println!("💡 Collections are fundamental to working with FHIR data");
    println!("💡 Use field access (e.g., 'patients.name') to extract data from object collections");
    println!("💡 Filter collections with 'where()' clauses for specific criteria");
    println!("💡 Some advanced collection functions may require full FHIRPath implementation");
    println!(
        "💡 Collections in FHIR often represent repeating elements like names, addresses, etc."
    );

    Ok(())
}
