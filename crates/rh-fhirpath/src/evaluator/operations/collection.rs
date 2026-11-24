//! Collection operations for FHIRPath values

use crate::error::*;
use crate::evaluator::operations::comparison::ComparisonEvaluator;
use crate::evaluator::types::FhirPathValue;
use crate::json::JsonNumbers;
use crate::FhirModel;
use serde_json::Value;
use std::cmp::Ordering;

/// Collection operations handler
pub struct CollectionEvaluator;

impl CollectionEvaluator {
    /// Evaluate union operation
    pub fn evaluate_union(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();
        for item in left.items().iter().chain(right.items()) {
            if !result
                .iter()
                .any(|existing| FhirPathValue::equals_static(existing, item))
            {
                result.push(item.clone());
            }
        }
        Ok(FhirPathValue::from_items(result))
    }

    /// Evaluate indexer operation
    pub fn evaluate_indexer(
        target: &FhirPathValue,
        index: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let Some(index) = index.singleton("Indexer")? else {
            return Ok(FhirPathValue::Empty);
        };
        let FhirPathValue::Integer(index) = index.system_value() else {
            return Err(FhirPathError::TypeError {
                message: "Indexer requires an Integer index".into(),
            });
        };
        Ok(usize::try_from(*index)
            .ok()
            .and_then(|index| target.items().get(index))
            .cloned()
            .unwrap_or(FhirPathValue::Empty))
    }

    /// Remove duplicates from a collection
    pub fn distinct(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::evaluate_union(target, &FhirPathValue::Empty)
    }

