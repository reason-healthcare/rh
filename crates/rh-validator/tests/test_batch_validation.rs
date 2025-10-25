//! Integration tests for parallel batch validation

use hl7_fhir_r4_core::resources::patient::Patient;
use rh_validator::FhirValidator;

#[test]
fn test_validate_batch_basic() {
    let validator = FhirValidator::new().unwrap();

    let resources = vec![
        r#"{"resourceType": "Patient", "id": "patient1"}"#,
        r#"{"resourceType": "Patient", "id": "patient2"}"#,
        r#"{"resourceType": "Patient", "id": "patient3"}"#,
    ];

    let results = validator.validate_batch::<Patient>(&resources).unwrap();

    assert_eq!(results.len(), 3);
    // All should have at least warnings (dom-6 narrative warning)
    for result in results {
        assert_eq!(result.resource_type, "Patient");
    }
}

#[test]
fn test_validate_batch_large() {
    let validator = FhirValidator::new().unwrap();

    // Create 100 patient resources
    let resources: Vec<String> = (1..=100)
        .map(|i| format!(r#"{{"resourceType": "Patient", "id": "patient{i}"  }}"#))
        .collect();

    let resource_refs: Vec<&str> = resources.iter().map(|s| s.as_str()).collect();

    let results = validator.validate_batch::<Patient>(&resource_refs).unwrap();

    assert_eq!(results.len(), 100);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.resource_type, "Patient", "Resource {i} failed");
    }
}

#[test]
fn test_validate_batch_with_errors() {
    let validator = FhirValidator::new().unwrap();

    let resources = vec![
        r#"{"resourceType": "Patient", "id": "valid", "active": true}"#,
        r#"{"resourceType": "Patient", "name": "invalid-type"}"#, // name should be array
        r#"{"resourceType": "Patient", "id": "valid2", "active": false}"#,
    ];

    let results = validator.validate_batch::<Patient>(&resources).unwrap();

    assert_eq!(results.len(), 3);
    // First and third are structurally valid (might have warnings though)
    // Second should fail to deserialize with a type error
    let first_has_no_type_errors = results[0].issues.iter().all(|issue| {
        issue.code != rh_validator::IssueCode::ValueType
            && issue.code != rh_validator::IssueCode::Structure
    });
    assert!(first_has_no_type_errors);

    let second_has_type_error = results[1].issues.iter().any(|issue| {
        issue.code == rh_validator::IssueCode::ValueType
            || issue.code == rh_validator::IssueCode::Structure
    });
    assert!(second_has_type_error);

    let third_has_no_type_errors = results[2].issues.iter().all(|issue| {
        issue.code != rh_validator::IssueCode::ValueType
            && issue.code != rh_validator::IssueCode::Structure
    });
    assert!(third_has_no_type_errors);
}

#[test]
fn test_validate_ndjson_basic() {
    let validator = FhirValidator::new().unwrap();

    let ndjson = r#"{"resourceType": "Patient", "id": "patient1"}
{"resourceType": "Patient", "id": "patient2"}
{"resourceType": "Patient", "id": "patient3"}"#;

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 1); // Line number 1
    assert_eq!(results[1].0, 2); // Line number 2
    assert_eq!(results[2].0, 3); // Line number 3

    for (_, result) in results {
        assert_eq!(result.resource_type, "Patient");
    }
}

#[test]
fn test_validate_ndjson_with_empty_lines() {
    let validator = FhirValidator::new().unwrap();

    let ndjson = r#"{"resourceType": "Patient", "id": "patient1"}

{"resourceType": "Patient", "id": "patient2"}


{"resourceType": "Patient", "id": "patient3"}"#;

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 1);
    assert_eq!(results[1].0, 3); // Skipped line 2 (empty)
    assert_eq!(results[2].0, 6); // Skipped lines 4-5 (empty)
}

#[test]
fn test_validate_ndjson_with_comments() {
    let validator = FhirValidator::new().unwrap();

    let ndjson = r#"# This is a comment
{"resourceType": "Patient", "id": "patient1"}
# Another comment
{"resourceType": "Patient", "id": "patient2"}"#;

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, 2); // Skipped line 1 (comment)
    assert_eq!(results[1].0, 4); // Skipped line 3 (comment)
}

#[test]
fn test_validate_ndjson_large() {
    let validator = FhirValidator::new().unwrap();

    // Create 1000 lines of NDJSON
    let ndjson: String = (1..=1000)
        .map(|i| format!(r#"{{"resourceType": "Patient", "id": "patient{i}"  }}"#))
        .collect::<Vec<_>>()
        .join("\n");

    let results = validator.validate_ndjson::<Patient>(&ndjson).unwrap();

    assert_eq!(results.len(), 1000);
    for (line_num, result) in &results {
        assert_eq!(result.resource_type, "Patient");
        assert!(*line_num >= 1 && *line_num <= 1000);
    }
}

#[test]
fn test_validate_ndjson_with_errors_preserves_line_numbers() {
    let validator = FhirValidator::new().unwrap();

    let ndjson = r#"{"resourceType": "Patient", "id": "valid", "active": true}
{"resourceType": "Patient", "name": "invalid"}
{"resourceType": "Patient", "id": "valid2", "active": false}"#;

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 1);
    assert_eq!(results[1].0, 2);
    assert_eq!(results[2].0, 3);

    // Check that line 2 has a structural or type error
    let line2_has_parse_error = results[1].1.issues.iter().any(|issue| {
        issue.code == rh_validator::IssueCode::Structure
            || issue.code == rh_validator::IssueCode::ValueType
    });
    assert!(line2_has_parse_error);
}

#[test]
fn test_batch_vs_sequential_same_results() {
    let validator = FhirValidator::new().unwrap();

    let resources = vec![
        r#"{"resourceType": "Patient", "id": "patient1", "active": true}"#,
        r#"{"resourceType": "Patient", "id": "patient2", "gender": "male"}"#,
        r#"{"resourceType": "Patient", "id": "patient3", "birthDate": "1990-01-01"}"#,
    ];

    // Batch validation
    let batch_results = validator.validate_batch::<Patient>(&resources).unwrap();

    // Sequential validation
    let sequential_results: Vec<_> = resources
        .iter()
        .map(|json| validator.validate_full::<Patient>(json).unwrap())
        .collect();

    assert_eq!(batch_results.len(), sequential_results.len());
    for (batch, sequential) in batch_results.iter().zip(sequential_results.iter()) {
        assert_eq!(batch.is_valid(), sequential.is_valid());
        assert_eq!(batch.error_count(), sequential.error_count());
        assert_eq!(batch.warning_count(), sequential.warning_count());
    }
}

#[test]
fn test_empty_batch() {
    let validator = FhirValidator::new().unwrap();
    let resources: Vec<&str> = vec![];

    let results = validator.validate_batch::<Patient>(&resources).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_empty_ndjson() {
    let validator = FhirValidator::new().unwrap();
    let ndjson = "";

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn test_ndjson_only_comments_and_empty() {
    let validator = FhirValidator::new().unwrap();
    let ndjson = r#"# Comment 1

# Comment 2

"#;

    let results = validator.validate_ndjson::<Patient>(ndjson).unwrap();
    assert_eq!(results.len(), 0);
}
