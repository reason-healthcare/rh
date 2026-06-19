//! Integration tests for date/time component extraction functions

use rh_fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
use rust_decimal::Decimal;
use serde_json::json;

fn extract_single_value(result: FhirPathValue) -> FhirPathValue {
    match result {
        FhirPathValue::Collection(mut values) => {
            if values.len() == 1 {
                values.pop().unwrap()
            } else {
                FhirPathValue::Collection(values)
            }
        }
        single => single,
    }
}

fn assert_empty(result: FhirPathValue) {
    match result {
        FhirPathValue::Empty => {}
        FhirPathValue::Collection(values) if values.is_empty() => {}
        other => panic!("Expected Empty value, got: {other:?}"),
    }
}

#[test]
fn test_year_of_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    let expression = parser.parse("@2023-07-15T14:30:25Z.yearOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(year) = value {
        assert_eq!(year, 2023);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }
}

#[test]
fn test_month_day_of_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test monthOf()
    let expression = parser.parse("@2023-07-15.monthOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(month) = value {
        assert_eq!(month, 7);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }

    // Test dayOf()
    let expression = parser.parse("@2023-07-15.dayOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(day) = value {
        assert_eq!(day, 15);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }
}

#[test]
fn test_time_components_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test hourOf()
    let expression = parser.parse("@T14:30:25.hourOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(hour) = value {
        assert_eq!(hour, 14);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }

    // Test minuteOf()
    let expression = parser.parse("@2023-07-15T14:30:25Z.minuteOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(minute) = value {
        assert_eq!(minute, 30);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }

    // Test secondOf()
    let expression = parser.parse("@T14:30:25.secondOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(second) = value {
        assert_eq!(second, 25);
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }

    // Test millisecondOf() with no millisecond precision.
    let expression = parser
        .parse("@2023-07-15T14:30:25Z.millisecondOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    assert_empty(result);
}

#[test]
fn test_timezone_offset_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test UTC offset
    let expression = parser
        .parse("@2023-07-15T14:30:25Z.timezoneOffsetOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Number(offset) = value {
        assert_eq!(offset, Decimal::from_str_exact("0.0").unwrap());
    } else {
        panic!("Expected Number value, got: {value:?}");
    }

    // Test negative offset
    let expression = parser
        .parse("@2023-07-15T14:30:25-05:00.timezoneOffsetOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Number(offset) = value {
        assert_eq!(offset, Decimal::from_str_exact("-5.0").unwrap());
    } else {
        panic!("Expected Number value, got: {value:?}");
    }
}

#[test]
fn test_date_time_extraction_integration() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test dateOf()
    let expression = parser.parse("@2023-07-15T14:30:25Z.dateOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Date(date) = value {
        assert_eq!(date.to_string(), "2023-07-15");
    } else {
        panic!("Expected Date value, got: {value:?}");
    }

    // Test timeOf()
    let expression = parser.parse("@2023-07-15T14:30:25Z.timeOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Time(time) = value {
        assert_eq!(time.to_string(), "14:30:25");
    } else {
        panic!("Expected Time value, got: {value:?}");
    }
}

#[test]
fn test_chained_component_operations() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test chaining with comparisons
    let expression = parser.parse("now().yearOf() > 2020").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Boolean(is_true) = value {
        assert!(is_true);
    } else {
        panic!("Expected Boolean value, got: {value:?}");
    }

    // Test chaining with equality
    let expression = parser.parse("@2023-07-15T14:30:25Z.monthOf() = 7").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Boolean(is_true) = value {
        assert!(is_true);
    } else {
        panic!("Expected Boolean value, got: {value:?}");
    }

    // Component functions require a singleton input.
    let expression = parser
        .parse("(@2023-07-15 | @2024-12-25).yearOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context);
    assert!(result.is_err(), "Expected error for multi-item input");
}

#[test]
fn test_component_extraction_error_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Invalid scalar inputs and absent timezone precision return Empty.
    let expression = parser.parse("'not a date'.yearOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();
    assert_empty(result);

    let expression = parser.parse("@2023-07-15.timezoneOffsetOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();
    assert_empty(result);
}
