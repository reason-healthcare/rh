use anyhow::Result;
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

/// Demonstrates tree navigation alternatives since FHIRPath descendants() function is not implemented.
/// Shows how to use the implemented children() function and repeat() for hierarchical data.
fn main() -> Result<()> {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    println!("🌳 Tree Navigation Alternatives Demo");
    println!("=====================================");
    println!("FHIRPath descendants() is NOT implemented, but children() IS available!");
    println!("Here are approaches for hierarchical navigation:\n");

    // Example 1: Using the implemented children() function
    let simple_tree = json!({
        "root": {
            "id": "root",
            "name": "Root Node",
            "children": [
                {
                    "id": "child1",
                    "name": "Child 1",
                    "children": [
                        {
                            "id": "grandchild1",
                            "name": "Grandchild 1",
                            "children": []
                        }
                    ]
                },
                {
                    "id": "child2",
                    "name": "Child 2",
                    "children": []
                }
            ]
        }
    });

    let context = EvaluationContext::new(simple_tree);

    println!("1️⃣ Using children() function (IMPLEMENTED):");
    println!("   Expression: root.children()");
    let expr = parser.parse("root.children()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");
    println!("   Note: Returns all immediate child properties");

    println!("\n   Get specific child property:");
    println!("   Expression: root.children.children()");
    let expr = parser.parse("root.children.children()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");
    println!("   Note: Children of the 'children' array elements");

    println!("\n2️⃣ Using repeat() for descendants-like behavior:");
    println!("   Expression: root.repeat(children)");
    let expr = parser.parse("root.repeat(children)")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");

    println!("\n   Extract just the names:");
    println!("   Expression: root.repeat(children).name");
    let expr = parser.parse("root.repeat(children).name")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");

    println!("\n   Count all descendants:");
    println!("   Expression: root.repeat(children).count()");
    let expr = parser.parse("root.repeat(children).count()")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");

    // Example 3: Direct field access (children() alternative for specific fields)
    println!("\n3️⃣ Direct field access:");
    println!("   Expression: root.children");
    let expr = parser.parse("root.children")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");

    println!("\n   Children names:");
    println!("   Expression: root.children.name");
    let expr = parser.parse("root.children.name")?;
    let result = evaluator.evaluate(&expr, &context)?;
    println!("   Result: {result:?}");

    // Example 4: FHIR Organization hierarchy example
    let fhir_org_data = json!({
        "organizations": [
            {
                "id": "org1",
                "name": "Healthcare System",
                "partOf": null,
                "type": "system"
            },
            {
                "id": "org2",
                "name": "General Hospital",
                "partOf": { "reference": "Organization/org1" },
                "type": "hospital"
            },
            {
                "id": "org3",
                "name": "Emergency Department",
                "partOf": { "reference": "Organization/org2" },
                "type": "department"
            },
            {
                "id": "org4",
                "name": "Cardiology Department",
                "partOf": { "reference": "Organization/org2" },
                "type": "department"
            }
        ]
    });

    let fhir_context = EvaluationContext::new(fhir_org_data);

    println!("\n4️⃣ FHIR Organization hierarchy navigation:");
    println!("   Find departments under hospital org2:");
    println!("   Expression: organizations.where(partOf.reference = 'Organization/org2').name");
    let expr = parser.parse("organizations.where(partOf.reference = 'Organization/org2').name")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Result: {result:?}");

    println!("\n   Find all sub-organizations (excluding root):");
    println!("   Expression: organizations.where(partOf.exists()).name");
    let expr = parser.parse("organizations.where(partOf.exists()).name")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Result: {result:?}");

    println!("\n   Use children() on organization objects:");
    println!("   Expression: organizations.children().where($this is String)");
    let expr = parser.parse("organizations.children().where($this is String)")?;
    let result = evaluator.evaluate(&expr, &fhir_context)?;
    println!("   Result: {result:?}");

    // Example 5: Complex tree with metadata
    let complex_tree = json!({
        "filesystem": {
            "name": "root",
            "type": "directory",
            "contents": [
                {
                    "name": "usr",
                    "type": "directory",
                    "contents": [
                        {
                            "name": "bin",
                            "type": "directory",
                            "contents": [
                                { "name": "ls", "type": "file", "size": 133168 },
                                { "name": "cat", "type": "file", "size": 35600 }
                            ]
                        },
                        { "name": "readme.txt", "type": "file", "size": 1024 }
                    ]
                },
                {
                    "name": "home",
                    "type": "directory",
                    "contents": [
                        { "name": "user.txt", "type": "file", "size": 512 }
                    ]
                }
            ]
        }
    });

    let fs_context = EvaluationContext::new(complex_tree);

    println!("\n5️⃣ File system tree navigation:");
    println!("   Get immediate children using children():");
    println!("   Expression: filesystem.children()");
    let expr = parser.parse("filesystem.children()")?;
    let result = evaluator.evaluate(&expr, &fs_context)?;
    println!("   Result: {result:?}");

    println!("\n   Find all files recursively:");
    println!("   Expression: filesystem.repeat(contents).where(type = 'file').name");
    let expr = parser.parse("filesystem.repeat(contents).where(type = 'file').name")?;
    let result = evaluator.evaluate(&expr, &fs_context)?;
    println!("   Result: {result:?}");

    println!("\n   Find all directories recursively:");
    println!("   Expression: filesystem.repeat(contents).where(type = 'directory').name");
    let expr = parser.parse("filesystem.repeat(contents).where(type = 'directory').name")?;
    let result = evaluator.evaluate(&expr, &fs_context)?;
    println!("   Result: {result:?}");

    println!("\n   Calculate total file count:");
    println!("   Expression: filesystem.repeat(contents).where(type = 'file').count()");
    let expr = parser.parse("filesystem.repeat(contents).where(type = 'file').count()")?;
    let result = evaluator.evaluate(&expr, &fs_context)?;
    println!("   Result: {result:?}");

    // Example 6: Multi-level filtering and navigation
    let medical_data = json!({
        "patient": {
            "name": [{ "family": "Doe", "given": ["John"] }],
            "encounters": [
                {
                    "id": "enc1",
                    "status": "finished",
                    "diagnosis": [
                        {
                            "condition": { "coding": [{ "system": "ICD-10", "code": "Z00.00" }] },
                            "rank": 1
                        }
                    ],
                    "observations": [
                        { "code": "vital-signs", "value": "120/80", "category": "vital" },
                        { "code": "height", "value": "175", "category": "vital" }
                    ]
                },
                {
                    "id": "enc2",
                    "status": "in-progress",
                    "diagnosis": [
                        {
                            "condition": { "coding": [{ "system": "ICD-10", "code": "M79.18" }] },
                            "rank": 1
                        }
                    ],
                    "observations": [
                        { "code": "pain-scale", "value": "7", "category": "assessment" }
                    ]
                }
            ]
        }
    });

    let medical_context = EvaluationContext::new(medical_data);

    println!("\n6️⃣ FHIR Patient data navigation:");
    println!("   Use children() to get all patient properties:");
    println!("   Expression: patient.children()");
    let expr = parser.parse("patient.children()")?;
    let result = evaluator.evaluate(&expr, &medical_context)?;
    println!("   Result: {result:?}");

    println!("\n   All observation codes from all encounters:");
    println!("   Expression: patient.encounters.observations.code");
    let expr = parser.parse("patient.encounters.observations.code")?;
    let result = evaluator.evaluate(&expr, &medical_context)?;
    println!("   Result: {result:?}");

    println!("\n   Vital signs only:");
    println!("   Expression: patient.encounters.observations.where(category = 'vital').code");
    let expr = parser.parse("patient.encounters.observations.where(category = 'vital').code")?;
    let result = evaluator.evaluate(&expr, &medical_context)?;
    println!("   Result: {result:?}");

    println!("\n   ICD-10 diagnosis codes:");
    println!("   Expression: patient.encounters.diagnosis.condition.coding.where(system = 'ICD-10').code");
    let expr = parser
        .parse("patient.encounters.diagnosis.condition.coding.where(system = 'ICD-10').code")?;
    let result = evaluator.evaluate(&expr, &medical_context)?;
    println!("   Result: {result:?}");

    println!("\n🎯 Summary of Tree Navigation Options:");
    println!("======================================");
    println!("✅ children() function: Get immediate child nodes (IMPLEMENTED)");
    println!("✅ Direct field access: obj.childField");
    println!("✅ Recursive traversal: obj.repeat(childField)");
    println!("✅ Filtered navigation: obj.repeat(field).where(criteria)");
    println!("✅ Multi-level access: obj.level1.level2.level3");
    println!("✅ Collection operations: count(), first(), where(), etc.");
    println!("❌ descendants() function: Not implemented");

    println!("\n💡 Best Practices:");
    println!("• Use children() to get immediate child properties");
    println!("• Use repeat() + children() for descendants-like behavior");
    println!("• Use repeat() with specific field names for transitive closure");
    println!("• Combine with where() for filtered traversal");
    println!("• Direct field access is most efficient for known structure");
    println!("• Use collection functions for aggregation and analysis");
    println!("• Remember: children() order is undefined!");

    Ok(())
}
