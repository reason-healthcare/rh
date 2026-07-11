//! Export FSH Instance to FHIR JSON

use crate::definition_index::DefinitionIndex;
use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use crate::tank::FshTank;
use crate::FshConfig;
use rh_hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType, FhirPrimitiveType};
use std::collections::{HashMap, HashSet};

struct InstanceExportContext<'a> {
    defs: &'a FhirDefs,
    config: &'a FshConfig,
    tank: &'a FshTank,
    definition_index: &'a DefinitionIndex,
}

struct InstanceShapeContext {
    extension_slice_urls: HashMap<String, String>,
}

pub fn export_instance(
    inst: &Instance,
    defs: &FhirDefs,
    config: &FshConfig,
    tank: &FshTank,
    definition_index: &DefinitionIndex,
) -> Result<serde_json::Value, FshError> {
    let context = InstanceExportContext {
        defs,
        config,
        tank,
        definition_index,
    };
    export_instance_with_context(inst, &context)
}

fn export_instance_with_context(
    inst: &Instance,
    context: &InstanceExportContext<'_>,
) -> Result<serde_json::Value, FshError> {
    let instance_of = &inst.metadata.instance_of;

    let is_logical_instance = context.tank.logicals.contains_key(instance_of.as_str());
    let resource_type_json = if is_logical_instance {
        if let Some(canonical) = &context.config.canonical {
            format!("{canonical}/StructureDefinition/{instance_of}")
        } else {
            instance_of.clone()
        }
    } else {
        resolve_instance_resource_type(instance_of, context.defs, context.definition_index)
            .unwrap_or_else(|| instance_of.clone())
    };
    let resource_type_for_metadata = resource_type_json.clone();

    let mut resource = if is_logical_instance {
        // Logical model instances don't always have an id (depends on Characteristics: #can-be-target)
        let logical = context.tank.logicals.get(instance_of.as_str()).unwrap();
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
        if get_field_info(&resource_type_for_metadata, "title").is_some()
            || resource_type_for_metadata == "ActorDefinition"
        {
            resource["title"] = serde_json::Value::String(title.clone());
        }
    }

    let usage = inst.metadata.usage.as_deref().unwrap_or("");
    if usage == "#definition" || usage == "definition" {
        if let Some(canonical) = &context.config.canonical {
            let url = format!("{canonical}/{instance_of}/{}", inst.metadata.name);
            resource["url"] = serde_json::Value::String(url);
        }
    }

    let mut path_contexts: Vec<(usize, FshPath)> = Vec::new();
    let shape_context = InstanceShapeContext {
        extension_slice_urls: extension_slice_urls(instance_of, context),
    };
    apply_local_profile_defaults(
        &mut resource,
        instance_of,
        &inst.rules,
        &resource_type_for_metadata,
        context,
        &shape_context,
    );
    for rule in &inst.rules {
        let indent = rule.location.column;
        while path_contexts
            .last()
            .is_some_and(|(context_indent, _)| *context_indent >= indent)
        {
            path_contexts.pop();
        }
        match &rule.value {
            InstanceRule::Assignment(a) => {
                let path = qualify_instance_path(&path_contexts, indent, &a.path);
                apply_assignment(
                    &mut resource,
                    &path,
                    &a.value,
                    &resource_type_for_metadata,
                    context,
                    &shape_context,
                );
            }
            InstanceRule::Insert(_) => {}
            InstanceRule::Path(path_rule) => {
                let path = qualify_instance_path(&path_contexts, indent, &path_rule.path);
                path_contexts.push((indent, path));
            }
        }
    }
    remove_export_markers(&mut resource);

    Ok(resource)
}

fn apply_local_profile_defaults(
    resource: &mut serde_json::Value,
    instance_of: &str,
    instance_rules: &[Spanned<InstanceRule>],
    resource_type: &str,
    context: &InstanceExportContext<'_>,
    shape_context: &InstanceShapeContext,
) {
    let mut profiles = Vec::new();
    let mut dependency_profiles = Vec::new();
    let mut current = Some(instance_of.to_string());
    let mut seen = HashSet::new();
    while let Some(profile_name) = current {
        if !seen.insert(profile_name.clone()) {
            break;
        }
        if let Some(profile) = context.tank.profiles.get(&profile_name) {
            profiles.push(profile);
            current = profile.metadata.parent.clone();
        } else if let Some(profile) = context.definition_index.lookup(&profile_name) {
            dependency_profiles.push(profile);
            current = profile
                .parent
                .clone()
                .or_else(|| profile.base_definition.clone());
        } else {
            break;
        }
    }

    let mut assigned_paths = Vec::new();
    for profile in &profiles {
        for rule in &profile.rules {
            if let SdRule::Assignment(assignment) = &rule.value {
                assigned_paths.push(assignment.path.segments.clone());
            }
        }
    }
    let mut instance_assigned_paths = Vec::new();
    for rule in instance_rules {
        if let InstanceRule::Assignment(assignment) = &rule.value {
            instance_assigned_paths.push(assignment.path.segments.clone());
        }
    }
    assigned_paths.extend(instance_assigned_paths.iter().cloned());

    let mut dependency_fixed_values = dependency_profiles
        .into_iter()
        .rev()
        .flat_map(|profile| profile.fixed_values.iter())
        .collect::<Vec<_>>();
    dependency_fixed_values
        .sort_by_key(|fixed_value| dependency_slice_order(&fixed_value.path, &assigned_paths));
    for fixed_value in dependency_fixed_values {
        let segments = dependency_fixed_path(&fixed_value.path);
        let is_overridden = assigned_paths
            .iter()
            .any(|assigned| paths_override(assigned, &segments));
        let slice_is_used =
            fixed_value.required || dependency_slice_is_used(&segments, &assigned_paths);
        if !segments.is_empty() && !is_overridden && slice_is_used {
            set_at_path(
                resource,
                &segments,
                fixed_value.value.clone(),
                shape_context,
                resource_type,
            );
        }
    }

    for profile in profiles.into_iter().rev() {
        materialize_required_slices(resource, profile, shape_context, resource_type);
        let mut path_contexts: Vec<(usize, FshPath)> = Vec::new();
        for rule in &profile.rules {
            let indent = rule.location.column;
            while path_contexts
                .last()
                .is_some_and(|(context_indent, _)| *context_indent >= indent)
            {
                path_contexts.pop();
            }
            match &rule.value {
                SdRule::Assignment(assignment) => {
                    let path = qualify_instance_path(&path_contexts, indent, &assignment.path);
                    if local_profile_path_is_used(
                        &path.segments,
                        &instance_assigned_paths,
                        profile,
                        resource_type,
                    ) {
                        apply_assignment(
                            resource,
                            &path,
                            &assignment.value,
                            resource_type,
                            context,
                            shape_context,
                        );
                    }
                }
                SdRule::Path(path_rule) => {
                    let path = qualify_instance_path(&path_contexts, indent, &path_rule.path);
                    path_contexts.push((indent, path));
                }
                _ => {}
            }
        }
    }
}

