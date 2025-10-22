use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Birth Place
///
/// The registered place of birth of the patient. A sytem may use the address.text if they don't store the birthPlace address in discrete elements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-birthPlace
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientBirthPlace {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientBirthPlace {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
