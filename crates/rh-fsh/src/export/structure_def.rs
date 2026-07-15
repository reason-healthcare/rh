//! Export FSH Profile/Extension/Logical/Resource to FHIR StructureDefinition JSON

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use crate::tank::FshTank;
use crate::DefinitionIndex;
use rh_hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType};
use std::sync::Arc;

struct StructureExportContext<'a> {
    defs: Arc<FhirDefs>,
    config: &'a crate::FshConfig,
    tank: &'a FshTank,
    definition_index: &'a DefinitionIndex,
}

pub fn export_profile(
    profile: &Profile,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    tank: &FshTank,
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
            tank,
            definition_index,
        },
    )
}

pub fn export_extension(
    ext: &Extension,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    tank: &FshTank,
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
            tank,
            definition_index,
        },
    )
}

pub fn export_logical(
    logical: &Logical,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    tank: &FshTank,
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
            tank,
            definition_index,
        },
    )
}

pub fn export_resource_def(
    res: &ResourceDef,
    defs: Arc<FhirDefs>,
    config: &crate::FshConfig,
    tank: &FshTank,
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
            tank,
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
        tank,
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
            } else if kind == "logical" {
                "http://hl7.org/fhir/StructureDefinition/Base".to_string()
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
                let elem = differential_element(&mut elements, &id, &path);
                let base_cardinality = base_cardinality(fhir_type, &r.path);
                if let Some(min) = r.min {
                    if base_cardinality.is_none_or(|(base_min, _)| min != base_min) {
                        elem["min"] = serde_json::json!(min);
                    }
                }
                if let Some(max) = &r.max {
                    if base_cardinality.is_none_or(|(_, base_max)| {
                        max != &base_max.map_or_else(|| "*".to_string(), |max| max.to_string())
                    }) {
                        elem["max"] = serde_json::Value::String(max.clone());
                    }
                } else if base_cardinality.is_none_or(|(_, base_max)| base_max.is_some()) {
                    elem["max"] = serde_json::Value::String("*".to_string());
                }
                apply_flags_to_element(elem, &r.flags);
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
                        let elem = differential_element(&mut elements, &elem_id, &elem_path);
                        apply_flags_to_element(elem, &r.flags);
                    }
                }
            }
            SdRule::Binding(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let strength = r.strength.as_deref().unwrap_or("required");
                let elem = differential_element(&mut elements, &id, &path);
                elem["binding"] = serde_json::json!({
                    "strength": strength,
                    "valueSet": super::value_set::resolve_value_set_url(&r.value_set, tank, config),
                });
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
                let elem = differential_element(&mut elements, &id, &path);
                if let FshValue::Code {
                    system: None, code, ..
                } = &r.value
                {
                    let key = if r.exactly {
                        "fixedCode"
                    } else {
                        "patternCode"
                    };
                    elem[key] = serde_json::Value::String(code.clone());
                } else {
                    elem["fixed"] = fsh_value_to_fixed(&r.value);
                }
            }
            SdRule::Only(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|type_reference| {
                        export_type_reference(type_reference, definition_index, &defs)
                    })
                    .collect();
                let elem = differential_element(&mut elements, &id, &path);
                elem["type"] = serde_json::Value::Array(type_list);
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
                    .map(|name| invariant_constraint(tank.invariants.get(name), name, &url))
                    .collect();
                let element = differential_element(&mut elements, &id, &path);
                let constraint = element
                    .as_object_mut()
                    .expect("differential element is an object")
                    .entry("constraint")
                    .or_insert_with(|| serde_json::Value::Array(Vec::new()));
                if let Some(values) = constraint.as_array_mut() {
                    values.extend(constraints);
                }
            }
            SdRule::CaretValue(r) => {
                if r.path.is_some() {
                    // Element-level caret rule: add to differential
                    let (path, id) = r
                        .path
                        .as_ref()
                        .map(|p| (make_fhir_path(p), make_fhir_id(p)))
                        .unwrap();
                    let elem = differential_element(&mut elements, &id, &path);
                    set_differential_caret_value(elem, &r.caret_path, fsh_value_to_fixed(&r.value));
                }
                // Root-level caret rules (path = None) are applied to the SD object below
            }
            SdRule::Contains(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let elem = differential_element(&mut elements, &id, &path);
                if elem.get("slicing").is_none() {
                    elem["slicing"] = serde_json::json!({ "rules": "open" });
                }
                for item in &r.items {
                    let slice_id = format!("{id}:{}", item.name);
                    let slice = differential_element(&mut elements, &slice_id, &path);
                    slice["sliceName"] = serde_json::Value::String(item.name.clone());
                    if let Some(min) = item.min {
                        slice["min"] = serde_json::json!(min);
                    }
                    if let Some(max) = &item.max {
                        slice["max"] = serde_json::Value::String(max.clone());
                    }
                }
            }
            SdRule::AddElement(r) => {
                let path = make_fhir_path(&r.path);
                let id = make_fhir_id(&r.path);
                let type_list: Vec<serde_json::Value> = r
                    .types
                    .iter()
                    .map(|t| serde_json::json!({ "code": t }))
                    .collect();
                let elem = differential_element(&mut elements, &id, &path);
                elem["min"] = serde_json::json!(r.min);
                elem["max"] = serde_json::Value::String(r.max.clone());
                elem["type"] = serde_json::Value::Array(type_list);
                elem["short"] = serde_json::Value::String(r.short.clone());
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

fn differential_element<'a>(
    elements: &'a mut Vec<serde_json::Value>,
    id: &str,
    path: &str,
) -> &'a mut serde_json::Value {
    if let Some(index) = elements
        .iter()
        .position(|element| element.get("id").and_then(|value| value.as_str()) == Some(id))
    {
        return &mut elements[index];
    }
    elements.push(serde_json::json!({ "id": id, "path": path }));
    elements.last_mut().expect("element was appended")
}

