use anyhow::Result;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

/// Demonstrates the children() function for tree navigation in FHIRPath.
/// The children() function returns a collection of immediate child nodes from all items in input.
/// NOTE: The ordering of children is undefined and may vary between platforms.
fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("🌳 FHIRPath children() Function Demo");
    println!("====================================");
    println!("The children() function returns all immediate child nodes.");
    println!("⚠️  Ordering is undefined - don't rely on specific order!\n");

    // Example 1: Simple object with various property types
    let simple_data = json!({
        "patient": {
            "id": "patient-001",
            "name": "John Doe",
            "age": 35,
            "active": true,
            "birthDate": "1988-03-15"
        }
    });

    let context = EvaluationContext::new(simple_data);

    println!("1️⃣ Basic children() on object:");
    println!("   Data: patient with id, name, age, active, birthDate");
    println!("   Expression: patient.children()");
    let expr = parser.parse("patient.children()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");
    println!("   Note: All property values returned (order undefined)");

    // Example 2: Nested objects
    let nested_data = json!({
        "organization": {
            "id": "org-001",
            "name": "General Hospital",
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "NY",
                "zip": "12345"
            },
            "contact": {
                "phone": "555-1234",
                "email": "info@hospital.com"
            },
            "active": true
        }
    });

    let nested_context = EvaluationContext::new(nested_data);

    println!("\n2️⃣ Nested objects:");
    println!("   Expression: organization.children()");
    let expr = parser.parse("organization.children()")?;
    let result = evaluator.evaluate(&expr, &nested_context)?;
    println!("   Result: {result:?}");
    println!("   Note: Returns immediate children (including nested objects)");

    println!("\n   Expression: organization.address.children()");
    let expr = parser.parse("organization.address.children()")?;
    let result = evaluator.evaluate(&expr, &nested_context)?;
    println!("   Result: {result:?}");
    println!("   Note: Children of the address object");

    // Example 3: Array of objects
    let array_data = json!({
        "patients": [
            {
                "id": "p1",
                "name": "Alice",
                "status": "active"
            },
            {
                "id": "p2",
                "name": "Bob",
                "status": "inactive"
            }
        ]
    });

    let array_context = EvaluationContext::new(array_data);

    println!("\n3️⃣ Collection of objects:");
    println!("   Expression: patients.children()");
    let expr = parser.parse("patients.children()")?;
    let result = evaluator.evaluate(&expr, &array_context)?;
    println!("   Result: {result:?}");
    println!("   Note: All children from all items in collection");

    // Example 4: FHIR Patient resource example
    let fhir_patient = json!({
        "resourceType": "Patient",
        "id": "example-patient",
        "name": [
            {
                "use": "official",
                "family": "Doe",
                "given": ["John", "James"]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-1234"
            },
            {
                "system": "email",
                "value": "john.doe@example.com"
            }
        ],
        "gender": "male",
        "birthDate": "1988-03-15",
        "active": true
    });

    let fhir_context = EvaluationContext::new(fhir_patient);

    println!("\n4️⃣ FHIR Patient resource:");
    println!("   Get all top-level properties:");
    println!("   Expression: children()");
    let expr = parser.parse("children()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Result: {result:?}");
    println!("   Note: All patient properties as immediate children");

    println!("\n   Get all name components:");
    println!("   Expression: name.children()");
    let expr = parser.parse("name.children()")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Result: {result:?}");
    println!("   Note: All properties from all name objects");

    // Example 5: Empty and primitive values
    let mixed_data = json!({
        "empty_object": {},
        "simple_string": "just a string",
        "simple_number": 42
    });

    let mixed_context = EvaluationContext::new(mixed_data);

    println!("\n5️⃣ Edge cases:");
    println!("   Empty object:");
    println!("   Expression: empty_object.children()");
    let expr = parser.parse("empty_object.children()")?;
    let result = evaluator.evaluate(&expr, &mixed_context)?;
    println!("   Result: {result:?}");
    println!("   Note: Empty object has no children");

    println!("\n   Primitive value:");
    println!("   Expression: simple_string.children()");
    let expr = parser.parse("simple_string.children()")?;
    let result = evaluator.evaluate(&expr, &mixed_context)?;
    println!("   Result: {result:?}");
    println!("   Note: Primitive values have no children");

    // Example 6: Combining with other functions
    let complex_data = json!({
        "bundle": {
            "resourceType": "Bundle",
            "entry": [
                {
                    "resource": {
                        "resourceType": "Patient",
                        "id": "patient1",
                        "name": "Alice"
                    }
                },
                {
                    "resource": {
                        "resourceType": "Observation",
                        "id": "obs1",
                        "value": 120
                    }
                }
            ]
        }
    });

    let complex_context = EvaluationContext::new(complex_data);

    println!("\n6️⃣ Combining with other functions:");
    println!("   Count children:");
    println!("   Expression: bundle.children().count()");
    let expr = parser.parse("bundle.children().count()")?;
    let result = evaluator.evaluate(&expr, &complex_context)?;
    println!("   Result: {result:?}");

    println!("\n   Filter children by type:");
    println!("   Expression: bundle.children().where($this is String)");
    let expr = parser.parse("bundle.children().where($this is String)")?;
    let result = evaluator.evaluate(&expr, &complex_context)?;
    println!("   Result: {result:?}");

    println!("\n   Get children of children (like descendants()):");
    println!("   Expression: bundle.entry.children()");
    let expr = parser.parse("bundle.entry.children()")?;
    let result = evaluator.evaluate(&expr, &complex_context)?;
    println!("   Result: {result:?}");

    println!("\n🎯 Key Points About children():");
    println!("===============================");
    println!("✅ Returns immediate child nodes only (not recursive)");
    println!("✅ Works on objects - returns all property values");
    println!("✅ Works on collections - returns children from all items");
    println!("⚠️  Order is undefined - don't use first()/last() expecting consistent results");
    println!("⚠️  Primitive values return empty (no children)");
    println!("⚠️  Empty objects return empty");
    println!("💡 Use children() + repeat() for recursive traversal");
    println!("💡 Combine with where() for filtered child access");
    println!("💡 Use count() to count immediate children");

    Ok(())
}
