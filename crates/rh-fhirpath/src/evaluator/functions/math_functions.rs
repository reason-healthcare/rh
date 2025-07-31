//! Math function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::math::MathEvaluator;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all math functions
pub fn register_math_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // abs() function
    functions.insert(
        "abs".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::abs(target)),
    );

    // ceiling() function
    functions.insert(
        "ceiling".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            MathEvaluator::ceiling(target)
        }),
    );

    // exp() function
    functions.insert(
        "exp".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::exp(target)),
    );

    // floor() function
    functions.insert(
        "floor".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::floor(target)),
    );

    // ln() function
    functions.insert(
        "ln".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::ln(target)),
    );

    // log() function - requires base parameter
    functions.insert(
        "log".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "log() requires exactly one parameter (base)".to_string(),
                });
            }
            MathEvaluator::log(target, &params[0])
        }),
    );

    // power() function - requires exponent parameter
    functions.insert(
        "power".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "power() requires exactly one parameter (exponent)".to_string(),
                });
            }
            MathEvaluator::power(target, &params[0])
        }),
    );

    // round() function - optional precision parameter
    functions.insert(
        "round".to_string(),
        Box::new(
            |target: &FhirPathValue, params: &[FhirPathValue]| match params.len() {
                0 => MathEvaluator::round(target, None),
                1 => MathEvaluator::round(target, Some(&params[0])),
                _ => Err(FhirPathError::InvalidOperation {
                    message: "round() accepts at most one parameter (precision)".to_string(),
                }),
            },
        ),
    );

    // sqrt() function
    functions.insert(
        "sqrt".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| MathEvaluator::sqrt(target)),
    );

    // truncate() function
    functions.insert(
        "truncate".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            MathEvaluator::truncate(target)
        }),
    );
}
