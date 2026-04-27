//! Export FSH CodeSystem to FHIR JSON

use crate::error::FshError;
use crate::parser::ast::*;

pub fn export_code_system(cs: &CodeSystem) -> Result<serde_json::Value, FshError> {
    let mut json = serde_json::json!({
        "resourceType": "CodeSystem",
        "id": cs.metadata.id.as_deref().unwrap_or(&cs.metadata.name),
        "name": cs.metadata.name,
        "status": "active",
        "content": "complete",
    });

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

    Ok(json)
}
