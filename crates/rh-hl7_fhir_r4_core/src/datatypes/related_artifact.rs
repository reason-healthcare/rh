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
