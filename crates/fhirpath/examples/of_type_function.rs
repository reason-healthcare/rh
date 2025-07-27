//! ofType Function Example
//!
//! This example demonstrates the `ofType(type)` function in FHIRPath expressions.
//! The ofType function filters collections, returning only items that match the specified type.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath ofType Function Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create a FHIR Bundle with mixed resource types
    let bundle = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient1",
                    "name": [{"family": "Smith", "given": ["John"]}],
                    "active": true
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final",
                    "valueQuantity": {"value": 120, "unit": "mmHg"}
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient2",
                    "name": [{"family": "Doe", "given": ["Jane"]}],
                    "active": false
                }
            },
            {
                "resource": {
                    "resourceType": "Practitioner",
                    "id": "prac1",
                    "name": [{"family": "Johnson", "given": ["Dr. Sarah"]}],
                    "active": true
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs2",
                    "status": "preliminary",
                    "valueQuantity": {"value": 80, "unit": "mmHg"}
                }
            }
        ],
        "mixedData": [
            "string value",
            42,
            true,
            3.15159,
            {"type": "custom", "data": "object"},
            false,
            99
        ]
    });

    let context = EvaluationContext::new(bundle);

    // Section 1: Basic Type Filtering
    println!("1. Basic Type Filtering:");
    println!("{}", "-".repeat(40));

    let type_examples = vec![
        (
            "mixedData.ofType(String)",
            "Filter strings from mixed data collection",
        ),
        (
            "mixedData.ofType(Integer)",
            "Filter integers from mixed data collection",
        ),
        (
            "mixedData.ofType(Boolean)",
            "Filter booleans from mixed data collection",
        ),
        (
            "mixedData.ofType(Decimal)",
            "Filter decimal numbers from mixed data collection",
        ),
    ];

    for (expression, description) in type_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 2: FHIR Resource Type Filtering
    println!("2. FHIR Resource Type Filtering:");
    println!("{}", "-".repeat(40));

    let resource_examples = vec![
        (
            "entry.resource.ofType(Patient)",
            "Filter Patient resources from bundle entries",
        ),
        (
            "entry.resource.ofType(Observation)",
            "Filter Observation resources from bundle entries",
        ),
        (
            "entry.resource.ofType(Practitioner)",
            "Filter Practitioner resources from bundle entries",
        ),
        (
            "entry.resource.ofType(Medication)",
            "Filter Medication resources (none exist in this bundle)",
        ),
    ];

    for (expression, description) in resource_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 3: Chaining ofType with Other Functions
    println!("3. Chaining ofType with Other Functions:");
    println!("{}", "-".repeat(40));

    let chaining_examples = vec![
        (
            "entry.resource.ofType(Patient).count()",
            "Count Patient resources in the bundle",
        ),
        (
            "entry.resource.ofType(Patient).where(active = true)",
            "Find active Patient resources",
        ),
        (
            "entry.resource.ofType(Observation).select(status)",
            "Get status of all Observations",
        ),
        (
            "entry.resource.ofType(Patient).name.given.first()",
            "Get first given name of all patients",
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

    // Section 4: Complex Type Filtering Scenarios
    println!("4. Complex Type Filtering Scenarios:");
    println!("{}", "-".repeat(40));

    let complex_examples = vec![
        (
            "entry.resource.ofType(Patient) | entry.resource.ofType(Practitioner)",
            "Union of Patient and Practitioner resources",
        ),
        (
            "entry.resource.ofType(Patient).exists() and entry.resource.ofType(Observation).exists()",
            "Check if bundle contains both Patients and Observations",
        ),
        (
            "entry.resource.ofType(Patient).where(active = true).count() > 0",
            "Check if any active patients exist",
        ),
    ];

    for (expression, description) in complex_examples {
        println!("Expression: {expression}");
        println!("Description: {description}");

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    println!("=== Summary ===");
    println!("The ofType() function is essential for FHIR processing:");
    println!("• Filters collections by type, returning only matching items");
    println!("• Works with both primitive types (String, Integer, Boolean, Decimal)");
    println!("• Supports FHIR resource type filtering (Patient, Observation, etc.)");
    println!("• Can be chained with other FHIRPath functions");
    println!("• Returns empty collection when no items match the specified type");
    println!("• Enables type-safe resource processing in FHIR bundles");

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
                    .take(3)
                    .map(|item| match item {
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
                        _ => format_result(item),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                if items.len() > 3 {
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