fn base_cardinality(root_type: &str, path: &FshPath) -> Option<(u32, Option<u32>)> {
    let mut current_type = root_type;
    let mut field = None;
    for segment in &path.segments {
        let name = match segment {
            FshPathSegment::Name(name) => name.as_str(),
            FshPathSegment::Slice { element, .. } => element.as_str(),
            FshPathSegment::ChoiceType(element) => element.as_str(),
            FshPathSegment::Index(_) | FshPathSegment::Extension(_) => continue,
        };
        let info = get_field_info(current_type, name).or_else(|| {
            normalize_choice_type_name(name)
                .as_deref()
                .and_then(|choice| get_field_info(current_type, choice))
        })?;
        current_type = match &info.field_type {
            FhirFieldType::Complex(type_name) | FhirFieldType::BackboneElement(type_name) => {
                type_name
            }
            _ => current_type,
        };
        field = Some(info);
    }
    field.map(|info| (info.min, info.max))
}

fn invariant_constraint(
    invariant: Option<&Invariant>,
    key: &str,
    source: &str,
) -> serde_json::Value {
    let mut constraint = serde_json::json!({ "key": key, "source": source });
    let Some(invariant) = invariant else {
        return constraint;
    };
    constraint["severity"] = serde_json::Value::String(
        invariant
            .severity
            .as_deref()
            .unwrap_or("error")
            .trim_start_matches('#')
            .to_string(),
    );
    if let Some(human) = &invariant.description {
        constraint["human"] = serde_json::Value::String(human.clone());
    }
    if let Some(expression) = &invariant.expression {
        constraint["expression"] = serde_json::Value::String(expression.clone());
    }
    if let Some(xpath) = &invariant.xpath {
        constraint["xpath"] = serde_json::Value::String(xpath.clone());
    }
    constraint
}

fn export_type_reference(
    type_reference: &str,
    definition_index: &DefinitionIndex,
    defs: &FhirDefs,
) -> serde_json::Value {
    let mut type_entry = serde_json::Map::new();
    let mut profile_key = None;
    let code = if let Some(targets) = type_reference
        .strip_prefix("Reference(")
        .and_then(|value| value.strip_suffix(')'))
    {
        profile_key = Some(targets);
        "Reference".to_string()
    } else {
        definition_index
            .resolve_base_type(type_reference, defs)
            .unwrap_or_else(|| type_reference.to_string())
    };
    type_entry.insert("code".to_string(), serde_json::Value::String(code));

    let profiles = profile_key
        .into_iter()
        .flat_map(|targets| targets.split(" or "))
        .map(|target| resolve_profile_url(target, definition_index, defs))
        .collect::<Vec<_>>();
    if !profiles.is_empty() {
        type_entry.insert("targetProfile".to_string(), serde_json::json!(profiles));
    } else if let Some(definition) = definition_index.lookup(type_reference) {
        if let Some(url) = &definition.url {
            type_entry.insert("profile".to_string(), serde_json::json!([url]));
        }
    }
    serde_json::Value::Object(type_entry)
}

