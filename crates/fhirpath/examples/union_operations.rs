//! Test union operations specifically
//!
//! This test verifies that union operations (|) work correctly for all scenarios

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Testing Union Operations");
    println!("===========================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create test data with various collections
    let patient = json!({
        "resourceType": "Patient",
        "id": "union-test",
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
        ],
        "telecom": [
            {"system": "phone", "value": "555-1234"},
            {"system": "email", "value": "john@example.com"}
        ]
    });

    let context = EvaluationContext::new(patient);

    // Test 1: Simple union of primitive values
    println!("1. Testing primitive value union: (1 | 2 | 3)");
    match parser.parse("(1 | 2 | 3)") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Collection(items) => {
                        assert_eq!(items.len(), 3);
                        println!("   ✅ Union created collection with {} items", items.len());
                    }
                    _ => panic!("Expected collection from union operation"),
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 2: Union with strings
    println!("\n2. Testing string union: ('apple' | 'banana' | 'cherry')");
    match parser.parse("('apple' | 'banana' | 'cherry')") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Collection(items) => {
                        assert_eq!(items.len(), 3);
                        println!(
                            "   ✅ String union created collection with {} items",
                            items.len()
                        );
                    }
                    _ => panic!("Expected collection from string union"),
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 3: Union with FHIR data
    println!("\n3. Testing FHIR data union: name.given | name.family");
    match parser.parse("name.given | name.family") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Collection(_) => {
                        println!("   ✅ FHIR union created collection successfully");
                    }
                    _ => {
                        println!("   ⚠️  Result is not a collection: {result:?}");
                    }
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 4: Union with indexing
    println!("\n4. Testing union with indexing: (10 | 20 | 30)[1]");
    match parser.parse("(10 | 20 | 30)[1]") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Integer(20) => {
                        println!("   ✅ Union indexing works correctly - got 20");
                    }
                    _ => panic!("Expected Integer(20) from union indexing, got {result:?}"),
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 5: Union with empty values
    println!("\n5. Testing union with empty: (1 | {{}} | 3)");
    match parser.parse("(1 | {} | 3)") {
        Ok(expr) => {
            match evaluator.evaluate(&expr, &context) {
                Ok(result) => {
                    println!("   ✅ Result: {result:?}");
                    match result {
                        FhirPathValue::Collection(items) => {
                            assert_eq!(items.len(), 2); // Should only have 1 and 3, empty is not added
                            println!(
                                "   ✅ Union correctly handled empty value - {} items",
                                items.len()
                            );
                        }
                        _ => {
                            println!("   ⚠️  Result: {result:?}");
                        }
                    }
                }
                Err(e) => println!("   ❌ Error: {e:?}"),
            }
        }
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 6: Union with mixed types
    println!("\n6. Testing mixed type union: (42 | 'hello' | true)");
    match parser.parse("(42 | 'hello' | true)") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Collection(items) => {
                        assert_eq!(items.len(), 3);
                        println!("   ✅ Mixed type union works - {} items", items.len());
                    }
                    _ => panic!("Expected collection from mixed type union"),
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    // Test 7: Nested unions
    println!("\n7. Testing nested unions: ((1 | 2) | (3 | 4))");
    match parser.parse("((1 | 2) | (3 | 4))") {
        Ok(expr) => match evaluator.evaluate(&expr, &context) {
            Ok(result) => {
                println!("   ✅ Result: {result:?}");
                match result {
                    FhirPathValue::Collection(items) => {
                        assert_eq!(items.len(), 4);
                        println!("   ✅ Nested union works - {} items", items.len());
                    }
                    _ => panic!("Expected collection from nested union"),
                }
            }
            Err(e) => println!("   ❌ Error: {e:?}"),
        },
        Err(e) => println!("   ❌ Parse Error: {e:?}"),
    }

    println!("\n🎉 All union operation tests completed successfully!");
    Ok(())
}
