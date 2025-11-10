/// Test to check what our validator reports for contained.json
///
/// **Known Issue**: The validator incorrectly reports 10 ext-1 invariant errors.
/// The ext-1 invariant ("Must have either extensions or value[x], not both") should
/// only apply to Extension elements, but is being applied to all elements.
/// This causes valid resources to be reported as INVALID (false negatives).
///
/// Expected: VALID (per Java validator)
/// Actual: INVALID (12 errors + 2 warnings)
#[test]
fn test_contained_validation() {
    let resource_json = r##"{
  "resourceType": "Questionnaire",
  "id": "questionnaire-test-frag",
  "url": "http://example.com/fhir/Questionnaire/test-1",
  "status": "draft",
  "item": [
    {
      "linkId": "q1",
      "type": "choice",
      "answerValueSet": "#options-1"
    }
  ],
  "contained": [
    {
      "resourceType": "ValueSet",
      "id": "options-1",
      "url": "http://example.com/fhir/ValueSet/options-1",
      "status": "draft"
    }
  ]
}"##;

    let resource: serde_json::Value =
        serde_json::from_str(resource_json).expect("Failed to parse JSON");

    let validator = rh_validator::FhirValidator::new(rh_validator::FhirVersion::R4, None)
        .expect("Failed to create validator");

    let result = validator
        .validate_auto(&resource)
        .expect("Failed to validate");

    println!("\n========================================");
    println!("RH Validator Results for contained.json");
    println!("========================================");
    println!("Total issues: {}", result.issues.len());
    println!();

    let errors: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Error)
        .collect();
    let warnings: Vec<_> = result
        .issues
        .iter()
        .filter(|i| i.severity == rh_validator::Severity::Warning)
        .collect();

    println!("Errors: {}", errors.len());
    println!("Warnings: {}", warnings.len());
    println!();

    if !errors.is_empty() {
        println!("ERROR ISSUES:");
        for (i, issue) in errors.iter().enumerate() {
            println!("  {}. [{:?}] {}", i + 1, issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
        }
        println!();
    }

    if !warnings.is_empty() {
        println!("WARNING ISSUES:");
        for (i, issue) in warnings.iter().enumerate() {
            println!("  {}. [{:?}] {}", i + 1, issue.code, issue.message);
            if let Some(path) = &issue.path {
                println!("     Path: {path}");
            }
        }
        println!();
    }

    println!("========================================");
    println!("Expected: VALID (Java validator)");
    println!(
        "RH reports: {} ({} errors + {} warnings)",
        if errors.is_empty() {
            "VALID"
        } else {
            "INVALID"
        },
        errors.len(),
        warnings.len()
    );
    println!("========================================\n");
}
