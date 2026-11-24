//! Comparison operations shared by operators, sorting, and collection functions.

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{BuildHasher, Hash, Hasher, RandomState};

use crate::ast::*;
use crate::error::*;
use crate::evaluator::metadata::{
    coding_equivalent, compare_model_quantities, extreme_numeric_equivalent, FhirModel, FhirNode,
};
use crate::evaluator::operations::{
    temporal::{
        compare_parts, equals_parts, equivalent_parts, temporal_value, TemporalKind, TemporalParts,
    },
    units::UnitConverter,
};
use crate::evaluator::types::FhirPathValue;
use rust_decimal::Decimal;
use serde_json::Value;

pub struct ComparisonEvaluator;

impl ComparisonEvaluator {
    pub fn evaluate_equality(
        left: &FhirPathValue,
        operator: &EqualityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match operator {
            EqualityOperator::Equal => Self::equal(left, right),
            EqualityOperator::NotEqual => Self::equal(left, right).map(|equal| !equal),
            EqualityOperator::Equivalent => Some(Self::values_equivalent(left, right)),
            EqualityOperator::NotEquivalent => Some(!Self::values_equivalent(left, right)),
        };
        Ok(boolean_value(result))
    }

    /// Compare ordered collections. A definite mismatch resolves an earlier unknown.
    pub(crate) fn equal(left: &FhirPathValue, right: &FhirPathValue) -> Option<bool> {
        let (left, right) = (left.items(), right.items());
        if left.is_empty() || right.is_empty() {
            return None;
        }
        if left.len() != right.len() {
            return Some(false);
        }

        let mut result = Some(true);
        for (left, right) in left.iter().zip(right) {
            match equal_item(left, right) {
                Some(false) => return Some(false),
                None => result = None,
                Some(true) => {}
            }
        }
        result
    }

    pub(crate) fn equality_fingerprints(
        value: &FhirPathValue,
        hash: &RandomState,
        model: FhirModel,
    ) -> Option<(u64, Option<u64>)> {
        use FhirPathValue::*;
        let [item] = value.items() else { return None };
        let system = item.system_value();
        let logical = match item {
            Node(node) if node.model.is_subtype(&node.fhir_type, "Quantity") => return None,
            Node(node) if node.raw.is_object() => {
                if node.model != model
                    || node.is_primitive()
                    || !matches!(system, Object(_) | TypedObject { .. })
                {
                    return None;
                }
                Some(logical_object_fingerprint(node, hash))
            }
            _ => None,
        };
        let system = match system {
            String(text) | TypedString { value: text, .. } => hash.hash_one((0, text)),
            Boolean(value) | TypedBoolean { value, .. } => hash.hash_one((1, value)),
            Integer(_) | Long(_) | Number(_) => hash.hash_one((2, decimal(system))),
            Object(value) | TypedObject { value, .. } => {
                let mut state = hash.build_hasher();
                3.hash(&mut state);
                hash_json(value, &mut state, 1);
                state.finish()
            }
            _ => return None,
        };
        Some((system, logical))
    }

    pub fn evaluate_inequality(
        left: &FhirPathValue,
        operator: &InequalityOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        Ok(boolean_value(Self::compare_order(left, right)?.map(
            |ord| match operator {
                InequalityOperator::LessThan => ord == Ordering::Less,
                InequalityOperator::LessThanOrEqual => ord != Ordering::Greater,
                InequalityOperator::GreaterThan => ord == Ordering::Greater,
                InequalityOperator::GreaterThanOrEqual => ord != Ordering::Less,
            },
        )))
    }