fn materialize_required_slices(
    resource: &mut serde_json::Value,
    profile: &Profile,
    shape_context: &InstanceShapeContext,
    resource_type: &str,
) {
    for rule in &profile.rules {
        let SdRule::Contains(contains) = &rule.value else {
            continue;
        };
        let [FshPathSegment::Name(element)] = contains.path.segments.as_slice() else {
            continue;
        };
        for item in &contains.items {
            if item.min.is_none_or(|min| min == 0) {
                continue;
            }
            let segments = [FshPathSegment::Slice {
                element: element.clone(),
                slice: item.alias.clone().unwrap_or_else(|| item.name.clone()),
            }];
            set_at_path(
                resource,
                &segments,
                serde_json::json!({}),
                shape_context,
                resource_type,
            );
        }
    }
}

fn local_profile_path_is_used(
    path: &[FshPathSegment],
    assigned_paths: &[Vec<FshPathSegment>],
    profile: &Profile,
    resource_type: &str,
) -> bool {
    if let Some((slice_index, element, slice)) =
        path.iter()
            .enumerate()
            .find_map(|(index, segment)| match segment {
                FshPathSegment::Slice { element, slice } => Some((index, element, slice)),
                _ => None,
            })
    {
        if slice_index + 1 == path.len() {
            return true;
        };
        if assigned_paths.iter().any(|assigned| {
            assigned.iter().any(|segment| {
                matches!(
                    segment,
                    FshPathSegment::Slice {
                        element: assigned_element,
                        slice: assigned_slice
                    } if assigned_element == element && assigned_slice == slice
                )
            })
        }) {
            return true;
        }
        return profile.rules.iter().any(|rule| {
            let SdRule::Contains(contains) = &rule.value else {
                return false;
            };
            let contains_element = contains.path.segments.last().and_then(path_segment_name);
            contains_element == Some(element.as_str())
                && contains.items.iter().any(|item| {
                    item.min.is_some_and(|min| min > 0)
                        && item.alias.as_deref().unwrap_or(&item.name) == slice
                })
        });
    }

    if path.len() == 1 {
        return true;
    }
    let root = path.first().and_then(path_segment_name);
    if assigned_paths
        .iter()
        .any(|assigned| assigned.first().and_then(path_segment_name) == root)
    {
        return true;
    }
    if root
        .is_some_and(|root| path_field_info(resource_type, root).is_some_and(|field| field.min > 0))
    {
        return true;
    }
    profile.rules.iter().any(|rule| {
        let SdRule::Card(card) = &rule.value else {
            return false;
        };
        card.min.is_some_and(|min| min > 0)
            && card.path.segments.first().and_then(path_segment_name) == root
    })
}

fn path_segment_name(segment: &FshPathSegment) -> Option<&str> {
    match segment {
        FshPathSegment::Name(name)
        | FshPathSegment::ChoiceType(name)
        | FshPathSegment::Slice { element: name, .. } => Some(name),
        FshPathSegment::Index(_) | FshPathSegment::Extension(_) => None,
    }
}

fn dependency_fixed_path(path: &str) -> Vec<FshPathSegment> {
    path.split('.')
        .skip(1)
        .map(|segment| {
            if let Some((element, slice)) = segment.split_once(':') {
                FshPathSegment::Slice {
                    element: element.to_string(),
                    slice: slice.to_string(),
                }
            } else {
                FshPathSegment::Name(segment.to_string())
            }
        })
        .collect()
}

fn paths_override(assigned: &[FshPathSegment], fixed: &[FshPathSegment]) -> bool {
    assigned.len() <= fixed.len()
        && assigned
            .iter()
            .zip(fixed)
            .all(|(assigned, fixed)| path_segments_match(assigned, fixed))
}

fn path_segments_match(left: &FshPathSegment, right: &FshPathSegment) -> bool {
    match (left, right) {
        (
            FshPathSegment::Slice {
                element: left_element,
                slice: left_slice,
            },
            FshPathSegment::Slice {
                element: right_element,
                slice: right_slice,
            },
        ) => left_element == right_element && left_slice == right_slice,
        _ => path_elements_match(path_segment_name(left), path_segment_name(right)),
    }
}

fn path_elements_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left, right) else {
        return false;
    };
    left == right || choice_element_matches(left, right) || choice_element_matches(right, left)
}

fn choice_element_matches(concrete: &str, choice: &str) -> bool {
    let Some(base) = choice.strip_suffix("[x]") else {
        return false;
    };
    concrete
        .strip_prefix(base)
        .and_then(|suffix| suffix.chars().next())
        .is_some_and(char::is_uppercase)
}

fn dependency_slice_is_used(
    fixed: &[FshPathSegment],
    assigned_paths: &[Vec<FshPathSegment>],
) -> bool {
    let Some((fixed_element, fixed_slice)) = fixed.iter().find_map(|segment| match segment {
        FshPathSegment::Slice { element, slice } => Some((element, slice)),
        _ => None,
    }) else {
        if fixed.len() == 1 {
            return true;
        }
        let fixed_root = fixed.first().and_then(path_segment_name);
        return assigned_paths
            .iter()
            .any(|assigned| assigned.first().and_then(path_segment_name) == fixed_root);
    };
    assigned_paths.iter().any(|assigned| {
        assigned.iter().any(|segment| {
            matches!(
                segment,
                FshPathSegment::Slice { element, slice }
                    if element == fixed_element && slice == fixed_slice
            )
        })
    })
}

fn dependency_slice_order(path: &str, assigned_paths: &[Vec<FshPathSegment>]) -> usize {
    let fixed = dependency_fixed_path(path);
    let Some((fixed_element, fixed_slice)) = fixed.iter().find_map(|segment| match segment {
        FshPathSegment::Slice { element, slice } => Some((element, slice)),
        _ => None,
    }) else {
        return 0;
    };
    assigned_paths
        .iter()
        .position(|assigned| {
            assigned.iter().any(|segment| {
                matches!(
                    segment,
                    FshPathSegment::Slice { element, slice }
                        if element == fixed_element && slice == fixed_slice
                )
            })
        })
        .unwrap_or(usize::MAX)
}

