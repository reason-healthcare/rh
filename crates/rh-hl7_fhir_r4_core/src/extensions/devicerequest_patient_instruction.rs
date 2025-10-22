use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Directions
///
/// Simple concise instructions to be read by the patient.  For example  “twice a day” rather than “BID.”.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/devicerequest-patientInstruction
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicerequestPatientInstruction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DevicerequestPatientInstruction {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
