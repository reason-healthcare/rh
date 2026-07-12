//! Immutable schema index used by semantic resolution and export.

use crate::definition_index::DefinitionIndex;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::{FshPathSegment, InstanceRule, SdRule};
use crate::parser::Spanned;
use crate::tank::FshTank;
use rh_hl7_fhir_r4_core::metadata::{get_field_info, FhirFieldType, FhirPrimitiveType, FieldInfo};
use std::collections::HashMap;

/// Schema information needed to shape one JSON field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementShape {
    pub field_type: FhirFieldType,
    pub min: u32,
    pub max: Option<u32>,
    pub is_choice_type: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExtensionSliceShape {
    pub name: String,
    pub url: String,
    pub min: u32,
}

struct CompatibilityField {
    owner: &'static str,
    field: &'static str,
    shape: ElementShape,
}

/// Declarative shapes absent from the generated R4 registry but required by the
/// pinned mixed-version conformance corpus. This table replaces exporter branches
/// and can be selected by FHIR-version-specific schema providers in a later slice.
static COMPATIBILITY_FIELDS: &[CompatibilityField] = &[
    CompatibilityField {
        owner: "ActorDefinition",
        field: "derivedFrom",
        shape: ElementShape {
            field_type: FhirFieldType::Primitive(FhirPrimitiveType::Canonical),
            min: 0,
            max: None,
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "ActorDefinition",
        field: "jurisdiction",
        shape: ElementShape {
            field_type: FhirFieldType::Complex("CodeableConcept"),
            min: 0,
            max: None,
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "ActorDefinition",
        field: "title",
        shape: ElementShape {
            field_type: FhirFieldType::Primitive(FhirPrimitiveType::String),
            min: 0,
            max: Some(1),
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "FamilyMemberHistory",
        field: "condition",
        shape: ElementShape {
            field_type: FhirFieldType::BackboneElement("FamilyMemberHistoryCondition"),
            min: 0,
            max: None,
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "FamilyMemberHistory",
        field: "relationship",
        shape: ElementShape {
            field_type: FhirFieldType::Complex("CodeableConcept"),
            min: 1,
            max: Some(1),
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "ExplanationOfBenefit",
        field: "adjudication",
        shape: ElementShape {
            field_type: FhirFieldType::BackboneElement("ExplanationOfBenefitItemAdjudication"),
            min: 0,
            max: None,
            is_choice_type: false,
        },
    },
    CompatibilityField {
        owner: "ClaimResponse",
        field: "adjudication",
        shape: ElementShape {
            field_type: FhirFieldType::BackboneElement("ClaimResponseItemAdjudication"),
            min: 0,
            max: None,
            is_choice_type: false,
        },
    },
];

/// Borrowed view of either generated core metadata or a compiled shape.
#[derive(Debug)]
pub enum FieldShape<'a> {
    Core(&'static FieldInfo),
    Compiled(&'a ElementShape),
    Owned(ElementShape),
}

/// Pre-resolved profile lineage and base type for one observed profile reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledProfileView {
    root_type: String,
    lineage: Vec<String>,
}

impl CompiledProfileView {
    pub fn root_type(&self) -> &str {
        &self.root_type
    }

    pub fn lineage(&self) -> &[String] {
        &self.lineage
    }
}

/// Lightweight profile-aware view over the immutable compiled schema.
#[derive(Debug, Clone, Copy)]
pub struct SchemaView<'a> {
    schema: &'a CompiledSchema,
    profile: Option<&'a CompiledProfileView>,
    fallback_root_type: &'a str,
}

impl<'a> SchemaView<'a> {
    pub fn root_type(&self) -> &str {
        self.profile.map_or(self.fallback_root_type, |profile| {
            profile.root_type.as_str()
        })
    }

    pub fn profile_lineage(&self) -> &[String] {
        self.profile.map_or(&[], |profile| profile.lineage())
    }

    pub fn field(&self, type_name: &str, field_name: &str) -> Option<FieldShape<'a>> {
        self.schema.field(type_name, field_name)
    }

    pub(crate) fn extension_url(&self, parent_url: Option<&str>, slice: &str) -> Option<&'a str> {
        self.schema.extension_url(parent_url, slice)
    }

    pub(crate) fn required_extension_slices(
        &self,
        extension_url: &str,
    ) -> impl Iterator<Item = &'a ExtensionSliceShape> {
        self.schema.required_extension_slices(extension_url)
    }
}

impl FieldShape<'_> {
    pub fn field_type(&self) -> &FhirFieldType {
        match self {
            Self::Core(field) => &field.field_type,
            Self::Compiled(field) => &field.field_type,
            Self::Owned(field) => &field.field_type,
        }
    }

    pub fn min(&self) -> u32 {
        match self {
            Self::Core(field) => field.min,
            Self::Compiled(field) => field.min,
            Self::Owned(field) => field.min,
        }
    }

    pub fn max(&self) -> Option<u32> {
        match self {
            Self::Core(field) => field.max,
            Self::Compiled(field) => field.max,
            Self::Owned(field) => field.max,
        }
    }
}

