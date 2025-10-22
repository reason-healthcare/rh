use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// interpreterRequired
///
/// This Patient requires an interpreter to communicate healthcare information to the practitioner.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-interpreterRequired
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInterpreterRequired {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientInterpreterRequired {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