    /// Check if all items in a collection are distinct
    pub fn is_distinct(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            _ => Ok(FhirPathValue::Boolean(true)),
        }
    }

    /// Check if collection is empty
    pub fn is_empty(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Ok(FhirPathValue::Boolean(items.is_empty()))
            }
            _ => Ok(FhirPathValue::Boolean(false)),
        }
    }

    /// Check if collection exists (has any items)
    pub fn exists(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Ok(FhirPathValue::Boolean(!items.is_empty()))
            }
            _ => Ok(FhirPathValue::Boolean(true)),
        }
    }

    /// Count items in collection
    pub fn count(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Integer(0)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                Ok(FhirPathValue::Integer(items.len() as i64))
            }
            _ => Ok(FhirPathValue::Integer(1)),
        }
    }

    /// Return the sole item or empty, and signal an error for multiple items.
    pub fn single(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target.items() {
            [] => Ok(FhirPathValue::Empty),
            [item] => Ok(item.clone()),
            items => Err(FhirPathError::EvaluationError {
                message: format!("single() requires exactly one item, found {}", items.len()),
            }),
        }
    }

    /// Get first item from collection
    pub fn first(target: &FhirPathValue, check_ordered: bool) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::UnorderedCollection(_) if check_ordered => {
                Err(FhirPathError::EvaluationError {
                    message: "Cannot apply order-dependent function to unordered collection"
                        .to_string(),
                })
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(items[0].clone())
                }
            }
            value => Ok(value.clone()),
        }
    }

    /// Get last item from collection
    pub fn last(target: &FhirPathValue, check_ordered: bool) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::UnorderedCollection(_) if check_ordered => {
                Err(FhirPathError::EvaluationError {
                    message: "Cannot apply order-dependent function to unordered collection"
                        .to_string(),
                })
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(items[items.len() - 1].clone())
                }
            }
            value => Ok(value.clone()),
        }
    }

    /// Get all items except the first
    pub fn tail(target: &FhirPathValue, check_ordered: bool) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::UnorderedCollection(_) if check_ordered => {
                Err(FhirPathError::EvaluationError {
                    message: "Cannot apply order-dependent function to unordered collection"
                        .to_string(),
                })
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
            _ => Ok(FhirPathValue::Empty),
        }
    }

    pub(crate) fn subsetting_count(
        parameters: &[FhirPathValue],
        operation: &str,
    ) -> FhirPathResult<Option<i64>> {
        let [count] = parameters else {
            return Err(FhirPathError::InvalidOperation {
                message: format!("{operation} requires exactly one parameter (count)"),
            });
        };
        match count.singleton(operation)?.map(FhirPathValue::system_value) {
            Some(FhirPathValue::Integer(count)) => Ok(Some(*count)),
            Some(FhirPathValue::Empty) | None => Ok(None),
            Some(_) => Err(FhirPathError::TypeError {
                message: format!("{operation} count parameter must be an Integer"),
            }),
        }
    }

    /// Skip the first n items from collection
    pub fn skip(
        target: &FhirPathValue,
        count: i64,
        check_ordered: bool,
    ) -> FhirPathResult<FhirPathValue> {
        if count < 0 {
            return Ok(target.clone());
        }

        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::UnorderedCollection(_) if check_ordered => {
                Err(FhirPathError::EvaluationError {
                    message: "Cannot apply order-dependent function to unordered collection"
                        .to_string(),
                })
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
            return Ok(FhirPathValue::Empty);
        }

        if count == 0 {
            return Ok(FhirPathValue::Empty);
        }

        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
    pub fn intersect(
        target: &FhirPathValue,
        other: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();
        for item in target.items() {
            if other
                .items()
                .iter()
                .any(|other| FhirPathValue::equals_static(item, other))
                && !result
                    .iter()
                    .any(|existing| FhirPathValue::equals_static(existing, item))
            {
                result.push(item.clone());
            }
        }
        Ok(FhirPathValue::from_items(result))
    }

    /// Exclude items in other collection from target collection
    pub fn exclude(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let result = target
            .items()
            .iter()
            .filter(|item| {
                !other
                    .items()
                    .iter()
                    .any(|other| FhirPathValue::equals_static(item, other))
            })
            .cloned()
            .collect();
        Ok(FhirPathValue::from_items(result))
    }

    /// Combine this collection with another, preserving duplicates
    /// Merges the input and other collections into a single collection without eliminating duplicate values.
    /// Combining an empty collection with a non-empty collection will return the non-empty collection.
    /// There is no expectation of order in the resulting collection.
    pub fn combine(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let result = target
            .items()
            .iter()
            .chain(other.items())
            .cloned()
            .collect();
        Ok(FhirPathValue::from_items(result))
    }

    /// Union operation - merge collections and remove duplicates
    /// The union function combines the target collection with the parameter collection,
    /// removing duplicate items. This is like combine() but with deduplication.
    pub fn union(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::evaluate_union(target, other)
    }

    pub(crate) fn compare_sort_keys(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<Ordering> {
        fn key_is_empty(value: &FhirPathValue) -> bool {
            match value {
                FhirPathValue::Node(node) => !node.has_raw_value(),
                value => value.system_value().is_empty(),
            }
        }
        match (key_is_empty(left), key_is_empty(right)) {
            (true, true) => Ok(Ordering::Equal),
            (true, false) => Ok(Ordering::Less),
            (false, true) => Ok(Ordering::Greater),
            (false, false) if ComparisonEvaluator::equal(left, right) == Some(true) => {
                Ok(Ordering::Equal)
            }
            (false, false) => ComparisonEvaluator::compare_order(left, right)?
                .filter(|order| *order != Ordering::Equal)
                .ok_or_else(|| FhirPathError::EvaluationError {
                    message: format!("sort() keys have no defined ordering: {left:?}, {right:?}"),
                }),
        }
    }

    // Merging indices allows stable sorting with a fallible comparator and a
    // single scratch allocation.
    pub(crate) fn sort_indices_by(
        indices: &mut [usize],
        scratch: &mut [usize],
        compare: &mut impl FnMut(usize, usize) -> FhirPathResult<Ordering>,
    ) -> FhirPathResult<()> {
        if indices.len() <= 1 {
            return Ok(());
        }
        let middle = indices.len() / 2;
        let (left, right) = indices.split_at_mut(middle);
        let (left_scratch, right_scratch) = scratch.split_at_mut(middle);
        Self::sort_indices_by(left, left_scratch, compare)?;
        Self::sort_indices_by(right, right_scratch, compare)?;

        let mut i = 0;
        let mut j = 0;
        for index in scratch.iter_mut() {
            if i < left.len()
                && (j == right.len() || compare(left[i], right[j])? != Ordering::Greater)
            {
                *index = left[i];
                i += 1;
            } else {
                *index = right[j];
                j += 1;
            }
        }
        indices.copy_from_slice(scratch);
        Ok(())
    }

    /// Check if all items in collection evaluate to true
    pub fn all(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    for item in items {
                        if !item.to_boolean() {
                            return Ok(FhirPathValue::Boolean(false));
                        }
                    }
                    Ok(FhirPathValue::Boolean(true))
                }
            }
            value => Ok(FhirPathValue::Boolean(value.to_boolean())),
        }
    }

    pub fn all_true(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::test_booleans(target, "allTrue()", true, true)
    }

    pub fn any_true(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::test_booleans(target, "anyTrue()", true, false)
    }

    pub fn all_false(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::test_booleans(target, "allFalse()", false, true)
    }

    pub fn any_false(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        Self::test_booleans(target, "anyFalse()", false, false)
    }

    fn test_booleans(
        target: &FhirPathValue,
        operation: &str,
        expected: bool,
        require_all: bool,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result = require_all;
        for item in target.items() {
            let value = match item.system_value() {
                FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => *value,
                other => {
                    return Err(FhirPathError::TypeError {
                        message: format!("{operation} requires Boolean items, got {other:?}"),
                    })
                }
            };
            if (value == expected) != require_all {
                result = !require_all;
            }
        }
        Ok(FhirPathValue::Boolean(result))
    }

    /// Check if target collection is a subset of the other collection
    /// Returns true if all items in target are contained in other
    pub fn subset_of(
        target: &FhirPathValue,
        other: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(FhirPathValue::Boolean(target.items().iter().all(|item| {
            other
                .items()
                .iter()
                .any(|other| FhirPathValue::equals_static(item, other))
        })))
    }

    /// Check if target collection is a superset of the other collection
    /// Returns true if target contains all items in other
    pub fn superset_of(
        target: &FhirPathValue,
        other: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        // A superset of B means B is a subset of A
        Self::subset_of(other, target)
    }

    /// Returns a collection with all immediate child nodes of all items in the input collection
    /// The ordering of children is undefined and may vary between platforms
    pub fn children(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Object(obj) | FhirPathValue::TypedObject { value: obj, .. } => {
                let children = Self::json_children(obj, None, FhirModel::default());
                if children.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::UnorderedCollection(children))
                }
            }
            FhirPathValue::FhirPrimitive { inner, .. } => Self::children(inner),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                let mut all_children = Vec::new();

                for item in items {
                    let item_children = Self::children(item)?;
                    match item_children {
                        FhirPathValue::Collection(mut children)
                        | FhirPathValue::UnorderedCollection(mut children) => {
                            all_children.append(&mut children);
                        }
                        FhirPathValue::Empty => {}
                        value => {
                            all_children.push(value);
                        }
                    }
                }

                if all_children.is_empty() {
                    Ok(FhirPathValue::Empty)
                } else {
                    Ok(FhirPathValue::UnorderedCollection(all_children))
                }
            }
            _ => Ok(FhirPathValue::Empty),
        }
    }

    pub(crate) fn json_children(
        object: &Value,
        numbers: Option<&JsonNumbers>,
        model: FhirModel,
    ) -> Vec<FhirPathValue> {
        let mut children = Vec::new();
        if let Some(fields) = object.as_object() {
            for (key, value) in fields {
                let numbers = numbers.and_then(|numbers| numbers.field(key));
                match FhirPathValue::from_json_with_numbers(value, numbers, model) {
                    FhirPathValue::Collection(items)
                    | FhirPathValue::UnorderedCollection(items) => {
                        children.extend(items);
                    }
                    child => children.push(child),
                }
            }
        }
        children
    }

    /// Returns a collection with all descendant nodes (all children, their children, etc.) of all items in the input collection
    /// This is a recursive operation that collects all nodes at any depth
    /// The ordering of descendants is undefined and may vary between platforms
    pub fn descendants(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let mut all_descendants = Vec::new();
        Self::collect_descendants(target, &mut all_descendants)?;

        if all_descendants.is_empty() {
            Ok(FhirPathValue::Empty)
        } else {
            Ok(FhirPathValue::Collection(all_descendants))
        }
    }

    /// Helper function to recursively collect all descendants
    fn collect_descendants(
        target: &FhirPathValue,
        descendants: &mut Vec<FhirPathValue>,
    ) -> FhirPathResult<()> {
        match target {
            FhirPathValue::Object(obj) | FhirPathValue::TypedObject { value: obj, .. } => {
                if let Some(object_map) = obj.as_object() {
                    for value in object_map.values() {
                        let child = FhirPathValue::from_json(value);
                        match &child {
                            FhirPathValue::Collection(items)
                            | FhirPathValue::UnorderedCollection(items) => {
                                for item in items {
                                    descendants.push(item.clone());
                                    Self::collect_descendants(item, descendants)?;
                                }
                            }
                            _ => {
                                descendants.push(child.clone());
                                Self::collect_descendants(&child, descendants)?;
                            }
                        }
                    }
                }
            }
            FhirPathValue::FhirPrimitive { inner, .. } => {
                Self::collect_descendants(inner, descendants)?;
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                for item in items {
                    Self::collect_descendants(item, descendants)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Immediate if function - returns true_result if criterion is truthy, otherwise otherwise_result
    /// This is similar to the conditional operator in C-like languages (? :)
    ///
    /// # Arguments
    /// * `criterion` - The condition to evaluate for truthiness
    /// * `true_result` - Value to return if criterion is truthy
    /// * `otherwise_result` - Optional value to return if criterion is falsy (defaults to empty)
    pub fn iif(
        criterion: &FhirPathValue,
        true_result: &FhirPathValue,
        otherwise_result: Option<&FhirPathValue>,
    ) -> FhirPathResult<FhirPathValue> {
        if criterion.singleton_boolean("iif() criterion")? == Some(true) {
            Ok(true_result.clone())
        } else {
            Ok(otherwise_result.cloned().unwrap_or(FhirPathValue::Empty))
        }
    }
}
