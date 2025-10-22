use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Processing Time
///
/// Period or duration of processing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/specimen-processingTime
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecimenProcessingTime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for SpecimenProcessingTime {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