fn extension_slice_urls(
    instance_of: &str,
    context: &InstanceExportContext<'_>,
) -> HashMap<String, String> {
    let mut urls = HashMap::new();
    let mut current = Some(instance_of.to_string());
    let mut seen = HashSet::new();
    while let Some(profile_name) = current {
        if !seen.insert(profile_name.clone()) {
            break;
        }
        let Some(profile) = context.tank.profiles.get(&profile_name) else {
            break;
        };
        collect_extension_slice_urls(&profile.rules, context, &mut urls);
        current = profile.metadata.parent.clone();
    }
    for profile in context.tank.profiles.values() {
        collect_extension_slice_urls(&profile.rules, context, &mut urls);
    }
    for extension in context.tank.extensions.values() {
        collect_extension_slice_urls(&extension.rules, context, &mut urls);
    }
    for definition in context.definition_index.definitions() {
        if definition.type_.as_deref() != Some("Extension") {
            continue;
        }
        let Some(url) = &definition.url else {
            continue;
        };
        for candidate in [definition.name.as_deref(), definition.id.as_deref()]
            .into_iter()
            .flatten()
        {
            for key in extension_lookup_keys(candidate) {
                urls.entry(key).or_insert_with(|| url.clone());
            }
        }
    }
    urls
}

fn extension_lookup_keys(value: &str) -> Vec<String> {
    let normalized: String = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    let mut values = vec![value.to_string(), format!("normalized:{normalized}")];
    let without_extension = normalized.strip_suffix("extension").unwrap_or(&normalized);
    for prefix in ["uscore", "mcode", "davinci", "crd", "dtr", "pas", "ips"] {
        if let Some(short) = without_extension.strip_prefix(prefix) {
            values.push(format!("normalized:{short}"));
        }
    }
    values
}

fn collect_extension_slice_urls(
    rules: &[Spanned<SdRule>],
    context: &InstanceExportContext<'_>,
    urls: &mut HashMap<String, String>,
) {
    for rule in rules {
        let SdRule::Contains(contains) = &rule.value else {
            continue;
        };
        if !matches!(
            contains.path.segments.last(),
            Some(FshPathSegment::Name(name)) if name == "extension"
        ) {
            continue;
        }
        for item in &contains.items {
            let slice_name = item.alias.as_deref().unwrap_or(&item.name);
            if let Some(url) = context
                .definition_index
                .lookup(&item.name)
                .and_then(|definition| definition.url.clone())
            {
                urls.entry(slice_name.to_string()).or_insert(url);
            }
        }
    }
}

const SLICE_MARKER: &str = "__rh_fsh_slice";

fn remove_export_markers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.remove(SLICE_MARKER);
            for child in map.values_mut() {
                remove_export_markers(child);
            }
            map.retain(
                |_, child| !matches!(child, serde_json::Value::Array(values) if values.is_empty()),
            );
        }
        serde_json::Value::Array(values) => {
            values.retain(|value| {
                !matches!(value, serde_json::Value::Object(object) if object.len() == 1 && object.contains_key(SLICE_MARKER))
            });
            for child in values {
                remove_export_markers(child);
            }
        }
        _ => {}
    }
}

fn qualify_instance_path(contexts: &[(usize, FshPath)], indent: usize, path: &FshPath) -> FshPath {
    let Some((_, parent)) = contexts
        .iter()
        .rev()
        .find(|(context_indent, _)| *context_indent < indent)
    else {
        return path.clone();
    };
    let mut segments = parent.segments.clone();
    segments.extend(path.segments.clone());
    FshPath { segments }
}

/// Apply an assignment rule by navigating the FSH path and setting the value
fn apply_assignment(
    root: &mut serde_json::Value,
    path: &FshPath,
    value: &FshValue,
    resource_type: &str,
    context: &InstanceExportContext<'_>,
    shape_context: &InstanceShapeContext,
) {
    let json_val = fsh_value_to_json(value, context);
    set_at_path(root, &path.segments, json_val, shape_context, resource_type);
}

fn fsh_value_to_json(value: &FshValue, context: &InstanceExportContext<'_>) -> serde_json::Value {
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
                let resolved_system = resolve_code_system_url(s, context.tank, context.config);
                // Code with system → Coding object (embedded in CodeableConcept by caller if needed)
                let mut obj = serde_json::json!({ "system": resolved_system, "code": code });
                if let Some(d) = display {
                    obj["display"] = serde_json::Value::String(d.clone());
                }
                return obj;
            }
            serde_json::Value::String(code.clone())
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
        } => {
            serde_json::json!({
                "numerator": fsh_value_to_json(numerator, context),
                "denominator": fsh_value_to_json(denominator, context),
            })
        }
        FshValue::Reference { target, display } => {
            let resolved = resolve_reference(
                target,
                context.tank,
                context.defs,
                context.config,
                context.definition_index,
            );
            let mut reference = serde_json::json!({ "reference": resolved });
            if let Some(display) = display {
                reference["display"] = serde_json::Value::String(display.clone());
            }
            reference
        }
        FshValue::Canonical(c) => serde_json::Value::String(resolve_canonical(c, context)),
        FshValue::Date(s) | FshValue::DateTime(s) => serde_json::Value::String(s.clone()),
        FshValue::InstanceRef(name) => {
            // Embed the referenced inline instance JSON (used for contained[+] = myInstance)
            if let Some(inst) = context.tank.instances.get(name) {
                let mut value =
                    export_instance_with_context(inst, context).unwrap_or(serde_json::Value::Null);
                if context
                    .defs
                    .get_sd(&inst.metadata.instance_of)
                    .is_some_and(|definition| definition.kind != "resource")
                {
                    if let serde_json::Value::Object(object) = &mut value {
                        object.remove("resourceType");
                        object.remove("id");
                    }
                }
                value
            } else {
                serde_json::Value::Null
            }
        }
    }
}

