//! FHIR Extension Validation Examples
//!
//! This example demonstrates how to use FHIR extensions for data validation,
//! quality checks, and ensuring FHIR resource compliance.

use serde_json::json;
use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext, FhirPathValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 FHIR Extension Validation Examples");
    println!("=====================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Example 1: Valid Patient with all required extensions
    println!("✅ Example 1: Valid Patient with Complete Extensions");
    println!("==================================================");
    
    let valid_patient = json!({
        "resourceType": "Patient",
        "id": "valid-patient",
        "meta": {
            "profile": ["http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"]
        },
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2106-3",
                            "display": "White"
                        }
                    }
                ]
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2186-5",
                            "display": "Not Hispanic or Latino"
                        }
                    }
                ]
            }
        ],
        "identifier": [{
            "use": "usual",
            "type": {
                "coding": [{
                    "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                    "code": "MR"
                }]
            },
            "system": "http://hospital.example.org",
            "value": "12345"
        }],
        "name": [{
            "use": "official",
            "family": "Smith",
            "given": ["John"]
        }],
        "gender": "male",
        "birthDate": "1980-01-01"
    });

    validate_patient(&parser, &evaluator, &valid_patient, "Valid Patient")?;

    // Example 2: Patient missing required extensions
    println!("\n❌ Example 2: Patient Missing Required Extensions");
    println!("===============================================");
    
    let invalid_patient = json!({
        "resourceType": "Patient", 
        "id": "invalid-patient",
        "name": [{
            "use": "official",
            "family": "Doe",
            "given": ["Jane"]
        }],
        "gender": "female"
        // Missing race/ethnicity extensions, identifiers, birthDate
    });

    validate_patient(&parser, &evaluator, &invalid_patient, "Invalid Patient")?;

    // Example 3: Patient with malformed extensions
    println!("\n⚠️  Example 3: Patient with Malformed Extensions");
    println!("===============================================");
    
    let malformed_patient = json!({
        "resourceType": "Patient",
        "id": "malformed-patient", 
        "extension": [
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
                // Missing nested extensions - this extension has no value
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity",
                "extension": [
                    {
                        "url": "ombCategory"
                        // Missing valueCoding
                    }
                ]
            }
        ],
        "name": [{
            "family": "Johnson" // Missing given name
        }],
        "gender": "other"
    });

    validate_patient(&parser, &evaluator, &malformed_patient, "Malformed Patient")?;

    // Example 4: Observation validation
    println!("\n🔬 Example 4: FHIR Observation Validation");
    println!("=========================================");
    
    let observation = json!({
        "resourceType": "Observation",
        "id": "vital-signs-1",
        "status": "final",
        "category": [{
            "coding": [{
                "system": "http://terminology.hl7.org/CodeSystem/observation-category",
                "code": "vital-signs"
            }]
        }],
        "code": {
            "coding": [{
                "system": "http://loinc.org",
                "code": "8867-4",
                "display": "Heart rate"
            }]
        },
        "subject": {
            "reference": "Patient/example"
        },
        "valueQuantity": {
            "value": 72,
            "unit": "beats/min",
            "system": "http://unitsofmeasure.org",
            "code": "/min"
        }
    });

    validate_observation(&parser, &evaluator, &observation)?;

    println!("\n✅ FHIR Extension Validation Examples Complete!");
    println!("These examples show how to:");
    println!("• Validate required FHIR extensions are present");
    println!("• Check extension values are properly formed");
    println!("• Verify standard terminology usage");
    println!("• Implement data quality rules");
    println!("• Create comprehensive FHIR resource validation");

    Ok(())
}

/// Validate a Patient resource using FHIR extension functions
fn validate_patient(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    patient: &serde_json::Value,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = EvaluationContext::new(patient.clone());
    
    println!("📋 Validating: {label}");
    println!("{}", serde_json::to_string_pretty(patient)?);
    println!();

    // Basic required fields
    test_validation(&parser, &evaluator, &context,
        "%resource.resourceType = 'Patient'",
        "Resource type is Patient")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.name.where(use = 'official').exists()",
        "Has official name")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.gender.exists()",
        "Has gender specified")?;

    // US Core race extension validation
    test_validation(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').exists()",
        "Has US Core race extension")?;

    test_validation(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')[0].hasValue()",
        "Race extension has value")?;

    test_validation(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'ombCategory').valueCoding.exists()",
        "Race has OMB category coding")?;

    // US Core ethnicity extension validation
    test_validation(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity').exists()",
        "Has US Core ethnicity extension")?;

    test_validation(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity')[0].hasValue()",
        "Ethnicity extension has value")?;

    // Identifier validation
    test_validation(&parser, &evaluator, &context,
        "%resource.identifier.exists()",
        "Has at least one identifier")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.identifier.where(type.coding.code = 'MR').exists()",
        "Has medical record number")?;

    // Birth date validation
    test_validation(&parser, &evaluator, &context,
        "%resource.birthDate.exists()",
        "Has birth date")?;

    println!();
    Ok(())
}

/// Validate an Observation resource
fn validate_observation(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    observation: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = EvaluationContext::new(observation.clone());
    
    println!("📋 Validating Observation");
    println!();

    // Basic observation validation
    test_validation(&parser, &evaluator, &context,
        "%resource.resourceType = 'Observation'",
        "Resource type is Observation")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.status.exists()",
        "Has status")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.code.exists()",
        "Has code")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.subject.exists()",
        "Has subject")?;

    // Terminology validation using extension variables
    test_validation(&parser, &evaluator, &context,
        "%resource.code.coding.where(system = %loinc).exists()",
        "Uses LOINC coding system")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.valueQuantity.system = %ucum", 
        "Uses UCUM unit system")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.category.coding.system.contains('terminology.hl7.org')",
        "Uses HL7 terminology for category")?;

    // Value validation
    test_validation(&parser, &evaluator, &context,
        "%resource.valueQuantity.value.exists() and %resource.valueQuantity.value > 0",
        "Has positive numeric value")?;

    test_validation(&parser, &evaluator, &context,
        "%resource.valueQuantity.unit.exists()",
        "Has unit specified")?;

    Ok(())
}

/// Helper function to test validation rules
fn test_validation(
    parser: &FhirPathParser,
    evaluator: &FhirPathEvaluator,
    context: &EvaluationContext,
    expression: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match parser.parse(expression) {
        Ok(parsed) => {
            match evaluator.evaluate(&parsed, context) {
                Ok(result) => {
                    let is_valid = match result {
                        FhirPathValue::Boolean(b) => b,
                        FhirPathValue::Collection(ref coll) => !coll.is_empty(),
                        _ => false,
                    };
                    
                    let status = if is_valid { "✅ PASS" } else { "❌ FAIL" };
                    println!("  {status}: {description}");
                },
                Err(e) => {
                    println!("  ❌ ERROR: {description} - {e:?}");
                }
            }
        },
        Err(e) => {
            println!("  ❌ PARSE ERROR: {description} - {e:?}");
        }
    }
    
    Ok(())
}
