//! Collection function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::collection::CollectionEvaluator;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Register basic collection functions (empty, exists, count, etc.)
pub fn register_collection_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // empty() function
    functions.insert(
        "empty".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::is_empty(target)
        }),
    );

    // exists() function
    functions.insert(
        "exists".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::exists(target)
        }),
    );

    // count() function
    functions.insert(
        "count".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::count(target)
        }),
    );

    // distinct() function
    functions.insert(
        "distinct".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::distinct(target)
        }),
    );

    // isDistinct() function
    functions.insert(
        "isDistinct".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::is_distinct(target)
        }),
    );

    // children() function
    functions.insert(
        "children".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::children(target)
        }),
    );

    // descendants() function
    functions.insert(
        "descendants".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::descendants(target)
        }),
    );
}

/// Register boolean collection functions (all, allTrue, anyTrue, etc.)
pub fn register_boolean_collection_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // all() function
    functions.insert(
        "all".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all(target)
        }),
    );

    // allTrue() function
    functions.insert(
        "allTrue".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all_true(target)
        }),
    );

    // anyTrue() function
    functions.insert(
        "anyTrue".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::any_true(target)
        }),
    );

    // allFalse() function
    functions.insert(
        "allFalse".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::all_false(target)
        }),
    );

    // anyFalse() function
    functions.insert(
        "anyFalse".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::any_false(target)
        }),
    );
}

/// Register subsetting functions (single, first, last, skip, take, etc.)
pub fn register_subsetting_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // single() function
    functions.insert(
        "single".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::single(target)
        }),
    );

    // first() function
    functions.insert(
        "first".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::first(target)
        }),
    );

    // last() function
    functions.insert(
        "last".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::last(target)
        }),
    );

    // tail() function
    functions.insert(
        "tail".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::tail(target)
        }),
    );

    // skip() function
    functions.insert(
        "skip".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "skip() requires exactly one parameter (count)".to_string(),
                });
            }
            match &params[0] {
                FhirPathValue::Integer(count) => CollectionEvaluator::skip(target, *count),
                _ => Err(FhirPathError::InvalidOperation {
                    message: "skip() count parameter must be an integer".to_string(),
                }),
            }
        }),
    );

    // take() function
    functions.insert(
        "take".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "take() requires exactly one parameter (count)".to_string(),
                });
            }
            match &params[0] {
                FhirPathValue::Integer(count) => CollectionEvaluator::take(target, *count),
                _ => Err(FhirPathError::InvalidOperation {
                    message: "take() count parameter must be an integer".to_string(),
                }),
            }
        }),
    );

    // intersect() function
    functions.insert(
        "intersect".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "intersect() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::intersect(target, &params[0])
        }),
    );

    // exclude() function
    functions.insert(
        "exclude".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "exclude() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::exclude(target, &params[0])
        }),
    );

    // combine() function
    functions.insert(
        "combine".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "combine() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::combine(target, &params[0])
        }),
    );

    // union() function
    functions.insert(
        "union".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "union() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::union(target, &params[0])
        }),
    );

    // subsetOf() function
    functions.insert(
        "subsetOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "subsetOf() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::subset_of(target, &params[0])
        }),
    );

    // supersetOf() function
    functions.insert(
        "supersetOf".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() != 1 {
                return Err(FhirPathError::InvalidOperation {
                    message: "supersetOf() requires exactly one parameter (other collection)"
                        .to_string(),
                });
            }
            CollectionEvaluator::superset_of(target, &params[0])
        }),
    );
}

/// Register control flow functions (iif)
pub fn register_control_flow_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // iif() function - immediate if (conditional operator)
    functions.insert(
        "iif".to_string(),
        Box::new(|_target: &FhirPathValue, params: &[FhirPathValue]| {
            if params.len() < 2 || params.len() > 3 {
                return Err(FhirPathError::InvalidOperation {
                    message: "iif() requires 2 or 3 parameters (criterion, true_result, [otherwise_result])".to_string(),
                });
            }
            let criterion = &params[0];
            let true_result = &params[1];
            let otherwise_result = params.get(2);
            CollectionEvaluator::iif(criterion, true_result, otherwise_result)
        }),
    );
}

pub fn register_sort_type_functions(functions: &mut HashMap<String, FhirPathFunction>) {
    // sort([expression]) — sort a collection, ascending by value or by expression
    // Argument-less form sorts scalars; $this-expression form sorts by a key.
    // NOTE: this is the 0- or 1-argument FunctionRef; the special-form variant
    // with $this expressions is handled via the aggregation dispatch. The
    // version registered here handles the common 0-argument case.
    functions.insert(
        "sort".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            let items: Vec<FhirPathValue> = match target {
                FhirPathValue::Empty => return Ok(FhirPathValue::Empty),
                FhirPathValue::Collection(v) => v.clone(),
                other => return Ok(other.clone()),
            };
            let mut sorted = items;
            sorted.sort_by(compare_for_sort);
            if sorted.is_empty() {
                Ok(FhirPathValue::Empty)
            } else if sorted.len() == 1 {
                Ok(sorted.into_iter().next().unwrap())
            } else {
                Ok(FhirPathValue::Collection(sorted))
            }
        }),
    );

    // type() — returns a TypeSpecifier object with namespace and name fields
    functions.insert(
        "type".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            let (namespace, name) = fhirpath_type_of(target);
            Ok(FhirPathValue::Object(serde_json::json!({
                "namespace": namespace,
                "name": name
            })))
        }),
    );

    // comparable(other) — true when both values can be compared via UCUM
    // For simplicity, we return true when both are quantities and share base unit.
    functions.insert(
        "comparable".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            let other = params.first().unwrap_or(&FhirPathValue::Empty);
            Ok(FhirPathValue::Boolean(are_comparable(target, other)))
        }),
    );
}

