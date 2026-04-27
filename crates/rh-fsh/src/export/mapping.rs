//! Export FSH Mapping to FHIR ConceptMap-like JSON

use crate::error::FshError;
use crate::parser::ast::*;

pub fn export_mapping(mapping: &Mapping) -> Result<serde_json::Value, FshError> {
    let meta = &mapping.metadata;
    let mut json = serde_json::json!({
        "resourceType": "ConceptMap",
        "id": meta.id.as_deref().unwrap_or(&meta.name),
        "name": meta.name,
        "status": "active",
    });

    if let Some(title) = &meta.title {
        json["title"] = serde_json::Value::String(title.clone());
    }
    if let Some(desc) = &meta.description {
        json["description"] = serde_json::Value::String(desc.clone());
    }
    if let Some(source) = &meta.source {
        json["sourceUri"] = serde_json::Value::String(source.clone());
    }
    if let Some(target) = &meta.target {
        json["targetUri"] = serde_json::Value::String(target.clone());
    }

    let elements: Vec<serde_json::Value> = mapping
        .rules
        .iter()
        .map(|r| {
            let mut elem = serde_json::json!({ "map": r.value.map });
            if let Some(path) = &r.value.path {
                elem["path"] = serde_json::Value::String(path.to_dot_string());
            }
            if let Some(comment) = &r.value.comment {
                elem["comment"] = serde_json::Value::String(comment.clone());
            }
            if let Some(lang) = &r.value.language {
                elem["language"] = serde_json::Value::String(lang.clone());
            }
            elem
        })
        .collect();

    if !elements.is_empty() {
        json["group"] = serde_json::json!([{ "element": elements }]);
    }

    Ok(json)
}