/// Resolve a local CodeSystem name to its `^url` value if defined in the tank.
fn resolve_code_system_url(system: &str, tank: &FshTank, config: &FshConfig) -> String {
    // If the system already looks like a URL, return as-is
    if system.starts_with("http://") || system.starts_with("https://") || system.starts_with("urn:")
    {
        return system
            .split_once('|')
            .map_or(system, |(url, _)| url)
            .to_string();
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
        if let Some(canonical) = &config.canonical {
            let id = cs.metadata.id.as_deref().unwrap_or(system);
            return format!("{}/CodeSystem/{id}", canonical.trim_end_matches('/'));
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
fn resolve_reference(
    id: &str,
    tank: &FshTank,
    defs: &FhirDefs,
    config: &FshConfig,
    definition_index: &DefinitionIndex,
) -> String {
    // If already qualified (contains '/') or is a special reference (starts with '#'), return as-is
    if id.contains('/') || id.starts_with('#') {
        return id.to_string();
    }
    // Look up instance in tank to get its resource type
    if let Some(inst) = tank.instances.get(id).or_else(|| {
        tank.instances
            .values()
            .find(|instance| instance_id(instance) == id)
    }) {
        let rt = resolve_instance_resource_type(&inst.metadata.instance_of, defs, definition_index)
            .unwrap_or_else(|| inst.metadata.instance_of.clone());
        // If the instanceOf is a Logical model, references use the full canonical URL
        if tank
            .logicals
            .contains_key(inst.metadata.instance_of.as_str())
        {
            if let Some(canonical) = &config.canonical {
                return format!(
                    "{canonical}/StructureDefinition/{}/{id}",
                    inst.metadata.instance_of
                );
            }
        }
        return format!("{rt}/{id}");
    }
    id.to_string()
}

fn resolve_canonical(value: &str, context: &InstanceExportContext<'_>) -> String {
    if value.starts_with("http://") || value.starts_with("https://") || value.starts_with("urn:") {
        return value.to_string();
    }
    if let Some(url) = context
        .definition_index
        .lookup(value)
        .and_then(|definition| definition.url.clone())
    {
        return url;
    }
    if let Some(instance) = context.tank.instances.get(value).or_else(|| {
        context
            .tank
            .instances
            .values()
            .find(|instance| instance_id(instance) == value)
    }) {
        if let Some(canonical) = &context.config.canonical {
            let resource_type = resolve_instance_resource_type(
                &instance.metadata.instance_of,
                context.defs,
                context.definition_index,
            )
            .unwrap_or_else(|| instance.metadata.instance_of.clone());
            return format!(
                "{}/{resource_type}/{}",
                canonical.trim_end_matches('/'),
                instance_id(instance)
            );
        }
    }
    value.to_string()
}

fn instance_id(instance: &Instance) -> &str {
    instance
        .rules
        .iter()
        .find_map(|rule| {
            let InstanceRule::Assignment(assignment) = &rule.value else {
                return None;
            };
            let [FshPathSegment::Name(path)] = assignment.path.segments.as_slice() else {
                return None;
            };
            if path != "id" {
                return None;
            }
            match &assignment.value {
                FshValue::Str(id) => Some(id.as_str()),
                _ => None,
            }
        })
        .unwrap_or(&instance.metadata.name)
}

fn resolve_instance_resource_type(
    instance_of: &str,
    defs: &FhirDefs,
    definition_index: &DefinitionIndex,
) -> Option<String> {
    definition_index.resolve_base_type(instance_of, defs)
}

/// Returns the inner FHIR type name for a complex/backbone field type, if navigable.
fn field_next_type(ft: &FhirFieldType) -> Option<&str> {
    match ft {
        FhirFieldType::Complex(t) | FhirFieldType::BackboneElement(t) => Some(t),
        _ => None,
    }
}

fn path_field_info(
    current_type: &str,
    field_name: &str,
) -> Option<&'static rh_hl7_fhir_r4_core::metadata::FieldInfo> {
    get_field_info(current_type, field_name).or_else(|| {
        normalize_choice_type_name(field_name)
            .and_then(|choice_name| get_field_info(current_type, &choice_name))
            .or_else(|| dynamic_choice_field(current_type, field_name).map(|(_, field)| field))
    })
}

fn fallback_backbone_type(current_type: &str, field_name: &str) -> Option<&'static str> {
    match (current_type, field_name) {
        ("ExplanationOfBenefit", "adjudication") => Some("ExplanationOfBenefitItemAdjudication"),
        ("ClaimResponse", "adjudication") => Some("ClaimResponseItemAdjudication"),
        _ => None,
    }
}

fn choice_field_type(current_type: &str, field_name: &str) -> Option<FhirFieldType> {
    let suffix = if let Some(choice_name) = normalize_choice_type_name(field_name) {
        let base = choice_name.trim_end_matches("[x]");
        field_name.strip_prefix(base)?
    } else {
        dynamic_choice_field(current_type, field_name)?.0
    };
    match suffix {
        "CodeableConcept" => Some(FhirFieldType::Complex("CodeableConcept")),
        "Coding" => Some(FhirFieldType::Complex("Coding")),
        "Identifier" => Some(FhirFieldType::Complex("Identifier")),
        "Quantity" => Some(FhirFieldType::Complex("Quantity")),
        "Reference" => Some(FhirFieldType::Reference),
        "Period" => Some(FhirFieldType::Complex("Period")),
        "Range" => Some(FhirFieldType::Complex("Range")),
        "Ratio" => Some(FhirFieldType::Complex("Ratio")),
        "Boolean" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Boolean)),
        "Integer" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Integer)),
        "Decimal" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Decimal)),
        "String" => Some(FhirFieldType::Primitive(FhirPrimitiveType::String)),
        "Date" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Date)),
        "DateTime" => Some(FhirFieldType::Primitive(FhirPrimitiveType::DateTime)),
        "Time" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Time)),
        "Code" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Code)),
        "Uri" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Uri)),
        "Url" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Url)),
        "Canonical" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Canonical)),
        "Instant" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Instant)),
        "Markdown" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Markdown)),
        "Oid" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Oid)),
        "Uuid" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Uri)),
        "Id" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Id)),
        "Base64Binary" => Some(FhirFieldType::Primitive(FhirPrimitiveType::Base64Binary)),
        "PositiveInt" => Some(FhirFieldType::Primitive(FhirPrimitiveType::PositiveInt)),
        "UnsignedInt" => Some(FhirFieldType::Primitive(FhirPrimitiveType::UnsignedInt)),
        _ => None,
    }
}