/// Read-only, pre-resolved field shapes shared by parallel exporters.
#[derive(Debug, Default)]
pub struct CompiledSchema {
    fields: HashMap<String, HashMap<String, ElementShape>>,
    profiles: HashMap<String, CompiledProfileView>,
    extension_urls: HashMap<String, String>,
    extension_slices: HashMap<String, Vec<ExtensionSliceShape>>,
}

impl CompiledSchema {
    /// Compile the field paths observable in the resolved tank.
    pub fn compile(tank: &FshTank, defs: &FhirDefs, definitions: &DefinitionIndex) -> Self {
        let mut schema = Self::default();

        for instance in tank.instances.values() {
            let root_type = definitions
                .resolve_base_type(&instance.metadata.instance_of, defs)
                .unwrap_or_else(|| instance.metadata.instance_of.clone());
            for rule in &instance.rules {
                match &rule.value {
                    InstanceRule::Assignment(assignment) => {
                        schema.compile_path(&root_type, &assignment.path.segments)
                    }
                    InstanceRule::Path(path) => {
                        schema.compile_path(&root_type, &path.path.segments)
                    }
                    InstanceRule::Insert(_) => {}
                }
            }
        }

        for profile in tank.profiles.values() {
            schema.compile_extension_urls(&profile.rules, definitions);
            let root_type = definitions
                .resolve_base_type(&profile.metadata.name, defs)
                .or_else(|| {
                    profile
                        .metadata
                        .parent
                        .as_deref()
                        .and_then(|parent| definitions.resolve_base_type(parent, defs))
                });
            let Some(root_type) = root_type else {
                continue;
            };
            for rule in &profile.rules {
                match &rule.value {
                    SdRule::Assignment(assignment) => {
                        schema.compile_path(&root_type, &assignment.path.segments)
                    }
                    SdRule::Path(path) => schema.compile_path(&root_type, &path.path.segments),
                    _ => {}
                }
            }
        }

        for extension in tank.extensions.values() {
            schema.compile_extension_urls(&extension.rules, definitions);
            let Some(definition) = definitions.lookup(&extension.metadata.name) else {
                continue;
            };
            let slices = compile_extension_slices(&extension.rules, definitions);
            if slices.is_empty() {
                continue;
            }
            for key in [
                definition.name.as_deref(),
                definition.id.as_deref(),
                definition.url.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                schema
                    .extension_slices
                    .entry(key.to_string())
                    .or_insert_with(|| slices.clone());
            }
        }

        for definition in definitions.definitions() {
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
                    schema
                        .extension_urls
                        .entry(key)
                        .or_insert_with(|| url.clone());
                }
            }
        }

        let mut observed_profiles = tank
            .instances
            .values()
            .map(|instance| instance.metadata.instance_of.as_str())
            .collect::<Vec<_>>();
        observed_profiles.sort_unstable();
        observed_profiles.dedup();
        for profile_ref in observed_profiles {
            if let Some(view) = compile_profile_view(profile_ref, tank, defs, definitions) {
                schema.profiles.insert(profile_ref.to_string(), view);
            }
        }

        schema
    }

    pub fn view<'a>(&'a self, profile_ref: &str, fallback_root_type: &'a str) -> SchemaView<'a> {
        let profile = self.profiles.get(profile_ref).or_else(|| {
            profile_ref
                .split_once('|')
                .and_then(|(canonical, _)| self.profiles.get(canonical))
        });
        SchemaView {
            schema: self,
            profile,
            fallback_root_type,
        }
    }

    /// Return generated core metadata first, then compiled dependency/local shapes.
    pub fn field<'a>(&'a self, type_name: &str, field_name: &str) -> Option<FieldShape<'a>> {
        if let Some(field) = get_field_info(type_name, field_name) {
            return Some(FieldShape::Core(field));
        }
        if let Some(field) = self
            .fields
            .get(type_name)
            .and_then(|fields| fields.get(field_name))
        {
            return Some(FieldShape::Compiled(field));
        }
        if let Some(field) = COMPATIBILITY_FIELDS
            .iter()
            .find(|field| field.owner == type_name && field.field == field_name)
        {
            return Some(FieldShape::Compiled(&field.shape));
        }
        if let Some(field) = concrete_choice_shape(type_name, field_name) {
            return Some(FieldShape::Owned(field));
        }
        (!field_name.ends_with("[x]"))
            .then(|| format!("{field_name}[x]"))
            .and_then(|choice| get_field_info(type_name, &choice))
            .map(FieldShape::Core)
    }

    pub fn cached_type_count(&self) -> usize {
        self.fields.len()
    }

    pub fn cached_field_count(&self) -> usize {
        self.fields.values().map(HashMap::len).sum()
    }

    pub fn cached_profile_count(&self) -> usize {
        self.profiles.len()
    }

    pub(crate) fn extension_url(&self, parent_url: Option<&str>, slice: &str) -> Option<&str> {
        parent_url
            .and_then(|url| self.extension_slices.get(url))
            .and_then(|slices| slices.iter().find(|candidate| candidate.name == slice))
            .map(|candidate| candidate.url.as_str())
            .or_else(|| {
                extension_lookup_keys(slice)
                    .into_iter()
                    .find_map(|key| self.extension_urls.get(&key).map(String::as_str))
            })
    }

    pub(crate) fn required_extension_slices(
        &self,
        extension_url: &str,
    ) -> impl Iterator<Item = &ExtensionSliceShape> {
        self.extension_slices
            .get(extension_url)
            .into_iter()
            .flatten()
            .filter(|slice| slice.min > 0)
    }

    fn compile_extension_urls(&mut self, rules: &[Spanned<SdRule>], definitions: &DefinitionIndex) {
        for slice in compile_extension_slices(rules, definitions) {
            for key in extension_lookup_keys(&slice.name) {
                self.extension_urls
                    .entry(key)
                    .or_insert_with(|| slice.url.clone());
            }
        }
    }

    fn compile_path(&mut self, root_type: &str, segments: &[FshPathSegment]) {
        let mut current_type = root_type.to_string();
        for segment in segments {
            let field_name = match segment {
                FshPathSegment::Name(name)
                | FshPathSegment::ChoiceType(name)
                | FshPathSegment::Slice { element: name, .. } => name.as_str(),
                FshPathSegment::Index(_) | FshPathSegment::Extension(_) => continue,
            };
            let Some(shape) = resolved_shape(&current_type, field_name) else {
                continue;
            };
            if get_field_info(&current_type, field_name).is_none() {
                self.fields
                    .entry(current_type.clone())
                    .or_default()
                    .entry(field_name.to_string())
                    .or_insert_with(|| shape.clone());
            }
            if let Some(next_type) = navigable_type(&shape.field_type) {
                current_type = next_type.to_string();
            }
        }
    }
}

