//! Export FSH ValueSet to FHIR JSON

use crate::error::FshError;
use crate::parser::ast::*;
use std::collections::HashMap;

pub fn export_value_set(
    vs: &ValueSet,
    config: &crate::FshConfig,
) -> Result<serde_json::Value, FshError> {
    let mut json = serde_json::json!({
        "resourceType": "ValueSet",
        "id": vs.metadata.id.as_deref().unwrap_or(&vs.metadata.name),
        "name": vs.metadata.name,
        "status": "active",
    });

    let status = config.status.as_deref().unwrap_or("active");
    json["status"] = serde_json::Value::String(status.to_string());

    if let Some(v) = &config.version {
        json["version"] = serde_json::Value::String(v.clone());
    }
    if let Some(fv) = &config.fhir_version {
        json["fhirVersion"] = serde_json::Value::String(fv.clone());
    }

    if let Some(canonical) = &config.canonical {
        let vs_id = vs.metadata.id.as_deref().unwrap_or(&vs.metadata.name);
        json["url"] = serde_json::Value::String(format!(
            "{}/ValueSet/{}",
            canonical.trim_end_matches('/'),
            vs_id
        ));
    }

    if let Some(title) = &vs.metadata.title {
        json["title"] = serde_json::Value::String(title.clone());
    }
    if let Some(desc) = &vs.metadata.description {
        json["description"] = serde_json::Value::String(desc.clone());
    }

    // Group include/exclude rules by system for same-system concept merging
    let mut include_map: HashMap<Option<String>, Vec<serde_json::Value>> = HashMap::new();
    let mut include_with_from: Vec<serde_json::Value> = Vec::new();
    let mut exclude_map: HashMap<Option<String>, Vec<serde_json::Value>> = HashMap::new();
    let mut exclude_with_from: Vec<serde_json::Value> = Vec::new();

    for comp in &vs.components {
        let c = &comp.value;
        let has_from = !c.from_vs.is_empty() || !c.filters.is_empty();

        if has_from {
            let mut entry = serde_json::json!({});
            if let Some(sys) = &c.system {
                entry["system"] = serde_json::Value::String(sys.clone());
            }
            if !c.from_vs.is_empty() {
                entry["valueSet"] = serde_json::json!(c.from_vs);
            }
            if !c.filters.is_empty() {
                let filters: Vec<serde_json::Value> = c
                    .filters
                    .iter()
                    .map(|f| {
                        serde_json::json!({
                            "property": f.property, "op": f.op, "value": f.value
                        })
                    })
                    .collect();
                entry["filter"] = serde_json::json!(filters);
            }
            if c.inclusion {
                include_with_from.push(entry);
            } else {
                exclude_with_from.push(entry);
            }
        } else {
            let concepts: Vec<serde_json::Value> = c
                .concepts
                .iter()
                .map(|ref_c| {
                    let mut cv = serde_json::json!({ "code": ref_c.code });
                    if let Some(disp) = &ref_c.display {
                        cv["display"] = serde_json::Value::String(disp.clone());
                    }
                    cv
                })
                .collect();
            if c.inclusion {
                include_map
                    .entry(c.system.clone())
                    .or_default()
                    .extend(concepts);
            } else {
                exclude_map
                    .entry(c.system.clone())
                    .or_default()
                    .extend(concepts);
            }
        }
    }

    let mut include_list: Vec<serde_json::Value> = Vec::new();
    for (sys, concepts) in include_map {
        let mut entry = serde_json::json!({});
        if let Some(s) = sys {
            entry["system"] = serde_json::Value::String(s);
        }
        if !concepts.is_empty() {
            entry["concept"] = serde_json::json!(concepts);
        }
        include_list.push(entry);
    }
    include_list.extend(include_with_from);

    let mut exclude_list: Vec<serde_json::Value> = Vec::new();
    for (sys, concepts) in exclude_map {
        let mut entry = serde_json::json!({});
        if let Some(s) = sys {
            entry["system"] = serde_json::Value::String(s);
        }
        if !concepts.is_empty() {
            entry["concept"] = serde_json::json!(concepts);
        }
        exclude_list.push(entry);
    }
    exclude_list.extend(exclude_with_from);

    let mut compose = serde_json::json!({});
    if !include_list.is_empty() {
        compose["include"] = serde_json::json!(include_list);
    }
    if !exclude_list.is_empty() {
        compose["exclude"] = serde_json::json!(exclude_list);
    }
    if compose != serde_json::json!({}) {
        json["compose"] = compose;
    }

    // Apply caret rules to override fields (^url, ^status, ^version, ^description, etc.)
    for cr in &vs.caret_rules {
        let val = fsh_value_to_json_simple(&cr.value.value);
        let key = &cr.value.caret_path;
        json[key] = val;
    }

    Ok(json)
}

fn fsh_value_to_json_simple(value: &crate::parser::ast::FshValue) -> serde_json::Value {
    use crate::parser::ast::FshValue;
    match value {
        FshValue::Str(s) => serde_json::Value::String(s.clone()),
        FshValue::Bool(b) => serde_json::Value::Bool(*b),
        FshValue::Integer(i) => serde_json::json!(i),
        FshValue::Decimal(d) => serde_json::json!(d),
        FshValue::Code { code, .. } => serde_json::Value::String(code.clone()),
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
        FshValue::Date(s) | FshValue::DateTime(s) => serde_json::Value::String(s.clone()),
        FshValue::Reference(r) => serde_json::json!({ "reference": r }),
        FshValue::Quantity { value, unit } => serde_json::json!({ "value": value, "unit": unit }),
        FshValue::Ratio {
            numerator,
            denominator,
        } => serde_json::json!({
            "numerator": fsh_value_to_json_simple(numerator),
            "denominator": fsh_value_to_json_simple(denominator),
        }),
        FshValue::InstanceRef(s) => serde_json::Value::String(s.clone()),
    }
}
