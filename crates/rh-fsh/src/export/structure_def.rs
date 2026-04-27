//! Export FSH Profile/Extension/Logical/Resource to FHIR StructureDefinition JSON

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use std::sync::Arc;

pub fn export_profile(
    profile: &Profile,
    defs: Arc<FhirDefs>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &profile.metadata,
        &profile.rules,
        "constraint",
        "resource",
        defs,
    )
}

pub fn export_extension(
    ext: &Extension,
    defs: Arc<FhirDefs>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &ext.metadata,
        &ext.rules,
        "constraint",
        "complex-type",
        defs,
    )
}

pub fn export_logical(logical: &Logical, defs: Arc<FhirDefs>) -> Result<serde_json::Value, FshError> {
    export_sd(
        &logical.metadata,
        &logical.rules,
        "specialization",
        "logical",
        defs,
    )
}

pub fn export_resource_def(
    res: &ResourceDef,
    defs: Arc<FhirDefs>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &res.metadata,
        &res.rules,
        "specialization",
        "resource",
        defs,
    )
}

fn export_sd(
    meta: &SdMetadata,
    rules: &[Spanned<SdRule>],
    derivation: &str,
    kind: &str,
    defs: Arc<FhirDefs>,
) -> Result<serde_json::Value, FshError> {
    let base_url = meta
        .parent
        .as_deref()
        .and_then(|p| defs.get_sd(p))
        .map(|sd| sd.url.clone())
        .unwrap_or_else(|| {
            "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()
        });

    let url = format!("http://hl7.org/fhir/StructureDefinition/{}", meta.name);

    let mut elements: Vec<serde_json::Value> = Vec::new();

    // Root element
    elements.push(serde_json::json!({
        "id": meta.name,
        "path": meta.name,
    }));

    for rule in rules {
        match &rule.value {
            SdRule::Card(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let mut elem = serde_json::json!({ "id": path, "path": path });
                if let Some(min) = r.min {
                    elem["min"] = serde_json::json!(min);
                }
                if let Some(max) = &r.max {
                    elem["max"] = serde_json::Value::String(max.clone());
                } else {
                    elem["max"] = serde_json::Value::String("*".to_string());
                }
                apply_flags_to_element(&mut elem, &r.flags);
                elements.push(elem);
            }
            SdRule::Flag(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let mut elem = serde_json::json!({ "id": path, "path": path });
                apply_flags_to_element(&mut elem, &r.flags);
                elements.push(elem);
            }
            SdRule::Binding(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let strength = r.strength.as_deref().unwrap_or("required");
                elements.push(serde_json::json!({
                    "id": path,
                    "path": path,
                    "binding": {
                        "strength": strength,
                        "valueSet": r.value_set,
                    }
                }));
            }
            SdRule::Assignment(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let val = fsh_value_to_fixed(&r.value);
                let mut elem = serde_json::json!({ "id": path, "path": path });
                elem["fixed"] = val;
                elements.push(elem);
            }
            SdRule::Only(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|t| serde_json::json!({ "code": t }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": path,
                    "path": path,
                    "type": type_list,
                }));
            }
            SdRule::Obeys(r) => {
                let path = r
                    .path
                    .as_ref()
                    .map(|p| format!("{}.{}", meta.name, p))
                    .unwrap_or_else(|| meta.name.clone());
                let constraints: Vec<serde_json::Value> = r
                    .invariants
                    .iter()
                    .map(|inv| serde_json::json!({ "key": inv }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": path,
                    "path": path,
                    "constraint": constraints,
                }));
            }
            SdRule::CaretValue(r) => {
                let path = r
                    .path
                    .as_ref()
                    .map(|p| format!("{}.{}", meta.name, p))
                    .unwrap_or_else(|| meta.name.clone());
                let val = fsh_value_to_fixed(&r.value);
                let mut elem = serde_json::json!({ "id": path, "path": path });
                elem[&r.caret_path] = val;
                elements.push(elem);
            }
            SdRule::Contains(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let slices: Vec<serde_json::Value> = r
                    .items
                    .iter()
                    .map(|item| {
                        let mut s = serde_json::json!({ "sliceName": item.name });
                        if let Some(min) = item.min {
                            s["min"] = serde_json::json!(min);
                        }
                        if let Some(max) = &item.max {
                            s["max"] = serde_json::Value::String(max.clone());
                        }
                        s
                    })
                    .collect();
                elements.push(serde_json::json!({
                    "id": path,
                    "path": path,
                    "slicing": { "rules": "open" },
                    "slices": slices,
                }));
            }
            SdRule::AddElement(r) => {
                let path = format!("{}.{}", meta.name, r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|t| serde_json::json!({ "code": t }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": path,
                    "path": path,
                    "min": r.min,
                    "max": r.max,
                    "type": type_list,
                    "short": r.short,
                }));
            }
            SdRule::Insert(_) | SdRule::Path(_) => {
                // InsertRules resolved; PathRules are path-only context setters
            }
        }
    }

    Ok(serde_json::json!({
        "resourceType": "StructureDefinition",
        "id": meta.id.as_deref().unwrap_or(&meta.name),
        "url": url,
        "name": meta.name,
        "title": meta.title.as_deref().unwrap_or(&meta.name),
        "status": "active",
        "kind": kind,
        "abstract": false,
        "type": meta.parent.as_deref().unwrap_or(&meta.name),
        "baseDefinition": base_url,
        "derivation": derivation,
        "differential": { "element": elements },
    }))
}

fn fsh_value_to_fixed(value: &FshValue) -> serde_json::Value {
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
        FshValue::Quantity { value, unit } => serde_json::json!({ "value": value, "unit": unit }),
        FshValue::Ratio { numerator, denominator } => serde_json::json!({
            "numerator": fsh_value_to_fixed(numerator),
            "denominator": fsh_value_to_fixed(denominator),
        }),
        FshValue::Reference(r) => serde_json::json!({ "reference": r }),
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
    }
}

fn apply_flags_to_element(elem: &mut serde_json::Value, flags: &[FshFlag]) {
    for flag in flags {
        match flag {
            FshFlag::MustSupport => {
                elem["mustSupport"] = serde_json::Value::Bool(true);
            }
            FshFlag::SummaryElement => {
                elem["isSummary"] = serde_json::Value::Bool(true);
            }
            FshFlag::Modifier => {
                elem["isModifier"] = serde_json::Value::Bool(true);
            }
            FshFlag::Normative | FshFlag::TrialUse | FshFlag::Draft => {
                // Standard status flags — could set extension
            }
        }
    }
}
