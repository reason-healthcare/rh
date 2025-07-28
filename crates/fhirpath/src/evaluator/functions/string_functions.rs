//! String function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::strings::StringEvaluator;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all string manipulation functions
pub fn register_string_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // length() function
    functions.insert(
        "length".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::length(target)
        }),
    );

    // upper() function
    functions.insert(
        "upper".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::upper(target)
        }),
    );

    // lower() function
    functions.insert(
        "lower".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::lower(target)
        }),
    );

    // trim() function
    functions.insert(
        "trim".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| StringEvaluator::trim(target)),
    );

    // substring() function
    functions.insert(
        "substring".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.is_empty() {
                return Err(FhirPathError::InvalidOperation {
                    message: "substring() requires at least one parameter (start index)"
                        .to_string(),
                });
            }
            let start = &params[0];
            let length = params.get(1);
            StringEvaluator::substring(target, start, length)
        }),
    );

    // startsWith() function
    functions.insert(
        "startsWith".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "startsWith() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::starts_with(target, &params[0])
        }),
    );

    // endsWith() function
    functions.insert(
        "endsWith".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "endsWith() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::ends_with(target, &params[0])
        }),
    );

    // indexOf() function
    functions.insert(
        "indexOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "indexOf() requires exactly one parameter".to_string(),
                });
            }
            StringEvaluator::index_of(target, &params[0])
        }),
    );

    // replace() function
    functions.insert(
        "replace".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 2 {
                return Err(FhirPathError::InvalidOperation {
                    message: "replace() requires exactly two parameters (pattern, replacement)"
                        .to_string(),
                });
            }
            StringEvaluator::replace(target, &params[0], &params[1])
        }),
    );

    // replaceMatches() function
    functions.insert(
        "replaceMatches".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 2 {
                return Err(FhirPathError::InvalidOperation {
                    message:
                        "replaceMatches() requires exactly two parameters (regex, substitution)"
                            .to_string(),
                });
            }
            StringEvaluator::replace_matches(target, &params[0], &params[1])
        }),
    );

    // split() function
    functions.insert(
        "split".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "split() requires exactly one parameter (delimiter)".to_string(),
                });
            }
            StringEvaluator::split(target, &params[0])
        }),
    );

    // join() function
    functions.insert(
        "join".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "join() requires exactly one parameter (delimiter)".to_string(),
                });
            }
            StringEvaluator::join(target, &params[0])
        }),
    );

    // matches() function
    functions.insert(
        "matches".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "matches() requires exactly one parameter (pattern)".to_string(),
                });
            }
            StringEvaluator::matches(target, &params[0])
        }),
    );

    // matchesFull() function
    functions.insert(
        "matchesFull".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "matchesFull() requires exactly one parameter (pattern)".to_string(),
                });
            }
            StringEvaluator::matches_full(target, &params[0])
        }),
    );

    // contains() function
    functions.insert(
        "contains".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "contains() requires exactly one parameter (substring)".to_string(),
                });
            }
            StringEvaluator::contains(target, &params[0])
        }),
    );

    // toChars() function
    functions.insert(
        "toChars".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            StringEvaluator::to_chars(target)
        }),
    );
}
