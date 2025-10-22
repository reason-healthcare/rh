use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// media
///
/// It contains enriched media representation of the alert message, such as a voice recording.  This may be used, for example for compliance with jurisdictional accessibility requirements, literacy issues, or translations of the unstructured text content in other languages.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/communication-media
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMedia {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CommunicationMedia {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
