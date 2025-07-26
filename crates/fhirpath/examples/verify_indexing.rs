//! Debug test for array indexing issue
//!
//! This test helps debug why name[1].given[0] fails while name[0].given[0] works

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debugging Array Indexing Issue");
    println!("==================================\n");

    // Create a sample FHIR Patient with multiple name entries
    let patient = json!({
        "resourceType": "Patient",
        "id": "debug-patient",
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
            }
        ]
    });

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(patient);

    // Test intermediate steps
    println!("Step 1: Test name[1]");
    match parser.parse("name[1]") {
        Ok(expr) => {
            match evaluator.evaluate(&expr, &context) {
                Ok(result) => {
                    println!("   ✅ name[1] = {result:?}");

                    // Let's check what type this result is
                    match &result {
                        fhirpath::FhirPathValue::Object(obj) => {
                            println!("   📊 It's an Object");
                            if let Some(given_val) = obj.get("given") {
                                println!("   📊 given field = {given_val:?}");
                            }
                        }
                        other => println!("   📊 Unexpected type: {other:?}"),
                    }
                }
                Err(e) => println!("   ❌ Error: {e:?}"),
            }
        }
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    println!("\nStep 2: Test name[1].given");
    match parser.parse("name[1].given") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => println!("   ✅ name[1].given = {result:?}"),
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    println!("\nStep 3: Test name[1].given[0] (the failing case)");
    match parser.parse("name[1].given[0]") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => println!("   ✅ name[1].given[0] = {result:?}"),
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    println!("\nStep 4: Test name[0].given[0] (the working case)");
    match parser.parse("name[0].given[0]") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => println!("   ✅ name[0].given[0] = {result:?}"),
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    Ok(())
}
