//! combine Function Example
//!
//! This example demonstrates the `combine(other)` function in FHIRPath expressions.
//! The combine function merges two collections into a single collection without eliminating
//! duplicate values, preserving all items from both collections.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath combine Function Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create a FHIR Bundle with various collections to demonstrate combine
    let bundle = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient1",
                    "name": [{"family": "Smith", "given": ["John"]}],
                    "telecom": [
                        {"system": "phone", "value": "555-1234"},
                        {"system": "email", "value": "john@example.com"}
                    ]
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient2",
                    "name": [{"family": "Doe", "given": ["Jane"]}],
                    "telecom": [
                        {"system": "phone", "value": "555-5678"},
                        {"system": "email", "value": "jane@example.com"},
                        {"system": "phone", "value": "555-9999"} // Duplicate system type
                    ]
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final",
                    "code": {"text": "Blood Pressure"},
                    "valueQuantity": {"value": 120, "unit": "mmHg"}
                }
            }
        ],
        "collections": {
            "numbers1": [1, 2, 3, 2], // Has duplicates
            "numbers2": [3, 4, 5, 3], // Has duplicates and overlap with numbers1
            "strings1": ["apple", "banana", "cherry"],
            "strings2": ["banana", "date", "elderberry", "banana"], // Has duplicates and overlap
            "mixed1": [1, "hello", true, 42],
            "mixed2": [false, "world", 2, "hello"], // Mixed types with overlap
            "empty": [],
            "tags1": ["urgent", "cardiology", "follow-up"],
            "tags2": ["cardiology", "outpatient", "routine"] // Overlapping tags
        }
    });

    let context = EvaluationContext::new(bundle);

    // Section 1: Basic Collection Combining
    println!("1. Basic Collection Combining:");
    println!("{}", "-".repeat(40));

    let basic_examples = vec![
        (
            "collections.numbers1.combine(collections.numbers2)",
            "Combine number collections (preserving duplicates)",
        ),
        (
            "collections.strings1.combine(collections.strings2)",
            "Combine string collections (preserving duplicates)",
        ),
        (
            "collections.mixed1.combine(collections.mixed2)",
            "Combine mixed-type collections",
        ),
    ];

    for (expression, description) in basic_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 2: Combining with Empty Collections
    println!("2. Combining with Empty Collections:");
    println!("{}", "-".repeat(40));

    let empty_examples = vec![
        (
            "collections.numbers1.combine(collections.empty)",
            "Combine non-empty with empty collection",
        ),
        (
            "collections.empty.combine(collections.strings1)",
            "Combine empty with non-empty collection",
        ),
        (
            "collections.empty.combine(collections.empty)",
            "Combine two empty collections",
        ),
    ];

    for (expression, description) in empty_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 3: FHIR Resource Combining
    println!("3. FHIR Resource Collection Combining:");
    println!("{}", "-".repeat(40));

    let fhir_examples = vec![
        (
            "entry.resource.name.given.combine(entry.resource.name.family)",
            "Combine all given names with all family names",
        ),
        (
            "entry.resource.ofType(Patient).telecom.system.combine(collections.tags1)",
            "Combine telecom systems with tag collection",
        ),
        (
            "entry.resource.ofType(Patient)[0].telecom.combine(entry.resource.ofType(Patient)[1].telecom)",
            "Combine telecom arrays from two patients",
        ),
    ];

    for (expression, description) in fhir_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 4: Chaining combine with Other Functions
    println!("4. Chaining combine with Other Functions:");
    println!("{}", "-".repeat(40));

    let chaining_examples = vec![
        (
            "collections.numbers1.combine(collections.numbers2).count()",
            "Count total items after combining (including duplicates)",
        ),
        (
            "collections.numbers1.combine(collections.numbers2).distinct()",
            "Remove duplicates after combining",
        ),
        (
            "collections.strings1.combine(collections.strings2).where($this != 'banana')",
            "Filter out specific values after combining",
        ),
        (
            "collections.tags1.combine(collections.tags2).distinct().count()",
            "Count unique tags after combining and deduplicating",
        ),
    ];

    for (expression, description) in chaining_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 5: Advanced Combining Scenarios
    println!("5. Advanced Combining Scenarios:");
    println!("{}", "-".repeat(40));

    let advanced_examples = vec![
        (
            "collections.numbers1.first().combine(collections.numbers2.last())",
            "Combine single items from different collections",
        ),
        (
            "entry.resource.ofType(Patient).id.combine(entry.resource.ofType(Observation).id)",
            "Combine Patient IDs with Observation IDs",
        ),
        (
            "(collections.strings1.combine(collections.strings2)).intersect(collections.strings1)",
            "Find items from first collection in the combined result",
        ),
    ];

    for (expression, description) in advanced_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 6: Demonstrating Duplicate Preservation
    println!("6. Demonstrating Duplicate Preservation:");
    println!("{}", "-".repeat(40));

    println!("The combine() function preserves duplicates, unlike union operations:");

    let duplicate_examples = vec![
        (
            "collections.numbers1",
            "Original numbers1 collection (has duplicate 2)",
        ),
        (
            "collections.numbers2",
            "Original numbers2 collection (has duplicate 3)",
        ),
        (
            "collections.numbers1.combine(collections.numbers2)",
            "Combined result (preserves all duplicates)",
        ),
        (
            "collections.numbers1.combine(collections.numbers2).distinct()",
            "Combined result with duplicates removed",
        ),
    ];

    for (expression, description) in duplicate_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    println!("=== Summary ===");
    println!("The combine() function is essential for collection manipulation:");
    println!("• Merges two collections into one without eliminating duplicates");
    println!("• Combining empty with non-empty returns the non-empty collection");
    println!("• Works with any FHIRPath value types (primitives, objects, mixed)");
    println!("• Preserves all items from both collections, maintaining duplicates");
    println!("• Can be chained with other FHIRPath functions for complex operations");
    println!("• No expectation of order in the resulting collection");
    println!("• Different from union (|) which has specific precedence rules");

    Ok(())
}