    /// Compare singleton values, retaining unknown precision or quantity ordering.
    pub(crate) fn compare_order(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<Option<Ordering>> {
        let left = left.singleton("Comparison")?;
        let right = right.singleton("Comparison")?;
        let (Some(left), Some(right)) = (left, right) else {
            return Ok(None);
        };
        if let Some(ordering) = compare_model_quantities(left, right) {
            return Ok(ordering);
        }
        let (left, right) = (left.system_value(), right.system_value());
        if left.items().is_empty() || right.items().is_empty() {
            return Ok(None);
        }
        if let Some((a, b)) = temporal_pair(left, right) {
            return Ok(compare_parts(&a, &b));
        }
        if let Some((a, au, b, bu)) = quantity_pair(left, right) {
            return Ok(UnitConverter::new()
                .compare_quantities(a, au, b, bu)
                .ok()
                .map(|value| value.cmp(&0)));
        }
        if let (Some(a), Some(b)) = (decimal(left), decimal(right)) {
            return Ok(Some(a.cmp(&b)));
        }
        if let (Some(a), Some(b)) = (left.as_str(), right.as_str()) {
            return Ok(Some(a.cmp(b)));
        }
        if let (Some(a), Some(b)) = (boolean(left), boolean(right)) {
            return Ok(Some(a.cmp(&b)));
        }
        Err(FhirPathError::TypeError {
            message: format!("Cannot compare {left:?} and {right:?}"),
        })
    }

    pub fn evaluate_membership(
        left: &FhirPathValue,
        operator: &MembershipOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let (item, collection, operation) = match operator {
            MembershipOperator::In => (left, right, "Left operand of 'in'"),
            MembershipOperator::Contains => (right, left, "Right operand of 'contains'"),
        };
        let Some(item) = item.singleton(operation)? else {
            return Ok(FhirPathValue::Empty);
        };
        Ok(FhirPathValue::Boolean(item.is_member_of(collection)?))
    }

    pub(crate) fn quantities_comparable(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> Option<bool> {
        let ([left], [right]) = (left.items(), right.items()) else {
            return None;
        };
        if let Some(ordering) = compare_model_quantities(left, right) {
            return Some(ordering.is_some());
        }
        let (left, right) = (left.system_value(), right.system_value());
        if let Some((_, left_unit, _, right_unit)) = quantity_pair(left, right) {
            return Some(UnitConverter::new().are_quantities_comparable(left_unit, right_unit));
        }
        decimal(left).zip(decimal(right)).map(|_| true)
    }

    pub fn evaluate_and(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match (
            left.singleton_boolean("Operand of 'and'")?,
            right.singleton_boolean("Operand of 'and'")?,
        ) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        };
        Ok(boolean_value(result))
    }

    pub fn evaluate_implies(
        left: &FhirPathValue,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let result = match (
            left.singleton_boolean("Operand of 'implies'")?,
            right.singleton_boolean("Operand of 'implies'")?,
        ) {
            (Some(false), _) | (_, Some(true)) => Some(true),
            (Some(true), right) => right,
            (None, _) => None,
        };
        Ok(boolean_value(result))
    }

    pub fn evaluate_or(
        left: &FhirPathValue,
        operator: &OrOperator,
        right: &FhirPathValue,
    ) -> FhirPathResult<FhirPathValue> {
        let left = left.singleton_boolean("Boolean operand")?;
        let right = right.singleton_boolean("Boolean operand")?;
        let result = match operator {
            OrOperator::Or => match (left, right) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            },
            OrOperator::Xor => left.zip(right).map(|(a, b)| a ^ b),
        };
        Ok(boolean_value(result))
    }

    /// Compare collections as multisets under FHIRPath equivalence.
    pub fn values_equivalent(left: &FhirPathValue, right: &FhirPathValue) -> bool {
        let (left, right) = (left.items(), right.items());
        if left.len() != right.len() {
            return false;
        }
        if left.len() <= 1 {
            return left.iter().zip(right).all(|(a, b)| equivalent_item(a, b));
        }

        // Decimal equivalence can connect one item to several candidates.
        // Match whole collections without relying on the choice of first match.
        let mut matches = vec![None; right.len()];
        for i in 0..left.len() {
            let mut visited = vec![false; right.len()];
            if !match_equivalent(i, left, right, &mut matches, &mut visited) {
                return false;
            }
        }
        true
    }
}

fn boolean_value(value: Option<bool>) -> FhirPathValue {
    value.map_or(FhirPathValue::Empty, FhirPathValue::Boolean)
}

fn boolean(value: &FhirPathValue) -> Option<bool> {
    match value {
        FhirPathValue::Boolean(value) | FhirPathValue::TypedBoolean { value, .. } => Some(*value),
        _ => None,
    }
}

fn decimal(value: &FhirPathValue) -> Option<Decimal> {
    match value {
        FhirPathValue::Integer(value) | FhirPathValue::Long(value) => Some(Decimal::from(*value)),
        FhirPathValue::Number(value) => Some(*value),
        _ => None,
    }
}

fn quantity_pair<'a>(
    left: &'a FhirPathValue,
    right: &'a FhirPathValue,
) -> Option<(Decimal, &'a Option<String>, Decimal, &'a Option<String>)> {
    use FhirPathValue::Quantity;
    match (left, right) {
        (Quantity { value: a, unit: au }, Quantity { value: b, unit: bu }) => {
            Some((*a, au, *b, bu))
        }
        (Quantity { value: a, unit: au }, b) => Some((*a, au, decimal(b)?, &None)),
        (a, Quantity { value: b, unit: bu }) => Some((decimal(a)?, &None, *b, bu)),
        _ => None,
    }
}

