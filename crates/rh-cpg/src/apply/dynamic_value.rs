use serde_json::Value;

use crate::context::ApplyContext;
use crate::error::CpgResult;
use crate::expression::evaluate_expression;

/// Evaluate a dynamicValue element and set the result at `path` on
/// `target_resource`, mutating the resource in place.
pub fn process_dynamic_value(
    dynamic_value: &Value,
    definitional_resource: &Value,
    target_resource: &mut Value,
    library_canonicals: &[String],
    ctx: &ApplyContext,
) -> CpgResult<()> {
    let path = dynamic_value.get("path").and_then(Value::as_str);
    let expression = dynamic_value.get("expression");

    let (Some(path), Some(expression)) = (path, expression) else {
        return Ok(());
    };

    let language = expression.get("language").and_then(Value::as_str);
    if !matches!(
        language,
        Some("text/fhirpath") | Some("text/cql-identifier")
    ) {
        return Ok(());
    }

    let mut result =
        evaluate_expression(expression, definitional_resource, library_canonicals, ctx)?;
    if language == Some("text/fhirpath") {
        if let Value::Array(items) = &result {
            result = items.first().cloned().unwrap_or(Value::Null);
        }
    }

    if path.is_empty() {
        return Ok(());
    }

    set_path(target_resource, path, result);
    Ok(())
}

fn set_path(target: &mut Value, path: &str, value: Value) -> Option<()> {
    if path.is_empty() {
        return Some(());
    }

    let segment_count = path.split('.').count();
    let mut current = target;
    for (index, segment) in path.split('.').enumerate() {
        let is_last = index == segment_count - 1;
        let (field_name, array_indices) = parse_segment(segment);

        if array_indices.is_empty() {
            if is_last {
                set_field(current, field_name, value);
                return Some(());
            }

            current = navigate_field(current, field_name)?;
        } else {
            current = get_or_create_array(current, field_name)?;

            let last_array_index = array_indices.len() - 1;
            for (array_index_position, array_index) in array_indices.iter().enumerate() {
                let is_last_array_step = is_last && array_index_position == last_array_index;
                let array_len = current.as_array().map_or(0, |array| array.len());
                if *array_index >= array_len {
                    current
                        .as_array_mut()
                        .expect("current is an array")
                        .resize(*array_index + 1, Value::Null);
                }

                current = if is_last_array_step {
                    let element = current
                        .as_array_mut()
                        .expect("current is an array")
                        .get_mut(*array_index)
                        .expect("array was just padded");
                    *element = value;
                    return Some(());
                } else {
                    current = navigate_element(current.as_array_mut()?.get_mut(*array_index)?)?;
                    current
                };
            }
        }
    }

    Some(())
}

fn parse_segment(segment: &str) -> (&str, Vec<usize>) {
    let Some(open_bracket) = segment.find('[') else {
        return (segment, Vec::new());
    };

    let field_name = &segment[..open_bracket];
    let mut indices = Vec::new();
    for index_text in segment[open_bracket..]
        .split_inclusive(']')
        .filter_map(|part| {
            part.strip_prefix('[')
                .and_then(|value| value.strip_suffix(']'))
        })
    {
        match index_text.parse::<usize>() {
            Ok(index) => indices.push(index),
            Err(_) => return (field_name, Vec::new()),
        }
    }

    (field_name, indices)
}

fn set_field(target: &mut Value, field_name: &str, value: Value) {
    let Some(object) = target.as_object_mut() else {
        return;
    };
    object.insert(field_name.to_string(), value);
}

fn get_or_create_array<'a>(target: &'a mut Value, field_name: &str) -> Option<&'a mut Value> {
    let object = target.as_object_mut()?;
    if !object.contains_key(field_name) {
        object.insert(field_name.to_string(), Value::Array(Vec::new()));
    }

    match object.get_mut(field_name) {
        Some(Value::Array(_)) | Some(Value::Null) => object.get_mut(field_name),
        _ => None,
    }
}

fn navigate_field<'a>(target: &'a mut Value, field_name: &str) -> Option<&'a mut Value> {
    let object = target.as_object_mut()?;
    if !object.contains_key(field_name) {
        object.insert(
            field_name.to_string(),
            Value::Object(serde_json::Map::new()),
        );
    }

    let Some(field) = object.get_mut(field_name) else {
        return None;
    };

    match field {
        Value::Object(_) | Value::Array(_) => Some(field),
        Value::Null => {
            *field = Value::Object(serde_json::Map::new());
            Some(field)
        }
        _ => None,
    }
}

fn navigate_element(target: &mut Value) -> Option<&mut Value> {
    match target {
        Value::Object(_) | Value::Array(_) => Some(target),
        Value::Null => {
            *target = Value::Object(serde_json::Map::new());
            Some(target)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_path_sets_simple_key() {
        let mut target = serde_json::json!({});
        set_path(&mut target, "authoredOn", serde_json::json!("2026-01-01"));
        assert_eq!(target, serde_json::json!({"authoredOn": "2026-01-01"}));
    }

    #[test]
    fn set_path_sets_nested_path() {
        let mut target = serde_json::json!({});
        set_path(&mut target, "a.b.c", serde_json::json!(1));
        assert_eq!(target, serde_json::json!({"a": {"b": {"c": 1}}}));
    }

    #[test]
    fn set_path_sets_array_index_path() {
        let mut target = serde_json::json!({});
        set_path(
            &mut target,
            "dosageInstruction[0].timing",
            serde_json::json!(2),
        );
        assert_eq!(
            target,
            serde_json::json!({"dosageInstruction": [{"timing": 2}]})
        );
    }

    #[test]
    fn set_path_creates_intermediate_objects() {
        let mut target = serde_json::json!({"a": {"b": null}});
        set_path(&mut target, "a.b.c", serde_json::json!("value"));
        assert_eq!(target, serde_json::json!({"a": {"b": {"c": "value"}}}));
    }

    #[test]
    fn set_path_pads_arrays_with_null() {
        let mut target = serde_json::json!({});
        set_path(&mut target, "items[1]", serde_json::json!("second"));
        assert_eq!(target, serde_json::json!({"items": [null, "second"]}));
    }
}
