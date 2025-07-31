//! FHIR Resource Navigation with Extensions
//!
//! This example demonstrates practical FHIR resource navigation scenarios
//! using extension functions and variables for real-world healthcare use cases.

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧭 FHIR Resource Navigation with Extensions");
    println!("===========================================\n");

    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Complex FHIR Patient resource with various extensions
    let complex_patient = json!({
        "resourceType": "Patient",
        "id": "complex-patient-001",
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
                            "code": "2028-9",
                            "display": "Asian"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Asian"
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
            },
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-birthPlace",
                "valueAddress": {
                    "city": "San Francisco",
                    "state": "CA",
                    "country": "USA"
                }
            },
            {
                "url": "http://example.org/fhir/StructureDefinition/patient-religion",
                "valueCodeableConcept": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v3-ReligiousAffiliation",
                        "code": "1013",
                        "display": "Christian (non-Catholic, non-specific)"
                    }]
                }
            }
        ],
        "identifier": [
            {
                "use": "usual",
                "type": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }]
                },
                "system": "http://hospital.example.org",
                "value": "12345678"
            },
            {
                "use": "secondary",
                "type": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "SS"
                    }]
                },
                "system": "http://hl7.org/fhir/sid/us-ssn",
                "value": "123-45-6789"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Chen",
                "given": ["Li", "Wei"],
                "prefix": ["Dr."]
            },
            {
                "use": "maiden",
                "family": "Wang"
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "+1-555-123-4567",
                "use": "home"
            },
            {
                "system": "email",
                "value": "li.chen@example.com",
                "use": "work"
            }
        ],
        "gender": "female",
        "birthDate": "1985-03-15"
    });

    let context = EvaluationContext::new(complex_patient);

    println!("📋 Working with complex Patient resource\n");

    // Scenario 1: Demographics and Identity
    println!("👤 Scenario 1: Patient Demographics and Identity");
    println!("-----------------------------------------------");

    test_expression(&parser, &evaluator, &context,
        "%resource.name.where(use = 'official').given.first() + ' ' + %resource.name.where(use = 'official').family",
        "Get official full name")?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.identifier.where(type.coding.code = 'MR').value",
        "Get medical record number",
    )?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.telecom.where(system = 'email' and use = 'work').value",
        "Get work email address",
    )?;

    // Scenario 2: Race and Ethnicity (US Core)
    println!("\n🌍 Scenario 2: Race and Ethnicity Information");
    println!("---------------------------------------------");

    test_expression(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'ombCategory').valueCoding.display",
        "Get race category display")?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity').extension.where(url = 'ombCategory').valueCoding.code",
        "Get ethnicity category code")?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race').extension.where(url = 'text').valueString",
        "Get race text description")?;

    // Scenario 3: Extended Demographics
    println!("\n🏠 Scenario 3: Extended Demographics");
    println!("-----------------------------------");

    test_expression(&parser, &evaluator, &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-birthPlace').valueAddress.city",
        "Get birth city")?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-religion').valueCodeableConcept.coding.display",
        "Get religious affiliation")?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://example.org/fhir/StructureDefinition/patient-birthPlace').valueAddress.state + ', ' + extension('http://example.org/fhir/StructureDefinition/patient-birthPlace').valueAddress.country",
        "Get birth state and country")?;

    // Scenario 4: Data Validation and Quality Checks
    println!("\n✅ Scenario 4: Data Validation and Quality Checks");
    println!("-------------------------------------------------");

    test_expression(&parser, &evaluator, &context,
        "%resource.name.where(use = 'official').exists() and %resource.identifier.where(type.coding.code = 'MR').exists()",
        "Check required demographics present")?;

    test_expression(&parser, &evaluator, &context,
        "extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-race')[0].hasValue() and extension('http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity')[0].hasValue()",
        "Check race and ethnicity extensions have values")?;

    test_expression(&parser, &evaluator, &context,
        "%resource.telecom.where(system = 'email').value.matches('[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}')",
        "Validate email format")?;

    // Scenario 5: Complex Navigation and Filtering
    println!("\n🔍 Scenario 5: Complex Navigation and Filtering");
    println!("----------------------------------------------");

    test_expression(
        &parser,
        &evaluator,
        &context,
        "%resource.extension.where(url.contains('us-core')).count()",
        "Count US Core extensions",
    )?;

    test_expression(&parser, &evaluator, &context,
        "%resource.descendants().where($this.system.exists() and $this.system.contains('terminology.hl7.org')).count()",
        "Count elements using HL7 terminology")?;

    test_expression(&parser, &evaluator, &context,
        "%resource.extension.extension.valueCoding.where(system.contains('2.16.840.1.113883.6.238')).display",
        "Get all OMB category displays")?;

    // Scenario 6: Practical Healthcare Workflows
    println!("\n🏥 Scenario 6: Practical Healthcare Workflows");
    println!("--------------------------------------------");

    test_expression(&parser, &evaluator, &context,
        "(%resource.name.where(use = 'official').prefix.first() + ' ' + %resource.name.where(use = 'official').given.first() + ' ' + %resource.name.where(use = 'official').family) + ' (' + %resource.gender + ', DOB: ' + %resource.birthDate + ')'",
        "Create patient summary string")?;

    test_expression(&parser, &evaluator, &context,
        "%resource.identifier.where(type.coding.code in ('MR' | 'SS')).select(type.coding.code + ': ' + value)",
        "List key identifiers")?;

    test_expression(
        &parser,
        &evaluator,
        &context,
        "extension().where(hasValue()).url",
        "List all extensions that have values",
    )?;

    println!("\n✅ FHIR Resource Navigation Examples Complete!");
    println!("These examples demonstrate:");
    println!("• Complex patient demographic extraction");
    println!("• US Core race and ethnicity handling");
    println!("• Extended demographic information access");
    println!("• Data validation and quality checks");
    println!("• Complex navigation and filtering patterns");
    println!("• Practical healthcare workflow scenarios");
    println!("• Combining extension functions with standard FHIRPath");

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
    println!("  Expression: {expression}");
    println!("  Purpose: {description}");

    match parser.parse(expression) {
        Ok(parsed) => match evaluator.evaluate(&parsed, context) {
            Ok(result) => {
                println!("  Result: {result:?}\n");
            }
            Err(e) => {
                println!("  Error: {e:?}\n");
            }
        },
        Err(e) => {
            println!("  Parse Error: {e:?}\n");
        }
    }

    Ok(())
}