fn resolve_profile_url(
    reference: &str,
    definition_index: &DefinitionIndex,
    defs: &FhirDefs,
) -> String {
    if reference.starts_with("http://")
        || reference.starts_with("https://")
        || reference.starts_with("urn:")
    {
        return reference.to_string();
    }
    if let Some(url) = definition_index
        .lookup(reference)
        .and_then(|definition| definition.url.as_ref())
    {
        return url.clone();
    }
    defs.get_sd(reference)
        .map(|definition| definition.url.clone())
        .unwrap_or_else(|| reference.to_string())
}

fn set_differential_caret_value(
    element: &mut serde_json::Value,
    path: &str,
    value: serde_json::Value,
) {
    if let Some(field) = path.strip_prefix("slicing.discriminator.") {
        if !element["slicing"].is_object() {
            element["slicing"] = serde_json::json!({});
        }
        let slicing = element["slicing"]
            .as_object_mut()
            .expect("slicing is an object");
        let discriminators = slicing
            .entry("discriminator")
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));
        if let Some(discriminators) = discriminators.as_array_mut() {
            if discriminators.is_empty() {
                discriminators.push(serde_json::json!({}));
            }
            discriminators[0][field] = value;
        }
        return;
    }
    if let Some(path) = path.strip_prefix("slicing.") {
        let slicing = element
            .as_object_mut()
            .expect("differential element is an object")
            .entry("slicing")
            .or_insert_with(|| serde_json::json!({}));
        set_nested_value(slicing, path, value);
        return;
    }
    if let Some(path) = path.strip_prefix("type.") {
        let types = element
            .as_object_mut()
            .expect("differential element is an object")
            .entry("type")
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));
        if let Some(types) = types.as_array_mut() {
            if types.is_empty() {
                types.push(serde_json::json!({}));
            }
            set_nested_value(&mut types[0], path, value);
        }
        return;
    }
    set_nested_value(element, path, value);
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
            if system.is_none() {
                return serde_json::Value::String(code.clone());
            }
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
                "value": fhir_decimal(*value),
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

