//! SQL-on-FHIR Extension Examples
//!
//! This example demonstrates how to use SQL-on-FHIR extension functions
//! for resource and reference key handling, enabling efficient joins
//! in SQL-based FHIR implementations.

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 SQL-on-FHIR Extension Examples");
    println!("=================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Example 1: Getting resource keys
    println!("📋 Example 1: Resource Key Extraction");
    println!("=====================================");

    let patient = json!({
        "resourceType": "Patient",
        "id": "patient-12345",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "identifier": [{
            "use": "usual",
            "type": {
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                    "code": "MR"
                }]
            },
            "system": "http://hospital.example.org",
            "value": "MR-2024-001"
        }],
        "name": [{
            "use": "official",
            "family": "Johnson",
            "given": ["Sarah", "Elizabeth"]
        }],
        "gender": "female",
        "birthDate": "1985-03-15"
    });

    let context = EvaluationContext::new(patient.clone());

    println!("Patient Resource:");
    println!("{}", serde_json::to_string_pretty(&patient)?);
    println!();

    // Get the resource key for SQL joins
    test_expression(
        &parser,
        &evaluator,
        &context,
        "getResourceKey()",
        "Get Patient resource key (ResourceType/id format)",
    )?;

    // Example 2: Reference key extraction
    println!("\n🔬 Example 2: Reference Key Extraction");
    println!("======================================");

    let observation = json!({
        "resourceType": "Observation",
        "id": "vitals-001",
        "status": "final",
        "category": [{
            "coding": [{
                "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                "code": "vital-signs",
                "display": "Vital Signs"
            }]
        }],
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "85354-9",
                "display": "Blood pressure panel with all children optional"
            }]
        },
        "subject": {
            "reference": "Patient/patient-12345"
        },
        "performer": [{
            "reference": "Practitioner/dr-smith-789"
        }],
        "component": [
            {
                "code": {
                    "coding": [{
                        "system": "http://loinc.org",
                        "code": "8480-6",
                        "display": "Systolic blood pressure"
                    }]
                },
                "valueQuantity": {
                    "value": 120,
                    "unit": "mmHg",
                    "system": "http://unitsofmeasure.org",
                    "code": "mm[Hg]"
                }
            },
            {
                "code": {
                    "coding": [{
                        "system": "http://loinc.org",
                        "code": "8462-4",
                        "display": "Diastolic blood pressure"
                    }]
                },
                "valueQuantity": {
                    "value": 80,
                    "unit": "mmHg",
                    "system": "http://unitsofmeasure.org",
                    "code": "mm[Hg]"
                }
            }
        ]
    });

    let obs_context = EvaluationContext::new(observation.clone());

    println!("Observation Resource:");
    println!("{}", serde_json::to_string_pretty(&observation)?);
    println!();

    // Get reference keys without type filter
    test_expression(
        &parser,
        &evaluator,
        &obs_context,
        "subject.getReferenceKey()",
        "Get subject reference key (any resource type)",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &obs_context,
        "performer[0].getReferenceKey()",
        "Get performer reference key",
    )?;

    // Get reference keys with type filters
    test_expression(
        &parser,
        &evaluator,
        &obs_context,
        "subject.getReferenceKey('Patient')",
        "Get subject reference key (Patient only)",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &obs_context,
        "subject.getReferenceKey('Organization')",
        "Get subject reference key (Organization only - should be empty)",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &obs_context,
        "performer[0].getReferenceKey('Practitioner')",
        "Get performer reference key (Practitioner only)",
    )?;

    // Example 3: Key consistency validation
    println!("\n🔐 Example 3: Key Consistency Validation");
    println!("========================================");

    println!("Demonstrating that resource keys and reference keys are now identical:");

    // Get the patient resource key
    let patient_key_result = parser
        .parse("getResourceKey()")
        .and_then(|parsed| evaluator.evaluate(&parsed, &context))?;

    // Get the reference key from the observation
    let reference_key_result = parser
        .parse("subject.getReferenceKey()")
        .and_then(|parsed| evaluator.evaluate(&parsed, &obs_context))?;

    println!("Patient resource key: {patient_key_result:?}");
    println!("Reference key from Observation: {reference_key_result:?}");

    // Both should now return the same value
    if patient_key_result == reference_key_result {
        println!("✅ Keys are identical! This enables efficient SQL joins.");
        if let FhirPathValue::String(key_str) = &patient_key_result {
            println!("   Both return ResourceType/id format: {key_str}");
        }
    } else {
        println!("❌ Keys don't match - this indicates an issue.");
    }

    // Example 4: SQL-on-FHIR with complex queries
    println!("\n📊 Example 4: Complex SQL-on-FHIR Queries");
    println!("==========================================");

    // Create a bundle with multiple related resources
    let bundle = json!({
        "resourceType": "Bundle",
        "id": "patient-data-bundle",
        "type": "collection",
        "entry": [
            {
                "resource": patient
            },
            {
                "resource": observation
            },
            {
                "resource": {
                    "resourceType": "Condition",
                    "id": "hypertension-001",
                    "clinicalStatus": {
                        "coding": [{
                            "system": "http://terminology.hl7.org/CodeSystem/condition-clinical",
                            "code": "active"
                        }]
                    },
                    "category": [{
                        "coding": [{
                            "system": "http://terminology.hl7.org/CodeSystem/condition-category",
                            "code": "problem-list-item"
                        }]
                    }],
                    "code": {
                        "coding": [{
                            "system": "http://snomed.info/sct",
                            "code": "38341003",
                            "display": "Hypertensive disorder"
                        }]
                    },
                    "subject": {
                        "reference": "Patient/patient-12345"
                    }
                }
            }
        ]
    });

    let bundle_context = EvaluationContext::new(bundle);

    // Extract resource types from the bundle entries (note: getResourceKey() works on individual resources)
    test_expression(
        &parser,
        &evaluator,
        &bundle_context,
        "entry.resource.resourceType",
        "Get all resource types from bundle entries",
    )?;

    // Extract all Patient references and their keys from bundle resources
    test_expression(
        &parser,
        &evaluator,
        &bundle_context,
        "entry.resource.subject.reference",
        "Get all subject references from bundle resources",
    )?;

    // Find resources that reference the same patient using resource key format
    test_expression(
        &parser,
        &evaluator,
        &bundle_context,
        "entry.resource.where(subject.reference = 'Patient/patient-12345').resourceType",
        "Find all resources referencing Patient/patient-12345",
    )?;

    println!("\n✅ SQL-on-FHIR Extension Examples Complete!");
    println!("These examples show how to:");
    println!("• Extract resource keys for SQL primary keys (ResourceType/id format)");
    println!("• Extract reference keys for SQL foreign keys (ResourceType/id format)");
    println!("• Use type filters to ensure referential integrity");
    println!("• Validate key consistency across resources");
    println!("• Build complex queries for SQL-on-FHIR implementations");
    Ok(())
}

