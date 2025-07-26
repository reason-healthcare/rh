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
            (FhirPathValue::Empty, FhirPathValue::Integer(_)) => {
                // Indexing an empty collection always returns empty
                Ok(FhirPathValue::Empty)
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

    /// Get single item from collection - fails if not exactly one item
    pub fn single(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Err(FhirPathError::InvalidOperation {
                message: "single() cannot be called on empty collection".to_string(),
            }),
            FhirPathValue::Collection(items) => {
                if items.len() == 1 {
                    Ok(items[0].clone())
                } else {
                    Err(FhirPathError::InvalidOperation {
                        message: format!("single() requires exactly one item, found {}", items.len()),
                    })
                }
            }
            value => Ok(value.clone()), // Single values return themselves
        }
    }

    /// Get first item from collection
    pub fn first(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(items[0].clone())
                }
            }
            value => Ok(value.clone()), // Single values return themselves
        }
    }

    /// Get last item from collection
    pub fn last(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(items[items.len() - 1].clone())
                }
            }
            value => Ok(value.clone()), // Single values return themselves
        }
    }

    /// Get all items except the first
    pub fn tail(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => {
                if items.len() <= 1 {
                    Ok(FhirPathValue::Empty)
                } else {
                    let tail_items = items[1..].to_vec();
                    if tail_items.len() == 1 {
                        Ok(tail_items.into_iter().next().unwrap())
                    } else {
                        Ok(FhirPathValue::Collection(tail_items))
                    }
                }
            }
            _ => Ok(FhirPathValue::Empty), // Single values have no tail
        }
    }

    /// Skip the first n items from collection
    pub fn skip(target: &FhirPathValue, count: i64) -> FhirPathResult<FhirPathValue> {
        if count < 0 {
            return Err(FhirPathError::InvalidOperation {
                message: "skip() count cannot be negative".to_string(),
            });
        }

        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => {
                let skip_count = count as usize;
                if skip_count >= items.len() {
                    Ok(FhirPathValue::Empty)
                } else {
                    let remaining_items = items[skip_count..].to_vec();
                    if remaining_items.is_empty() {
                        Ok(FhirPathValue::Empty)
                    } else if remaining_items.len() == 1 {
                        Ok(remaining_items.into_iter().next().unwrap())
                    } else {
                        Ok(FhirPathValue::Collection(remaining_items))
                    }
                }
            }
            value => {
                if count == 0 {
                    Ok(value.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Take the first n items from collection
    pub fn take(target: &FhirPathValue, count: i64) -> FhirPathResult<FhirPathValue> {
        if count < 0 {
            return Err(FhirPathError::InvalidOperation {
                message: "take() count cannot be negative".to_string(),
            });
        }

        if count == 0 {
            return Ok(FhirPathValue::Empty);
        }

        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => {
                let take_count = count as usize;
                let taken_items = if take_count >= items.len() {
                    items.clone()
                } else {
                    items[..take_count].to_vec()
                };

                if taken_items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else if taken_items.len() == 1 {
                    Ok(taken_items.into_iter().next().unwrap())
                } else {
                    Ok(FhirPathValue::Collection(taken_items))
                }
            }
            value => {
                if count >= 1 {
                    Ok(value.clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
        }
    }

    /// Get intersection of two collections
    pub fn intersect(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let target_items = match target {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => items.clone(),
            value => vec![value.clone()],
        };

        let other_items = match other {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => items.clone(),
            value => vec![value.clone()],
        };

        let mut intersection = Vec::new();
        for item in target_items {
            if other_items
                .iter()
                .any(|other_item| FhirPathValue::equals_static(&item, other_item))
            {
                // Only add if not already in intersection (to maintain distinctness)
                if !intersection
                    .iter()
                    .any(|existing| FhirPathValue::equals_static(existing, &item))
                {
                    intersection.push(item);
                }
            }
        }

        if intersection.is_empty() {
            Ok(FhirPathValue::Empty)
        } else if intersection.len() == 1 {
            Ok(intersection.into_iter().next().unwrap())
        } else {
            Ok(FhirPathValue::Collection(intersection))
        }
    }

    /// Exclude items in other collection from target collection
    pub fn exclude(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let target_items = match target {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) => items.clone(),
            value => vec![value.clone()],
        };

        let other_items = match other {
            FhirPathValue::Empty => return Ok(target.clone()),
            FhirPathValue::Collection(items) => items.clone(),
            value => vec![value.clone()],
        };

        let mut result = Vec::new();
        for item in target_items {
            if !other_items
                .iter()
                .any(|other_item| FhirPathValue::equals_static(&item, other_item))
            {
                result.push(item);
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
}
