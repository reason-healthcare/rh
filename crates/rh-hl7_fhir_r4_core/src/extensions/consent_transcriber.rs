use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Transcriber
///
/// Any person/thing who transcribed the consent into the system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-Transcriber
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentTranscriber {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConsentTranscriber {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
