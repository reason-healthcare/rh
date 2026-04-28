//! Export FSH Profile/Extension/Logical/Resource to FHIR StructureDefinition JSON

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use std::sync::Arc;

pub fn export_profile(
    profile: &Profile,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    parent_types: &std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, FshError> {
    let fhir_type = profile
        .metadata
        .parent
        .as_deref()
        .and_then(|p| resolve_fhir_type(p, parent_types, &defs))
        .unwrap_or_else(|| profile.metadata.name.clone());
    export_sd(
        &profile.metadata,
        &profile.rules,
        "constraint",
        "resource",
        &fhir_type,
        defs,
        config,
    )
}

pub fn export_extension(
    ext: &Extension,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    _parent_types: &std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &ext.metadata,
        &ext.rules,
        "constraint",
        "complex-type",
        "Extension",
        defs,
        config,
    )
}

pub fn export_logical(
    logical: &Logical,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    _parent_types: &std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &logical.metadata,
        &logical.rules,
        "specialization",
        "logical",
        &logical.metadata.name,
        defs,
        config,
    )
}

pub fn export_resource_def(
    res: &ResourceDef,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    _parent_types: &std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &res.metadata,
        &res.rules,
        "specialization",
        "resource",
        &res.metadata.name,
        defs,
        config,
    )
}

/// Resolve the FHIR base type by walking up the parent chain.
/// Returns the first parent name that is a known FHIR type.
fn resolve_fhir_type(
    name: &str,
    parent_types: &std::collections::HashMap<String, String>,
    defs: &FhirDefs,
) -> Option<String> {
    if defs.get_sd(name).is_some() {
        // It's a built-in FHIR type
        let sd = defs.get_sd(name).unwrap();
        return Some(sd.base_type.clone());
    }
    // Walk up via the pre-computed parent_types map
    if let Some(parent) = parent_types.get(name) {
        return resolve_fhir_type(parent, parent_types, defs);
    }
    None
}

