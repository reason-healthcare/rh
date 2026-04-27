//! Export FSH ValueSet to FHIR JSON

use crate::error::FshError;
use crate::parser::ast::*;

pub fn export_value_set(vs: &ValueSet) -> Result<serde_json::Value, FshError> {
    let mut json = serde_json::json!({
        "resourceType": "ValueSet",
        "id": vs.metadata.id.as_deref().unwrap_or(&vs.metadata.name),
        "name": vs.metadata.name,
        "status": "active",
    });

    if let Some(title) = &vs.metadata.title {
        json["title"] = serde_json::Value::String(title.clone());
    }
    if let Some(desc) = &vs.metadata.description {
        json["description"] = serde_json::Value::String(desc.clone());
    }

    let mut include_list = Vec::new();
    let mut exclude_list = Vec::new();

    for comp in &vs.components {
        let c = &comp.value;
        let mut entry = serde_json::json!({});

        if let Some(sys) = &c.system {
            entry["system"] = serde_json::Value::String(sys.clone());
        }

        if !c.from_vs.is_empty() {
            entry["valueSet"] = serde_json::json!(c.from_vs);
        }

        if !c.concepts.is_empty() {
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
            entry["concept"] = serde_json::json!(concepts);
        }

        if !c.filters.is_empty() {
            let filters: Vec<serde_json::Value> = c
                .filters
                .iter()
                .map(|f| {
                    serde_json::json!({
                        "property": f.property,
                        "op": f.op,
                        "value": f.value,
                    })
                })
                .collect();
            entry["filter"] = serde_json::json!(filters);
        }

        if c.inclusion {
            include_list.push(entry);
        } else {
            exclude_list.push(entry);
        }
    }

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

    Ok(json)
}
