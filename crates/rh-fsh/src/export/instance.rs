//! Export FSH Instance to FHIR JSON

use crate::error::FshError;
use crate::parser::ast::*;

pub fn export_instance(inst: &Instance) -> Result<serde_json::Value, FshError> {
    let mut resource = serde_json::json!({
        "resourceType": inst.metadata.instance_of,
        "id": inst.metadata.name,
    });

    for rule in &inst.rules {
        match &rule.value {
            InstanceRule::Assignment(a) => {
                apply_assignment(&mut resource, &a.path, &a.value);
            }
            InstanceRule::Insert(_) => {
                // InsertRules should have been resolved already
            }
        }
    }

    Ok(resource)
}

/// Apply an assignment rule by navigating the FSH path and setting the value
fn apply_assignment(root: &mut serde_json::Value, path: &FshPath, value: &FshValue) {
    let json_val = fsh_value_to_json(value);
    set_at_path(root, &path.segments, json_val);
}

fn fsh_value_to_json(value: &FshValue) -> serde_json::Value {
    match value {
        FshValue::Str(s) => serde_json::Value::String(s.clone()),
        FshValue::Bool(b) => serde_json::Value::Bool(*b),
        FshValue::Integer(i) => serde_json::json!(i),
        FshValue::Decimal(d) => serde_json::json!(d),
        FshValue::Code { system, code, display } => {
            let mut obj = serde_json::json!({ "code": code });
            if let Some(s) = system {
                obj["system"] = serde_json::Value::String(s.clone());
            }
            if let Some(d) = display {
                obj["display"] = serde_json::Value::String(d.clone());
            }
            obj
        }
        FshValue::Quantity { value, unit } => {
            serde_json::json!({ "value": value, "unit": unit })
        }
        FshValue::Ratio { numerator, denominator } => {
            serde_json::json!({
                "numerator": fsh_value_to_json(numerator),
                "denominator": fsh_value_to_json(denominator),
            })
        }
        FshValue::Reference(r) => serde_json::json!({ "reference": r }),
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
    }
}

fn set_at_path(node: &mut serde_json::Value, segments: &[FshPathSegment], value: serde_json::Value) {
    if segments.is_empty() {
        *node = value;
        return;
    }

    match &segments[0] {
        FshPathSegment::Name(name) => {
            if segments.len() == 1 {
                if let serde_json::Value::Object(map) = node {
                    map.insert(name.clone(), value);
                }
            } else {
                if let serde_json::Value::Object(map) = node {
                    let child = map
                        .entry(name.clone())
                        .or_insert_with(|| serde_json::json!({}));
                    set_at_path(child, &segments[1..], value);
                }
            }
        }
        FshPathSegment::Index(idx) => {
            if let serde_json::Value::Array(arr) = node {
                let i = *idx as usize;
                while arr.len() <= i {
                    arr.push(serde_json::json!({}));
                }
                if segments.len() == 1 {
                    arr[i] = value;
                } else {
                    set_at_path(&mut arr[i], &segments[1..], value);
                }
            }
        }
        FshPathSegment::Slice(slice_name) | FshPathSegment::ChoiceType(slice_name) => {
            // Treat slice/choice type like a named field
            if let serde_json::Value::Object(map) = node {
                let key = slice_name.clone();
                if segments.len() == 1 {
                    map.insert(key, value);
                } else {
                    let child = map.entry(key).or_insert_with(|| serde_json::json!({}));
                    set_at_path(child, &segments[1..], value);
                }
            }
        }
        FshPathSegment::Extension(url) => {
            if let serde_json::Value::Object(map) = node {
                let exts = map.entry("extension").or_insert_with(|| serde_json::json!([]));
                if let serde_json::Value::Array(arr) = exts {
                    if segments.len() == 1 {
                        arr.push(serde_json::json!({ "url": url, "valueString": value }));
                    } else {
                        let mut ext_obj = serde_json::json!({ "url": url });
                        set_at_path(&mut ext_obj, &segments[1..], value);
                        arr.push(ext_obj);
                    }
                }
            }
        }
    }
}
