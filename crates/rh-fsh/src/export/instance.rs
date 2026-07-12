//! Export FSH Instance to FHIR JSON

use crate::definition_index::DefinitionIndex;
use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::*;
use crate::schema::{CompiledSchema, FieldShape, SchemaView};
use crate::semantic::{SemanticOperation, SemanticProgram};
use crate::tank::FshTank;
use crate::FshConfig;
use rh_hl7_fhir_r4_core::metadata::FhirFieldType;
use std::collections::{BTreeMap, HashSet};

struct InstanceExportContext<'a> {
    defs: &'a FhirDefs,
    config: &'a FshConfig,
    tank: &'a FshTank,
    definition_index: &'a DefinitionIndex,
    schema: &'a CompiledSchema,
}

struct InstanceShapeContext<'a> {
    schema_view: SchemaView<'a>,
}

/// Internal typed node representation with deterministic object ordering.
#[derive(Debug, Clone, PartialEq)]
enum InstanceNode {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<InstanceNode>),
    Object(BTreeMap<String, InstanceNode>),
}

impl InstanceNode {
    fn object() -> Self {
        Self::Object(BTreeMap::new())
    }

    fn array() -> Self {
        Self::Array(Vec::new())
    }

    fn get(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(object) => object.get(key),
            _ => None,
        }
    }

    fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, Self>> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    fn insert(&mut self, key: impl Into<String>, value: Self) {
        if let Self::Object(object) = self {
            object.insert(key.into(), value);
        }
    }

    fn into_json(self) -> serde_json::Value {
        match self {
            Self::Null => serde_json::Value::Null,
            Self::Bool(value) => serde_json::Value::Bool(value),
            Self::Number(value) => serde_json::Value::Number(value),
            Self::String(value) => serde_json::Value::String(value),
            Self::Array(values) => serde_json::Value::Array(
                values
                    .into_iter()
                    .filter(|value| !value.is_slice_marker_only())
                    .map(Self::into_json)
                    .collect(),
            ),
            Self::Object(values) => {
                let values = values.into_iter().filter_map(|(key, value)| {
                    if key == SLICE_MARKER {
                        return None;
                    }
                    let value = value.into_json();
                    if matches!(&value, serde_json::Value::Array(values) if values.is_empty()) {
                        return None;
                    }
                    Some((key, value))
                });
                serde_json::Value::Object(values.collect())
            }
        }
    }

    fn is_slice_marker_only(&self) -> bool {
        matches!(self, Self::Object(object) if object.len() == 1 && object.contains_key(SLICE_MARKER))
    }
}

impl From<serde_json::Value> for InstanceNode {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(value) => Self::Bool(value),
            serde_json::Value::Number(value) => Self::Number(value),
            serde_json::Value::String(value) => Self::String(value),
            serde_json::Value::Array(values) => {
                Self::Array(values.into_iter().map(Self::from).collect())
            }
            serde_json::Value::Object(values) => Self::Object(
                values
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value)))
                    .collect(),
            ),
        }
    }
}

/// Schema-typed mutation tree used between semantic lowering and JSON serialization.
struct TypedInstanceTree<'a> {
    root: InstanceNode,
    root_type: &'a str,
    shape_context: InstanceShapeContext<'a>,
}

impl<'a> TypedInstanceTree<'a> {
    fn new(root: InstanceNode, root_type: &'a str, schema_view: SchemaView<'a>) -> Self {
        Self {
            root,
            root_type,
            shape_context: InstanceShapeContext { schema_view },
        }
    }

    fn apply(&mut self, segments: &[FshPathSegment], value: serde_json::Value) {
        apply_at_path(
            &mut self.root,
            segments,
            InstanceNode::from(value),
            &self.shape_context,
            self.root_type,
        );
    }

    fn root_type(&self) -> &'a str {
        self.root_type
    }

    fn schema_view(&self) -> SchemaView<'a> {
        self.shape_context.schema_view
    }

    fn into_json(self) -> serde_json::Value {
        self.root.into_json()
    }
}

