use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Original Text
///
/// A human language representation of the concept (resource/element) as seen/selected/uttered by the user who entered the data and/or which represents the full intended meaning of the user. This can be provided either directly as text, or as a url that is a reference to a portion of the narrative of a resource ([DomainResource.text](narrative.html)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/originalText
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalText {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OriginalText {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
