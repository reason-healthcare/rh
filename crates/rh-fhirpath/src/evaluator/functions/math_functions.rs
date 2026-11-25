//! Math function registration for FHIRPath.

use std::collections::HashMap;

use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::math::MathEvaluator;
use crate::evaluator::types::FhirPathValue;

use super::FhirPathFunction;

pub fn register_math_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    type UnaryMath = fn(&FhirPathValue) -> FhirPathResult<FhirPathValue>;
    let unary: [(&str, UnaryMath); 7] = [
        ("abs", MathEvaluator::abs),
        ("ceiling", MathEvaluator::ceiling),
        ("exp", MathEvaluator::exp),
        ("floor", MathEvaluator::floor),
        ("ln", MathEvaluator::ln),
        ("sqrt", MathEvaluator::sqrt),
        ("truncate", MathEvaluator::truncate),
    ];
    for (name, evaluate) in unary {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: format!("{name}() takes no parameters"),
                    });
                }
                evaluate(target)
            }),
        );
    }

    type BinaryMath = fn(&FhirPathValue, &FhirPathValue) -> FhirPathResult<FhirPathValue>;
    let binary: [(&str, &str, BinaryMath); 2] = [
        ("log", "base", MathEvaluator::log),
        ("power", "exponent", MathEvaluator::power),
    ];
    for (name, parameter, evaluate) in binary {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                let [value] = params else {
                    return Err(FhirPathError::InvalidOperation {
                        message: format!("{name}() requires one {parameter} parameter"),
                    });
                };
                evaluate(target, value)
            }),
        );
    }

    functions.insert(
        "round".to_owned(),
        Box::new(|target, params| match params {
            [] => MathEvaluator::round(target, None),
            [precision] => MathEvaluator::round(target, Some(precision)),
            _ => Err(FhirPathError::InvalidOperation {
                message: "round() accepts one optional precision parameter".to_string(),
            }),
        }),
    );
}