pub fn export_instance(
    inst: &Instance,
    defs: &FhirDefs,
    config: &FshConfig,
    tank: &FshTank,
    definition_index: &DefinitionIndex,
    schema: &CompiledSchema,
) -> Result<serde_json::Value, FshError> {
    let context = InstanceExportContext {
        defs,
        config,
        tank,
        definition_index,
        schema,
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
            InstanceNode::Object(BTreeMap::from([
                (
                    "resourceType".to_string(),
                    InstanceNode::String(resource_type_json),
                ),
                (
                    "id".to_string(),
                    InstanceNode::String(inst.metadata.name.clone()),
                ),
            ]))
        } else {
            InstanceNode::Object(BTreeMap::from([(
                "resourceType".to_string(),
                InstanceNode::String(resource_type_json),
            )]))
        }
    } else {
        InstanceNode::Object(BTreeMap::from([
            (
                "resourceType".to_string(),
                InstanceNode::String(resource_type_json),
            ),
            (
                "id".to_string(),
                InstanceNode::String(inst.metadata.name.clone()),
            ),
        ]))
    };

    if let Some(title) = &inst.metadata.title {
        if context
            .schema
            .field(&resource_type_for_metadata, "title")
            .is_some()
        {
            resource.insert("title", InstanceNode::String(title.clone()));
        }
    }

    let usage = inst.metadata.usage.as_deref().unwrap_or("");
    if usage == "#definition" || usage == "definition" {
        if let Some(canonical) = &context.config.canonical {
            let url = format!("{canonical}/{instance_of}/{}", inst.metadata.name);
            resource.insert("url", InstanceNode::String(url));
        }
    }

    let schema_view = context
        .schema
        .view(instance_of, &resource_type_for_metadata);
    let mut tree = TypedInstanceTree::new(resource, &resource_type_for_metadata, schema_view);
    apply_local_profile_defaults(&mut tree, instance_of, &inst.rules, context);
    let program =
        SemanticProgram::lower_instance(&inst.rules, |value| fsh_value_to_json(value, context));
    for operation in program.into_operations() {
        match operation {
            SemanticOperation::Assign(assignment) => {
                tree.apply(assignment.path.segments(), assignment.value)
            }
            SemanticOperation::EstablishContext { path, .. } if path.has_trailing_append() => {
                tree.apply(path.segments(), serde_json::json!({}));
            }
            SemanticOperation::EstablishContext { .. } => {}
        }
    }
    Ok(tree.into_json())
}

fn apply_local_profile_defaults(
    tree: &mut TypedInstanceTree<'_>,
    instance_of: &str,
    instance_rules: &[Spanned<InstanceRule>],
    context: &InstanceExportContext<'_>,
) {
    let mut profiles = Vec::new();
    let mut dependency_profiles = Vec::new();
    let schema_view = tree.schema_view();
    if schema_view.profile_lineage().is_empty() {
        collect_profile_chain(
            instance_of,
            context,
            &mut profiles,
            &mut dependency_profiles,
        );
    } else {
        for profile_name in schema_view.profile_lineage() {
            if let Some(profile) = context.tank.profiles.get(profile_name) {
                profiles.push(profile);
            } else if let Some(profile) = context.definition_index.lookup(profile_name) {
                dependency_profiles.push(profile);
            }
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
            tree.apply(&segments, fixed_value.value.clone());
        }
    }

    for profile in profiles.into_iter().rev() {
        materialize_required_slices(tree, profile);
        let program = SemanticProgram::lower_profile(&profile.rules, |value| {
            fsh_value_to_json(value, context)
        });
        for operation in program.into_operations() {
            match operation {
                SemanticOperation::Assign(assignment) => {
                    if local_profile_path_is_used(
                        assignment.path.segments(),
                        &instance_assigned_paths,
                        profile,
                        tree.root_type(),
                        tree.schema_view(),
                    ) {
                        tree.apply(assignment.path.segments(), assignment.value);
                    }
                }
                SemanticOperation::EstablishContext { .. } => {}
            }
        }
    }
}

