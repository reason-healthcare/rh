//! Export FSH Profile/Extension/Logical/Resource to FHIR StructureDefinition JSON

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use crate::DefinitionIndex;
use std::sync::Arc;

struct StructureExportContext<'a> {
    defs: Arc<FhirDefs>,
    config: &'a crate::FshConfig,
    definition_index: &'a DefinitionIndex,
}

pub fn export_profile(
    profile: &Profile,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    definition_index: &DefinitionIndex,
) -> Result<serde_json::Value, FshError> {
    let fhir_type = profile
        .metadata
        .parent
        .as_deref()
        .and_then(|p| definition_index.resolve_base_type(p, &defs))
        .unwrap_or_else(|| profile.metadata.name.clone());
    export_sd(
        &profile.metadata,
        &profile.rules,
        "constraint",
        "resource",
        &fhir_type,
        StructureExportContext {
            defs,
            config,
            definition_index,
        },
    )
}

pub fn export_extension(
    ext: &Extension,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    definition_index: &DefinitionIndex,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &ext.metadata,
        &ext.rules,
        "constraint",
        "complex-type",
        "Extension",
        StructureExportContext {
            defs,
            config,
            definition_index,
        },
    )
}

pub fn export_logical(
    logical: &Logical,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    definition_index: &DefinitionIndex,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &logical.metadata,
        &logical.rules,
        "specialization",
        "logical",
        &logical.metadata.name,
        StructureExportContext {
            defs,
            config,
            definition_index,
        },
    )
}

pub fn export_resource_def(
    res: &ResourceDef,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    definition_index: &DefinitionIndex,
) -> Result<serde_json::Value, FshError> {
    export_sd(
        &res.metadata,
        &res.rules,
        "specialization",
        "resource",
        &res.metadata.name,
        StructureExportContext {
            defs,
            config,
            definition_index,
        },
    )
}

fn export_sd(
    meta: &SdMetadata,
    rules: &[Spanned<SdRule>],
    derivation: &str,
    kind: &str,
    fhir_type: &str,
    context: StructureExportContext<'_>,
) -> Result<serde_json::Value, FshError> {
    let StructureExportContext {
        defs,
        config,
        definition_index,
    } = context;
    let base_url = meta
        .parent
        .as_deref()
        .and_then(|p| defs.get_sd(p))
        .map(|sd| sd.url.clone())
        .or_else(|| {
            meta.parent.as_deref().and_then(|parent| {
                definition_index.lookup(parent).and_then(|definition| {
                    definition.url.as_ref().map(|url| {
                        parent
                            .split_once('|')
                            .map(|(_, version)| format!("{url}|{version}"))
                            .unwrap_or_else(|| url.clone())
                    })
                })
            })
        })
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

    let base_path = if kind == "logical" { sd_id } else { fhir_type };
    let sd_type = if kind == "logical" {
        config
            .canonical
            .as_ref()
            .map(|canonical| {
                format!(
                    "{}/StructureDefinition/{sd_id}",
                    canonical.trim_end_matches('/')
                )
            })
            .unwrap_or_else(|| sd_id.to_string())
    } else {
        fhir_type.to_string()
    };

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
                inject_choice_slice_elements(&mut elements, &r.path, base_path);
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
            inject_simple_extension_elements(&mut elements, &url);
        } else {
            inject_complex_extension_elements(&mut elements, &url, rules);
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
        "type": sd_type,
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
    let mut caret_paths = CaretPathWriter::with_defs(&defs);
    for rule in rules {
        if let SdRule::CaretValue(r) = &rule.value {
            if r.path.is_none() {
                let val = fsh_value_to_caret(&r.value);
                // Handle nested caret paths like "slicing.discriminator.type" using dot notation
                caret_paths.set(&mut sd, &r.caret_path, val);
            }
        }
    }

    Ok(sd)
}