fn dynamic_choice_field<'a>(
    current_type: &str,
    field_name: &'a str,
) -> Option<(&'a str, &'static rh_hl7_fhir_r4_core::metadata::FieldInfo)> {
    field_name.char_indices().find_map(|(index, character)| {
        if index == 0 || !character.is_ascii_uppercase() {
            return None;
        }
        let (base, suffix) = field_name.split_at(index);
        get_field_info(current_type, &format!("{base}[x]")).map(|field| (suffix, field))
    })
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

fn shape_value_for_field(
    value: serde_json::Value,
    field_type: Option<&FhirFieldType>,
) -> serde_json::Value {
    if matches!(field_type, Some(FhirFieldType::Primitive(_))) {
        if let Some(code) = value.get("code") {
            return code.clone();
        }
    }
    wrap_codeable_concept_if_needed(value, field_type)
}

/// Navigate a FHIR path in the JSON tree and set the leaf value.
///
/// Uses `FhirDefs` to determine whether each field has max cardinality `*` (array)
/// or `1` (scalar), matching sushi's JSON output shape without a full snapshot.
fn set_at_path(
    node: &mut serde_json::Value,
    segments: &[FshPathSegment],
    value: serde_json::Value,
    shape_context: &InstanceShapeContext,
    current_type: &str,
) {
    if segments.is_empty() {
        *node = value;
        return;
    }

    match &segments[0] {
        FshPathSegment::Name(name) => {
            let fi = path_field_info(current_type, name);
            let family_history_condition =
                current_type == "FamilyMemberHistory" && name == "condition";
            let family_history_relationship =
                current_type == "FamilyMemberHistory" && name == "relationship";
            let fallback_backbone = fallback_backbone_type(current_type, name);
            let field_type = choice_field_type(current_type, name)
                .or_else(|| fi.map(|field| field.field_type.clone()))
                .or_else(|| {
                    family_history_relationship.then_some(FhirFieldType::Complex("CodeableConcept"))
                });
            let is_array = family_history_condition
                || fallback_backbone.is_some()
                || fi.is_some_and(|f| f.max.is_none());
            let is_primitive = field_type
                .as_ref()
                .is_some_and(|field_type| matches!(field_type, FhirFieldType::Primitive(_)));
            let next_type = field_type
                .as_ref()
                .and_then(field_next_type)
                .or(fallback_backbone)
                .unwrap_or(if family_history_condition {
                    "FamilyMemberHistoryCondition"
                } else {
                    current_type
                });

            if let serde_json::Value::Object(map) = node {
                if segments.len() > 1 && is_primitive {
                    let child = map
                        .entry(format!("_{name}"))
                        .or_insert_with(|| serde_json::json!({}));
                    set_at_path(child, &segments[1..], value, shape_context, "Element");
                    return;
                }
                if segments.len() == 1 {
                    // Wrap Coding-shaped values in CodeableConcept when required by the field type
                    let value = shape_value_for_field(value, field_type.as_ref());
                    if is_array {
                        map.insert(name.clone(), serde_json::json!([value]));
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
                            set_at_path(last, &segments[1..], value, shape_context, next_type);
                        }
                    } else {
                        // Scalar field, or array field with explicit index following.
                        let default = if is_array || next_is_index {
                            serde_json::json!([])
                        } else {
                            serde_json::json!({})
                        };
                        let child = map.entry(name.clone()).or_insert_with(|| default);
                        set_at_path(child, &segments[1..], value, shape_context, next_type);
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
                    set_at_path(
                        &mut arr[i],
                        &segments[1..],
                        value,
                        shape_context,
                        current_type,
                    );
                }
            }
        }
        FshPathSegment::Slice { element, slice } => {
            if let serde_json::Value::Object(map) = node {
                handle_slice_segment(
                    map,
                    element,
                    slice,
                    segments,
                    value,
                    shape_context,
                    current_type,
                );
            }
        }
        FshPathSegment::ChoiceType(element) => {
            if let serde_json::Value::Object(map) = node {
                let key = element.clone();
                if segments.len() == 1 {
                    map.insert(key, value);
                } else {
                    let child = map.entry(key).or_insert_with(|| serde_json::json!({}));
                    set_at_path(child, &segments[1..], value, shape_context, current_type);
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
                        set_at_path(
                            &mut ext_obj,
                            &segments[1..],
                            value,
                            shape_context,
                            "Extension",
                        );
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
    shape_context: &InstanceShapeContext,
    current_type: &str,
) {
    // Soft-indexing operators: [+] appends, [=] reuses last
    if slice == "+" || slice == "=" {
        let fi = get_field_info(current_type, element);
        let next_type = fi
            .and_then(|f| field_next_type(&f.field_type))
            .or_else(|| fallback_backbone_type(current_type, element))
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
                set_at_path(
                    &mut shadow[idx],
                    &segments[1..],
                    value,
                    shape_context,
                    "Element",
                );
            }
        } else {
            let arr = map.get_mut(&element_key).unwrap();
            if let serde_json::Value::Array(arr) = arr {
                if segments.len() == 1 {
                    arr[idx] = shape_value_for_field(value, fi.map(|field| &field.field_type));
                } else {
                    set_at_path(
                        &mut arr[idx],
                        &segments[1..],
                        value,
                        shape_context,
                        next_type,
                    );
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
            .or_else(|| fallback_backbone_type(current_type, element))
            .unwrap_or(current_type);
        let is_primitive =
            fi.is_some_and(|field| matches!(field.field_type, FhirFieldType::Primitive(_)));
        let is_array = fi.is_none_or(|field| field.max.is_none());
        if !is_array && idx == 0 {
            if is_primitive && segments.len() > 1 {
                let shadow = map
                    .entry(format!("_{element}"))
                    .or_insert_with(|| serde_json::json!({}));
                set_at_path(shadow, &segments[1..], value, shape_context, "Element");
            } else if segments.len() == 1 {
                map.insert(
                    element.to_string(),
                    shape_value_for_field(value, fi.map(|field| &field.field_type)),
                );
            } else {
                let child = map
                    .entry(element.to_string())
                    .or_insert_with(|| serde_json::json!({}));
                set_at_path(child, &segments[1..], value, shape_context, next_type);
            }
            return;
        }
        if is_primitive && segments.len() > 1 {
            let shadow = map
                .entry(format!("_{element}"))
                .or_insert_with(|| serde_json::json!([]));
            if let serde_json::Value::Array(values) = shadow {
                while values.len() <= idx {
                    values.push(serde_json::Value::Null);
                }
                if values[idx].is_null() {
                    values[idx] = serde_json::json!({});
                }
                set_at_path(
                    &mut values[idx],
                    &segments[1..],
                    value,
                    shape_context,
                    "Element",
                );
            }
            return;
        }
        let arr = map
            .entry(element.to_string())
            .or_insert_with(|| serde_json::json!([]));
        if let serde_json::Value::Array(arr) = arr {
            while arr.len() <= idx {
                arr.push(serde_json::json!({}));
            }
            if segments.len() == 1 {
                arr[idx] = shape_value_for_field(value, fi.map(|field| &field.field_type));
            } else {
                set_at_path(
                    &mut arr[idx],
                    &segments[1..],
                    value,
                    shape_context,
                    next_type,
                );
            }
        }
        return;
    }

    // Regular named slice: navigate to a stable repetition for the slice.
    let fi = get_field_info(current_type, element);
    let next_type = fi
        .and_then(|f| field_next_type(&f.field_type))
        .or_else(|| fallback_backbone_type(current_type, element))
        .unwrap_or(current_type);
    let key = element.to_string();
    let array = map.entry(key).or_insert_with(|| serde_json::json!([]));
    let serde_json::Value::Array(values) = array else {
        return;
    };
    if let Some(FshPathSegment::Index(repetition)) = segments.get(1) {
        let repetition = *repetition as usize;
        let mut template = values
            .iter()
            .find(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
            .cloned()
            .unwrap_or_else(|| serde_json::json!({ SLICE_MARKER: slice }));
        if element == "extension" && template.get("url").is_none() {
            template["url"] = serde_json::Value::String(named_extension_url(slice, shape_context));
        }
        while values
            .iter()
            .filter(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
            .count()
            <= repetition
        {
            let insert_at = named_slice_insert_index(values, slice);
            values.insert(insert_at, template.clone());
        }
        let index = values
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice)
            })
            .nth(repetition)
            .map(|(index, _)| index)
            .unwrap();
        if segments.len() == 2 {
            values[index] = value;
        } else {
            set_at_path(
                &mut values[index],
                &segments[2..],
                value,
                shape_context,
                next_type,
            );
        }
        return;
    }
    let index = values
        .iter()
        .position(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
        .or_else(|| {
            if element == "extension" {
                return None;
            }
            let mut unmarked = values
                .iter()
                .enumerate()
                .filter(|(_, item)| item.get(SLICE_MARKER).is_none());
            let candidate = unmarked.next().map(|(index, _)| index);
            candidate.filter(|_| unmarked.next().is_none())
        })
        .unwrap_or_else(|| {
            let mut item = serde_json::json!({ SLICE_MARKER: slice });
            if element == "extension" {
                item["url"] = serde_json::Value::String(named_extension_url(slice, shape_context));
            }
            let index = named_slice_insert_index(values, slice);
            values.insert(index, item);
            index
        });
    if let Some(object) = values[index].as_object_mut() {
        object
            .entry(SLICE_MARKER.to_string())
            .or_insert_with(|| serde_json::Value::String(slice.to_string()));
    }
    if segments.len() == 1 {
        let mut value = shape_value_for_field(value, fi.map(|field| &field.field_type));
        if let serde_json::Value::Object(object) = &mut value {
            object.insert(
                SLICE_MARKER.to_string(),
                serde_json::Value::String(slice.to_string()),
            );
        }
        values[index] = value;
    } else {
        set_at_path(
            &mut values[index],
            &segments[1..],
            value,
            shape_context,
            next_type,
        );
    }
}

fn named_slice_insert_index(values: &[serde_json::Value], slice: &str) -> usize {
    if let Some(index) = values
        .iter()
        .rposition(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
    {
        return index + 1;
    }
    values.len()
}

fn named_extension_url(slice: &str, shape_context: &InstanceShapeContext) -> String {
    shape_context
        .extension_slice_urls
        .get(slice)
        .or_else(|| {
            extension_lookup_keys(slice)
                .into_iter()
                .find_map(|key| shape_context.extension_slice_urls.get(&key))
        })
        .cloned()
        .unwrap_or_else(|| {
            if slice.contains('-')
                && !slice.starts_with("http://")
                && !slice.starts_with("https://")
                && !slice.starts_with("urn:")
            {
                format!("http://hl7.org/fhir/StructureDefinition/{slice}")
            } else {
                slice.to_string()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependencies::{DependencyDefinitionSet, DependencyStructureDefinition};
    use crate::parser::ast::{Profile, SdMetadata};
    use crate::{build_definition_index, FshConfig};
    use std::path::PathBuf;

    #[test]
    fn concrete_choice_assignments_override_generic_dependency_defaults() {
        let assigned = [
            FshPathSegment::Slice {
                element: "component".to_string(),
                slice: "frequency".to_string(),
            },
            FshPathSegment::Name("valueQuantity".to_string()),
        ];
        let fixed = [
            FshPathSegment::Slice {
                element: "component".to_string(),
                slice: "frequency".to_string(),
            },
            FshPathSegment::Name("value[x]".to_string()),
            FshPathSegment::Name("system".to_string()),
        ];

        assert!(paths_override(&assigned, &fixed));
    }

    #[test]
    fn materializes_named_extension_slices_with_canonical_urls() {
        let config = FshConfig {
            canonical: Some("http://example.org/fhir".to_string()),
            ..Default::default()
        };
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config,
            ..Default::default()
        })
        .compile(
            r#"
Extension: RelatedCondition
Id: related-condition
* value[x] only Reference(Condition)

Profile: Tumor
Parent: BodyStructure
* extension contains RelatedCondition named relatedCondition 0..1

Instance: tumor-example
InstanceOf: Tumor
* extension[relatedCondition].valueReference = Reference(Condition/example)
"#,
            "named-extension.fsh",
        )
        .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "tumor-example")
            .expect("instance exists");

        assert_eq!(
            instance["extension"][0]["url"],
            "http://example.org/fhir/StructureDefinition/related-condition"
        );
        assert_eq!(
            instance["extension"][0]["valueReference"]["reference"],
            "Condition/example"
        );
    }

    #[test]
    fn applies_local_profile_assignments_as_instance_defaults() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: FinalObservation
Parent: Observation
* status = #final
* code = http://loinc.org#1234-5 "Example"

Instance: observation-example
InstanceOf: FinalObservation
"#,
                "profile-defaults.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "observation-example")
            .expect("instance exists");

        assert_eq!(instance["status"], "final");
        assert_eq!(instance["code"]["coding"][0]["code"], "1234-5");
    }

    #[test]
    fn replaces_repeating_fields_on_plain_assignment() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: report-example
InstanceOf: DiagnosticReport
* performer = Reference(Organization/first)
* performer = Reference(Practitioner/second)
"#,
                "repeating-replacement.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "report-example")
            .expect("instance exists");

        assert_eq!(instance["performer"].as_array().unwrap().len(), 1);
        assert_eq!(instance["performer"][0]["reference"], "Practitioner/second");
    }

    #[test]
    fn keeps_indexed_scalar_fields_scalar() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: eob-example
InstanceOf: ExplanationOfBenefit
* insurance[0].coverage[0] = Reference(Coverage/example)
"#,
                "indexed-scalar.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "eob-example")
            .expect("instance exists");

        assert_eq!(
            instance["insurance"][0]["coverage"]["reference"],
            "Coverage/example"
        );
    }

    #[test]
    fn resolves_bare_references_by_exported_instance_id() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: named-practitioner
InstanceOf: Practitioner
* id = "exported-practitioner"

Instance: observation-example
InstanceOf: Observation
* performer = Reference(exported-practitioner)
"#,
                "reference-exported-id.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "observation-example")
            .expect("instance exists");

        assert_eq!(
            instance["performer"][0]["reference"],
            "Practitioner/exported-practitioner"
        );
    }

    #[test]
    fn resolves_local_canonical_targets() {
        let config = FshConfig {
            canonical: Some("http://example.org/fhir".to_string()),
            ..Default::default()
        };
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config,
            ..Default::default()
        })
        .compile(
            r#"
Profile: LocalPatient
Parent: Patient
Id: local-patient

Instance: local-operation
InstanceOf: OperationDefinition
* id = "local-op"
* parameter[0].targetProfile = Canonical(LocalPatient)

Instance: local-capability
InstanceOf: CapabilityStatement
* instantiates = Canonical(local-operation)
"#,
            "local-canonical.fsh",
        )
        .expect("FSH compiles");
        let operation = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "local-op")
            .expect("operation exists");
        let capability = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "local-capability")
            .expect("capability exists");

        assert_eq!(
            operation["parameter"][0]["targetProfile"][0],
            "http://example.org/fhir/StructureDefinition/local-patient"
        );
        assert_eq!(
            capability["instantiates"][0],
            "http://example.org/fhir/OperationDefinition/local-op"
        );
    }

    #[test]
    fn exports_extensions_on_primitive_choice_fields() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: device-use-example