fn collect_profile_chain<'a>(
    instance_of: &str,
    context: &'a InstanceExportContext<'_>,
    profiles: &mut Vec<&'a Profile>,
    dependency_profiles: &mut Vec<&'a crate::IndexedStructureDefinition>,
) {
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
}

fn materialize_required_slices(tree: &mut TypedInstanceTree<'_>, profile: &Profile) {
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
            tree.apply(&segments, serde_json::json!({}));
        }
    }
}

fn local_profile_path_is_used(
    path: &[FshPathSegment],
    assigned_paths: &[Vec<FshPathSegment>],
    profile: &Profile,
    resource_type: &str,
    schema_view: SchemaView<'_>,
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
    if root.is_some_and(|root| {
        path_field_info(schema_view, resource_type, root).is_some_and(|field| field.min() > 0)
    }) {
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

const SLICE_MARKER: &str = "__rh_fsh_slice";

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
        let exported_id = instance_id(inst);
        if inst
            .metadata
            .usage
            .as_deref()
            .is_some_and(|usage| usage == "#inline" || usage == "inline")
            && is_contained_instance(id, tank)
        {
            return format!("#{exported_id}");
        }
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
        return format!("{rt}/{exported_id}");
    }
    id.to_string()
}

fn is_contained_instance(target: &str, tank: &FshTank) -> bool {
    tank.instances.values().any(|instance| {
        instance.rules.iter().any(|rule| {
            let InstanceRule::Assignment(assignment) = &rule.value else {
                return false;
            };
            assignment.path.segments.first().and_then(path_segment_name) == Some("contained")
                && matches!(&assignment.value, FshValue::InstanceRef(value) if value == target)
        })
    })
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
        if let Some(url) = instance_assigned_string(instance, "url") {
            return url.to_string();
        }
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
    instance_assigned_string(instance, "id").unwrap_or(&instance.metadata.name)
}

fn instance_assigned_string<'a>(instance: &'a Instance, requested_path: &str) -> Option<&'a str> {
    instance.rules.iter().find_map(|rule| {
        if rule.location.column != 1 {
            return None;
        }
        let InstanceRule::Assignment(assignment) = &rule.value else {
            return None;
        };
        let [FshPathSegment::Name(assigned_path)] = assignment.path.segments.as_slice() else {
            return None;
        };
        if assigned_path != requested_path {
            return None;
        }
        match &assignment.value {
            FshValue::Str(id) => Some(id.as_str()),
            _ => None,
        }
    })
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

fn path_field_info<'a>(
    schema_view: SchemaView<'a>,
    current_type: &str,
    field_name: &str,
) -> Option<FieldShape<'a>> {
    schema_view.field(current_type, field_name).or_else(|| {
        normalize_choice_type_name(field_name)
            .and_then(|choice_name| schema_view.field(current_type, &choice_name))
    })
}

/// Wrap a Coding-shaped object as a CodeableConcept if the target field requires it.
///
/// A "Coding-shaped" value is `{"code": ..., "system": ...}` (with optional display).
/// When the field type is `CodeableConcept`, FHIR requires `{"coding": [{...}]}`.
fn wrap_codeable_concept_if_needed(
    value: InstanceNode,
    field_type: Option<&FhirFieldType>,
) -> InstanceNode {
    if matches!(field_type, Some(FhirFieldType::Complex("CodeableConcept"))) {
        if let InstanceNode::Object(ref map) = value {
            if map.contains_key("code") || map.contains_key("system") {
                return InstanceNode::Object(BTreeMap::from([(
                    "coding".to_string(),
                    InstanceNode::Array(vec![value]),
                )]));
            }
        }
    }
    value
}

