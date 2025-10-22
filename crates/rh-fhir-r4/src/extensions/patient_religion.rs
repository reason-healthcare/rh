use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// religion
///
/// The patient's professed religious affiliations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-religion
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientReligion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientReligion {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
