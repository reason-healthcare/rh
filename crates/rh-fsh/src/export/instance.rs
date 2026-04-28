//! Export FSH Instance to FHIR JSON

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use crate::tank::FshTank;
use crate::FshConfig;
use rh_hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType};

pub fn export_instance(
    inst: &Instance,
    defs: &FhirDefs,
    config: &FshConfig,
    tank: &FshTank,
) -> Result<serde_json::Value, FshError> {
    let instance_of = &inst.metadata.instance_of;

    // Detect if this is an instance of a Logical model — if so, resourceType is the SD URL
    let (resource_type_json, resource_type_for_metadata) =
        if tank.logicals.contains_key(instance_of.as_str()) {
            let url = if let Some(canonical) = &config.canonical {
                format!("{canonical}/StructureDefinition/{instance_of}")
            } else {
                instance_of.clone()
            };
            (url.clone(), url)
        } else {
            (instance_of.clone(), instance_of.clone())
        };

    let is_logical_instance = tank.logicals.contains_key(instance_of.as_str());

    let mut resource = if is_logical_instance {
        // Logical model instances don't always have an id (depends on Characteristics: #can-be-target)
        let logical = tank.logicals.get(instance_of.as_str()).unwrap();
        let has_can_be_target = logical
            .metadata
            .characteristics
            .iter()
            .any(|c| c == "#can-be-target" || c == "can-be-target");
        if has_can_be_target {
            serde_json::json!({
                "resourceType": resource_type_json,
                "id": inst.metadata.name,
            })
        } else {
            serde_json::json!({ "resourceType": resource_type_json })
        }
    } else {
        serde_json::json!({
            "resourceType": resource_type_json,
            "id": inst.metadata.name,
        })
    };

    if let Some(title) = &inst.metadata.title {
        resource["title"] = serde_json::Value::String(title.clone());
    }

    let usage = inst.metadata.usage.as_deref().unwrap_or("");
    if usage == "#definition" || usage == "definition" {
        if let Some(canonical) = &config.canonical {
            let url = format!("{canonical}/{instance_of}/{}", inst.metadata.name);
            resource["url"] = serde_json::Value::String(url);
        }
    }

    for rule in &inst.rules {
        match &rule.value {
            InstanceRule::Assignment(a) => {
                apply_assignment(
                    &mut resource,
                    &a.path,
                    &a.value,
                    defs,
                    &resource_type_for_metadata,
                    tank,
                    config,
                );
            }
            InstanceRule::Insert(_) => {}
        }
    }

    Ok(resource)
}

/// Apply an assignment rule by navigating the FSH path and setting the value
fn apply_assignment(
    root: &mut serde_json::Value,
    path: &FshPath,
    value: &FshValue,
    defs: &FhirDefs,
    resource_type: &str,
    tank: &FshTank,
    config: &FshConfig,
) {
    let json_val = fsh_value_to_json(value, tank, defs, config);
    set_at_path(root, &path.segments, json_val, defs, resource_type);
}

fn fsh_value_to_json(
    value: &FshValue,
    tank: &FshTank,
    defs: &FhirDefs,
    config: &FshConfig,
) -> serde_json::Value {
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
            if system.is_none() && display.is_none() {
                // Simple bare code (e.g., #male) → plain string for scalar FHIR code fields
                return serde_json::Value::String(code.clone());
            }
            if let Some(s) = system {
                // Resolve local CodeSystem name to its ^url if present in the tank
                let resolved_system = resolve_code_system_url(s, tank);
                // Code with system → Coding object (embedded in CodeableConcept by caller if needed)
                let mut obj = serde_json::json!({ "system": resolved_system, "code": code });
                if let Some(d) = display {
                    obj["display"] = serde_json::Value::String(d.clone());
                }
                return obj;
            }
            serde_json::Value::String(code.clone())
        }
        FshValue::Quantity { value, unit } => {
            serde_json::json!({ "value": value, "unit": unit })
        }
        FshValue::Ratio {
            numerator,
            denominator,
        } => {
            serde_json::json!({
                "numerator": fsh_value_to_json(numerator, tank, defs, config),
                "denominator": fsh_value_to_json(denominator, tank, defs, config),
            })
        }
        FshValue::Reference(r) => {
            let resolved = resolve_reference(r, tank, config);
            serde_json::json!({ "reference": resolved })
        }
        FshValue::Canonical(c) => serde_json::Value::String(c.clone()),
        FshValue::Date(s) | FshValue::DateTime(s) => serde_json::Value::String(s.clone()),
        FshValue::InstanceRef(name) => {
            // Embed the referenced inline instance JSON (used for contained[+] = myInstance)
            if let Some(inst) = tank.instances.get(name) {
                export_instance(inst, defs, config, tank).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
    }
}

