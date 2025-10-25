use anyhow::Result;
use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::{FhirValidator, ValidatorConfig, ValueSetRegistry};

fn main() -> Result<()> {
    println!("=== FHIR Binding Validation Example ===\n");

    example_1_valid_gender_codes()?;
    example_2_invalid_code_in_json()?;
    example_3_custom_valueset_registry()?;
    example_4_skip_binding_validation()?;

    Ok(())
}

fn example_1_valid_gender_codes() -> Result<()> {
    println!("Example 1: Valid Gender Codes");
    println!("------------------------------");

    let validator = FhirValidator::new()?;

    let valid_codes = vec!["male", "female", "other", "unknown"];

    for code in valid_codes {
        let json = format!(
            r#"{{
                "resourceType": "Patient",
                "id": "example",
                "gender": "{code}"
            }}"#,
        );

        let result = validator.validate_full::<Patient>(&json)?;
        println!("Gender '{code}': {} issues", result.issues.len());
    }

    println!();
    Ok(())
}

fn example_2_invalid_code_in_json() -> Result<()> {
    println!("Example 2: Invalid Code in JSON");
    println!("--------------------------------");

    let validator = FhirValidator::new()?;

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "invalid-code"
    }"#;

    match validator.validate_full::<Patient>(json) {
        Ok(result) => {
            println!("Validation result: {} issues", result.issues.len());
            for issue in &result.issues {
                println!("  - {}: {:?}", issue.severity, issue.details);
            }
        }
        Err(e) => {
            println!("Structural validation error (expected for invalid enum value):");
            println!("  {e}");
        }
    }

    println!();
    Ok(())
}

fn example_3_custom_valueset_registry() -> Result<()> {
    println!("Example 3: Custom ValueSet Registry");
    println!("------------------------------------");

    let mut registry = ValueSetRegistry::new();

    let custom_valueset_json = r#"{
        "resourceType": "ValueSet",
        "url": "http://hl7.org/fhir/ValueSet/administrative-gender",
        "expansion": {
            "contains": [
                {
                    "system": "http://hl7.org/fhir/administrative-gender",
                    "code": "male",
                    "display": "Male"
                },
                {
                    "system": "http://hl7.org/fhir/administrative-gender",
                    "code": "female",
                    "display": "Female"
                }
            ]
        }
    }"#;

    registry
        .load_from_json(custom_valueset_json)
        .map_err(|e| anyhow::anyhow!("Failed to load ValueSet: {e}"))?;
    println!("Loaded custom ValueSet with only 'male' and 'female' codes");

    let config = ValidatorConfig::new().with_valueset_registry(registry);
    let validator = FhirValidator::with_config(config)?;

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "male"
    }"#;

    let result = validator.validate_full::<Patient>(json)?;
    println!("Gender 'male': {} issues", result.issues.len());

    println!("\nNote: Patient.gender uses a strongly-typed enum (AdministrativeGender),");
    println!("so the type system provides compile-time safety. Binding validation");
    println!("primarily applies to JSON validation and CodeableConcept/Coding fields.");

    println!();
    Ok(())
}

fn example_4_skip_binding_validation() -> Result<()> {
    println!("Example 4: Skip Binding Validation");
    println!("-----------------------------------");

    let config = ValidatorConfig::new().with_skip_bindings(true);
    let validator = FhirValidator::with_config(config)?;

    let json = r#"{
        "resourceType": "Patient",
        "id": "example",
        "gender": "male"
    }"#;

    let result = validator.validate_full::<Patient>(json)?;
    println!(
        "Validation with skip_bindings=true: {} issues",
        result.issues.len()
    );
    println!("(Binding validation is disabled)");

    println!();
    Ok(())
}
