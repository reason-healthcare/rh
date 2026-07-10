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
        .filter(|concept| concept.value.hierarchy.is_empty())
        .map(|c| export_concept(&c.value, &cs.concepts))
        .collect();

    if !concepts.is_empty() {
        json["count"] = serde_json::json!(cs.concepts.len());
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

fn export_concept(concept: &ConceptRule, all: &[Spanned<ConceptRule>]) -> serde_json::Value {
    let mut json = serde_json::json!({ "code": concept.code });
    if let Some(display) = &concept.display {
        json["display"] = serde_json::Value::String(display.clone());
    }
    if let Some(definition) = &concept.definition {
        json["definition"] = serde_json::Value::String(definition.clone());
    }
    let mut child_path = concept.hierarchy.clone();
    child_path.push(concept.code.clone());
    let children: Vec<_> = all
        .iter()
        .filter(|candidate| candidate.value.hierarchy == child_path)
        .map(|candidate| export_concept(&candidate.value, all))
        .collect();
    if !children.is_empty() {
        json["concept"] = serde_json::Value::Array(children);
    }
    json
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
        FshValue::Reference { target, display } => {
            let mut reference = serde_json::json!({ "reference": target });
            if let Some(display) = display {
                reference["display"] = serde_json::Value::String(display.clone());
            }
            reference
        }
        FshValue::Quantity {
            value,
            unit,
            display,
        } => {
            let mut quantity = serde_json::json!({
                "value": value,
                "system": "http://unitsofmeasure.org",
                "code": unit,
            });
            if let Some(display) = display {
                quantity["unit"] = serde_json::Value::String(display.clone());
            }
            quantity
        }
        FshValue::Ratio {
            numerator,
            denominator,
        } => serde_json::json!({
            "numerator": fsh_value_to_json_simple(numerator),
            "denominator": fsh_value_to_json_simple(denominator),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::span::SourceLocation;

    #[test]
    fn emits_count_for_all_concepts() {
        let code_system = CodeSystem {
            metadata: CsMetadata {
                name: "ExampleCodeSystem".to_string(),
                id: Some("example-cs".to_string()),
                title: None,
                description: None,
            },
            concepts: vec![
                Spanned::new(
                    ConceptRule {
                        code: "one".to_string(),
                        display: None,
                        definition: None,
                        hierarchy: Vec::new(),
                    },
                    SourceLocation::default(),
                ),
                Spanned::new(
                    ConceptRule {
                        code: "two".to_string(),
                        display: None,
                        definition: None,
                        hierarchy: Vec::new(),
                    },
                    SourceLocation::default(),
                ),
            ],
            caret_rules: Vec::new(),
        };

        let json = export_code_system(&code_system, &crate::FshConfig::default())
            .expect("CodeSystem exports");

        assert_eq!(json["count"], 2);
    }
}
