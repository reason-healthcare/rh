use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// proficiency
///
/// Proficiency level of the communication.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-proficiency
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProficiency {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientProficiency {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
