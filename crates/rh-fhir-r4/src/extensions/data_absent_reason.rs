use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Why value is missing
///
/// Provides a reason why the expected value or elements in the element that is extended are missing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/data-absent-reason
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAbsentReason {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DataAbsentReason {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
