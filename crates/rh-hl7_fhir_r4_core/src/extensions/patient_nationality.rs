use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// nationality
///
/// The nationality of the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-nationality
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientNationality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientNationality {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
