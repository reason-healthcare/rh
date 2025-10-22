use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// adoptionInfo
///
/// Code indication the adoption status of the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-adoptionInfo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdoptionInfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientAdoptionInfo {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
