use crate::bindings::narrative_status::NarrativeStatus;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Narrative
///
/// Base StructureDefinition for Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Narrative
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Narrative
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// generated | extensions | additional | empty
    pub status: NarrativeStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Limited xhtml content
    pub div: StringType,
    /// Extension element for the 'div' primitive field. Contains metadata and extensions.
    pub _div: Option<Element>,
}
/// Narrative Link
///
/// A human language representation of the concept (resource/element), as a url that is a reference to a portion of the narrative of a resource ([DomainResource.text](narrative.html)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/narrativeLink
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Narrative {
    fn default() -> Self {
        Self {
            base: Element::default(),
            status: NarrativeStatus::default(),
            _status: Default::default(),
            div: StringType::default(),
            _div: Default::default(),
        }
    }
}

impl Default for NarrativeLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
