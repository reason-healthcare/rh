//! Export FSH CodeSystem to FHIR JSON

use crate::error::FshError;
use crate::parser::ast::*;

pub fn export_code_system(
    cs: &CodeSystem,
    config: &crate::FshConfig,
) -> Result<serde_json::Value, FshError> {
    let mut json = serde_json::json!({
        "resourceType": "CodeSystem",
        "id": cs.metadata.id.as_deref().unwrap_or(&cs.metadata.name),
        "name": cs.metadata.name,
        "status": "active",
        "content": "complete",
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
        let cs_id = cs.metadata.id.as_deref().unwrap_or(&cs.metadata.name);
        json["url"] = serde_json::Value::String(format!(
            "{}/CodeSystem/{}",
            canonical.trim_end_matches('/'),
            cs_id
        ));
    }

    if let Some(title) = &cs.metadata.title {
        json["title"] = serde_json::Value::String(title.clone());
    }
    if let Some(desc) = &cs.metadata.description {
        json["description"] = serde_json::Value::String(desc.clone());
    }

    let concepts: Vec<serde_json::Value> = cs
        .concepts
        .iter()
        .map(|c| {
            let mut cv = serde_json::json!({ "code": c.value.code });
            if let Some(disp) = &c.value.display {
                cv["display"] = serde_json::Value::String(disp.clone());
            }
            if let Some(def) = &c.value.definition {
                cv["definition"] = serde_json::Value::String(def.clone());
            }
            cv
        })
        .collect();

    if !concepts.is_empty() {
        json["concept"] = serde_json::json!(concepts);
    }

    // Apply caret rules to override fields (^url, ^status, ^version, ^description, etc.)
    for cr in &cs.caret_rules {
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
        FshValue::InstanceRef(s) => serde_json::Value::String(s.clone()),
        FshValue::Reference(r) => serde_json::json!({ "reference": r }),
        FshValue::Quantity { value, unit } => serde_json::json!({ "value": value, "unit": unit }),
        FshValue::Ratio {
            numerator,
            denominator,
        } => serde_json::json!({
            "numerator": fsh_value_to_json_simple(numerator),
            "denominator": fsh_value_to_json_simple(denominator),
        }),
    }
}
