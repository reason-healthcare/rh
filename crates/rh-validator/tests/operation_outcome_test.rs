use rh_validator::{IssueCode, ValidationIssue, ValidationResult};

#[test]
fn test_operation_outcome_structure() {
    let mut result = ValidationResult::valid();
    result = result.with_issue(ValidationIssue::error(
        IssueCode::Required,
        "Missing required field",
    ));

    let outcome = result.to_operation_outcome();

    assert_eq!(outcome.resource_type, "OperationOutcome");
    assert_eq!(outcome.issue.len(), 1);
    assert_eq!(outcome.issue[0].severity, "error");
    assert_eq!(outcome.issue[0].code, "required");
    assert_eq!(
        outcome.issue[0].diagnostics,
        Some("Missing required field".to_string())
    );
}

#[test]
fn test_operation_outcome_severity_mapping() {
    let mut result = ValidationResult::valid();

    result = result.with_issue(ValidationIssue::error(IssueCode::Required, "Error message"));
    result = result.with_issue(ValidationIssue::warning(
        IssueCode::Value,
        "Warning message",
    ));
    result = result.with_issue(ValidationIssue::info(
        IssueCode::Informational,
        "Info message",
    ));

    let outcome = result.to_operation_outcome();

    assert_eq!(outcome.issue.len(), 3);
    assert_eq!(outcome.issue[0].severity, "error");
    assert_eq!(outcome.issue[1].severity, "warning");
    assert_eq!(outcome.issue[2].severity, "information");
}

#[test]
fn test_operation_outcome_code_mapping() {
    let test_cases = vec![
        (IssueCode::Structure, "structure"),
        (IssueCode::Required, "required"),
        (IssueCode::Value, "value"),
        (IssueCode::Invariant, "invariant"),
        (IssueCode::CodeInvalid, "code-invalid"),
        (IssueCode::NotFound, "not-found"),
    ];

    for (issue_code, expected_fhir_code) in test_cases {
        let mut result = ValidationResult::valid();
        result = result.with_issue(ValidationIssue::error(issue_code, "Test message"));

        let outcome = result.to_operation_outcome();
        assert_eq!(outcome.issue[0].code, expected_fhir_code);
    }
}

#[test]
fn test_operation_outcome_with_path() {
    let mut result = ValidationResult::valid();
    result = result.with_issue(
        ValidationIssue::error(IssueCode::Required, "Missing field").with_path("Patient.name"),
    );

    let outcome = result.to_operation_outcome();

    assert_eq!(
        outcome.issue[0].location,
        Some(vec!["Patient.name".to_string()])
    );
    assert_eq!(
        outcome.issue[0].expression,
        Some(vec!["Patient.name".to_string()])
    );
}

#[test]
fn test_operation_outcome_without_path() {
    let mut result = ValidationResult::valid();
    result = result.with_issue(ValidationIssue::error(
        IssueCode::Structure,
        "Invalid structure",
    ));

    let outcome = result.to_operation_outcome();

    assert_eq!(outcome.issue[0].location, None);
    assert_eq!(outcome.issue[0].expression, None);
}

#[test]
fn test_operation_outcome_multiple_issues() {
    let mut result = ValidationResult::valid();

    result = result.with_issue(
        ValidationIssue::error(IssueCode::Required, "Missing name").with_path("Patient.name"),
    );
    result = result.with_issue(
        ValidationIssue::error(IssueCode::Required, "Missing gender").with_path("Patient.gender"),
    );
    result = result.with_issue(ValidationIssue::warning(IssueCode::Value, "Invalid format"));

    let outcome = result.to_operation_outcome();

    assert_eq!(outcome.issue.len(), 3);
    assert_eq!(
        outcome.issue[0].diagnostics,
        Some("Missing name".to_string())
    );
    assert_eq!(
        outcome.issue[1].diagnostics,
        Some("Missing gender".to_string())
    );
    assert_eq!(
        outcome.issue[2].diagnostics,
        Some("Invalid format".to_string())
    );
}

#[test]
fn test_operation_outcome_json_serialization() {
    let mut result = ValidationResult::valid();
    result = result.with_issue(
        ValidationIssue::error(IssueCode::Required, "Missing required field 'name'")
            .with_path("Patient.name"),
    );

    let outcome = result.to_operation_outcome();
    let json = serde_json::to_value(&outcome).unwrap();

    assert_eq!(json["resourceType"], "OperationOutcome");
    assert_eq!(json["issue"][0]["severity"], "error");
    assert_eq!(json["issue"][0]["code"], "required");
    assert_eq!(
        json["issue"][0]["diagnostics"],
        "Missing required field 'name'"
    );
    assert_eq!(json["issue"][0]["location"][0], "Patient.name");
    assert_eq!(json["issue"][0]["expression"][0], "Patient.name");
}

#[test]
fn test_operation_outcome_empty_issues() {
    let result = ValidationResult::valid();
    let outcome = result.to_operation_outcome();

    assert_eq!(outcome.resource_type, "OperationOutcome");
    assert_eq!(outcome.issue.len(), 0);
}

#[test]
fn test_operation_outcome_camel_case() {
    let mut result = ValidationResult::valid();
    result = result.with_issue(ValidationIssue::error(IssueCode::Required, "Test"));

    let outcome = result.to_operation_outcome();
    let json = serde_json::to_value(&outcome).unwrap();

    assert!(json.get("resourceType").is_some());
    assert!(json.get("issue").is_some());
}

#[test]
fn test_operation_outcome_all_issue_codes() {
    let all_codes = vec![
        IssueCode::Structure,
        IssueCode::Required,
        IssueCode::Value,
        IssueCode::Invariant,
        IssueCode::Invalid,
        IssueCode::CodeInvalid,
        IssueCode::Extension,
        IssueCode::Forbidden,
        IssueCode::Incomplete,
        IssueCode::TooCostly,
        IssueCode::BusinessRule,
        IssueCode::Conflict,
        IssueCode::NotSupported,
        IssueCode::Duplicate,
        IssueCode::NotFound,
        IssueCode::TooLong,
        IssueCode::CodeInvalidInValueSet,
        IssueCode::InvalidDisplay,
        IssueCode::Processing,
        IssueCode::NotAllowed,
        IssueCode::Informational,
    ];

    for code in all_codes {
        let mut result = ValidationResult::valid();
        result = result.with_issue(ValidationIssue::error(code.clone(), "Test"));

        let outcome = result.to_operation_outcome();
        assert!(!outcome.issue[0].code.is_empty());
    }
}