fn export_sd(
    meta: &SdMetadata,
    rules: &[Spanned<SdRule>],
    derivation: &str,
    kind: &str,
    fhir_type: &str,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
) -> Result<serde_json::Value, FshError> {
    let base_url = meta
        .parent
        .as_deref()
        .and_then(|p| defs.get_sd(p))
        .map(|sd| sd.url.clone())
        .unwrap_or_else(|| {
            // Custom parent profile — build its URL
            if let Some(parent) = &meta.parent {
                let parent_id = parent; // use as-is; ideally we'd kebab-case it
                if let Some(canonical) = &config.canonical {
                    format!(
                        "{}/StructureDefinition/{}",
                        canonical.trim_end_matches('/'),
                        parent_id
                    )
                } else {
                    format!("http://hl7.org/fhir/StructureDefinition/{}", parent_id)
                }
            } else {
                "http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()
            }
        });

    let sd_id = meta.id.as_deref().unwrap_or(&meta.name);
    let url = if let Some(canonical) = &config.canonical {
        format!(
            "{}/StructureDefinition/{}",
            canonical.trim_end_matches('/'),
            sd_id
        )
    } else {
        format!("http://hl7.org/fhir/StructureDefinition/{}", sd_id)
    };

    let status = config.status.as_deref().unwrap_or("active");

    // base_path for differential element paths is the FHIR resource type
    let base_path = fhir_type;

    // Helper: build FHIR SD path and id from base_path + FshPath.
    // An empty FshPath (from `* . MS` dot-path rule) refers to the root element.
    let make_fhir_path = |path: &FshPath| -> String { path.to_fhir_path(base_path) };
    let make_fhir_id = |path: &FshPath| -> String { path.to_fhir_id(base_path) };

    let mut elements: Vec<serde_json::Value> = Vec::new();

    // Add root element. For extensions and logicals always include it (with short/definition from metadata).
    // For other SDs, only include it when a rule explicitly targets the root element.
    let is_extension = kind == "complex-type";
    let is_logical = kind == "logical";
    let has_root_rule = rules.iter().any(|r| match &r.value {
        // CaretValue with no path sets SD-level properties, NOT a differential element
        SdRule::Obeys(ob) => ob.path.is_none(),
        SdRule::Flag(f) => f.paths.iter().any(|p| p.is_empty()),
        SdRule::Card(c) => c.path.is_empty(),
        _ => false,
    });
    if is_extension || is_logical || has_root_rule {
        let mut root_elem = serde_json::json!({ "id": base_path, "path": base_path });
        if is_extension || is_logical {
            if let Some(title) = &meta.title {
                root_elem["short"] = serde_json::Value::String(title.clone());
            }
            if let Some(desc) = &meta.description {
                root_elem["definition"] = serde_json::Value::String(desc.clone());
            }
        }
        elements.push(root_elem);
    }

    for rule in rules {
        match &rule.value {
            SdRule::Card(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let mut elem = serde_json::json!({ "id": id, "path": path });
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
                for path in &r.paths {
                    if path.is_empty() {
                        // Root-targeting flag (e.g., `* . MS`) — update root element in place
                        if let Some(root) = elements.first_mut() {
                            apply_flags_to_element(root, &r.flags);
                        }
                    } else {
                        let elem_path = make_fhir_path(path);
                        let elem_id = make_fhir_id(path);
                        let mut elem = serde_json::json!({ "id": elem_id, "path": elem_path });
                        apply_flags_to_element(&mut elem, &r.flags);
                        elements.push(elem);
                    }
                }
            }
            SdRule::Binding(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let strength = r.strength.as_deref().unwrap_or("required");
                elements.push(serde_json::json!({
                    "id": id,
                    "path": path,
                    "binding": {
                        "strength": strength,
                        "valueSet": r.value_set,
                    }
                }));
            }
            SdRule::Assignment(r) => {
                // Skip rules where the code system is an unresolved alias (not a URL or URN)
                if let FshValue::Code {
                    system: Some(sys), ..
                } = &r.value
                {
                    if !sys.contains("://") && !sys.starts_with("urn:") {
                        continue;
                    }
                }
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let val = fsh_value_to_fixed(&r.value);
                let mut elem = serde_json::json!({ "id": id, "path": path });
                elem["fixed"] = val;
                elements.push(elem);
            }
            SdRule::Only(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|t| serde_json::json!({ "code": t }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": id,
                    "path": path,
                    "type": type_list,
                }));
            }
            SdRule::Obeys(r) => {
                let (path, id) = r
                    .path
                    .as_ref()
                    .map(|p| (make_fhir_path(p), make_fhir_id(p)))
                    .unwrap_or_else(|| (base_path.to_string(), base_path.to_string()));
                let constraints: Vec<serde_json::Value> = r
                    .invariants
                    .iter()
                    .map(|inv| serde_json::json!({ "key": inv }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": id,
                    "path": path,
                    "constraint": constraints,
                }));
            }
            SdRule::CaretValue(r) => {
                if r.path.is_some() {
                    // Element-level caret rule: add to differential
                    let (path, id) = r
                        .path
                        .as_ref()
                        .map(|p| (make_fhir_path(p), make_fhir_id(p)))
                        .unwrap();
                    let val = fsh_value_to_fixed(&r.value);
                    let mut elem = serde_json::json!({ "id": id, "path": path });
                    elem[&r.caret_path] = val;
                    elements.push(elem);
                }
                // Root-level caret rules (path = None) are applied to the SD object below
            }
            SdRule::Contains(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
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
                    "id": id,
                    "path": path,
                    "slicing": { "rules": "open" },
                    "slices": slices,
                }));
            }
            SdRule::AddElement(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|t| serde_json::json!({ "code": t }))
                    .collect();
                elements.push(serde_json::json!({
                    "id": id,
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

    // For extensions, inject standard elements that sushi auto-generates.
    // Detect whether this is a simple extension (no Contains rules → no sub-extensions).
    if is_extension {
        let has_contains = rules
            .iter()
            .any(|r| matches!(&r.value, SdRule::Contains(_)));
        if !has_contains {
            // Simple extension: Extension.extension (max:0), Extension.url (fixedUri), Extension.value[x]
            let ext_ext = serde_json::json!({
                "id": "Extension.extension",
                "path": "Extension.extension",
                "max": "0"
            });
            let ext_url = serde_json::json!({
                "id": "Extension.url",
                "path": "Extension.url",
                "fixedUri": url
            });
            // Merge any existing Extension.value[x] element from rules with the standard one.
            // Find and remove any existing value[x] element from elements vec.
            let user_value_x: Option<serde_json::Value> = {
                let pos = elements.iter().position(|e| {
                    e.get("path").and_then(|v| v.as_str()) == Some("Extension.value[x]")
                });
                pos.map(|i| elements.remove(i))
            };
            // Insert standard elements right after root element
            let insert_pos = if elements.is_empty() { 0 } else { 1 };
            elements.insert(insert_pos, ext_url);
            elements.insert(insert_pos, ext_ext);
            // Append value[x] element — use user-provided one or a minimal default
            let val_elem = user_value_x.unwrap_or_else(|| {
                serde_json::json!({
                    "id": "Extension.value[x]",
                    "path": "Extension.value[x]"
                })
            });
            elements.push(val_elem);
        } else {
            // Complex extension: For each named slice, inject Extension.extension.extension and
            // Extension.extension.url sub-elements that sushi auto-generates.
            let slice_names: Vec<String> = rules
                .iter()
                .filter_map(|r| {
                    if let SdRule::Contains(c) = &r.value {
                        Some(
                            c.items
                                .iter()
                                .map(|item| item.name.clone())
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();

            for slice_name in &slice_names {
                elements.push(serde_json::json!({
                    "id": format!("Extension.extension:{slice_name}.extension"),
                    "path": "Extension.extension.extension",
                    "max": "0"
                }));
                elements.push(serde_json::json!({
                    "id": format!("Extension.extension:{slice_name}.url"),
                    "path": "Extension.extension.url",
                    "fixedUri": slice_name
                }));
            }
            // Complex extension also needs top-level url and value[x] (no direct values allowed)
            elements.push(serde_json::json!({
                "id": "Extension.url",
                "path": "Extension.url",
                "fixedUri": url
            }));
            elements.push(serde_json::json!({
                "id": "Extension.value[x]",
                "path": "Extension.value[x]",
                "max": "0"
            }));
        }
    }

    let mut sd = serde_json::json!({
        "resourceType": "StructureDefinition",
        "id": meta.id.as_deref().unwrap_or(&meta.name),
        "url": url,
        "name": meta.name,
        "title": meta.title.as_deref().unwrap_or(&meta.name),
        "status": status,
        "kind": kind,
        "abstract": false,
        "type": fhir_type,
        "baseDefinition": base_url,
        "derivation": derivation,
        "differential": { "element": elements },
    });

    if let Some(v) = &config.version {
        sd["version"] = serde_json::Value::String(v.clone());
    }
    if let Some(fv) = &config.fhir_version {
        sd["fhirVersion"] = serde_json::Value::String(fv.clone());
    }
    if let Some(desc) = &meta.description {
        sd["description"] = serde_json::Value::String(desc.clone());
    }

    // Apply root-level caret rules to the SD object (e.g., * ^abstract = true)
    for rule in rules {
        if let SdRule::CaretValue(r) = &rule.value {
            if r.path.is_none() {
                let val = fsh_value_to_fixed(&r.value);
                // Handle nested caret paths like "slicing.discriminator.type" using dot notation
                set_nested_value(&mut sd, &r.caret_path, val);
            }
        }
    }

    Ok(sd)
}

fn fsh_value_to_fixed(value: &FshValue) -> serde_json::Value {
    match value {
        FshValue::Str(s) => serde_json::Value::String(s.clone()),
        FshValue::Bool(b) => serde_json::Value::Bool(*b),
        FshValue::Integer(i) => serde_json::json!(i),
        FshValue::Decimal(d) => serde_json::json!(d),
        FshValue::Code {
            system,
            code,
            display,
        } => {
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
        FshValue::Ratio {
            numerator,
            denominator,
        } => serde_json::json!({
            "numerator": fsh_value_to_fixed(numerator),
            "denominator": fsh_value_to_fixed(denominator),
        }),
        FshValue::Reference(r) => serde_json::json!({ "reference": r }),
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
        FshValue::Date(s) | FshValue::DateTime(s) => serde_json::Value::String(s.clone()),
        FshValue::InstanceRef(s) => serde_json::Value::String(s.clone()),
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

/// Set a (potentially nested dot-path) key on a JSON object.
fn set_nested_value(obj: &mut serde_json::Value, path: &str, val: serde_json::Value) {
    let parts: Vec<&str> = path.splitn(2, '.').collect();
    if parts.len() == 1 {
        if let Some(map) = obj.as_object_mut() {
            map.insert(parts[0].to_string(), val);
        }
    } else if let Some(map) = obj.as_object_mut() {
        let child = map
            .entry(parts[0].to_string())
            .or_insert_with(|| serde_json::json!({}));
        set_nested_value(child, parts[1], val);
    }
}