InstanceOf: DeviceUseStatement
* timingDateTime.extension.url = "http://hl7.org/fhir/StructureDefinition/data-absent-reason"
* timingDateTime.extension.valueCode = #unknown
"#,
                "primitive-choice-extension.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "device-use-example")
            .expect("instance exists");

        assert_eq!(
            instance["_timingDateTime"]["extension"][0]["valueCode"],
            "unknown"
        );
    }

    #[test]
    fn exports_complex_choice_fields_without_primitive_shadows() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: claim-example
InstanceOf: Claim
* item[0].extension[itemTrace].valueIdentifier.system = "http://example.org/trace"
* item[0].extension[itemTrace].valueIdentifier.value = "1122334"
"#,
                "complex-choice-extension.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "claim-example")
            .expect("instance exists");

        assert_eq!(
            instance["item"][0]["extension"][0]["valueIdentifier"]["value"],
            "1122334"
        );
        assert!(instance["item"][0]["extension"][0]
            .get("_valueIdentifier")
            .is_none());
    }

    #[test]
    fn exports_nested_named_extensions() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: response-example
InstanceOf: ClaimResponse
* error[1].extension[errorElement].extension[error].valueString = "2010A-NM103"
"#,
                "nested-named-extension.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "response-example")
            .expect("instance exists");

        assert_eq!(
            instance["error"][1]["extension"][0]["extension"][0]["valueString"],
            "2010A-NM103"
        );
    }

    #[test]
    fn preserves_path_contexts_from_inserted_rule_sets() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
