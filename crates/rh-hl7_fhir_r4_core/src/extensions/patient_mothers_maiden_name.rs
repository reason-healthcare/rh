use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Mother's Maiden Name
///
/// Mother's maiden (unmarried) name, commonly collected to help verify patient identity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-mothersMaidenName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMothersMaidenName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientMothersMaidenName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
