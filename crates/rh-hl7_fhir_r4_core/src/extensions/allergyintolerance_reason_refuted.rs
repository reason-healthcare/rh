use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// reasonRefuted
///
/// A code capturing the explanation of why the allergy or intolerance has been refuted. Should be specified only if the status is refuted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-reasonRefuted
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceReasonRefuted {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceReasonRefuted {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
