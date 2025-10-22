use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// citizenship
///
/// The patient's legal status as citizen of a country.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-citizenship
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCitizenship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientCitizenship {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