/// Format a FhirPathValue for display
fn format_result(value: &FhirPathValue) -> String {
    match value {
        FhirPathValue::Boolean(b) => format!("Boolean({b})"),
        FhirPathValue::String(s) => format!("String(\"{s}\")"),
        FhirPathValue::Number(n) => format!("Number({n})"),
        FhirPathValue::Integer(i) => format!("Integer({i})"),
        FhirPathValue::Date(d) => format!("Date({d})"),
        FhirPathValue::DateTime(dt) => format!("DateTime({dt})"),
        FhirPathValue::Time(t) => format!("Time({t})"),
        FhirPathValue::Quantity { value, unit } => {
            format!("Quantity({} {})", value, unit.as_deref().unwrap_or(""))
        }
        FhirPathValue::Collection(items) => {
            if items.is_empty() {
                "Empty Collection".to_string()
            } else if items.len() == 1 {
                format!("Collection(1 item): [{}]", format_result(&items[0]))
            } else {
                let preview = items
                    .iter()
                    .take(5)
                    .map(|item| match item {
                        FhirPathValue::Object(obj) => {
                            if let Some(resource_type) = obj.get("resourceType") {
                                format!(
                                    "{}({})",
                                    resource_type.as_str().unwrap_or("Resource"),
                                    obj.get("id").and_then(|id| id.as_str()).unwrap_or("no-id")
                                )
                            } else if let Some(system) = obj.get("system") {
                                format!(
                                    "{}={}",
                                    system.as_str().unwrap_or("system"),
                                    obj.get("value").and_then(|v| v.as_str()).unwrap_or("value")
                                )
                            } else {
                                "Object".to_string()
                            }
                        }
                        _ => format_result(item),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                if items.len() > 5 {
                    format!("Collection({} items): [{}...]", items.len(), preview)
                } else {
                    format!("Collection({} items): [{}]", items.len(), preview)
                }
            }
        }
        FhirPathValue::Object(obj) => {
            if let Some(resource_type) = obj.get("resourceType") {
                format!(
                    "{}({})",
                    resource_type.as_str().unwrap_or("Resource"),
                    obj.get("id").and_then(|id| id.as_str()).unwrap_or("no-id")
                )
            } else {
                "Object".to_string()
            }
        }
        FhirPathValue::Empty => "Empty".to_string(),
        _ => format!("{value:?}"),
    }
}