fn inject_choice_slice_elements(
    elements: &mut Vec<serde_json::Value>,
    path: &FshPath,
    base_path: &str,
) {
    let Some(FshPathSegment::Name(resolved_name)) = path.segments.first() else {
        return;
    };
    let Some(choice_name) = normalize_choice_type_name(resolved_name) else {
        return;
    };
    let choice_id = format!("{base_path}.{choice_name}");
    if !elements
        .iter()
        .any(|element| element.get("id").and_then(|id| id.as_str()) == Some(choice_id.as_str()))
    {
        elements.push(serde_json::json!({
            "id": choice_id,
            "path": format!("{base_path}.{choice_name}"),
            "slicing": {
                "discriminator": [{ "type": "type", "path": "$this" }],
                "ordered": false,
                "rules": "open"
            }
        }));
    }
    let slice_id = format!("{base_path}.{choice_name}:{resolved_name}");
    if elements
        .iter()
        .any(|element| element.get("id").and_then(|id| id.as_str()) == Some(slice_id.as_str()))
    {
        return;
    }
    let type_name = resolved_name
        .strip_prefix(choice_name.trim_end_matches("[x]"))
        .unwrap_or(resolved_name);
    elements.push(serde_json::json!({
        "id": slice_id,
        "path": format!("{base_path}.{choice_name}"),
        "sliceName": resolved_name,
        "min": 0,
        "max": "1",
        "type": [{ "code": type_name }]
    }));
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
            "numerator": fsh_value_to_fixed(numerator),
            "denominator": fsh_value_to_fixed(denominator),
        }),
        FshValue::Reference { target, display } => {
            let mut reference = serde_json::json!({ "reference": target });
            if let Some(display) = display {
                reference["display"] = serde_json::Value::String(display.clone());
            }
            reference
        }
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
        FshValue::Date(s) | FshValue::DateTime(s) => serde_json::Value::String(s.clone()),
        FshValue::InstanceRef(s) => serde_json::Value::String(s.clone()),
    }
}

fn fsh_value_to_caret(value: &FshValue) -> serde_json::Value {
    if let FshValue::Code {
        system: None,
        code,
        display: None,
    } = value
    {
        return serde_json::Value::String(code.clone());
    }
    fsh_value_to_fixed(value)
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

#[derive(Default)]
struct CaretPathWriter<'a> {
    defs: Option<&'a FhirDefs>,
    current_indices: std::collections::HashMap<String, usize>,
}

impl<'a> CaretPathWriter<'a> {
    fn with_defs(defs: &'a FhirDefs) -> Self {
        Self {
            defs: Some(defs),
            current_indices: std::collections::HashMap::new(),
        }
    }

    fn set(&mut self, root: &mut serde_json::Value, path: &str, value: serde_json::Value) {
        let path = self.resolve_soft_indices(path);
        set_nested_value(root, &path, value);
    }

    fn resolve_soft_indices(&mut self, path: &str) -> String {
        let mut resolved = Vec::new();
        let mut current_type = "StructureDefinition".to_string();
        for segment in path.split('.') {
            let (name, selector) = parse_path_selector(segment);
            let field = self
                .defs
                .and_then(|defs| defs.get_element(&current_type, name));
            let key = if resolved.is_empty() {
                name.to_string()
            } else {
                format!("{}.{}", resolved.join("."), name)
            };
            let resolved_segment = match selector {
                Some("+") => {
                    let index = self.current_indices.get(&key).map_or(0, |index| index + 1);
                    self.current_indices.insert(key, index);
                    format!("{name}[{index}]")
                }
                Some("=") => {
                    let index = self.current_indices.get(&key).copied().unwrap_or(0);
                    self.current_indices.insert(key, index);
                    format!("{name}[{index}]")
                }
                Some(index) if index.parse::<usize>().is_ok() => {
                    let index = index.parse::<usize>().expect("validated numeric index");
                    self.current_indices.insert(key, index);
                    format!("{name}[{index}]")
                }
                _ if field.as_ref().is_some_and(|field| field.max.is_none()) => {
                    self.current_indices.entry(key).or_insert(0);
                    format!("{name}[0]")
                }
                _ => segment.to_string(),
            };
            resolved.push(resolved_segment);
            if let Some(next_type) = field.and_then(|field| field.types.into_iter().next()) {
                current_type = next_type;
            }
        }
        resolved.join(".")
    }
}

fn parse_path_selector(segment: &str) -> (&str, Option<&str>) {
    let Some((name, suffix)) = segment.split_once('[') else {
        return (segment, None);
    };
    (name, suffix.strip_suffix(']'))
}

