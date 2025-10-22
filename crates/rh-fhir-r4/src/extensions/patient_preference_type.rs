use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// preferenceType
///
/// Indicates what mode of communication the patient prefers to use for the indicated language.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-preferenceType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPreferenceType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientPreferenceType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
