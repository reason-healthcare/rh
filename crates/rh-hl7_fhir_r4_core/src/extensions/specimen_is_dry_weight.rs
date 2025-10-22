use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Is Dry Weight
///
/// If the recorded quantity of the specimen is reported as a weight, if it is a dry weight.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-isDryWeight
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenIsDryWeight {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for SpecimenIsDryWeight {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
