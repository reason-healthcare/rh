//! DateTime function registration for FHIRPath

use crate::error::*;
use crate::evaluator::datetime::DateTimeEvaluator;
use crate::evaluator::values::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all date/time functions
pub fn register_datetime_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // now() function - returns current date and time
    functions.insert(
        "now".to_string(),
        Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "now() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::now()
        }),
    );

    // today() function - returns current date
    functions.insert(
        "today".to_string(),
        Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "today() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::today()
        }),
    );

    // timeOfDay() function - returns current time
    functions.insert(
        "timeOfDay".to_string(),
        Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "timeOfDay() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::time_of_day()
        }),
    );

    // yearOf() function - extract year from Date or DateTime
    functions.insert(
        "yearOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "yearOf() function takes no parameters".to_string(),
                });
            }
            match target {
                FhirPathValue::Collection(items) => {
                    let mut results = Vec::new();
                    for item in items {
                        results.push(DateTimeEvaluator::year_of(item)?);
                    }
                    Ok(FhirPathValue::Collection(results))
                }
                single => DateTimeEvaluator::year_of(single),
            }
        }),
    );

    // monthOf() function - extract month from Date or DateTime
    functions.insert(
        "monthOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "monthOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::month_of(target)
        }),
    );

    // dayOf() function - extract day from Date or DateTime
    functions.insert(
        "dayOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "dayOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::day_of(target)
        }),
    );

    // hourOf() function - extract hour from Time or DateTime
    functions.insert(
        "hourOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "hourOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::hour_of(target)
        }),
    );

    // minuteOf() function - extract minute from Time or DateTime
    functions.insert(
        "minuteOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "minuteOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::minute_of(target)
        }),
    );

    // secondOf() function - extract second from Time or DateTime
    functions.insert(
        "secondOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "secondOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::second_of(target)
        }),
    );

    // millisecondOf() function - extract millisecond from Time or DateTime
    functions.insert(
        "millisecondOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "millisecondOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::millisecond_of(target)
        }),
    );

    // timezoneOffsetOf() function - extract timezone offset from DateTime
    functions.insert(
        "timezoneOffsetOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "timezoneOffsetOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::timezone_offset_of(target)
        }),
    );

    // dateOf() function - extract date from DateTime
    functions.insert(
        "dateOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "dateOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::date_of(target)
        }),
    );

    // timeOf() function - extract time from DateTime
    functions.insert(
        "timeOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "timeOf() function takes no parameters".to_string(),
                });
            }
            DateTimeEvaluator::time_of(target)
        }),
    );
}
