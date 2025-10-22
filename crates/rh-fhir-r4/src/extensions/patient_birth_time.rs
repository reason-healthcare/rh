use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Birth Time
///
/// The time of day that the Patient was born. This includes the date to ensure that the timezone information can be communicated effectively.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-birthTime
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientBirthTime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientBirthTime {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
