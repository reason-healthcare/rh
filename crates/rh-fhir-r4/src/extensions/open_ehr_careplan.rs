use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// careplan
///
/// Additional details about the clinical management provided for this Reaction Event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-careplan
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRCareplan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRCareplan {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