fn fhir_decimal(value: f64) -> serde_json::Value {
    if value.fract() == 0.0 && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
        return serde_json::Value::Number(serde_json::Number::from(value as i64));
    }
    serde_json::Number::from_f64(value)
        .map(serde_json::Value::Number)
        .unwrap_or(serde_json::Value::Null)
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

    #[test]
    fn emits_complete_root_obeys_constraint() {
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config: crate::FshConfig {
                canonical: Some("http://example.org/fhir".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(
            r#"
Invariant: patient-name
Description: "A patient needs a family name."
Expression: "name.family.exists()"
Severity: #warning
XPath: "f:name/f:family"

Profile: NamedPatient
Parent: Patient
* obeys patient-name
"#,
            "obeys-constraint.fsh",
        )
        .expect("FSH compiles");
        let structure_definition = package
            .resources
            .iter()
            .find(|resource| resource["name"] == "NamedPatient")
            .expect("StructureDefinition exists");
        let root = structure_definition["differential"]["element"]
            .as_array()
            .expect("differential elements")
            .iter()
            .find(|element| element["id"] == "Patient")
            .expect("root differential element");

        assert_eq!(
            root["constraint"][0],
            serde_json::json!({
                "key": "patient-name",
                "source": "http://example.org/fhir/StructureDefinition/NamedPatient",
                "severity": "warning",
                "human": "A patient needs a family name.",
                "expression": "name.family.exists()",
                "xpath": "f:name/f:family",
            })
        );
    }

    #[test]
    fn exports_dtr_bundle_logical_and_extension_regressions() {
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config: crate::FshConfig {
                canonical: Some("http://example.org/dtr".to_string()),
                status: Some("active".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(
            r#"
Invariant: dtrb-1
Description: "The first resource in the bundle is a Questionnaire."
Expression: "entry.first().resource.is(Questionnaire)"
Severity: #error

Profile: DTRBaseQuestionnaire
Parent: Questionnaire
Id: dtr-base-questionnaire

Profile: DTRQuestionnaireResponse
Parent: QuestionnaireResponse
Id: dtr-questionnaireresponse

Profile: DTRQuestionnairePackageBundle
Parent: Bundle
Id: DTR-QPackageBundle
* obeys dtrb-1
* type = #collection
* entry 1..*
* entry ^slicing.discriminator.type = #type
* entry ^slicing.discriminator.path = "resource"
* entry ^slicing.rules = #closed
* entry contains questionnaire 1..1 MS
* entry[questionnaire].resource 1..1
* entry[questionnaire].resource only DTRBaseQuestionnaire

Profile: DTRQuestionnaireResponseBundle
Parent: Bundle
Id: DTR-QRBundle
* entry 1..*
* entry contains questionnaireResponse 1..1 MS
* entry[questionnaireResponse].resource 1..1
* entry[questionnaireResponse].resource only DTRQuestionnaireResponse

Logical: DTRMetricData
Id: DTRMetricData
* action 1..* BackboneElement "Actions" "Actions in a DTR session."

Extension: DTRQuestionnaireContext
Id: dtr-questionnaire-context
* ^context[0].type = #element
* ^context[=].expression = "Questionnaire"
* value[x] only boolean
"#,
            "dtr-structure-definitions.fsh",
        )
        .expect("FSH compiles");
        let find = |id: &str| {
            package
                .resources
                .iter()
                .find(|resource| resource["id"] == id)
                .expect("StructureDefinition exists")
        };
        let package_bundle = find("DTR-QPackageBundle");
        let package_root = package_bundle["differential"]["element"]
            .as_array()
            .expect("differential elements")
            .iter()
            .find(|element| element["id"] == "Bundle")
            .expect("Bundle root");
        assert_eq!(
            package_root["constraint"][0]["source"],
            "http://example.org/dtr/StructureDefinition/DTR-QPackageBundle"
        );
        assert_eq!(
            package_bundle["differential"]["element"]
                .as_array()
                .expect("differential elements")
                .iter()
                .find(|element| element["id"] == "Bundle.type")
                .expect("Bundle type")["patternCode"],
            "collection"
        );
        let bundle_entry = package_bundle["differential"]["element"]
            .as_array()
            .expect("differential elements")
            .iter()
            .find(|element| element["id"] == "Bundle.entry")
            .expect("Bundle entry");
        assert_eq!(bundle_entry["slicing"]["rules"], "closed");
        assert_eq!(
            bundle_entry["slicing"]["discriminator"],
            serde_json::json!([{ "type": "type", "path": "resource" }])
        );
        let questionnaire_resource = package_bundle["differential"]["element"]
            .as_array()
            .expect("differential elements")
            .iter()
            .find(|element| element["id"] == "Bundle.entry:questionnaire.resource")
            .expect("questionnaire slice resource");
        assert_eq!(questionnaire_resource["type"][0]["code"], "Questionnaire");
        assert_eq!(
            questionnaire_resource["type"][0]["profile"][0],
            "http://example.org/dtr/StructureDefinition/dtr-base-questionnaire"
        );

        let response_bundle = find("DTR-QRBundle");
        assert_eq!(
            response_bundle["baseDefinition"],
            "http://hl7.org/fhir/StructureDefinition/Bundle"
        );
        assert_eq!(response_bundle["derivation"], "constraint");

        let logical = find("DTRMetricData");
        assert_eq!(
            logical["baseDefinition"],
            "http://hl7.org/fhir/StructureDefinition/Base"
        );
        assert_eq!(logical["derivation"], "specialization");
        assert_eq!(
            logical["type"],
            "http://example.org/dtr/StructureDefinition/DTRMetricData"
        );

        let extension = find("dtr-questionnaire-context");
        assert_eq!(
            extension["context"],
            serde_json::json!([{ "type": "element", "expression": "Questionnaire" }])
        );
    }
}
