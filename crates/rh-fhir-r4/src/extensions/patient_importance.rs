use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// importance
///
/// The importance of the patient (e.g. VIP).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-importance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientImportance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientImportance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
