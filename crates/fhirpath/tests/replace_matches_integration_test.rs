use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser};
use serde_json::json;

#[test]
fn test_replace_matches_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();

    // Test basic digit replacement
    let data = json!({
        "text": "Patient ID: 12345, Visit: 67890"
    });

    let expr = parser
        .parse("text.replaceMatches('[0-9]+', 'XXX')")
        .unwrap();
    let context = EvaluationContext::new(data);
    let result = evaluator.evaluate(&expr, &context).unwrap();

    // Check if we got the expected result
    println!("Result: {:?}", result);
    // The result should be a String with the replacements made
    assert!(matches!(result, fhirpath::FhirPathValue::String(_)));
    if let fhirpath::FhirPathValue::String(s) = result {
        assert_eq!(s, "Patient ID: XXX, Visit: XXX");
    }
}
