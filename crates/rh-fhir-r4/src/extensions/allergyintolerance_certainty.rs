use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// certainty
///
/// Statement about the degree of clinical certainty that the specific substance was the cause of the manifestation in this reaction event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-certainty
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceCertainty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceCertainty {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
