/// FHIRPath - Collection Operations Example
///
/// This example demonstrates how to work with collections and arrays
/// in FHIRPath expressions.
use anyhow::Result;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
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
        println!("   {expr_str} = {result:?}");
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
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
        println!("   {expr_str} = {result:?}");
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
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
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // 9️⃣ Collection Set Operations
    println!("\n9️⃣ Collection Set Operations:");

    // Set operations - intersect, exclude, union
    let set_expressions = vec![
        "numbers.intersect(3 | 4 | 5 | 6)", // intersection
        "numbers.exclude(1 | 2)",           // difference
        "numbers | (10 | 11)",              // union
        "numbers.distinct()",               // remove duplicates
        "numbers.isDistinct()",             // check if all items are unique
        "(1 | 2 | 2 | 3).distinct()",       // remove duplicates from collection with duplicates
        "(1 | 2 | 2 | 3).isDistinct()",     // check distinctness of collection with duplicates
    ];

    for expr_str in set_expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Evaluation Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // 🔟 Subset and Superset Operations (NEW!)
    println!("\n🔟 Subset and Superset Operations:");

    let subset_superset_expressions = vec![
        "(1 | 2 | 3).subsetOf(numbers)", // subset test
        "numbers.supersetOf(3 | 4 | 5)", // superset test
        "names.subsetOf('Alice' | 'Bob' | 'Charlie' | 'Diana' | 'Eve')", // string subset
        "('Alice' | 'Bob').subsetOf(names)", // partial subset
        "{}.subsetOf(numbers)",          // empty set is subset of any set
        "numbers.supersetOf({})",        // any set is superset of empty set
        "(1 | 2 | 3).subsetOf(3 | 2 | 1)", // equal sets (order independent)
    ];

    for expr_str in subset_superset_expressions {
        match parser.parse(expr_str) {
            Ok(expr) => match evaluator.evaluate(&expr, &context) {
                Ok(result) => println!("   {expr_str} = {result:?}"),
                Err(e) => println!("   {expr_str} = Evaluation Error: {e}"),
            },
            Err(e) => println!("   {expr_str} = Parse Error: {e}"),
        }
    }

    // Repeat function examples
    println!("\n🔄 Repeat Function Examples");
    println!("===========================");

    // Create hierarchical test data
    let hierarchy_data = json!({
        "organizations": [
            {
                "id": "org1",
                "name": "Healthcare System",
                "parent": null,
                "type": "system"
            },
            {
                "id": "org2",
                "name": "General Hospital",
                "parent": "org1",
                "type": "hospital"
            },
            {
                "id": "org3",
                "name": "Emergency Department",
                "parent": "org2",
                "type": "department"
            },
            {
                "id": "org4",
                "name": "Trauma Unit",
                "parent": "org3",
                "type": "unit"
            },
            {
                "id": "org5",
                "name": "Outpatient Clinic",
                "parent": "org1",
                "type": "clinic"
            }
        ],
        "tree": {
            "id": "root",
            "name": "Root Node",
            "children": [
                {
                    "id": "branch1",
                    "name": "Branch 1",
                    "children": [
                        {
                            "id": "leaf1",
                            "name": "Leaf 1",
                            "children": []
                        },
                        {
                            "id": "leaf2",
                            "name": "Leaf 2",
                            "children": []
                        }
                    ]
                },
                {
                    "id": "branch2",
                    "name": "Branch 2",
                    "children": [
                        {
                            "id": "leaf3",
                            "name": "Leaf 3",
                            "children": []
                        }
                    ]
                }
            ]
        }
    });

    let hierarchy_context = EvaluationContext::new(hierarchy_data);

    println!("Test Data: Organization hierarchy and tree structure");

    // Find all organizations starting with root
    let expr = parser.parse("organizations.first().repeat(organizations)")?;
    let result = evaluator.evaluate(&expr, &hierarchy_context)?;
    println!("🌲 All organizations (starting from first): {result:?}");

    // Get all nodes in tree by following children
    let expr = parser.parse("tree.repeat(children)")?;
    let result = evaluator.evaluate(&expr, &hierarchy_context)?;
    println!("🌳 All nodes in tree (following children): {result:?}");

    // Get just the names of all tree nodes
    let expr = parser.parse("tree.repeat(children).name")?;
    let result = evaluator.evaluate(&expr, &hierarchy_context)?;
    println!("📝 All node names: {result:?}");

    // Count total nodes in tree
    let expr = parser.parse("tree.repeat(children).count()")?;
    let result = evaluator.evaluate(&expr, &hierarchy_context)?;
    println!("🔢 Total nodes in tree: {result:?}");

    // Find leaf nodes using basic expressions
    let expr = parser.parse("tree.repeat(children)")?;
    let result = evaluator.evaluate(&expr, &hierarchy_context)?;
    println!("🍃 All tree nodes: {result:?}");

    // Demonstrate simple numeric repeat
    let progression_data = json!({
        "values": [1, 2, 3]
    });
    let progression_context = EvaluationContext::new(progression_data);

    let expr = parser.parse("values.repeat(values)")?;
    let result = evaluator.evaluate(&expr, &progression_context)?;
    println!("🔢 Numeric repeat example: {result:?}");

    println!("\n✅ All collection operation examples completed!");
    println!("💡 Collections are fundamental to working with FHIR data");
    println!("💡 Use field access (e.g., 'patients.name') to extract data from object collections");
    println!("💡 Filter collections with 'where()' clauses for specific criteria");
    println!("💡 Use repeat() for transitive closure operations on hierarchical data");
    println!("💡 repeat() applies projection recursively until no new items are found");
    println!(
        "💡 Collections in FHIR often represent repeating elements like names, addresses, etc."
    );

    Ok(())
}
