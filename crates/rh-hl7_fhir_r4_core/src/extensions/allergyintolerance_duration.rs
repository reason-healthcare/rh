use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// duration
///
/// The amount of time that the Adverse Reaction persisted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-duration
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceDuration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceDuration {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
