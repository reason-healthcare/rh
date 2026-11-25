//! Boolean function registration for FHIRPath

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register all boolean operation functions
pub fn register_boolean_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // not() function
    functions.insert(
        "not".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| not_function(target)),
    );
}

/// Negate the input under FHIRPath singleton Boolean semantics.
fn not_function(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    Ok(value
        .singleton_boolean("not() input")?
        .map_or(FhirPathValue::Empty, |value| FhirPathValue::Boolean(!value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_function_with_boolean_true() {
        let result = not_function(&FhirPathValue::Boolean(true)).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_not_function_with_boolean_false() {
        let result = not_function(&FhirPathValue::Boolean(false)).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(true));
    }

    #[test]
    fn test_not_function_with_empty_collection() {
        let result = not_function(&FhirPathValue::Empty).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_not_function_with_single_item_collection() {
        let collection = FhirPathValue::Collection(vec![FhirPathValue::Boolean(true)]);
        let result = not_function(&collection).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }

    #[test]
    fn test_not_function_with_multiple_item_collection() {
        let collection = FhirPathValue::Collection(vec![
            FhirPathValue::Boolean(true),
            FhirPathValue::Boolean(false),
        ]);
        assert!(not_function(&collection).is_err());
    }

    #[test]
    fn test_not_function_with_non_boolean() {
        // Non-boolean scalars are truthy; not(truthy) = false
        let result = not_function(&FhirPathValue::String("hello".to_string())).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));

        let result = not_function(&FhirPathValue::Integer(42)).unwrap();
        assert_eq!(result, FhirPathValue::Boolean(false));
    }
}
