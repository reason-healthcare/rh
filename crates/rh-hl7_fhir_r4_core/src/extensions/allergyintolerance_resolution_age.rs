use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// resolutionAge
///
/// The estimated patient age at which the allergy or intolerance resolved. Should be specified only if the status is resolved.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/allergyintolerance-resolutionAge
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyintoleranceResolutionAge {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AllergyintoleranceResolutionAge {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
