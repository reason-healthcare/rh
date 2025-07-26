//! Collection operations for FHIRPath values

use crate::error::*;
use crate::evaluator::values::FhirPathValue;

/// Collection operations handler
pub struct CollectionEvaluator;

impl CollectionEvaluator {
    /// Evaluate union operation
    pub fn evaluate_union(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();

        // Add items from left operand
        match left {
            FhirPathValue::Collection(items) => {
                result.extend(items.clone());
            }
            FhirPathValue::Empty => {
                // Don't add anything
            }
            value => {
                result.push(value.clone());
            }
        }

        // Add items from right operand
        match right {
            FhirPathValue::Collection(items) => {
                result.extend(items.clone());
            }
            FhirPathValue::Empty => {
                // Don't add anything
            }
            value => {
                result.push(value.clone());
            }
        }

        if result.is_empty() {
            Ok(FhirPathValue::Empty)
        } else if result.len() == 1 {
            Ok(result.into_iter().next().unwrap())
        } else {
            Ok(FhirPathValue::Collection(result))
        }
    }

    /// Evaluate indexer operation
    pub fn evaluate_indexer(
        target: &FhirPathValue,
        index: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        match (target, index) {
            (FhirPathValue::Collection(items), FhirPathValue::Integer(idx)) => {
                let idx = *idx as usize;
                if idx < items.len() {
                    Ok(items[idx].clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            _ => Err(FhirPathError::InvalidOperation {
                message: "Invalid indexer operation".to_string(),
            }),
        }
    }

    /// Remove duplicates from a collection  
    pub fn distinct(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Collection(items) => {
                let mut unique_items = Vec::new();
                for item in items {
                    if !unique_items
                        .iter()
                        .any(|existing| FhirPathValue::equals_static(existing, item))
                    {
                        unique_items.push(item.clone());
                    }
                }

                if unique_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else if unique_items.len() == 1 {
                    Ok(unique_items.into_iter().next().unwrap())
                } else {
                    Ok(FhirPathValue::Collection(unique_items))
                }
            }
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            value => Ok(value.clone()), // Single values are already distinct
        }
    }

    /// Check if all items in a collection are distinct
    pub fn is_distinct(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Collection(items) => {
                let mut seen = Vec::new();
                for item in items {
                    if seen
                        .iter()
                        .any(|existing| FhirPathValue::equals_static(existing, item))
                    {
                        return Ok(FhirPathValue::Boolean(false));
                    }
                    seen.push(item.clone());
                }
                Ok(FhirPathValue::Boolean(true))
            }
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)), // Empty is distinct
            _ => Ok(FhirPathValue::Boolean(true)),                    // Single values are distinct
        }
    }

    /// Check if collection is empty
    pub fn is_empty(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Collection(items) => Ok(FhirPathValue::Boolean(items.is_empty())),
            _ => Ok(FhirPathValue::Boolean(false)), // Single values are not empty
        }
    }

    /// Check if collection exists (has any items)
    pub fn exists(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
            FhirPathValue::Collection(items) => Ok(FhirPathValue::Boolean(!items.is_empty())),
            _ => Ok(FhirPathValue::Boolean(true)), // Single values exist
        }
    }

    /// Count items in collection
    pub fn count(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Integer(0)),
            FhirPathValue::Collection(items) => Ok(FhirPathValue::Integer(items.len() as i64)),
            _ => Ok(FhirPathValue::Integer(1)), // Single values have count 1
        }
    }
}