/// Set a (potentially nested dot-path) key on a JSON object.
fn set_nested_value(obj: &mut serde_json::Value, path: &str, val: serde_json::Value) {
    let parts: Vec<&str> = path.splitn(2, '.').collect();
    let (name, index) = parse_indexed_key(parts[0]);
    if let Some(index) = index {
        if let Some(map) = obj.as_object_mut() {
            let array = map
                .entry(name.to_string())
                .or_insert_with(|| serde_json::json!([]));
            if let Some(array) = array.as_array_mut() {
                while array.len() <= index {
                    array.push(serde_json::json!({}));
                }
                if parts.len() == 1 {
                    array[index] = val;
                } else {
                    set_nested_value(&mut array[index], parts[1], val);
                }
            }
        }
    } else if parts.len() == 1 {
        if let Some(map) = obj.as_object_mut() {
            map.insert(name.to_string(), val);
        }
    } else if let Some(map) = obj.as_object_mut() {
        let child = map
            .entry(name.to_string())
            .or_insert_with(|| serde_json::json!({}));
        set_nested_value(child, parts[1], val);
    }
}

fn parse_indexed_key(key: &str) -> (&str, Option<usize>) {
    let Some((name, suffix)) = key.split_once('[') else {
        return (key, None);
    };
    let index = suffix
        .strip_suffix(']')
        .and_then(|value| value.parse().ok());
    (name, index)
}

/// Inject auto-generated elements for a **simple** extension (no sub-extensions).
///
/// Adds `Extension.extension` (max:0), `Extension.url` (fixedUri), and
/// `Extension.value[x]`. Any user-declared value[x] element is merged.
fn inject_simple_extension_elements(elements: &mut Vec<serde_json::Value>, url: &str) {
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
    let user_value_x: Option<serde_json::Value> = {
        let pos = elements
            .iter()
            .position(|e| e.get("path").and_then(|v| v.as_str()) == Some("Extension.value[x]"));
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
}

/// Inject auto-generated elements for a **complex** extension (has sub-extensions).
///
/// For each named slice, injects `Extension.extension:<slice>.extension` (max:0)
/// and `Extension.extension:<slice>.url` (fixedUri). Also injects top-level
/// `Extension.url` and `Extension.value[x]` (max:0, no direct values allowed).
fn inject_complex_extension_elements(
    elements: &mut Vec<serde_json::Value>,
    url: &str,
    rules: &[Spanned<SdRule>],
) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_nested_values_through_indexed_caret_paths() {
        let mut resource = serde_json::json!({});

        set_nested_value(
            &mut resource,
            "contact[0].telecom[0].system",
            serde_json::json!("url"),
        );
        set_nested_value(
            &mut resource,
            "contact[0].telecom[0].value",
            serde_json::json!("https://example.org"),
        );

        assert_eq!(resource["contact"][0]["telecom"][0]["system"], "url");
        assert_eq!(
            resource["contact"][0]["telecom"][0]["value"],
            "https://example.org"
        );
    }

    #[test]
    fn resolves_soft_indices_across_related_caret_rules() {
        let mut resource = serde_json::json!({});
        let mut writer = CaretPathWriter::default();

        writer.set(
            &mut resource,
            "context[+].type",
            serde_json::json!("element"),
        );
        writer.set(
            &mut resource,
            "context[=].expression",
            serde_json::json!("Claim"),
        );
        writer.set(
            &mut resource,
            "context[+].type",
            serde_json::json!("extension"),
        );
        writer.set(
            &mut resource,
            "context[=].expression",
            serde_json::json!("http://example.org/StructureDefinition/nested"),
        );

        assert_eq!(
            resource["context"],
            serde_json::json!([
                { "type": "element", "expression": "Claim" },
                {
                    "type": "extension",
                    "expression": "http://example.org/StructureDefinition/nested"
                }
            ])
        );
    }

    #[test]
    fn wraps_unindexed_repeating_caret_fields_from_fhir_schema() {
        let defs = FhirDefs::r4();
        let mut resource = serde_json::json!({});
        let mut writer = CaretPathWriter::with_defs(&defs);

        writer.set(
            &mut resource,
            "contact.telecom.system",
            serde_json::json!("url"),
        );
        writer.set(
            &mut resource,
            "contact.telecom.value",
            serde_json::json!("http://example.org/contact"),
        );

        assert_eq!(
            resource["contact"],
            serde_json::json!([{
                "telecom": [{
                    "system": "url",
                    "value": "http://example.org/contact"
                }]
            }])
        );
    }
}