RuleSet: CapabilityRest
* rest
  * mode = #server
  * documentation = "Server capabilities"

Instance: capability-example
InstanceOf: CapabilityStatement
* insert CapabilityRest
"#,
                "ruleset-path-context.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "capability-example")
            .expect("instance exists");

        assert_eq!(instance["rest"][0]["documentation"], "Server capabilities");
        assert!(instance.get("documentation").is_none());
    }

    #[test]
    fn wraps_dynamically_resolved_choice_types() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: administration-example
InstanceOf: MedicationAdministration
* medicationCodeableConcept = http://www.nlm.nih.gov/research/umls/rxnorm#3002
"#,
                "dynamic-choice.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "administration-example")
            .expect("instance exists");

        assert_eq!(
            instance["medicationCodeableConcept"]["coding"][0]["code"],
            "3002"
        );
    }

    #[test]
    fn resolves_local_and_versioned_code_system_canonicals() {
        let config = FshConfig {
            canonical: Some("http://example.org/fhir".to_string()),
            ..Default::default()
        };
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config,
            ..Default::default()
        })
        .compile(
            r#"
Alias: $Role = http://terminology.hl7.org/CodeSystem/claimcareteamrole|1.0.0

Instance: versioned-code-example
InstanceOf: Observation
* code = $Role#primary
"#,
            "code-system-canonical.fsh",
        )
        .expect("FSH compiles");
        let versioned = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "versioned-code-example")
            .expect("versioned instance exists");

        assert_eq!(
            versioned["code"]["coding"][0]["system"],
            "http://terminology.hl7.org/CodeSystem/claimcareteamrole"
        );

        let mut tank = FshTank::new();
        tank.code_systems.insert(
            "LocalStatus".to_string(),
            CodeSystem {
                metadata: CsMetadata {
                    name: "LocalStatus".to_string(),
                    id: Some("local-status".to_string()),
                    title: None,
                    description: None,
                },
                concepts: Vec::new(),
                caret_rules: Vec::new(),
            },
        );
        assert_eq!(
            resolve_code_system_url(
                "LocalStatus",
                &tank,
                &FshConfig {
                    canonical: Some("http://example.org/fhir".to_string()),
                    ..Default::default()
                }
            ),
            "http://example.org/fhir/CodeSystem/local-status"
        );
    }

    #[test]
    fn materializes_repeated_named_slice_repetitions() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: GeneObservation
Parent: Observation
* component contains gene 0..*
* component[gene].code = http://loinc.org#48018-6

Instance: gene-example
InstanceOf: GeneObservation
* component[gene][0].valueString = "BRCA1"
* component[gene][1].valueString = "BRCA2"
"#,
                "repeated-slice.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "gene-example")
            .expect("instance exists");

        assert_eq!(instance["component"].as_array().unwrap().len(), 2);
        assert_eq!(
            instance["component"][0]["code"]["coding"][0]["code"],
            "48018-6"
        );
        assert_eq!(instance["component"][1]["valueString"], "BRCA2");
    }

    #[test]
    fn associates_single_unmarked_repetition_with_a_named_slice() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: GeneObservation
