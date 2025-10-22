use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// disability
///
/// Value(s) identifying physical or mental condition(s) that limits a person's movements, senses, or activities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-disability
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDisability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PatientDisability {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
