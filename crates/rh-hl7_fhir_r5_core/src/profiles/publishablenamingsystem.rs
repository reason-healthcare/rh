use crate::profiles::shareablenamingsystem::Shareablenamingsystem;
use serde::{Deserialize, Serialize};
/// Publishable NamingSystem
///
/// Defines and enforces the minimum expectations for publication and distribution of a naming system, typically as part of an artifact repository or implementation guide publication
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/publishablenamingsystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablenamingsystem
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Publishablenamingsystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareablenamingsystem,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("nsd-1", rh_foundation::Severity::Error, "Root systems cannot have uuid identifiers", "kind != 'root' or uniqueId.all(type != 'uuid')"),
    rh_foundation::Invariant::new("nsd-2", rh_foundation::Severity::Error, "Can't have more than one preferred identifier for a type", "uniqueId.where(preferred = true).select(type).isDistinct()"),
    rh_foundation::Invariant::new("nsd-3", rh_foundation::Severity::Error, "Can't have more than one authoritative identifier for a type/period combination (only one authoritative identifier allowed at any given point of time)", "uniqueId.where(authoritative = 'true').select(type.toString() & period.start.toString() & period.end.toString()).isDistinct()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "NamingSystem.kind",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-type|5.0.0",
            )
            .with_description("Identifies the purpose of the naming system."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "NamingSystem.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.uniqueId.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-identifier-type|5.0.0",
            )
            .with_description(
                "Identifies the style of unique identifier used to identify a namespace.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NamingSystem", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contained", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contact", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.responsible", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.useContext", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.topic", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.author", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.editor", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.endorser", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId", 1, None),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NamingSystem.uniqueId.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.preferred", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NamingSystem.uniqueId.authoritative",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Publishablenamingsystem {
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

impl crate::traits::resource::ResourceMutators for Publishablenamingsystem {
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

impl crate::traits::resource::ResourceExistence for Publishablenamingsystem {
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

impl crate::traits::publishablenamingsystem::PublishablenamingsystemMutators
    for Publishablenamingsystem
{
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Publishablenamingsystem {
    fn resource_type(&self) -> &'static str {
        "NamingSystem"
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
        Some("http://hl7.org/fhir/StructureDefinition/publishablenamingsystem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::publishablenamingsystem::{
    PublishablenamingsystemAccessors, PublishablenamingsystemExistence,
    PublishablenamingsystemMutators,
};
