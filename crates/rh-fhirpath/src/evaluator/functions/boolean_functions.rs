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

/// Logical NOT function - negates a boolean value
///
/// According to FHIRPath specification:
/// - not(true) -> false
/// - not(false) -> true
/// - not({}) -> {} (empty collection)
/// - If input is a collection with multiple items, returns empty
/// - If input is not boolean, returns empty
fn not_function(value: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
    match value {
        // If the collection is empty, return empty
        FhirPathValue::Empty => Ok(FhirPathValue::Empty),

        // If the collection contains more than one item, return empty
        FhirPathValue::Collection(items) => {
            if items.len() == 1 {
                not_function(&items[0])
            } else {
                Ok(FhirPathValue::Empty)
            }
        }

        // Boolean values: negate them
        FhirPathValue::Boolean(b) => Ok(FhirPathValue::Boolean(!b)),

        // All other types return empty (cannot negate non-boolean values)
        _ => Ok(FhirPathValue::Empty),
    }
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
        let result = not_function(&collection).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }

    #[test]
    fn test_not_function_with_non_boolean() {
        let result = not_function(&FhirPathValue::String("hello".to_string())).unwrap();
        assert_eq!(result, FhirPathValue::Empty);

        let result = not_function(&FhirPathValue::Integer(42)).unwrap();
        assert_eq!(result, FhirPathValue::Empty);
    }
}
