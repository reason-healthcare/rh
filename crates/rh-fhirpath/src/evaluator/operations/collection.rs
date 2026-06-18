//! Collection operations for FHIRPath values

use crate::error::*;
use crate::evaluator::types::FhirPathValue;
use rust_decimal::Decimal;

/// Collection operations handler
pub struct CollectionEvaluator;

impl CollectionEvaluator {
    /// Evaluate union operation
    pub fn evaluate_union(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let mut result: Vec<FhirPathValue> = Vec::new();

        let push_dedup = |result: &mut Vec<FhirPathValue>, v: FhirPathValue| {
            if !result
                .iter()
                .any(|existing| FhirPathValue::equals_static(existing, &v))
            {
                result.push(v);
            }
        };

        // Add items from left operand
        match left {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                for item in items {
                    push_dedup(&mut result, item.clone());
                }
            }
            FhirPathValue::Empty => {}
            value => push_dedup(&mut result, value.clone()),
        }

        // Add items from right operand, deduplicating against result so far
        match right {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                for item in items {
                    push_dedup(&mut result, item.clone());
                }
            }
            FhirPathValue::Empty => {}
            value => push_dedup(&mut result, value.clone()),
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
            (
                FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items),
                FhirPathValue::Integer(idx),
            ) => {
                let idx = *idx as usize;
                if idx < items.len() {
                    Ok(items[idx].clone())
                } else {
                    Ok(FhirPathValue::Empty)
                }
            }
            (FhirPathValue::Empty, FhirPathValue::Integer(_)) => Ok(FhirPathValue::Empty),
            (value, FhirPathValue::Integer(idx)) => {
                if *idx == 0 {
                    Ok(value.clone())
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
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
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
            value => Ok(value.clone()),
        }
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

    /// Get single item from collection - fails if not exactly one item
    pub fn single(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.len() == 1 {
                    Ok(items[0].clone())
                } else {
                    Err(FhirPathError::InvalidOperation {
                        message: format!(
                            "single() requires exactly one item, found {}",
                            items.len()
                        ),
                    })
                }
            }
            value => Ok(value.clone()),
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
        let target_items = match target {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            value => vec![value.clone()],
        };

        let other_items = match other {
            FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            value => vec![value.clone()],
        };

        let mut intersection = Vec::new();
        for item in target_items {
            if other_items
                .iter()
                .any(|other_item| FhirPathValue::equals_static(&item, other_item))
                && !intersection
                    .iter()
                    .any(|existing| FhirPathValue::equals_static(existing, &item))
            {
                intersection.push(item);
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
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            value => vec![value.clone()],
        };

        let other_items = match other {
            FhirPathValue::Empty => return Ok(target.clone()),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
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

    /// Combine this collection with another, preserving duplicates
    /// Merges the input and other collections into a single collection without eliminating duplicate values.
    /// Combining an empty collection with a non-empty collection will return the non-empty collection.
    /// There is no expectation of order in the resulting collection.
    pub fn combine(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();

        match target {
            FhirPathValue::Empty => {}
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                result.extend(items.clone());
            }
            value => {
                result.push(value.clone());
            }
        }

        match other {
            FhirPathValue::Empty => {}
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                result.extend(items.clone());
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

    /// Union operation - merge collections and remove duplicates
    /// The union function combines the target collection with the parameter collection,
    /// removing duplicate items. This is like combine() but with deduplication.
    pub fn union(target: &FhirPathValue, other: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        let mut result = Vec::new();

        match target {
            FhirPathValue::Empty => {}
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                result.extend(items.clone());
            }
            value => {
                result.push(value.clone());
            }
        }

        match other {
            FhirPathValue::Empty => {}
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                result.extend(items.clone());
            }
            value => {
                result.push(value.clone());
            }
        }

        let mut unique_items = Vec::new();
        for item in result {
            if !unique_items
                .iter()
                .any(|existing| FhirPathValue::equals_static(existing, &item))
            {
                unique_items.push(item);
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

    /// Check if all items in collection are true (boolean true values only).
    /// Per spec, evaluation is undefined (returns empty) if any item is not boolean.
    pub fn all_true(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    for item in items {
                        match item {
                            FhirPathValue::Boolean(true) => continue,
                            FhirPathValue::Boolean(false) => {
                                return Ok(FhirPathValue::Boolean(false))
                            }
                            FhirPathValue::Empty => {}
                            _ => return Ok(FhirPathValue::Empty),
                        }
                    }
                    Ok(FhirPathValue::Boolean(true))
                }
            }
            value => match value {
                FhirPathValue::Boolean(true) => Ok(FhirPathValue::Boolean(true)),
                FhirPathValue::Boolean(false) => Ok(FhirPathValue::Boolean(false)),
                _ => Ok(FhirPathValue::Empty),
            },
        }
    }

    /// Check if any item in collection evaluates to true
    pub fn any_true(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Boolean(false))
                } else {
                    for item in items {
                        match item {
                            FhirPathValue::Boolean(true) => {
                                return Ok(FhirPathValue::Boolean(true))
                            }
                            _ => continue,
                        }
                    }
                    Ok(FhirPathValue::Boolean(false))
                }
            }
            value => match value {
                FhirPathValue::Boolean(true) => Ok(FhirPathValue::Boolean(true)),
                _ => Ok(FhirPathValue::Boolean(false)),
            },
        }
    }

    /// Check if all items in collection are false (boolean false values only)
    pub fn all_false(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(true)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Boolean(true))
                } else {
                    for item in items {
                        match item {
                            FhirPathValue::Boolean(false) => continue,
                            FhirPathValue::Boolean(true) => {
                                return Ok(FhirPathValue::Boolean(false))
                            }
                            _ => return Ok(FhirPathValue::Boolean(false)),
                        }
                    }
                    Ok(FhirPathValue::Boolean(true))
                }
            }
            value => match value {
                FhirPathValue::Boolean(false) => Ok(FhirPathValue::Boolean(true)),
                _ => Ok(FhirPathValue::Boolean(false)),
            },
        }
    }

    /// Check if any item in collection is false (boolean false values only)
    pub fn any_false(target: &FhirPathValue) -> FhirPathResult<FhirPathValue> {
        match target {
            FhirPathValue::Empty => Ok(FhirPathValue::Boolean(false)),
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                if items.is_empty() {
                    Ok(FhirPathValue::Boolean(false))
                } else {
                    for item in items {
                        match item {
                            FhirPathValue::Boolean(false) => {
                                return Ok(FhirPathValue::Boolean(true))
                            }
                            _ => continue,
                        }
                    }
                    Ok(FhirPathValue::Boolean(false))
                }
            }
            value => match value {
                FhirPathValue::Boolean(false) => Ok(FhirPathValue::Boolean(true)),
                _ => Ok(FhirPathValue::Boolean(false)),
            },
        }
    }

    /// Check if target collection is a subset of the other collection
    /// Returns true if all items in target are contained in other
    pub fn subset_of(
        target: &FhirPathValue,
        other: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let target_items = match target {
            FhirPathValue::Empty => vec![],
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            value => vec![value.clone()],
        };

        let other_items = match other {
            FhirPathValue::Empty => vec![],
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items) => {
                items.clone()
            }
            value => vec![value.clone()],
        };

        if target_items.is_empty() {
            return Ok(FhirPathValue::Boolean(true));
        }

        for target_item in &target_items {
            let mut found = false;
            for other_item in &other_items {
                if Self::values_equal(target_item, other_item) {
                    found = true;
                    break;
                }
            }
            if !found {
                return Ok(FhirPathValue::Boolean(false));
            }
        }

        Ok(FhirPathValue::Boolean(true))
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
                let mut children = Vec::new();

                if let Some(object_map) = obj.as_object() {
                    for value in object_map.values() {
                        match FhirPathValue::from_json(value) {
                            FhirPathValue::Collection(items)
                            | FhirPathValue::UnorderedCollection(items) => {
                                children.extend(items);
                            }
                            child => children.push(child),
                        }
                    }
                }

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
        // Per spec §6.9: the criterion must evaluate to a single Boolean (or
        // empty, which is treated as false). A non-Boolean singleton is a
        // semantic error; a multi-item collection is an execution error.
        let effective_criterion = match criterion {
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.len() == 1 =>
            {
                &items[0]
            }
            FhirPathValue::Collection(items) | FhirPathValue::UnorderedCollection(items)
                if items.is_empty() =>
            {
                &FhirPathValue::Empty
            }
            FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_) => {
                return Err(FhirPathError::EvaluationError {
                    message:
                        "iif() criterion must be a single Boolean; got a multi-item collection"
                            .to_string(),
                });
            }
            other => other,
        };

        let is_truthy = match effective_criterion {
            FhirPathValue::Boolean(b) => *b,
            FhirPathValue::TypedBoolean { value, .. } => *value,
            FhirPathValue::Empty => false,
            other => {
                return Err(FhirPathError::TypeError {
                    message: format!("iif() criterion must be Boolean, got {other:?}"),
                });
            }
        };

        if is_truthy {
            Ok(true_result.clone())
        } else {
            match otherwise_result {
                Some(otherwise) => Ok(otherwise.clone()),
                None => Ok(FhirPathValue::Empty),
            }
        }
    }

    /// Helper function to check if two FhirPathValues are equal
    fn values_equal(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        match (left, right) {
            (FhirPathValue::String(a), FhirPathValue::String(b)) => a == b,
            (FhirPathValue::Integer(a), FhirPathValue::Integer(b)) => a == b,
            (FhirPathValue::Number(a), FhirPathValue::Number(b)) => a == b,
            (FhirPathValue::Boolean(a), FhirPathValue::Boolean(b)) => a == b,
            (FhirPathValue::DateTime(a), FhirPathValue::DateTime(b)) => a == b,
            (FhirPathValue::Date(a), FhirPathValue::Date(b)) => a == b,
            (FhirPathValue::Time(a), FhirPathValue::Time(b)) => a == b,
            (
                FhirPathValue::Quantity {
                    value: a_val,
                    unit: a_unit,
                },
                FhirPathValue::Quantity {
                    value: b_val,
                    unit: b_unit,
                },
            ) => a_val == b_val && a_unit == b_unit,
            // Mixed numeric types
            (FhirPathValue::Integer(a), FhirPathValue::Number(b)) => Decimal::from(*a) == *b,
            (FhirPathValue::Number(a), FhirPathValue::Integer(b)) => *a == Decimal::from(*b),
            // Objects need deep comparison
            (FhirPathValue::Object(a), FhirPathValue::Object(b)) => a == b,
            (
                FhirPathValue::TypedObject { value: a, .. },
                FhirPathValue::TypedObject { value: b, .. },
            ) => a == b,
            (FhirPathValue::TypedObject { value: a, .. }, FhirPathValue::Object(b))
            | (FhirPathValue::Object(a), FhirPathValue::TypedObject { value: b, .. }) => a == b,
            // Collections need element-wise comparison (order doesn't matter for sets)
            (FhirPathValue::Collection(a), FhirPathValue::Collection(b))
            | (FhirPathValue::Collection(a), FhirPathValue::UnorderedCollection(b))
            | (FhirPathValue::UnorderedCollection(a), FhirPathValue::Collection(b))
            | (FhirPathValue::UnorderedCollection(a), FhirPathValue::UnorderedCollection(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for item_a in a {
                    if !b.iter().any(|item_b| Self::values_equal(item_a, item_b)) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}
