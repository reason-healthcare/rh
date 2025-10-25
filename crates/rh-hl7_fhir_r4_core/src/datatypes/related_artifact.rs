use crate::bindings::related_artifact_type::RelatedArtifactType;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::element::Element;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// RelatedArtifact
///
/// Base StructureDefinition for RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: RelatedArtifact
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedArtifact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of
    #[serde(rename = "type")]
    pub type_: RelatedArtifactType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Short label
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Brief description of the related artifact
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Bibliographic citation for the artifact
    pub citation: Option<StringType>,
    /// Extension element for the 'citation' primitive field. Contains metadata and extensions.
    pub _citation: Option<Element>,
    /// Where the artifact can be accessed
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// What document is being referenced
    pub document: Option<Attachment>,
    /// What resource is being referenced
    pub resource: Option<StringType>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
}

impl Default for RelatedArtifact {
    fn default() -> Self {
        Self {
            base: Element::default(),
            type_: Default::default(),
            _type: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            citation: Default::default(),
            _citation: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            document: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "RelatedArtifact.type",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/related-artifact-type|4.0.1",
        )
        .with_description("The type of relationship to the related artifact.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RelatedArtifact.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.extension", 0, None),
            rh_foundation::ElementCardinality::new("RelatedArtifact.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.citation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.document", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.resource", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for RelatedArtifact {
    fn resource_type(&self) -> &'static str {
        "RelatedArtifact"
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
        Some("http://hl7.org/fhir/StructureDefinition/RelatedArtifact")
    }
}