fn shape_value_for_field(value: InstanceNode, field_type: Option<&FhirFieldType>) -> InstanceNode {
    if matches!(field_type, Some(FhirFieldType::Primitive(_))) {
        if let Some(code) = value.get("code") {
            return code.clone();
        }
    }
    wrap_codeable_concept_if_needed(value, field_type)
}

/// Navigate a FHIR path in the typed tree and set the leaf value.
///
/// Uses `FhirDefs` to determine whether each field has max cardinality `*` (array)
/// or `1` (scalar), matching sushi's JSON output shape without a full snapshot.
fn apply_at_path(
    node: &mut InstanceNode,
    segments: &[FshPathSegment],
    value: InstanceNode,
    shape_context: &InstanceShapeContext,
    current_type: &str,
) {
    if segments.is_empty() {
        *node = value;
        return;
    }

    match &segments[0] {
        FshPathSegment::Name(name) => {
            let fi = path_field_info(shape_context.schema_view, current_type, name);
            let field_type = fi.as_ref().map(|field| field.field_type().clone());
            let is_array = fi.as_ref().is_some_and(|field| field.max().is_none());
            let is_primitive = field_type
                .as_ref()
                .is_some_and(|field_type| matches!(field_type, FhirFieldType::Primitive(_)));
            let next_type = field_type
                .as_ref()
                .and_then(field_next_type)
                .unwrap_or(current_type);

            if let InstanceNode::Object(map) = node {
                if segments.len() > 1 && is_primitive {
                    let child = map
                        .entry(format!("_{name}"))
                        .or_insert_with(InstanceNode::object);
                    apply_at_path(child, &segments[1..], value, shape_context, "Element");
                    return;
                }
                if segments.len() == 1 {
                    // Wrap Coding-shaped values in CodeableConcept when required by the field type
                    let value = shape_value_for_field(value, field_type.as_ref());
                    if is_array {
                        map.insert(name.clone(), InstanceNode::Array(vec![value]));
                    } else {
                        map.insert(name.clone(), value);
                    }
                } else {
                    let next_is_index = matches!(segments.get(1), Some(FshPathSegment::Index(_)));
                    if is_array && !next_is_index {
                        // Array field accessed without an explicit index — navigate to last element
                        // (creating the first one if the array is empty).
                        let arr = map.entry(name.clone()).or_insert_with(InstanceNode::array);
                        if let InstanceNode::Array(a) = arr {
                            if a.is_empty() {
                                a.push(InstanceNode::object());
                            }
                            let last = a.last_mut().unwrap();
                            apply_at_path(last, &segments[1..], value, shape_context, next_type);
                        }
                    } else {
                        // Scalar field, or array field with explicit index following.
                        let default = if is_array || next_is_index {
                            InstanceNode::array()
                        } else {
                            InstanceNode::object()
                        };
                        let child = map.entry(name.clone()).or_insert_with(|| default);
                        apply_at_path(child, &segments[1..], value, shape_context, next_type);
                    }
                }
            }
        }
        FshPathSegment::Index(idx) => {
            if let InstanceNode::Array(arr) = node {
                let i = *idx as usize;
                while arr.len() <= i {
                    arr.push(InstanceNode::object());
                }
                if segments.len() == 1 {
                    arr[i] = value;
                } else {
                    apply_at_path(
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
            if let InstanceNode::Object(map) = node {
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
            if let InstanceNode::Object(map) = node {
                let key = element.clone();
                if segments.len() == 1 {
                    map.insert(key, value);
                } else {
                    let child = map.entry(key).or_insert_with(InstanceNode::object);
                    apply_at_path(child, &segments[1..], value, shape_context, current_type);
                }
            }
        }
        FshPathSegment::Extension(url) => {
            if let InstanceNode::Object(map) = node {
                let exts = map
                    .entry("extension".to_string())
                    .or_insert_with(InstanceNode::array);
                if let InstanceNode::Array(arr) = exts {
                    if segments.len() == 1 {
                        arr.push(InstanceNode::Object(BTreeMap::from([
                            ("url".to_string(), InstanceNode::String(url.clone())),
                            ("valueString".to_string(), value),
                        ])));
                    } else {
                        let mut ext_obj = InstanceNode::Object(BTreeMap::from([(
                            "url".to_string(),
                            InstanceNode::String(url.clone()),
                        )]));
                        apply_at_path(
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
    map: &mut BTreeMap<String, InstanceNode>,
    element: &str,
    slice: &str,
    segments: &[FshPathSegment],
    value: InstanceNode,
    shape_context: &InstanceShapeContext,
    current_type: &str,
) {
    // Soft-indexing operators: [+] appends, [=] reuses last
    if slice == "+" || slice == "=" {
        let fi = path_field_info(shape_context.schema_view, current_type, element);
        let next_type = fi
            .as_ref()
            .and_then(|f| field_next_type(f.field_type()))
            .unwrap_or(current_type);
        let element_key = element.to_string();

        if fi.as_ref().is_some_and(|field| field.max().is_some()) {
            if segments.len() == 1 {
                map.insert(
                    element_key,
                    shape_value_for_field(value, fi.as_ref().map(FieldShape::field_type)),
                );
            } else {
                let child = map.entry(element_key).or_insert_with(InstanceNode::object);
                apply_at_path(child, &segments[1..], value, shape_context, next_type);
            }
            return;
        }

        let arr_len = {
            let arr = map
                .entry(element_key.clone())
                .or_insert_with(InstanceNode::array);
            if slice == "+" {
                if let InstanceNode::Array(a) = arr {
                    a.push(InstanceNode::object());
                }
            }
            if let InstanceNode::Array(a) = arr {
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
                if let InstanceNode::Array(a) = v {
                    a.get(idx)
                } else {
                    None
                }
            })
            .is_some_and(|v| !v.is_object() && !v.is_null());

        if is_primitive && segments.len() > 1 {
            let shadow_key = format!("_{element_key}");
            let shadow_arr = map.entry(shadow_key).or_insert_with(InstanceNode::array);
            if let InstanceNode::Array(shadow) = shadow_arr {
                while shadow.len() <= idx {
                    shadow.push(InstanceNode::Null);
                }
                if shadow[idx].is_null() {
                    shadow[idx] = InstanceNode::object();
                }
                apply_at_path(
                    &mut shadow[idx],
                    &segments[1..],
                    value,
                    shape_context,
                    "Element",
                );
            }
        } else {
            let arr = map.get_mut(&element_key).unwrap();
            if let InstanceNode::Array(arr) = arr {
                if segments.len() == 1 {
                    arr[idx] =
                        shape_value_for_field(value, fi.as_ref().map(FieldShape::field_type));
                } else {
                    apply_at_path(
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
        let fi = path_field_info(shape_context.schema_view, current_type, element);
        let next_type = fi
            .as_ref()
            .and_then(|f| field_next_type(f.field_type()))
            .unwrap_or(current_type);
        let is_primitive = fi
            .as_ref()
            .is_some_and(|field| matches!(field.field_type(), FhirFieldType::Primitive(_)));
        let is_array = fi.as_ref().is_none_or(|field| field.max().is_none());
        if !is_array && idx == 0 {
            if is_primitive && segments.len() > 1 {
                let shadow = map
                    .entry(format!("_{element}"))
                    .or_insert_with(InstanceNode::object);
                apply_at_path(shadow, &segments[1..], value, shape_context, "Element");
            } else if segments.len() == 1 {
                map.insert(
                    element.to_string(),
                    shape_value_for_field(value, fi.as_ref().map(FieldShape::field_type)),
                );
            } else {
                let child = map
                    .entry(element.to_string())
                    .or_insert_with(InstanceNode::object);
                apply_at_path(child, &segments[1..], value, shape_context, next_type);
            }
            return;
        }
        if is_primitive && segments.len() > 1 {
            let shadow = map
                .entry(format!("_{element}"))
                .or_insert_with(InstanceNode::array);
            if let InstanceNode::Array(values) = shadow {
                while values.len() <= idx {
                    values.push(InstanceNode::Null);
                }
                if values[idx].is_null() {
                    values[idx] = InstanceNode::object();
                }
                apply_at_path(
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
            .or_insert_with(InstanceNode::array);
        if let InstanceNode::Array(arr) = arr {
            while arr.len() <= idx {
                arr.push(InstanceNode::object());
            }
            if segments.len() == 1 {
                arr[idx] = shape_value_for_field(value, fi.as_ref().map(FieldShape::field_type));
            } else {
                apply_at_path(
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
    let fi = path_field_info(shape_context.schema_view, current_type, element);
    let next_type = fi
        .as_ref()
        .and_then(|f| field_next_type(f.field_type()))
        .unwrap_or(current_type);
    let parent_extension_url = map
        .get("url")
        .and_then(InstanceNode::as_str)
        .map(str::to_string);
    let key = element.to_string();
    let array = map.entry(key).or_insert_with(InstanceNode::array);
    let InstanceNode::Array(values) = array else {
        return;
    };
    if let Some(FshPathSegment::Index(repetition)) = segments.get(1) {
        let repetition = *repetition as usize;
        let mut template = values
            .iter()
            .find(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
            .cloned()
            .unwrap_or_else(|| {
                if element == "extension" {
                    named_extension_node(slice, parent_extension_url.as_deref(), shape_context)
                } else {
                    slice_marker_node(slice)
                }
            });
        if element == "extension" && template.get("url").is_none() {
            template.insert(
                "url",
                InstanceNode::String(named_extension_url(
                    slice,
                    parent_extension_url.as_deref(),
                    shape_context,
                )),
            );
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
        if element == "extension" && values[index].get("url").is_none() {
            values[index].insert(
                "url",
                InstanceNode::String(named_extension_url(
                    slice,
                    parent_extension_url.as_deref(),
                    shape_context,
                )),
            );
        }
        if segments.len() == 2 {
            values[index] = value;
        } else {
            apply_at_path(
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
            let item = if element == "extension" {
                named_extension_node(slice, parent_extension_url.as_deref(), shape_context)
            } else {
                slice_marker_node(slice)
            };
            let index = named_slice_insert_index(values, slice);
            values.insert(index, item);
            index
        });
    if let Some(object) = values[index].as_object_mut() {
        object
            .entry(SLICE_MARKER.to_string())
            .or_insert_with(|| InstanceNode::String(slice.to_string()));
    }
    if segments.len() == 1 {
        let mut value = shape_value_for_field(value, fi.as_ref().map(FieldShape::field_type));
        if let InstanceNode::Object(object) = &mut value {
            object.insert(
                SLICE_MARKER.to_string(),
                InstanceNode::String(slice.to_string()),
            );
        }
        values[index] = value;
    } else {
        apply_at_path(
            &mut values[index],
            &segments[1..],
            value,
            shape_context,
            next_type,
        );
    }
}

fn slice_marker_node(slice: &str) -> InstanceNode {
    InstanceNode::Object(BTreeMap::from([(
        SLICE_MARKER.to_string(),
        InstanceNode::String(slice.to_string()),
    )]))
}

fn named_slice_insert_index(values: &[InstanceNode], slice: &str) -> usize {
    if let Some(index) = values
        .iter()
        .rposition(|item| item.get(SLICE_MARKER).and_then(|value| value.as_str()) == Some(slice))
    {
        return index + 1;
    }
    values.len()
}

fn named_extension_node(
    slice: &str,
    parent_url: Option<&str>,
    shape_context: &InstanceShapeContext<'_>,
) -> InstanceNode {
    let url = named_extension_url(slice, parent_url, shape_context);
    let mut node = slice_marker_node(slice);
    node.insert("url", InstanceNode::String(url.clone()));
    materialize_required_extension_slices(&mut node, &url, shape_context, &mut HashSet::new());
    node
}

fn materialize_required_extension_slices(
    node: &mut InstanceNode,
    extension_url: &str,
    shape_context: &InstanceShapeContext<'_>,
    seen: &mut HashSet<String>,
) {
    if !seen.insert(extension_url.to_string()) {
        return;
    }
    let slices = shape_context
        .schema_view
        .required_extension_slices(extension_url)
        .cloned()
        .collect::<Vec<_>>();
    if !slices.is_empty() {
        let mut children = Vec::new();
        for slice in slices {
            for _ in 0..slice.min {
                let mut child = slice_marker_node(&slice.name);
                child.insert("url", InstanceNode::String(slice.url.clone()));
                materialize_required_extension_slices(&mut child, &slice.url, shape_context, seen);
                children.push(child);
            }
        }
        node.insert("extension", InstanceNode::Array(children));
    }
    seen.remove(extension_url);
}

fn named_extension_url(
    slice: &str,
    parent_url: Option<&str>,
    shape_context: &InstanceShapeContext<'_>,
) -> String {
    shape_context
        .schema_view
        .extension_url(parent_url, slice)
        .map(str::to_string)
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
    fn typed_node_serialization_is_deterministic_and_hides_internal_markers() {
        let node = InstanceNode::Object(BTreeMap::from([
            ("z".to_string(), InstanceNode::from(serde_json::json!(1))),
            ("a".to_string(), InstanceNode::from(serde_json::json!(2))),
            (
                "empty".to_string(),
                InstanceNode::Array(vec![slice_marker_node("unused")]),
            ),
            (
                SLICE_MARKER.to_string(),
                InstanceNode::String("root".to_string()),
            ),
        ]));

        let serialized = serde_json::to_string(&node.into_json()).expect("node serializes");

        assert_eq!(serialized, r#"{"a":2,"z":1}"#);
    }

    #[test]
    fn shapes_coded_values_inside_inline_element_metadata() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: medication-request
InstanceOf: MedicationRequest
* status = #active
* intent = #order
* medicationCodeableConcept = #example
* subject = Reference(Patient/example)
* dosageInstruction.timing.repeat.periodUnit = UCUM#d
"#,
                "inline-element-metadata.fsh",
            )
            .expect("FSH compiles");
        let resource = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "medication-request")
            .expect("instance exported");

        assert_eq!(
            resource["dosageInstruction"][0]["timing"]["repeat"]["periodUnit"],
            "d"
        );
    }

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
    fn keeps_references_scalar_inside_indexed_backbone_elements() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: eob-example
InstanceOf: ExplanationOfBenefit
* insurance[0].coverage[+] = Reference(Coverage/example)
* insurance[0].focal = true
"#,
                "indexed-backbone-reference.fsh",
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
    fn reuses_soft_index_path_context_for_nested_assignments() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: parameters-example
InstanceOf: Parameters
* parameter[+]
  * name = "response"
  * valueReference = Reference(Patient/example)
"#,
                "soft-index-context.fsh",
            )
            .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "parameters-example")
            .expect("instance exists");

        assert_eq!(instance["parameter"].as_array().unwrap().len(), 1);
        assert_eq!(instance["parameter"][0]["name"], "response");
        assert_eq!(
            instance["parameter"][0]["valueReference"]["reference"],
            "Patient/example"
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
    fn resolves_instance_urls_and_inline_references() {
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org/fhir".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(
            r#"
Instance: questionnaire-source
InstanceOf: Questionnaire
* url = "http://example.org/questionnaire-source"

Instance: questionnaire-copy
InstanceOf: Questionnaire
* derivedFrom = Canonical(questionnaire-source)

Instance: contained-payer
InstanceOf: Organization
Usage: #inline

Instance: payer-list
InstanceOf: List
* contained[0] = contained-payer
* entry[0].item = Reference(contained-payer)

Instance: capability-example
InstanceOf: CapabilityStatement
* url = Canonical(capability-example)
* rest
  * resource[0]
    * extension[0]
      * url = "http://example.org/nested-extension"
"#,
            "instance-targets.fsh",
        )
        .expect("FSH compiles");
        let questionnaire = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "questionnaire-copy")
            .expect("questionnaire exists");
        let list = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "payer-list")
            .expect("list exists");
        let capability = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "capability-example")
            .expect("capability exists");

        assert_eq!(
            questionnaire["derivedFrom"][0],
            "http://example.org/questionnaire-source"
        );
        assert_eq!(list["entry"][0]["item"]["reference"], "#contained-payer");
        assert_eq!(
            capability["url"],
            "http://example.org/fhir/CapabilityStatement/capability-example"
        );
    }

    #[test]
    fn exports_r5_actor_derived_from_as_an_array() {
        let package = crate::FshCompiler::new(crate::CompilerOptions::default())
            .compile(
                r#"
Instance: actor-example
InstanceOf: ActorDefinition
* derivedFrom = "http://example.org/ActorDefinition/base"
* jurisdiction = urn:iso:std:iso:3166#US
"#,
                "actor-derived-from.fsh",
            )
            .expect("FSH compiles");
        let actor = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "actor-example")
            .expect("actor exists");

        assert_eq!(
            actor["derivedFrom"],
            serde_json::json!(["http://example.org/ActorDefinition/base"])
        );
        assert_eq!(actor["jurisdiction"][0]["coding"][0]["code"], "US");
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
    fn materializes_required_nested_extensions_before_optional_assignments() {
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org/fhir".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(
            r#"
Extension: RequiredChild
Id: required-child
* value[x] only CodeableConcept

Extension: OptionalChild
Id: optional-child
* value[x] only Reference

Extension: ComplexExtension
Id: complex-extension
* extension contains OptionalChild named optional 0..1 and RequiredChild named required 1..1

Profile: NestedExtensionPatient
Parent: Patient
* extension contains ComplexExtension named complex 0..1

Instance: nested-extension-patient
InstanceOf: NestedExtensionPatient
* extension[complex].extension[optional].valueReference = Reference(Practitioner/example)
"#,
            "required-nested-extension.fsh",
        )
        .expect("FSH compiles");
        let instance = package
            .resources
            .iter()
            .find(|resource| resource["id"] == "nested-extension-patient")
            .expect("instance exists");

        assert_eq!(
            instance["extension"][0]["extension"][0]["url"],
            "http://example.org/fhir/StructureDefinition/required-child"
        );
        assert_eq!(
            instance["extension"][0]["extension"][1]["url"],
            "http://example.org/fhir/StructureDefinition/optional-child"
        );
        assert_eq!(
            instance["extension"][0]["extension"][1]["valueReference"]["reference"],
            "Practitioner/example"
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
    fn omits_unused_required_extension_placeholders() {
        let package = crate::FshCompiler::new(crate::CompilerOptions {
            config: FshConfig {
                canonical: Some("http://example.org/fhir".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .compile(
            r#"
Extension: RequiredExtension
Id: required-extension
* value[x] only string

Profile: RequiredExtensionRequest
Parent: CommunicationRequest
* extension contains RequiredExtension named required 1..1

Instance: request-example
InstanceOf: RequiredExtensionRequest
"#,
            "required-extension-url.fsh",
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
        let schema = CompiledSchema::compile(&tank, defs.as_ref(), &definition_index);
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

            let resource = export_instance(
                &inst,
                defs.as_ref(),
                &config,
                &tank,
                &definition_index,
                &schema,
            )
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