/// Helper function to test and display FHIRPath expressions
fn test_expression(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 {description}");
    println!("   Expression: {expression}");

    match parser.parse(expression) {
        Ok(parsed) => match evaluator.evaluate(&parsed, context) {
            Ok(result) => {
                let display_result = match result {
                    FhirPathValue::String(s) => format!("\"{s}\""),
                    FhirPathValue::Integer(i) => i.to_string(),
                    FhirPathValue::Boolean(b) => b.to_string(),
                    FhirPathValue::Collection(ref coll) if coll.is_empty() => "{}".to_string(),
                    FhirPathValue::Collection(ref coll) => {
                        let items: Vec<String> = coll
                            .iter()
                            .map(|item| match item {
                                FhirPathValue::String(s) => format!("\"{s}\""),
                                FhirPathValue::Integer(i) => i.to_string(),
                                FhirPathValue::Boolean(b) => b.to_string(),
                                other => format!("{other:?}"),
                            })
                            .collect();
                        format!("{{ {} }}", items.join(", "))
                    }
                    other => format!("{other:?}"),
                };
                println!("   Result: {display_result}");
            }
            Err(e) => {
                println!("   ❌ ERROR: {e:?}");
            }
        },
        Err(e) => {
            println!("   ❌ PARSE ERROR: {e:?}");
        }
    }

    println!();
    Ok(())
}
