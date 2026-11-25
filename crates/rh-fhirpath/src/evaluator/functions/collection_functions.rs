//! Collection function registration for FHIRPath

use crate::error::*;
use crate::evaluator::operations::collection::CollectionEvaluator;
use crate::evaluator::operations::comparison::ComparisonEvaluator;
use crate::evaluator::types::FhirPathValue;
use std::collections::HashMap;

use super::FhirPathFunction;

/// Compare sort keys for an infallible caller, treating unavailable ordering as equal.
pub fn compare_for_sort_pub(a: &FhirPathValue, b: &FhirPathValue) -> std::cmp::Ordering {
    CollectionEvaluator::compare_sort_keys(a, b).unwrap_or(std::cmp::Ordering::Equal)
}

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

    // first() function - also handled directly by evaluator for ordered checks
    functions.insert(
        "first".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::first(target, false)
        }),
    );

    // last() function - also handled directly by evaluator for ordered checks
    functions.insert(
        "last".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::last(target, false)
        }),
    );

    // tail() function - also handled directly by evaluator for ordered checks
    functions.insert(
        "tail".to_string(),
        Box::new(|target: &FhirPathValue, _params: &[FhirPathValue]| {
            CollectionEvaluator::tail(target, false)
        }),
    );

    functions.insert(
        "skip".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            match CollectionEvaluator::subsetting_count(params, "skip()")? {
                Some(count) => CollectionEvaluator::skip(target, count, false),
                None => Ok(FhirPathValue::Empty),
            }
        }),
    );

    functions.insert(
        "take".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            match CollectionEvaluator::subsetting_count(params, "take()")? {
                Some(count) => CollectionEvaluator::take(target, count),
                None => Ok(FhirPathValue::Empty),
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
            if !(1..=2).contains(&params.len()) {
                return Err(FhirPathError::InvalidOperation {
                    message:
                        "combine() requires one or two parameters (other collection, preserveOrder)"
                            .to_string(),
                });
            }
            if let Some(preserve_order) = params.get(1) {
                preserve_order.singleton_boolean("combine() preserveOrder")?;
            }
            // Append in both modes. The default permits any order, including this one.
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
    // Key selectors are handled by the expression evaluator.
    functions.insert(
        "sort".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            if !params.is_empty() {
                return Err(FhirPathError::FunctionError {
                    message: "sort() key selectors require an expression".into(),
                });
            }
            let items = target.items();
            let mut indices: Vec<usize> = (0..items.len()).collect();
            let mut scratch = indices.clone();
            CollectionEvaluator::sort_indices_by(&mut indices, &mut scratch, &mut |a, b| {
                CollectionEvaluator::compare_sort_keys(&items[a], &items[b])
            })?;
            Ok(FhirPathValue::from_items(
                indices
                    .into_iter()
                    .map(|index| items[index].clone())
                    .collect(),
            ))
        }),
    );

    functions.insert("type".into(), Box::new(reflect_types));

    functions.insert(
        "comparable".to_string(),
        Box::new(|target: &FhirPathValue, params: &[FhirPathValue]| {
            let [other] = params else {
                return Err(FhirPathError::FunctionError {
                    message: "comparable() requires exactly one Quantity".into(),
                });
            };
            Ok(ComparisonEvaluator::quantities_comparable(target, other)
                .map_or(FhirPathValue::Empty, FhirPathValue::Boolean))
        }),
    );
}

fn reflect_types(
    target: &FhirPathValue,
    params: &[FhirPathValue],
) -> FhirPathResult<FhirPathValue> {
    if !params.is_empty() {
        return Err(FhirPathError::FunctionError {
            message: "type() takes no arguments".into(),
        });
    }
    let reflected=target.items().iter().filter_map(|item|{
        let (namespace,name)=fhirpath_type_of(item)?;
        let info=if namespace=="System" && !matches!(item,FhirPathValue::TypedObject{..}) {"System.SimpleTypeInfo"} else {"System.ClassInfo"};
        let value=serde_json::json!({"namespace":namespace,"name":name,"baseType":fhirpath_base_type_of(item)});
        Some(FhirPathValue::TypedObject { value, fhir_type:info.into() })
    });
    Ok(FhirPathValue::from_items(reflected.collect()))
}

fn fhirpath_type_of(v: &FhirPathValue) -> Option<(String, String)> {
    use rh_hl7_fhir_r4_core::metadata::FhirPrimitiveType;
    Some(match v {
        FhirPathValue::Node(node) if node.is_system_projection() => {
            return Some((
                "System".to_owned(),
                node.fhir_type.strip_prefix("System.")?.to_owned(),
            ))
        }
        FhirPathValue::Node(node) if node.is_untyped_object() => {
            return fhirpath_type_of(&node.inner);
        }
        FhirPathValue::Node(node) => ("FHIR".to_owned(), node.fhir_type.clone()),
        FhirPathValue::TypedObject { fhir_type, .. } => {
            let (namespace, name) = fhir_type
                .split_once('.')
                .filter(|(ns, _)| *ns == "System")
                .unwrap_or(("FHIR", fhir_type));
            (namespace.into(), name.into())
        }
        FhirPathValue::Object(json) => {
            if let Some(rt) = json.get("resourceType").and_then(|r| r.as_str()) {
                return Some(("FHIR".into(), rt.into()));
            }
            return None;
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
                    return Some(("FHIR".into(), s));
                }
            };
            ("FHIR".to_string(), name.to_string())
        }
        FhirPathValue::TypedBoolean { .. } => ("FHIR".to_string(), "boolean".to_string()),
        FhirPathValue::TypedDateTime { fhir_type, .. } => {
            let name = match fhir_type {
                FhirPrimitiveType::Instant => "instant",
                _ => "dateTime",
            };
            ("FHIR".to_string(), name.to_string())
        }
        other => {
            let name = match other {
                FhirPathValue::Collection(_) | FhirPathValue::UnorderedCollection(_) => {
                    "Collection"
                }
                _ => other.system_scalar_name().unwrap_or("Any"),
            };
            ("System".to_string(), name.to_string())
        }
    })
}

fn fhirpath_base_type_of(v: &FhirPathValue) -> String {
    match v {
        FhirPathValue::TypedObject { fhir_type, .. } if fhir_type.starts_with("System.") => {
            "System.TypeInfo".into()
        }
        FhirPathValue::Node(node) if node.is_untyped_object() => fhirpath_base_type_of(&node.inner),
        FhirPathValue::Node(node) => node
            .model
            .base(&node.fhir_type)
            .map(|s| format!("FHIR.{s}"))
            .unwrap_or_else(|| "System.Any".into()),
        FhirPathValue::Object(json) if json.get("resourceType").is_some() => {
            "FHIR.DomainResource".to_string()
        }
        FhirPathValue::TypedString { .. }
        | FhirPathValue::TypedBoolean { .. }
        | FhirPathValue::TypedDateTime { .. }
        | FhirPathValue::FhirPrimitive { .. } => "FHIR.Element".to_string(),
        _ => "System.Any".to_string(),
    }
}
