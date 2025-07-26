//! Integration tests for date/time component extraction functions

use fhirpath::{EvaluationContext, FhirPathEvaluator, FhirPathParser, FhirPathValue};
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

    // Test millisecondOf() - since parser doesn't support milliseconds in literals,
    // we'll test with a datetime without milliseconds (should return 0)
    let expression = parser
        .parse("@2023-07-15T14:30:25Z.millisecondOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    let value = extract_single_value(result);
    if let FhirPathValue::Integer(ms) = value {
        assert_eq!(ms, 0); // No milliseconds in the literal
    } else {
        panic!("Expected Integer value, got: {value:?}");
    }
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
        assert_eq!(offset, 0.0);
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
        assert_eq!(offset, -5.0);
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
        assert!(time.to_string().starts_with("14:30:25"));
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

    // Test with collections
    let expression = parser
        .parse("(@2023-07-15 | @2024-12-25).yearOf()")
        .unwrap();
    let result = evaluator.evaluate(&expression, &context).unwrap();

    if let FhirPathValue::Collection(values) = result {
        assert_eq!(values.len(), 2);
        if let FhirPathValue::Integer(year1) = &values[0] {
            assert_eq!(*year1, 2023);
        } else {
            panic!("Expected Integer value, got: {:?}", values[0]);
        }
        if let FhirPathValue::Integer(year2) = &values[1] {
            assert_eq!(*year2, 2024);
        } else {
            panic!("Expected Integer value, got: {:?}", values[1]);
        }
    } else {
        panic!("Expected collection result, got: {result:?}");
    }
}

#[test]
fn test_component_extraction_error_cases() {
    let parser = FhirPathParser::new();
    let evaluator = FhirPathEvaluator::new();
    let context = EvaluationContext::new(json!({}));

    // Test with invalid date
    let expression = parser.parse("'not a date'.yearOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context);
    assert!(result.is_err(), "Expected error for invalid date");

    // Test timezone offset on a date without timezone
    let expression = parser.parse("@2023-07-15.timezoneOffsetOf()").unwrap();
    let result = evaluator.evaluate(&expression, &context);
    assert!(result.is_err(), "Expected error for date without timezone");
}
