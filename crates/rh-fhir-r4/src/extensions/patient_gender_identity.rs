use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// genderIdentity
///
/// The gender the patient identifies with. The Patient's gender identity is used as guidance (e.g. for staff) about how to interact with the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-genderIdentity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientGenderIdentity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientGenderIdentity {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
