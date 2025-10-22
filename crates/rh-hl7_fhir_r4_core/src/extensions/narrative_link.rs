use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
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

impl Default for NarrativeLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