fn temporal_pair(
    left: &FhirPathValue,
    right: &FhirPathValue,
) -> Option<(TemporalParts, TemporalParts)> {
    let (_, left_kind, left) = temporal_value(left)?;
    let (_, right_kind, right) = temporal_value(right)?;
    use TemporalKind::{Date, DateTime, Time};
    match (left_kind, right_kind) {
        (Time, Time) | (Date | DateTime, Date | DateTime) => Some((left, right)),
        _ => None,
    }
}

fn equal_item(left: &FhirPathValue, right: &FhirPathValue) -> Option<bool> {
    use FhirPathValue::*;
    if let Some(ordering) = compare_model_quantities(left, right) {
        return ordering.map(|value| value == Ordering::Equal);
    }
    let logical_objects = logical_object_pair(left, right);
    let (left, right) = (left.system_value(), right.system_value());
    if matches!(left, Empty) || matches!(right, Empty) {
        return None;
    }
    if let Some((a, b)) = temporal_pair(left, right) {
        return equals_parts(&a, &b);
    }
    if let Some((a, au, b, bu)) = quantity_pair(left, right) {
        return UnitConverter::new()
            .compare_quantities(a, au, b, bu)
            .ok()
            .map(|ord| ord == 0);
    }
    if let (Some(a), Some(b)) = (decimal(left), decimal(right)) {
        return Some(a == b);
    }
    if let (Some(a), Some(b)) = (left.as_str(), right.as_str()) {
        return Some(a == b);
    }
    if let (Some(a), Some(b)) = (boolean(left), boolean(right)) {
        return Some(a == b);
    }
    if let Some((left, right)) = logical_objects {
        return compare_logical_objects(left, right, ComparisonEvaluator::equal);
    }
    match (left, right) {
        (Object(a) | TypedObject { value: a, .. }, Object(b) | TypedObject { value: b, .. }) => {
            if let (TypedObject { fhir_type: a, .. }, TypedObject { fhir_type: b, .. }) =
                (left, right)
            {
                if a != b {
                    return Some(false);
                }
            }
            Some(json_equal(a, b))
        }
        (DateTimePrecision(a), DateTimePrecision(b)) => Some(a == b),
        _ => Some(false),
    }
}

fn json_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|b| json_equal(v, b)))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| json_equal(a, b))
        }
        (Value::Number(_), Value::Number(_)) => {
            let a = FhirPathValue::from_json(left);
            let b = FhirPathValue::from_json(right);
            decimal(&a) == decimal(&b)
        }
        _ => left == right,
    }
}

fn hash_json(value: &Value, state: &mut impl Hasher, depth: usize) {
    std::mem::discriminant(value).hash(state);
    match value {
        Value::Object(fields) if depth > 0 => {
            let mut fields: Vec<_> = fields.iter().collect();
            fields.sort_unstable_by_key(|(key, _)| *key);
            fields.len().hash(state);
            for (key, value) in fields {
                key.hash(state);
                hash_json(value, state, depth - 1);
            }
        }
        Value::Array(items) if depth > 0 => {
            items.len().hash(state);
            for item in items {
                hash_json(item, state, depth - 1);
            }
        }
        Value::Number(_) => decimal(&FhirPathValue::from_json(value)).hash(state),
        Value::String(value) => value.hash(state),
        Value::Bool(value) => value.hash(state),
        _ => {}
    }
}

fn logical_object_fingerprint(node: &FhirNode, hash: &RandomState) -> u64 {
    fn can_have_string_or_boolean(value: &Value) -> bool {
        let is_string_or_boolean = |v: &Value| v.is_string() || v.is_boolean();
        is_string_or_boolean(value)
            || value
                .as_array()
                .is_some_and(|items| items.iter().any(is_string_or_boolean))
    }

    let mut state = hash.build_hasher();
    4.hash(&mut state);
    node.fhir_type.hash(&mut state);
    let names: BTreeSet<_> = node
        .raw
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, value)| can_have_string_or_boolean(value))
        .filter_map(|(key, _)| logical_child_name(node, key))
        .filter(|name| logical_child_name(node, name) == Some(name))
        .collect();
    for name in names {
        let Ok(value) = node.member(name, false) else {
            continue;
        };
        if let [item] = value.items() {
            let item = item.system_value();
            if let Some(text) = item.as_str() {
                (name, 0, text).hash(&mut state);
            } else if let Some(value) = boolean(item) {
                (name, 1, value).hash(&mut state);
            }
        }
    }
    state.finish()
}

