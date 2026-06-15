use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::related_artifact_type::RelatedArtifactType;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::date::DateType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// RelatedArtifact
///
/// RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: RelatedArtifact
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedArtifact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as
    #[serde(rename = "type")]
    pub type_: RelatedArtifactType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Additional classifiers
    ///
    /// Binding: example (Additional classifiers for the related artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/citation-artifact-classifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<CodeableConcept>,
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
    /// What document is being referenced
    pub document: Option<Attachment>,
    /// What artifact is being referenced
    pub resource: Option<StringType>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// What artifact, if not a conformance resource
    #[serde(rename = "resourceReference")]
    pub resource_reference: Option<Reference>,
    /// draft | active | retired | unknown
    #[serde(rename = "publicationStatus")]
    pub publication_status: Option<PublicationStatus>,
    /// Extension element for the 'publicationStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationStatus")]
    pub _publication_status: Option<Element>,
    /// Date of publication of the artifact being referred to
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<DateType>,
    /// Extension element for the 'publicationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_publicationDate")]
    pub _publication_date: Option<Element>,
}

impl Default for RelatedArtifact {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            type_: Default::default(),
            _type: Default::default(),
            classifier: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            citation: Default::default(),
            _citation: Default::default(),
            document: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            resource_reference: Default::default(),
            publication_status: Default::default(),
            _publication_status: Default::default(),
            publication_date: Default::default(),
            _publication_date: Default::default(),
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
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
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
                "RelatedArtifact.publicationStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("Publication status of an artifact being referred to."),
            rh_foundation::ElementBinding::new(
                "RelatedArtifact.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/related-artifact-type|5.0.0",
            )
            .with_description("The type of relationship to the related artifact."),
        ]
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
            rh_foundation::ElementCardinality::new("RelatedArtifact.classifier", 0, None),
            rh_foundation::ElementCardinality::new("RelatedArtifact.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.citation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.document", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.resourceReference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.publicationStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedArtifact.publicationDate", 0, Some(1)),
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