Parent: Observation
* component contains gene 0..1

Instance: gene-example
InstanceOf: GeneObservation
* component.code = http://loinc.org#48018-6
* component[gene].valueString = "BRCA1"
"#,
                "unmarked-named-slice.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "gene-example")
            .expect("instance exists");

        assert_eq!(instance["component"].as_array().unwrap().len(), 1);
        assert_eq!(instance["component"][0]["valueString"], "BRCA1");
    }

    #[test]
    fn does_not_materialize_unused_optional_profile_slices() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: OptionalCareTeamClaim
Parent: Claim
* careTeam contains attending 0..1
* careTeam[attending].extension[claim-scope].valueBoolean = true

Instance: claim-example
InstanceOf: OptionalCareTeamClaim
"#,
                "optional-slice.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "claim-example")
            .expect("instance exists");

        assert!(instance.get("careTeam").is_none());
    }

    #[test]
    fn preserves_declared_order_for_required_slices() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: OrderedAppointment
Parent: Appointment
* participant contains Patient 1..1 and PrimaryPerformer 1..1
* participant[PrimaryPerformer].type = http://terminology.hl7.org/CodeSystem/v3-ParticipationType#PPRF

Instance: appointment-example
InstanceOf: OrderedAppointment
* participant[Patient]
  * actor = Reference(Patient/example)
* participant[PrimaryPerformer]
  * actor = Reference(Practitioner/example)
"#,
                "required-slice-order.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "appointment-example")
            .expect("instance exists");

        assert_eq!(
            instance["participant"][0]["actor"]["reference"],
            "Patient/example"
        );
        assert_eq!(
            instance["participant"][1]["actor"]["reference"],
            "Practitioner/example"
        );
    }

    #[test]
    fn omits_empty_required_slice_placeholders() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Profile: RequiredExtensionRequest
Parent: CommunicationRequest
* extension contains MissingExtension named missing 1..1

Instance: request-example
InstanceOf: RequiredExtensionRequest
"#,
                "empty-required-slice.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "request-example")
            .expect("instance exists");

        assert!(instance.get("extension").is_none());
    }

    #[test]
    fn exports_representative_profile_instances_with_base_resource_types() {
        let mut tank = FshTank::new();
        tank.profiles.insert(
            "CancerPatient".to_string(),
            Profile {
                metadata: SdMetadata {
                    name: "CancerPatient".to_string(),
                    parent: Some("USCorePatientProfile".to_string()),
                    id: Some("mcode-cancer-patient".to_string()),
                    title: Some("Cancer Patient Profile".to_string()),
                    description: None,
                    characteristics: Vec::new(),
                },
                rules: Vec::new(),
            },
        );
        let config = FshConfig::default();
        let dependencies = DependencyDefinitionSet {
            structure_definitions: vec![
                dependency_sd(
                    "hl7.fhir.us.core",
                    "6.1.0",
                    "USCorePatientProfile",
                    "us-core-patient",
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient",
                    "Patient",
                ),
                dependency_sd(
                    "hl7.fhir.us.core",
                    "6.1.0",
                    "USCorePractitionerProfile",
                    "us-core-practitioner",
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-practitioner",
                    "Practitioner",
                ),
                dependency_sd(
                    "hl7.fhir.us.davinci-crd",
                    "2.2.1",
                    "CRDServiceRequest",
                    "profile-servicerequest",
                    "http://hl7.org/fhir/us/davinci-crd/StructureDefinition/profile-servicerequest",
                    "ServiceRequest",
                ),
                dependency_sd(
                    "hl7.fhir.us.davinci-crd",
                    "2.2.1",
                    "CRDCoverage",
                    "profile-coverage",
                    "http://hl7.org/fhir/us/davinci-crd/StructureDefinition/profile-coverage",
                    "Coverage",
                ),
                dependency_sd(
                    "hl7.fhir.uv.sdc",
                    "4.0.0",
                    "SDCQuestionnaireAdapt",
                    "sdc-questionnaire-adapt",
                    "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-adapt",
                    "Questionnaire",
                ),
                dependency_sd(
                    "hl7.fhir.uv.genomics-reporting",
                    "2.0.0",
                    "GenomicFinding",
                    "finding",
                    "http://hl7.org/fhir/uv/genomics-reporting/StructureDefinition/finding",
                    "Observation",
                ),
                dependency_sd(
                    "hl7.fhir.uv.genomics-reporting",
                    "2.0.0",
                    "GenomicsReport",
                    "genomics-report",
                    "http://hl7.org/fhir/uv/genomics-reporting/StructureDefinition/genomics-report",
                    "DiagnosticReport",
                ),
            ],
            warnings: Vec::new(),
        };
        let definition_index = build_definition_index(&tank, &config, &dependencies);
        let defs = FhirDefs::r4();
        let cases = [
            ("USCorePatientProfile", "Patient"),
            ("USCorePractitionerProfile", "Practitioner"),
            ("CRDServiceRequest", "ServiceRequest"),
            ("CRDCoverage", "Coverage"),
            ("SDCQuestionnaireAdapt", "Questionnaire"),
            ("GenomicFinding", "Observation"),
            ("GenomicsReport", "DiagnosticReport"),
            ("CancerPatient", "Patient"),
        ];

        for (instance_of, expected_resource_type) in cases {
            let inst = Instance {
                metadata: InstanceMetadata {
                    name: format!("example-{expected_resource_type}"),
                    instance_of: instance_of.to_string(),
                    usage: Some("#example".to_string()),
                    title: None,
                    description: None,
                },
                rules: Vec::new(),
            };

            let resource = export_instance(&inst, defs.as_ref(), &config, &tank, &definition_index)
                .expect("instance exports");

            assert_eq!(resource["resourceType"], expected_resource_type);
        }
    }

    fn dependency_sd(
        package_id: &str,
        version: &str,
        name: &str,
        id: &str,
        url: &str,
        type_: &str,
    ) -> DependencyStructureDefinition {
        DependencyStructureDefinition {
            package_id: package_id.to_string(),
            version: version.to_string(),
            path: PathBuf::from(format!("StructureDefinition-{id}.json")),
            id: Some(id.to_string()),
            url: Some(url.to_string()),
            name: Some(name.to_string()),
            title: None,
            kind: Some("resource".to_string()),
            type_: Some(type_.to_string()),
            base_definition: Some(format!("http://hl7.org/fhir/StructureDefinition/{type_}")),
            derivation: Some("constraint".to_string()),
            fixed_values: Vec::new(),
        }
    }
}
