use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// congregation
///
/// A group or place of religious practice that may provide services to the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-congregation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCongregation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientCongregation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
