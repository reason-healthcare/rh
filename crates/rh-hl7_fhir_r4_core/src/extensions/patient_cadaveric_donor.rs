use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// cadavericDonor
///
/// Flag indicating whether the patient authorized the donation of body parts after death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-cadavericDonor
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCadavericDonor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientCadavericDonor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