fn equivalent_item(left: &FhirPathValue, right: &FhirPathValue) -> bool {
    if let Some(equivalent) = coding_equivalent(left, right) {
        return equivalent;
    }
    if let Some(equivalent) = extreme_numeric_equivalent(left, right) {
        return equivalent;
    }
    if let Some(ordering) = compare_model_quantities(left, right) {
        return ordering == Some(Ordering::Equal);
    }
    let logical_objects = logical_object_pair(left, right);
    let (left, right) = (left.system_value(), right.system_value());
    if let (Some(a), Some(b)) = (left.as_str(), right.as_str()) {
        return string_equivalent(a, b);
    }
    if let (Some(a), Some(b)) = (decimal(left), decimal(right)) {
        let precision = a.normalize().scale().min(b.normalize().scale());
        return a.round_dp(precision) == b.round_dp(precision);
    }
    if let Some((a, au, b, bu)) = quantity_pair(left, right) {
        return UnitConverter::new().equivalent_quantities(a, au, b, bu);
    }
    if let Some((a, b)) = temporal_pair(left, right) {
        return equivalent_parts(&a, &b);
    }
    if let Some((left, right)) = logical_objects {
        return compare_logical_objects(left, right, |left, right| {
            Some(ComparisonEvaluator::values_equivalent(left, right))
        }) == Some(true);
    }
    use FhirPathValue::{Object, TypedObject};
    match (left, right) {
        (Object(a) | TypedObject { value: a, .. }, Object(b) | TypedObject { value: b, .. }) => {
            if let (TypedObject { fhir_type: a, .. }, TypedObject { fhir_type: b, .. }) =
                (left, right)
            {
                if a != b {
                    return false;
                }
            }
            json_equivalent(a, b)
        }
        _ => equal_item(left, right) == Some(true),
    }
}

fn logical_object_pair<'a>(
    left: &'a FhirPathValue,
    right: &'a FhirPathValue,
) -> Option<(&'a FhirNode, &'a FhirNode)> {
    match (left, right) {
        (FhirPathValue::Node(a), FhirPathValue::Node(b))
            if a.raw.is_object() && b.raw.is_object() =>
        {
            Some((a, b))
        }
        _ => None,
    }
}

fn compare_logical_objects(
    left: &FhirNode,
    right: &FhirNode,
    compare: impl Fn(&FhirPathValue, &FhirPathValue) -> Option<bool>,
) -> Option<bool> {
    if left.fhir_type != right.fhir_type {
        return Some(false);
    }

    let names: BTreeSet<_> = [left, right]
        .into_iter()
        .flat_map(|node| {
            node.raw.as_object().into_iter().flat_map(|object| {
                object
                    .keys()
                    .filter_map(|key| logical_child_name(node, key))
            })
        })
        .collect();
    let mut result = Some(true);
    for name in names {
        let a = left.member(name, false).ok()?;
        let b = right.member(name, false).ok()?;
        match (a.is_empty(), b.is_empty()) {
            (true, true) => continue,
            (true, false) | (false, true) => return Some(false),
            _ => {}
        }
        match compare(&a, &b) {
            Some(false) => return Some(false),
            None => result = None,
            Some(true) => {}
        }
    }
    result
}

fn logical_child_name<'a>(node: &FhirNode, key: &'a str) -> Option<&'a str> {
    let key = key.strip_prefix('_').unwrap_or(key);
    if key == "resourceType"
        || node.raw["resourceType"]
            .as_str()
            .is_some_and(|resource| node.model.is_subtype(resource, key))
    {
        return None;
    }
    Some(
        node.model
            .field(&node.fhir_type, key)
            .map_or(key, |(name, _)| name),
    )
}

fn json_equivalent(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|b| json_equivalent(v, b)))
        }
        (Value::Array(_), Value::Array(_)) => ComparisonEvaluator::values_equivalent(
            &FhirPathValue::from_json(left),
            &FhirPathValue::from_json(right),
        ),
        (Value::String(a), Value::String(b)) => string_equivalent(a, b),
        (Value::Number(_), Value::Number(_)) => equivalent_item(
            &FhirPathValue::from_json(left),
            &FhirPathValue::from_json(right),
        ),
        _ => left == right,
    }
}

fn string_equivalent(left: &str, right: &str) -> bool {
    fn normalize_whitespace(value: &str) -> String {
        value
            .chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }
    unicase::eq(&normalize_whitespace(left), &normalize_whitespace(right))
}

fn match_equivalent(
    index: usize,
    left: &[FhirPathValue],
    right: &[FhirPathValue],
    matches: &mut [Option<usize>],
    visited: &mut [bool],
) -> bool {
    for j in 0..right.len() {
        if matches[j].is_none() && equivalent_item(&left[index], &right[j]) {
            matches[j] = Some(index);
            return true;
        }
    }
    for j in 0..right.len() {
        if visited[j] || !equivalent_item(&left[index], &right[j]) {
            continue;
        }
        visited[j] = true;
        if matches[j].is_some_and(|other| match_equivalent(other, left, right, matches, visited)) {
            matches[j] = Some(index);
            return true;
        }
    }
    false
}