/// Public alias for use by the evaluator's expression-based sort.
pub fn compare_for_sort_pub(a: &FhirPathValue, b: &FhirPathValue) -> std::cmp::Ordering {
    compare_for_sort(a, b)
}

fn compare_for_sort(a: &FhirPathValue, b: &FhirPathValue) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    use FhirPathValue::*;
    // Empty/missing values sort LAST (treated as greater than all concrete values).
    let a_empty = matches!(a, Empty) || matches!(a, Collection(c) if c.is_empty());
    let b_empty = matches!(b, Empty) || matches!(b, Collection(c) if c.is_empty());
    if a_empty && b_empty {
        return Ordering::Equal;
    }
    if a_empty {
        return Ordering::Greater;
    }
    if b_empty {
        return Ordering::Less;
    }
    match (a, b) {
        (Boolean(x), Boolean(y)) => x.cmp(y),
        (Integer(x), Integer(y)) | (Long(x), Long(y)) => x.cmp(y),
        (Integer(x), Long(y)) | (Long(x), Integer(y)) => x.cmp(y),
        (Number(x), Number(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Integer(x), Number(y)) | (Long(x), Number(y)) => {
            (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal)
        }
        (Number(x), Integer(y)) | (Number(x), Long(y)) => {
            x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal)
        }
        (String(x), String(y))
        | (Date(x), Date(y))
        | (DateTime(x), DateTime(y))
        | (Time(x), Time(y)) => x.cmp(y),
        _ => Ordering::Equal,
    }
}

fn fhirpath_type_of(v: &FhirPathValue) -> (String, String) {
    use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;
    match v {
        FhirPathValue::Object(json) => {
            if let Some(rt) = json.get("resourceType").and_then(|r| r.as_str()) {
                return ("FHIR".to_string(), rt.to_string());
            }
            ("FHIR".to_string(), "Any".to_string())
        }
        FhirPathValue::TypedString { fhir_type, .. } => {
            let name = match fhir_type {
                FhirPrimitiveType::String => "string",
                FhirPrimitiveType::Code => "code",
                FhirPrimitiveType::Id => "id",
                FhirPrimitiveType::Uri => "uri",
                FhirPrimitiveType::Url => "url",
                FhirPrimitiveType::Canonical => "canonical",
                FhirPrimitiveType::Oid => "oid",
                FhirPrimitiveType::Markdown => "markdown",
                FhirPrimitiveType::Base64Binary => "base64Binary",
                other => {
                    let s = format!("{:?}", other);
                    return ("FHIR".to_string(), s);
                }
            };
            ("FHIR".to_string(), name.to_string())
        }
        other => {
            let name = match other {
                FhirPathValue::Boolean(_) => "Boolean",
                FhirPathValue::String(_) => "String",
                FhirPathValue::Integer(_) | FhirPathValue::Long(_) => "Integer",
                FhirPathValue::Number(_) => "Decimal",
                FhirPathValue::Date(_) => "Date",
                FhirPathValue::DateTime(_) => "DateTime",
                FhirPathValue::Time(_) => "Time",
                FhirPathValue::Quantity { .. } => "Quantity",
                _ => "Any",
            };
            ("System".to_string(), name.to_string())
        }
    }
}

/// UCUM base-unit classification for `comparable()`.
/// Two quantities are comparable when they share the same dimension.
fn ucum_dimension(unit: &str) -> Option<&'static str> {
    match unit {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "[in_i]" => Some("length"),
        "kg" | "g" | "mg" | "lb" | "oz" => Some("mass"),
        "s" | "ms" | "min" | "h" | "d" | "wk" | "mo" | "a" => Some("time"),
        "L" | "mL" | "dL" => Some("volume"),
        "Pa" | "kPa" | "mm[Hg]" => Some("pressure"),
        _ => None,
    }
}

fn are_comparable(a: &FhirPathValue, b: &FhirPathValue) -> bool {
    match (a, b) {
        (FhirPathValue::Quantity { unit: ua, .. }, FhirPathValue::Quantity { unit: ub, .. }) => {
            let ua_str = ua.as_deref().unwrap_or("");
            let ub_str = ub.as_deref().unwrap_or("");
            match (ucum_dimension(ua_str), ucum_dimension(ub_str)) {
                (Some(da), Some(db)) => da == db,
                _ => ua == ub,
            }
        }
        _ => false,
    }
}