fn compile_extension_slices(
    rules: &[Spanned<SdRule>],
    definitions: &DefinitionIndex,
) -> Vec<ExtensionSliceShape> {
    rules
        .iter()
        .filter_map(|rule| match &rule.value {
            SdRule::Contains(contains)
                if matches!(contains.path.segments.last(), Some(FshPathSegment::Name(name)) if name == "extension") =>
            {
                Some(&contains.items)
            }
            _ => None,
        })
        .flatten()
        .filter_map(|item| {
            let url = definitions.lookup(&item.name)?.url.clone()?;
            Some(ExtensionSliceShape {
                name: item.alias.clone().unwrap_or_else(|| item.name.clone()),
                url,
                min: item.min.unwrap_or(0),
            })
        })
        .collect()
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

fn compile_profile_view(
    profile_ref: &str,
    tank: &FshTank,
    defs: &FhirDefs,
    definitions: &DefinitionIndex,
) -> Option<CompiledProfileView> {
    let root_type = definitions.resolve_base_type(profile_ref, defs)?;
    let mut lineage = Vec::new();
    let mut current = Some(profile_ref.to_string());
    let mut seen = std::collections::HashSet::new();
    while let Some(profile_name) = current {
        if !seen.insert(profile_name.clone()) {
            break;
        }
        lineage.push(profile_name.clone());
        current = tank
            .profiles
            .get(&profile_name)
            .and_then(|profile| profile.metadata.parent.clone())
            .or_else(|| {
                definitions.lookup(&profile_name).and_then(|definition| {
                    definition
                        .parent
                        .clone()
                        .or_else(|| definition.base_definition.clone())
                })
            });
        if current
            .as_deref()
            .is_some_and(|parent| defs.get_sd(parent).is_some())
        {
            if let Some(parent) = current.take() {
                lineage.push(parent);
            }
        }
    }
    Some(CompiledProfileView { root_type, lineage })
}

fn navigable_type(field_type: &FhirFieldType) -> Option<&str> {
    match field_type {
        FhirFieldType::Complex(name) | FhirFieldType::BackboneElement(name) => Some(name),
        _ => None,
    }
}

fn resolved_shape(type_name: &str, field_name: &str) -> Option<ElementShape> {
    get_field_info(type_name, field_name)
        .map(element_shape)
        .or_else(|| concrete_choice_shape(type_name, field_name))
        .or_else(|| {
            (!field_name.ends_with("[x]"))
                .then(|| format!("{field_name}[x]"))
                .and_then(|choice| get_field_info(type_name, &choice))
                .map(element_shape)
        })
}

fn element_shape(field: &'static FieldInfo) -> ElementShape {
    ElementShape {
        field_type: field.field_type.clone(),
        min: field.min,
        max: field.max,
        is_choice_type: field.is_choice_type,
    }
}

fn concrete_choice_shape(type_name: &str, field_name: &str) -> Option<ElementShape> {
    let (base, suffix) = field_name.char_indices().find_map(|(index, character)| {
        (index > 0 && character.is_ascii_uppercase()).then(|| field_name.split_at(index))
    })?;
    let choice = get_field_info(type_name, &format!("{base}[x]"))?;
    Some(ElementShape {
        field_type: choice_type(suffix)?,
        min: choice.min,
        max: choice.max,
        is_choice_type: true,
    })
}

fn choice_type(suffix: &str) -> Option<FhirFieldType> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        build_definition_index, DependencyDefinitionSet, DependencyStructureDefinition, FshConfig,
        FshParser,
    };
    use std::path::PathBuf;

    #[test]
    fn compiles_observed_instance_paths() {
        let document = FshParser::parse(
            "Instance: example\nInstanceOf: Observation\n* valueString = \"normal\"\n",
            "schema.fsh",
        )
        .expect("FSH parses");
        let mut tank = FshTank::new();
        tank.add_document(document).expect("document indexes");
        let defs = FhirDefs::r4();
        let definitions = build_definition_index(
            &tank,
            &FshConfig::default(),
            &DependencyDefinitionSet::default(),
        );

        let schema = CompiledSchema::compile(&tank, &defs, &definitions);

        assert!(schema.cached_type_count() > 0);
        assert_eq!(schema.cached_field_count(), 1);
        let field = schema.field("Observation", "valueString").unwrap();
        assert_eq!(
            field.field_type(),
            &FhirFieldType::Primitive(FhirPrimitiveType::String)
        );
        assert_eq!(field.max(), Some(1));
    }

    #[test]
    fn compiles_profile_aware_views_for_observed_instances() {
        let document = FshParser::parse(
            "Profile: LocalPatient\nParent: Patient\n\nInstance: example\nInstanceOf: LocalPatient\n* active = true\n",
            "schema-view.fsh",
        )
        .expect("FSH parses");
        let mut tank = FshTank::new();
        tank.add_document(document).expect("document indexes");
        let defs = FhirDefs::r4();
        let definitions = build_definition_index(
            &tank,
            &FshConfig::default(),
            &DependencyDefinitionSet::default(),
        );

        let schema = CompiledSchema::compile(&tank, &defs, &definitions);
        let view = schema.view("LocalPatient", "Resource");

        assert_eq!(schema.cached_profile_count(), 1);
        assert_eq!(view.root_type(), "Patient");
        assert_eq!(view.profile_lineage()[0], "LocalPatient");
        assert!(view
            .profile_lineage()
            .last()
            .is_some_and(|parent| parent == "Patient"));
        assert!(view.field("Patient", "active").is_some());
    }

    #[test]
    fn compiles_versioned_dependency_profile_lineage() {
        let document = FshParser::parse(
            "Instance: example\nInstanceOf: USCorePatientProfile|6.1.0\n* active = true\n",
            "dependency-schema-view.fsh",
        )
        .expect("FSH parses");
        let mut tank = FshTank::new();
        tank.add_document(document).expect("document indexes");
        let dependencies = DependencyDefinitionSet {
            structure_definitions: vec![DependencyStructureDefinition {
                package_id: "hl7.fhir.us.core".to_string(),
                version: "6.1.0".to_string(),
                path: PathBuf::from("StructureDefinition-us-core-patient.json"),
                id: Some("us-core-patient".to_string()),
                url: Some(
                    "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient".to_string(),
                ),
                name: Some("USCorePatientProfile".to_string()),
                title: None,
                kind: Some("resource".to_string()),
                type_: Some("Patient".to_string()),
                base_definition: Some(
                    "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
                ),
                derivation: Some("constraint".to_string()),
                fixed_values: Vec::new(),
            }],
            warnings: Vec::new(),
        };
        let defs = FhirDefs::r4();
        let definitions = build_definition_index(&tank, &FshConfig::default(), &dependencies);

        let schema = CompiledSchema::compile(&tank, &defs, &definitions);
        let view = schema.view("USCorePatientProfile|6.1.0", "Resource");

        assert_eq!(view.root_type(), "Patient");
        assert_eq!(view.profile_lineage()[0], "USCorePatientProfile|6.1.0");
        assert!(view
            .profile_lineage()
            .iter()
            .any(|parent| parent.ends_with("/Patient")));
    }
}
