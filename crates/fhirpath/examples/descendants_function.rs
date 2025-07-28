//! Example demonstrating the descendants() tree navigation function in FHIRPath
//!
//! The descendants() function returns a collection of all descendant nodes (recursively)
//! from the input collection, including children, grandchildren, and all deeper levels.
//! Unlike children() which only returns immediate children, descendants() traverses the
//! entire tree structure.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("=== FHIRPath descendants() Function Examples ===\n");

    // Example 1: Simple nested structure
    println!("1. Simple nested structure:");
    let simple_data = json!({
        "person": {
            "name": {
                "given": "John",
                "family": "Doe"
            },
            "age": 30
        }
    });
    let context = EvaluationContext::new(simple_data);

    let expr = parser.parse("person.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   person.descendants() = {result:?}");
    println!("   → Returns all nested values: name object, given, family, age\n");

    // Example 2: Complex FHIR-like structure
    println!("2. Complex FHIR Patient structure:");
    let patient_data = json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [
            {
                "use": "official",
                "given": ["John", "William"],
                "family": "Doe"
            },
            {
                "use": "nickname",
                "given": ["Johnny"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234",
                "use": "home"
            },
            {
                "system": "email",
                "value": "john.doe@example.com",
                "use": "work"
            }
        ],
        "address": [
            {
                "use": "home",
                "line": ["123 Main St", "Apt 4B"],
                "city": "Springfield",
                "state": "IL",
                "postalCode": "62701",
                "country": {
                    "code": "US",
                    "display": "United States"
                }
            }
        ],
        "active": true,
        "birthDate": "1990-01-15"
    });
    let context = EvaluationContext::new(patient_data);

    let expr = parser.parse("descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   Patient.descendants() contains {} items",
        if let fhirpath::evaluator::types::FhirPathValue::Collection(items) = &result {
            items.len()
        } else {
            0
        }
    );
    println!("   → Includes all nested values from entire Patient resource\n");

    // Example 3: Specific field descendants
    println!("3. Descendants of specific fields:");

    let expr = parser.parse("name.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   name.descendants() = {result:?}");
    println!("   → All values nested within name arrays and objects\n");

    let expr = parser.parse("address.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   address.descendants() contains {} items",
        if let fhirpath::evaluator::types::FhirPathValue::Collection(items) = &result {
            items.len()
        } else {
            0
        }
    );
    println!("   → All address components including nested country object\n");

    // Example 4: Comparison with children()
    println!("4. Comparison: descendants() vs children():");
    let nested_data = json!({
        "root": {
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deep"
                    }
                }
            },
            "sibling": "shallow"
        }
    });
    let context = EvaluationContext::new(nested_data);

    let expr = parser.parse("root.children()")?;
    let children_result = evaluator.evaluate(&expr, &context)?;
    println!("   root.children() = {children_result:?}");
    println!("   → Only immediate children: level1 object and sibling");

    let expr = parser.parse("root.descendants()")?;
    let descendants_result = evaluator.evaluate(&expr, &context)?;
    println!("   root.descendants() = {descendants_result:?}");
    println!("   → All descendants: level1, level2, level3, value, sibling\n");

    // Example 5: Array handling
    println!("5. Array descendants:");
    let array_data = json!({
        "data": {
            "items": [
                {
                    "id": 1,
                    "metadata": {
                        "category": "A",
                        "tags": ["urgent", "review"]
                    }
                },
                {
                    "id": 2,
                    "metadata": {
                        "category": "B",
                        "tags": ["normal"]
                    }
                }
            ]
        }
    });
    let context = EvaluationContext::new(array_data);

    let expr = parser.parse("data.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!(
        "   data.descendants() contains {} items",
        if let fhirpath::evaluator::types::FhirPathValue::Collection(items) = &result {
            items.len()
        } else {
            0
        }
    );
    println!("   → Includes array elements and all their nested content\n");

    // Example 6: Empty and primitive handling
    println!("6. Edge cases:");
    let edge_data = json!({
        "empty_object": {},
        "simple_string": "hello",
        "simple_number": 42
    });
    let context = EvaluationContext::new(edge_data);

    let expr = parser.parse("empty_object.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   empty_object.descendants() = {result:?}");
    println!("   → Empty object has no descendants");

    let expr = parser.parse("simple_string.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   simple_string.descendants() = {result:?}");
    println!("   → Primitives have no descendants");

    let expr = parser.parse("simple_number.descendants()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   simple_number.descendants() = {result:?}");
    println!("   → Primitives have no descendants\n");

    // Example 7: Real-world use cases
    println!("7. Real-world use cases:");
    println!("   • Find all string values: descendants().ofType(String)");
    println!("   • Count total nodes: descendants().count()");
    println!("   • Check for specific values: descendants().where($this = 'target')");
    println!("   • Extract all identifiers: descendants().where($this matches '^[A-Z]{{2}}\\d+$')");
    println!("\n   Note: descendants() provides deep tree traversal for comprehensive");
    println!("   data analysis and searching across complex nested structures.");

    Ok(())
}
