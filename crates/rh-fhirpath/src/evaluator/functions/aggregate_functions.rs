//! Aggregates for collections of compatible values.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem::discriminant;

use crate::error::{FhirPathError, FhirPathResult};
use crate::evaluator::operations::{ArithmeticEvaluator, ComparisonEvaluator};
use crate::evaluator::types::FhirPathValue;

use super::FhirPathFunction;
use FhirPathValue::{
    Date, DateTime, Empty, Integer, Long, Number, Quantity, String as PlainString, Time,
    TypedDateTime, TypedString,
};

pub fn register_aggregate_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    for (name, operation) in [
        ("sum", Aggregate::Sum),
        ("avg", Aggregate::Average),
        ("min", Aggregate::Extremum(Ordering::Less)),
        ("max", Aggregate::Extremum(Ordering::Greater)),
    ] {
        functions.insert(
            name.to_owned(),
            Box::new(move |target, params| {
                if !params.is_empty() {
                    return Err(FhirPathError::InvalidOperation {
                        message: format!("{name}() takes no parameters"),
                    });
                }
                operation.evaluate(target, name)
            }),
        );
    }
}

enum Aggregate {
    Sum,
    Average,
    Extremum(Ordering),
}

impl Aggregate {
    fn evaluate(&self, target: &FhirPathValue, name: &str) -> FhirPathResult<FhirPathValue> {
        let Some((first, items)) = target.items().split_first() else {
            return Ok(Empty);
        };
        let type_error = || FhirPathError::TypeError {
            message: format!("{name}() requires items of the same supported type"),
        };
        let first_value = normalized(first);
        let numeric = matches!(
            first_value,
            Integer(_) | Long(_) | Number(_) | Quantity { .. }
        );
        let ordered = numeric
            || matches!(
                first_value,
                Date(_) | DateTime(_) | Time(_) | PlainString(_)
            );
        if !(numeric || ordered && matches!(self, Self::Extremum(_))) {
            return Err(type_error());
        }
        // Validate every type even if an earlier operation yields an empty
        // result (e.g. from overflow or incompatible units).
        if items
            .iter()
            .any(|item| discriminant(&first_value) != discriminant(&normalized(item)))
        {
            return Err(type_error());
        }

        match self {
            Self::Sum => items.iter().try_fold(first_value, |total, item| {
                ArithmeticEvaluator::add_values(&total, item)
            }),
            Self::Average => average(target.items()),
            Self::Extremum(wanted) => {
                let mut candidate = first;
                for item in items {
                    match ComparisonEvaluator::compare_order(item, candidate)? {
                        Some(order) if order == *wanted => candidate = item,
                        Some(_) => {}
                        None => return Ok(Empty),
                    }
                }
                Ok(candidate.clone())
            }
        }
    }
}

fn average(items: &[FhirPathValue]) -> FhirPathResult<FhirPathValue> {
    let count = i64::try_from(items.len()).map_or(Empty, Long);
    let addends = items.iter().map(|value| match normalized(value) {
        Integer(value) | Long(value) => Number(value.into()),
        value => value,
    });
    let mut values = addends.clone();
    let Some(first) = values.next() else {
        return Ok(Empty);
    };
    let sum = values.try_fold(first, |total, item| {
        ArithmeticEvaluator::add_values(&total, &item)
    })?;
    if !matches!(sum, Empty) {
        return ArithmeticEvaluator::divide_values(&sum, &count);
    }
    // Dividing before summing avoids an intermediate overflow when the
    // average fits even though the sum of all measurements does not.
    let mut scaled = addends.map(|value| ArithmeticEvaluator::divide_values(&value, &count));
    let Some(first) = scaled.next() else {
        return Ok(Empty);
    };
    scaled.try_fold(first?, |total, item| {
        ArithmeticEvaluator::add_values(&total, &item?)
    })
}

fn normalized(value: &FhirPathValue) -> FhirPathValue {
    match value.system_value() {
        TypedString { value, .. } => PlainString(value.clone()),
        TypedDateTime { value, .. } => DateTime(value.clone()),
        value => value.clone(),
    }
}
