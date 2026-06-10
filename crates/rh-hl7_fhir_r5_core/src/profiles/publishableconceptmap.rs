use crate::profiles::shareableconceptmap::Shareableconceptmap;
use serde::{Deserialize, Serialize};
/// Publishable ConceptMap
///
/// Defines and enforces the minimum expectations for publication and distribution of a concept map, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishableconceptmap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareableconceptmap
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Publishableconceptmap {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareableconceptmap,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cmd-1", rh_foundation::Severity::Error, "If the map is source-is-broader-than-target or not-related-to, there SHALL be some comments, unless the status is 'draft'", "comment.exists() or (%resource.status = 'draft') or relationship.empty() or ((relationship != 'source-is-broader-than-target') and (relationship != 'not-related-to'))"),
    rh_foundation::Invariant::new("cmd-10", rh_foundation::Severity::Error, "If the mode is not 'other-map', otherMap is not allowed", "(mode != 'other-map') implies otherMap.empty()"),
    rh_foundation::Invariant::new("cmd-11", rh_foundation::Severity::Error, "If the property type is code, a system SHALL be specified", "type = 'code' implies system.exists()"),
    rh_foundation::Invariant::new("cmd-2", rh_foundation::Severity::Error, "If the mode is 'fixed', either a code or valueSet must be provided, but not both.", "(mode = 'fixed') implies ((code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists()))"),
    rh_foundation::Invariant::new("cmd-3", rh_foundation::Severity::Error, "If the mode is 'other-map', a url for the other map must be provided", "(mode = 'other-map') implies otherMap.exists()"),
    rh_foundation::Invariant::new("cmd-4", rh_foundation::Severity::Error, "If noMap is present, target SHALL NOT be present", "(noMap.exists() and noMap=true) implies target.empty()"),
    rh_foundation::Invariant::new("cmd-5", rh_foundation::Severity::Error, "Either code or valueSet SHALL be present but not both.", "(code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-6", rh_foundation::Severity::Error, "One of value[x] or valueSet must exist, but not both.", "(value.exists() and valueSet.empty()) or (value.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-7", rh_foundation::Severity::Error, "Either code or valueSet SHALL be present but not both.", "(code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-8", rh_foundation::Severity::Error, "If the mode is not 'fixed', code, display and valueSet are not allowed", "(mode != 'fixed') implies (code.empty() and display.empty() and valueSet.empty())"),
    rh_foundation::Invariant::new("cmd-9", rh_foundation::Severity::Error, "If the mode is not 'other-map', relationship must be provided", "(mode != 'other-map') implies relationship.exists()"),
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ConceptMap.additionalAttribute.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-attribute-type|5.0.0").with_description("The type of a mapping attribute value."),
    rh_foundation::ElementBinding::new("ConceptMap.group.element.target.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-map-relationship|5.0.0").with_description("The relationship between concepts."),
    rh_foundation::ElementBinding::new("ConceptMap.group.unmapped.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-unmapped-mode|5.0.0").with_description("Defines which action to take if there is no match in the group."),
    rh_foundation::ElementBinding::new("ConceptMap.group.unmapped.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-map-relationship|5.0.0").with_description("The default relationship value to apply between the source and target concepts when no concept mapping is specified."),
    rh_foundation::ElementBinding::new("ConceptMap.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("ConceptMap.property.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-property-type|5.0.0").with_description("The type of a property value."),
    rh_foundation::ElementBinding::new("ConceptMap.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ConceptMap", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.contained", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.extension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.extension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.contact", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.topic", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.author", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.editor", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.endorser", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.property", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.uri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.additionalAttribute", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.additionalAttribute.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.uri",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.sourceScope[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.targetScope[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.extension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.source", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.target", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element", 1, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.valueSet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.noMap", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.target", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.comment",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.attribute",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.product",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.relationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.otherMap",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Publishableconceptmap {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for Publishableconceptmap {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Publishableconceptmap {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::publishableconceptmap::PublishableconceptmapMutators for Publishableconceptmap {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Publishableconceptmap {
    fn resource_type(&self) -> &'static str {
        "ConceptMap"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/publishableconceptmap")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::publishableconceptmap::{
    PublishableconceptmapAccessors, PublishableconceptmapExistence, PublishableconceptmapMutators,
};
