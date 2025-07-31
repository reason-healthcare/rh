//! Integration tests for FHIRPath date/time functions

use chrono::{DateTime, Local, NaiveDate, NaiveTime, Timelike, Utc};
use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use serde_json::json;

#[test]
fn test_now_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test now() function call
    let expr = parser.parse("now()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::DateTime(dt_str) => {
            // Verify it's a valid ISO 8601 datetime with UTC timezone
            assert!(dt_str.ends_with('Z'), "DateTime should end with Z for UTC");

            // Try to parse it back to verify format
            let parsed = DateTime::parse_from_rfc3339(&dt_str);
            assert!(parsed.is_ok(), "DateTime should be valid RFC3339: {dt_str}");

            // Verify it's close to current time (within 1 second)
            let parsed_utc = parsed.unwrap().with_timezone(&Utc);
            let system_now = Utc::now();
            let diff = (system_now - parsed_utc).num_seconds().abs();
            assert!(
                diff <= 1,
                "now() should return current time within 1 second"
            );
        }
        _ => panic!("now() should return a DateTime value, got: {result:?}"),
    }
}

#[test]
fn test_today_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test today() function call
    let expr = parser.parse("today()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::Date(date_str) => {
            // Verify it's a valid ISO 8601 date
            assert_eq!(
                date_str.len(),
                10,
                "Date should be 10 characters (YYYY-MM-DD)"
            );

            // Try to parse it back to verify format
            let parsed = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");
            assert!(
                parsed.is_ok(),
                "Date should be valid YYYY-MM-DD: {date_str}"
            );

            // Verify it's today's date
            let system_today = Local::now().date_naive();
            let parsed_date = parsed.unwrap();
            assert_eq!(
                parsed_date, system_today,
                "today() should return current date"
            );
        }
        _ => panic!("today() should return a Date value, got: {result:?}"),
    }
}

#[test]
fn test_time_of_day_function() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test timeOfDay() function call
    let expr = parser.parse("timeOfDay()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();

    match result {
        FhirPathValue::Time(time_str) => {
            // Verify it's a valid ISO 8601 time
            assert!(
                time_str.len() >= 8,
                "Time should be at least 8 characters (HH:MM:SS)"
            );

            // Try to parse it back to verify format
            let parsed = NaiveTime::parse_from_str(&time_str, "%H:%M:%S%.3f");
            assert!(
                parsed.is_ok(),
                "Time should be valid HH:MM:SS.sss: {time_str}"
            );

            // Verify it's reasonable (within current day)
            let system_time = Local::now().time();
            let parsed_time = parsed.unwrap();

            // Allow for up to 1 second difference
            let system_seconds = system_time.num_seconds_from_midnight();
            let parsed_seconds = parsed_time.num_seconds_from_midnight();
            let diff = (system_seconds as i32 - parsed_seconds as i32).abs();
            assert!(
                diff <= 1,
                "timeOfDay() should return current time within 1 second, diff: {diff} seconds"
            );
        }
        _ => panic!("timeOfDay() should return a Time value, got: {result:?}"),
    }
}

#[test]
fn test_datetime_functions_no_parameters() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test that datetime functions reject parameters
    let expr = parser.parse("now(5)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err(), "now() should reject parameters");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("takes no parameters"));

    let expr = parser.parse("today('test')").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err(), "today() should reject parameters");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("takes no parameters"));

    let expr = parser.parse("timeOfDay(1, 2)").unwrap();
    let result = evaluator.evaluate(&expr, &context);
    assert!(result.is_err(), "timeOfDay() should reject parameters");
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("takes no parameters"));
}

#[test]
fn test_datetime_functions_in_expressions() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test using datetime functions in complex expressions
    let expr = parser.parse("now().exists()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::Boolean(true),
        "now().exists() should be true"
    );

    let expr = parser.parse("today().exists()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::Boolean(true),
        "today().exists() should be true"
    );

    let expr = parser.parse("timeOfDay().exists()").unwrap();
    let result = evaluator.evaluate(&expr, &context).unwrap();
    assert_eq!(
        result,
        FhirPathValue::Boolean(true),
        "timeOfDay().exists() should be true"
    );
}

#[test]
fn test_multiple_datetime_function_calls() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test that multiple calls return consistent values (within reasonable bounds)
    let expr = parser.parse("now()").unwrap();
    let result1 = evaluator.evaluate(&expr, &context).unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));

    let result2 = evaluator.evaluate(&expr, &context).unwrap();

    // Both should be DateTime values
    assert!(matches!(result1, FhirPathValue::DateTime(_)));
    assert!(matches!(result2, FhirPathValue::DateTime(_)));

    // They should be different (since time has passed)
    assert_ne!(
        result1, result2,
        "Multiple now() calls should return different values"
    );

    // Test today() returns same value (should be same day)
    let expr = parser.parse("today()").unwrap();
    let today1 = evaluator.evaluate(&expr, &context).unwrap();
    let today2 = evaluator.evaluate(&expr, &context).unwrap();

    // Both should be the same date
    assert_eq!(
        today1, today2,
        "Multiple today() calls should return same date"
    );
}
