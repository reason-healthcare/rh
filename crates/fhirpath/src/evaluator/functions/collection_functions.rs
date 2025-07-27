//! Collection function registration for FHIRPath

use crate::error::*;
use crate::evaluator::collection::CollectionEvaluator;
use crate::evaluator::values::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register basic collection functions (empty, exists, count, etc.)
pub fn register_collection_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // empty() function
    functions.insert(
        "empty".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::is_empty(target)
        }),
    );

    // exists() function
    functions.insert(
        "exists".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::exists(target)
        }),
    );

    // count() function
    functions.insert(
        "count".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::count(target)
        }),
    );

    // distinct() function
    functions.insert(
        "distinct".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::distinct(target)
        }),
    );

    // isDistinct() function
    functions.insert(
        "isDistinct".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::is_distinct(target)
        }),
    );
}

/// Register boolean collection functions (all, allTrue, anyTrue, etc.)
pub fn register_boolean_collection_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // all() function
    functions.insert(
        "all".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all(target)
        }),
    );

    // allTrue() function
    functions.insert(
        "allTrue".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all_true(target)
        }),
    );

    // anyTrue() function
    functions.insert(
        "anyTrue".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::any_true(target)
        }),
    );

    // allFalse() function
    functions.insert(
        "allFalse".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all_false(target)
        }),
    );

    // anyFalse() function
    functions.insert(
        "anyFalse".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::any_false(target)
        }),
    );
}

/// Register subsetting functions (single, first, last, skip, take, etc.)
pub fn register_subsetting_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // single() function
    functions.insert(
        "single".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::single(target)
        }),
    );

    // first() function
    functions.insert(
        "first".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::first(target)
        }),
    );

    // last() function
    functions.insert(
        "last".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::last(target)
        }),
    );

    // tail() function
    functions.insert(
        "tail".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::tail(target)
        }),
    );

    // skip() function
    functions.insert(
        "skip".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "skip() requires exactly one parameter (count)".to_string(),
                });
            }
            match &params[0] {
                FhirPathValue::Integer(count) => CollectionEvaluator::skip(target, *count),
                _ => Err(FhirPathError::InvalidOperation {
                    message: "skip() count parameter must be an integer".to_string(),
                }),
            }
        }),
    );

    // take() function
    functions.insert(
        "take".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "take() requires exactly one parameter (count)".to_string(),
                });
            }
            match &params[0] {
                FhirPathValue::Integer(count) => CollectionEvaluator::take(target, *count),
                _ => Err(FhirPathError::InvalidOperation {
                    message: "take() count parameter must be an integer".to_string(),
                }),
            }
        }),
    );

    // intersect() function
    functions.insert(
        "intersect".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "intersect() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::intersect(target, &params[0])
        }),
    );

    // exclude() function
    functions.insert(
        "exclude".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "exclude() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::exclude(target, &params[0])
        }),
    );

    // combine() function
    functions.insert(
        "combine".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "combine() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::combine(target, &params[0])
        }),
    );

    // subsetOf() function
    functions.insert(
        "subsetOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "subsetOf() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::subset_of(target, &params[0])
        }),
    );

    // supersetOf() function
    functions.insert(
        "supersetOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "supersetOf() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::superset_of(target, &params[0])
        }),
    );
}

/// Register control flow functions (iif)
pub fn register_control_flow_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // iif() function - immediate if (conditional operator)
    functions.insert(
        "iif".to_string(),
        Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() < 2 || params.len() > 3 {
                return Err(FhirPathError::InvalidOperation {
                    message: "iif() requires 2 or 3 parameters (criterion, true_result, [otherwise_result])".to_string(),
                });
            }
            let criterion = &params[0];
            let true_result = &params[1];
            let otherwise_result = params.get(2);
            CollectionEvaluator::iif(criterion, true_result, otherwise_result)
        }),
    );
}
