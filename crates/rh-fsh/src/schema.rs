//! Immutable schema index used by semantic resolution and export.

use crate::definition_index::DefinitionIndex;
use crate::fhirdefs::FhirDefs;
use crate::parser::ast::{FshPathSegment, InstanceRule, SdRule};
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

/// Borrowed view of either generated core metadata or a compiled shape.
#[derive(Debug)]
pub enum FieldShape<'a> {
    Core(&'static FieldInfo),
    Compiled(&'a ElementShape),
    Owned(ElementShape),
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

        schema
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
    use crate::{build_definition_index, DependencyDefinitionSet, FshConfig, FshParser};

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
}
