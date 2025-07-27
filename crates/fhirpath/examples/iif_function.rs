//! iif Function Example
//!
//! This example demonstrates the `iif(criterion, true-result [, otherwise-result])` function in FHIRPath expressions.
//! The iif function is an immediate if, also known as a conditional operator (like C's ? : operator).
//! It evaluates the criterion and returns true-result if the criterion is truthy, otherwise returns otherwise-result.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIRPath iif Function Example ===\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Create a FHIR Bundle with various resources to demonstrate iif usage
    let bundle = json!({
        "resourceType": "Bundle",
        "entry": [
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient1",
                    "active": true,
                    "name": [{"family": "Smith", "given": ["John"]}],
                    "birthDate": "1990-05-15",
                    "telecom": [
                        {"system": "phone", "value": "555-1234"},
                        {"system": "email", "value": "john@example.com"}
                    ],
                    "gender": "male"
                }
            },
            {
                "resource": {
                    "resourceType": "Patient",
                    "id": "patient2",
                    "active": false,
                    "name": [{"family": "Doe", "given": ["Jane"]}],
                    "birthDate": "1985-12-25",
                    "gender": "female"
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs1",
                    "status": "final",
                    "code": {"text": "Blood Pressure"},
                    "valueQuantity": {"value": 120, "unit": "mmHg"},
                    "subject": {"reference": "Patient/patient1"}
                }
            },
            {
                "resource": {
                    "resourceType": "Observation",
                    "id": "obs2",
                    "status": "preliminary",
                    "code": {"text": "Heart Rate"},
                    "valueQuantity": {"value": 75, "unit": "bpm"},
                    "subject": {"reference": "Patient/patient1"}
                }
            }
        ],
        "testData": {
            "conditions": {
                "trueBool": true,
                "falseBool": false,
                "positiveInt": 42,
                "zeroInt": 0,
                "nonEmptyString": "hello",
                "emptyString": "",
                "nonEmptyArray": [1, 2, 3],
                "emptyArray": []
            },
            "messages": {
                "success": "SUCCESS",
                "failure": "FAILURE",
                "default": "DEFAULT",
                "active": "ACTIVE",
                "inactive": "INACTIVE"
            }
        }
    });

    let context = EvaluationContext::new(bundle);

    // Section 1: Basic iif Usage
    println!("1. Basic iif Usage:");
    println!("{}", "-".repeat(50));

    let basic_examples = vec![
        (
            "''.iif(testData.conditions.trueBool, testData.messages.success, testData.messages.failure)",
            "True boolean condition",
        ),
        (
            "''.iif(testData.conditions.falseBool, testData.messages.success, testData.messages.failure)",
            "False boolean condition",
        ),
        (
            "''.iif(testData.conditions.trueBool, testData.messages.success)",
            "True condition without otherwise-result",
        ),
        (
            "''.iif(testData.conditions.falseBool, testData.messages.success)",
            "False condition without otherwise-result (returns empty)",
        ),
    ];

    for (expression, description) in basic_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 2: FHIRPath Truthiness Behavior
    println!("2. FHIRPath Truthiness Behavior:");
    println!("{}", "-".repeat(50));

    let truthiness_examples = vec![
        (
            "''.iif(testData.conditions.positiveInt, 'TRUTHY', 'FALSY')",
            "Positive integer (truthy)",
        ),
        (
            "''.iif(testData.conditions.zeroInt, 'TRUTHY', 'FALSY')",
            "Zero integer (truthy in FHIRPath)",
        ),
        (
            "''.iif(testData.conditions.nonEmptyString, 'TRUTHY', 'FALSY')",
            "Non-empty string (truthy)",
        ),
        (
            "''.iif(testData.conditions.emptyString, 'TRUTHY', 'FALSY')",
            "Empty string (truthy in FHIRPath)",
        ),
        (
            "''.iif(testData.conditions.nonEmptyArray, 'TRUTHY', 'FALSY')",
            "Non-empty array (truthy)",
        ),
        (
            "''.iif(testData.conditions.emptyArray, 'TRUTHY', 'FALSY')",
            "Empty array (falsy)",
        ),
    ];

    for (expression, description) in truthiness_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 3: Basic Conditional Logic
    println!("3. Basic Conditional Logic:");
    println!("{}", "-".repeat(50));

    let conditional_examples = vec![
        (
            "''.iif(5 > 3, 'Five is greater', 'Five is not greater')",
            "Simple numeric comparison",
        ),
        (
            "''.iif('hello'.length() > 0, 'Has content', 'Empty')",
            "String length check",
        ),
        (
            "''.iif(testData.conditions.nonEmptyArray.count() > 2, 'Many items', 'Few items')",
            "Collection count check",
        ),
        (
            "''.iif(testData.conditions.nonEmptyArray.exists(), testData.conditions.nonEmptyArray.first(), 'None')",
            "Existence-based value retrieval",
        ),
    ];

    for (expression, description) in conditional_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 4: Mathematical and String Conditions
    println!("4. Mathematical and String Conditions:");
    println!("{}", "-".repeat(50));

    let math_examples = vec![
        (
            "''.iif(10 div 3 = 3, 'Integer division works', 'Math error')",
            "Integer division check",
        ),
        (
            "''.iif('FHIRPath'.startsWith('FHIR'), 'FHIR prefix found', 'Different prefix')",
            "String prefix check",
        ),
        (
            "''.iif(testData.conditions.positiveInt + 8 = 50, 'Sum is 50', testData.conditions.positiveInt + 8)",
            "Mathematical expression in condition",
        ),
        (
            "''.iif('test'.upper() = 'TEST', 'Case conversion works', 'Case issue')",
            "String transformation check",
        ),
    ];

    for (expression, description) in math_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 5: Collection-Based Conditions
    println!("5. Collection-Based Conditions:");
    println!("{}", "-".repeat(50));

    let collection_examples = vec![
        (
            "testData.conditions.nonEmptyArray.iif(count() > 2, 'Has many items', 'Has few items')",
            "Collection size conditional message",
        ),
        (
            "testData.conditions.emptyArray.iif(exists(), 'Not empty', 'Is empty')",
            "Empty collection check",
        ),
        (
            "''.iif('test string'.length() > 5, 'Long string', 'Short string')",
            "String length categorization",
        ),
    ];

    for (expression, description) in collection_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 6: Nested and Chained iif
    println!("6. Nested and Chained iif:");
    println!("{}", "-".repeat(50));

    let nested_examples = vec![
        (
            "''.iif(5 > 3, ''.iif(2 + 2 = 4, 'Both true', 'Math wrong'), 'First false')",
            "Nested iif with mathematical conditions",
        ),
        (
            "''.iif(entry.resource.count() > 1, 'Multiple resources', 'Single resource')",
            "Simple resource count check",
        ),
    ];

    for (expression, description) in nested_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(&expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 7: Different Return Types
    println!("7. Different Return Types:");
    println!("{}", "-".repeat(50));

    let return_type_examples = vec![
        (
            "''.iif(testData.conditions.trueBool, testData.conditions.positiveInt, testData.conditions.zeroInt)",
            "Returning integers",
        ),
        (
            "''.iif(testData.conditions.trueBool, testData.conditions.trueBool, testData.conditions.falseBool)",
            "Returning booleans",
        ),
        (
            "''.iif(testData.conditions.trueBool, testData.conditions.nonEmptyArray, testData.conditions.emptyArray)",
            "Returning arrays/collections",
        ),
        (
            "''.iif(testData.conditions.trueBool, entry.resource.first(), 'No resource')",
            "Returning FHIR resources",
        ),
    ];

    for (expression, description) in return_type_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    // Section 8: Real-World Scenarios
    println!("8. Real-World Scenarios:");
    println!("{}", "-".repeat(50));

    let clinical_examples = vec![
        (
            "''.iif(entry.resource.count() > 0, 'Resources available for processing', 'No resources found')",
            "Resource availability check",
        ),
        (
            "''.iif(entry.resource.count() = 4, 'Expected resource count', 'Unexpected count')",
            "Resource count validation",
        ),
        (
            "''.iif('hello world'.contains('world'), 'Contains world', 'Missing world')",
            "Text content validation",
        ),
    ];

    for (expression, description) in clinical_examples {
        println!("Expression: {}", expression);
        println!("Description: {}", description);

        let expr = parser.parse(expression)?;
        let result = evaluator.evaluate(&expr, &context)?;
        println!("Result: {}", format_result(&result));
        println!();
    }

    println!("=== Summary ===");
    println!("The iif() function is essential for conditional logic in FHIRPath:");
    println!("• Immediate if (conditional operator) like C's ? : operator");
    println!("• Evaluates criterion for truthiness and returns appropriate result");
    println!("• In FHIRPath: only false, empty collections, and Empty are falsy");
    println!("• Zero integers, empty strings, and other values are truthy");
    println!("• Can return any FHIRPath value type (primitives, objects, collections)");
    println!("• Supports optional otherwise-result parameter (defaults to empty)");
    println!("• Can be nested and chained for complex conditional logic");
    println!("• Useful for clinical decision making and data processing workflows");

    Ok(())
}

/// Format a FhirPathValue for display
fn format_result(value: &FhirPathValue) -> String {
    match value {
        FhirPathValue::Boolean(b) => format!("Boolean({})", b),
        FhirPathValue::String(s) => format!("String(\"{}\")", s),
        FhirPathValue::Number(n) => format!("Number({})", n),
        FhirPathValue::Integer(i) => format!("Integer({})", i),
        FhirPathValue::Date(d) => format!("Date({})", d),
        FhirPathValue::DateTime(dt) => format!("DateTime({})", dt),
        FhirPathValue::Time(t) => format!("Time({})", t),
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
        _ => format!("{:?}", value),
    }
}
