use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// animal
///
/// This patient is known to be an animal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-animal
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAnimal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientAnimal {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
