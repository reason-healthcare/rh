use crate::profiles::shareablelibrary::Shareablelibrary;
use serde::{Deserialize, Serialize};
/// Logic Library
///
/// The logic library profile sets the minimum expectations for computable and/or executable libraries, including support for terminology and dependency declaration, parameters, and data requirements
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/logiclibrary
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Library
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/shareablelibrary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Logiclibrary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Shareablelibrary,
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
                "Library.content.contentType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "Library.content.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language."),
            rh_foundation::ElementBinding::new(
                "Library.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Library.relatedArtifact.publicationStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("Publication status of an artifact being referred to."),
            rh_foundation::ElementBinding::new(
                "Library.relatedArtifact.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/related-artifact-type|5.0.0",
            )
            .with_description("The type of relationship to the related artifact."),
            rh_foundation::ElementBinding::new(
                "Library.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Library", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.contained", 0, None),
            rh_foundation::ElementCardinality::new("Library.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Library.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Library.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.contact", 0, None),
            rh_foundation::ElementCardinality::new("Library.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Library.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("Library.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.topic", 0, None),
            rh_foundation::ElementCardinality::new("Library.author", 0, None),
            rh_foundation::ElementCardinality::new("Library.editor", 0, None),
            rh_foundation::ElementCardinality::new("Library.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("Library.endorser", 0, None),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.classifier", 0, None),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.citation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.document", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.relatedArtifact.resource", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Library.relatedArtifact.resourceReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Library.relatedArtifact.publicationStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Library.relatedArtifact.publicationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Library.parameter", 0, None),
            rh_foundation::ElementCardinality::new("Library.dataRequirement", 0, None),
            rh_foundation::ElementCardinality::new("Library.content", 0, None),
            rh_foundation::ElementCardinality::new("Library.content.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.extension", 0, None),
            rh_foundation::ElementCardinality::new("Library.content.contentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.data", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.size", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.hash", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.creation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.height", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.width", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.frames", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.duration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Library.content.pages", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Logiclibrary {
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

impl crate::traits::resource::ResourceMutators for Logiclibrary {
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

impl crate::traits::resource::ResourceExistence for Logiclibrary {
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

impl crate::traits::logiclibrary::LogiclibraryMutators for Logiclibrary {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Logiclibrary {
    fn resource_type(&self) -> &'static str {
        "Library"
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
        Some("http://hl7.org/fhir/StructureDefinition/logiclibrary")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::logiclibrary::{
    LogiclibraryAccessors, LogiclibraryExistence, LogiclibraryMutators,
};
