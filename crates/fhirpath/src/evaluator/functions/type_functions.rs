use crate::ast::TypeSpecifier;
use crate::error::*;
use crate::evaluator::types::{operations::TypeEvaluator, FhirPathValue};
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all type functions (backward compatibility)
pub fn register_type_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // is() function - backward compatibility for 'is' operator
    functions.insert(
        "is".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::FunctionError {
                    message: "is() function requires exactly one parameter (type specifier)"
                        .to_string(),
                });
            }

            // Extract type specifier from parameter
            let type_spec = match &params[0] {
                FhirPathValue::String(type_name) => TypeSpecifier {
                    qualified_name: vec![type_name.clone()],
                },
                _ => {
                    return Err(FhirPathError::FunctionError {
                        message: "is() function parameter must be a type name string".to_string(),
                    })
                }
            };

            TypeEvaluator::evaluate_type_operation(
                target,
                &crate::ast::TypeOperator::Is,
                &type_spec,
            )
        }),
    );

    // as() function - backward compatibility for 'as' operator
    functions.insert(
        "as".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::FunctionError {
                    message: "as() function requires exactly one parameter (type specifier)"
                        .to_string(),
                });
            }

            // Extract type specifier from parameter
            let type_spec = match &params[0] {
                FhirPathValue::String(type_name) => TypeSpecifier {
                    qualified_name: vec![type_name.clone()],
                },
                _ => {
                    return Err(FhirPathError::FunctionError {
                        message: "as() function parameter must be a type name string".to_string(),
                    })
                }
            };

            TypeEvaluator::evaluate_type_operation(
                target,
                &crate::ast::TypeOperator::As,
                &type_spec,
            )
        }),
    );
}