/// Resolve a local CodeSystem name to its `^url` value if defined in the tank.
fn resolve_code_system_url(system: &str, tank: &FshTank) -> String {
    // If the system already looks like a URL, return as-is
    if system.starts_with("http://") || system.starts_with("https://") || system.starts_with("urn:")
    {
        return system.to_string();
    }
    // Look up by CodeSystem name or id in the tank
    if let Some(cs) = tank.code_systems.get(system) {
        for rule in &cs.caret_rules {
            if rule.value.caret_path == "url" {
                if let FshValue::Str(url) = &rule.value.value {
                    return url.clone();
                }
            }
        }
        // Try to find by id
        if let Some(id) = &cs.metadata.id {
            for (_name, cs2) in &tank.code_systems {
                if cs2.metadata.id.as_deref() == Some(id.as_str()) {
                    for rule in &cs2.caret_rules {
                        if rule.value.caret_path == "url" {
                            if let FshValue::Str(url) = &rule.value.value {
                                return url.clone();
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Try matching by id
        for (_name, cs) in &tank.code_systems {
            if cs.metadata.id.as_deref() == Some(system) {
                for rule in &cs.caret_rules {
                    if rule.value.caret_path == "url" {
                        if let FshValue::Str(url) = &rule.value.value {
                            return url.clone();
                        }
                    }
                }
            }
        }
    }
    system.to_string()
}

/// Resolve a bare instance id to `ResourceType/id` format using the tank.
fn resolve_reference(id: &str, tank: &FshTank, config: &FshConfig) -> String {
    // If already qualified (contains '/') or is a special reference (starts with '#'), return as-is
    if id.contains('/') || id.starts_with('#') {
        return id.to_string();
    }
    // Look up instance in tank to get its resource type
    if let Some(inst) = tank.instances.get(id) {
        let rt = &inst.metadata.instance_of;
        // If the instanceOf is a Logical model, references use the full canonical URL
        if tank.logicals.contains_key(rt.as_str()) {
            if let Some(canonical) = &config.canonical {
                return format!("{canonical}/StructureDefinition/{rt}/{id}");
            }
        }
        return format!("{rt}/{id}");
    }
    id.to_string()
}

/// Returns the inner FHIR type name for a complex/backbone field type, if navigable.
fn field_next_type(ft: &FhirFieldType) -> Option<&str> {
    match ft {
        FhirFieldType::Complex(t) | FhirFieldType::BackboneElement(t) => Some(t),
        _ => None,
    }
}

/// Wrap a Coding-shaped object as a CodeableConcept if the target field requires it.
///
/// A "Coding-shaped" value is `{"code": ..., "system": ...}` (with optional display).
/// When the field type is `CodeableConcept`, FHIR requires `{"coding": [{...}]}`.
fn wrap_codeable_concept_if_needed(
    value: serde_json::Value,
    field_type: Option<&FhirFieldType>,
) -> serde_json::Value {
    if matches!(field_type, Some(FhirFieldType::Complex("CodeableConcept"))) {
        if let serde_json::Value::Object(ref map) = value {
            if map.contains_key("code") || map.contains_key("system") {
                return serde_json::json!({ "coding": [value] });
            }
        }
    }
    value
}

/// Navigate a FHIR path in the JSON tree and set the leaf value.
///
/// Uses `FhirDefs` to determine whether each field has max cardinality `*` (array)
/// or `1` (scalar), matching sushi's JSON output shape without a full snapshot.
fn set_at_path(
    node: &mut serde_json::Value,
    segments: &[FshPathSegment],
    value: serde_json::Value,
    _defs: &FhirDefs,
    current_type: &str,
) {
    if segments.is_empty() {
        *node = value;
        return;
    }

    match &segments[0] {
        FshPathSegment::Name(name) => {
            let fi = get_field_info(current_type, name);
            let is_array = fi.is_some_and(|f| f.max.is_none());
            let next_type = fi
                .and_then(|f| field_next_type(&f.field_type))
                .unwrap_or(current_type);

            if let serde_json::Value::Object(map) = node {
                if segments.len() == 1 {
                    // Wrap Coding-shaped values in CodeableConcept when required by the field type
                    let value = wrap_codeable_concept_if_needed(value, fi.map(|f| &f.field_type));
                    if is_array {
                        let arr = map
                            .entry(name.clone())
                            .or_insert_with(|| serde_json::json!([]));
                        if let serde_json::Value::Array(a) = arr {
                            a.push(value);
                        }
                    } else {
                        map.insert(name.clone(), value);
                    }
                } else {
                    let next_is_index = matches!(segments.get(1), Some(FshPathSegment::Index(_)));
                    if is_array && !next_is_index {
                        // Array field accessed without an explicit index — navigate to last element
                        // (creating the first one if the array is empty).
                        let arr = map
                            .entry(name.clone())
                            .or_insert_with(|| serde_json::json!([]));
                        if let serde_json::Value::Array(a) = arr {
                            if a.is_empty() {
                                a.push(serde_json::json!({}));
                            }
                            let last = a.last_mut().unwrap();
                            set_at_path(last, &segments[1..], value, _defs, next_type);
                        }
                    } else {
                        // Scalar field, or array field with explicit index following.
                        let default = if is_array || next_is_index {
                            serde_json::json!([])
                        } else {
                            serde_json::json!({})
                        };
                        let child = map.entry(name.clone()).or_insert_with(|| default);
                        set_at_path(child, &segments[1..], value, _defs, next_type);
                    }
                }
            }
        }
        FshPathSegment::Index(idx) => {
            if let serde_json::Value::Array(arr) = node {
                let i = *idx as usize;
                while arr.len() <= i {
                    arr.push(serde_json::json!({}));
                }
                if segments.len() == 1 {
                    arr[i] = value;
                } else {
                    set_at_path(&mut arr[i], &segments[1..], value, _defs, current_type);
                }
            }
        }
        FshPathSegment::Slice { element, slice } => {
            if let serde_json::Value::Object(map) = node {
                handle_slice_segment(map, element, slice, segments, value, _defs, current_type);
            }
        }
        FshPathSegment::ChoiceType(element) => {
            if let serde_json::Value::Object(map) = node {
                let key = element.clone();
                if segments.len() == 1 {
                    map.insert(key, value);
                } else {
                    let child = map.entry(key).or_insert_with(|| serde_json::json!({}));
                    set_at_path(child, &segments[1..], value, _defs, current_type);
                }
            }
        }
        FshPathSegment::Extension(url) => {
            if let serde_json::Value::Object(map) = node {
                let exts = map
                    .entry("extension")
                    .or_insert_with(|| serde_json::json!([]));
                if let serde_json::Value::Array(arr) = exts {
                    if segments.len() == 1 {
                        arr.push(serde_json::json!({ "url": url, "valueString": value }));
                    } else {
                        let mut ext_obj = serde_json::json!({ "url": url });
                        set_at_path(&mut ext_obj, &segments[1..], value, _defs, "Extension");
                        arr.push(ext_obj);
                    }
                }
            }
        }
    }
}

/// Handle a `Slice { element, slice }` segment — the three sub-cases:
/// 1. Soft-indexing (`[+]` appends, `[=]` reuses last element)
/// 2. Numeric index (`[0]`, `[1]`, …)
/// 3. Named slice (navigates to the element field by name)
fn handle_slice_segment(
    map: &mut serde_json::Map<String, serde_json::Value>,
    element: &str,
    slice: &str,
    segments: &[FshPathSegment],
    value: serde_json::Value,
    _defs: &FhirDefs,
    current_type: &str,
) {
    // Soft-indexing operators: [+] appends, [=] reuses last
    if slice == "+" || slice == "=" {
        let fi = get_field_info(current_type, element);
        let next_type = fi
            .and_then(|f| field_next_type(&f.field_type))
            .unwrap_or(current_type);
        let element_key = element.to_string();

        let arr_len = {
            let arr = map
                .entry(element_key.clone())
                .or_insert_with(|| serde_json::json!([]));
            if slice == "+" {
                if let serde_json::Value::Array(a) = arr {
                    a.push(serde_json::json!({}));
                }
            }
            if let serde_json::Value::Array(a) = arr {
                a.len()
            } else {
                0
            }
        };

        if arr_len == 0 {
            return;
        }
        let idx = arr_len - 1;

        // Check for FHIR "extension on primitive" pattern.
        let is_primitive = map
            .get(&element_key)
            .and_then(|v| {
                if let serde_json::Value::Array(a) = v {
                    a.get(idx)
                } else {
                    None
                }
            })
            .is_some_and(|v| !v.is_object() && !v.is_null());

        if is_primitive && segments.len() > 1 {
            let shadow_key = format!("_{element_key}");
            let shadow_arr = map
                .entry(shadow_key)
                .or_insert_with(|| serde_json::json!([]));
            if let serde_json::Value::Array(shadow) = shadow_arr {
                while shadow.len() <= idx {
                    shadow.push(serde_json::Value::Null);
                }
                if shadow[idx].is_null() {
                    shadow[idx] = serde_json::json!({});
                }
                set_at_path(&mut shadow[idx], &segments[1..], value, _defs, "Element");
            }
        } else {
            let arr = map.get_mut(&element_key).unwrap();
            if let serde_json::Value::Array(arr) = arr {
                if segments.len() == 1 {
                    arr[idx] = value;
                } else {
                    set_at_path(&mut arr[idx], &segments[1..], value, _defs, next_type);
                }
            }
        }
        return;
    }

    // Numeric index (e.g., extension[0])
    if let Ok(idx) = slice.parse::<usize>() {
        let fi = get_field_info(current_type, element);
        let next_type = fi
            .and_then(|f| field_next_type(&f.field_type))
            .unwrap_or(current_type);
        let arr = map
            .entry(element.to_string())
            .or_insert_with(|| serde_json::json!([]));
        if let serde_json::Value::Array(arr) = arr {
            while arr.len() <= idx {
                arr.push(serde_json::json!({}));
            }
            if segments.len() == 1 {
                arr[idx] = value;
            } else {
                set_at_path(&mut arr[idx], &segments[1..], value, _defs, next_type);
            }
        }
        return;
    }

    // Regular named slice: navigate to the element field
    let fi = get_field_info(current_type, element);
    let next_type = fi
        .and_then(|f| field_next_type(&f.field_type))
        .unwrap_or(current_type);
    let key = element.to_string();
    if segments.len() == 1 {
        map.insert(key, value);
    } else {
        let child = map.entry(key).or_insert_with(|| serde_json::json!({}));
        set_at_path(child, &segments[1..], value, _defs, next_type);
    }
}
