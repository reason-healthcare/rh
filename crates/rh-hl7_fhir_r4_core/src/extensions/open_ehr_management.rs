use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// management
///
/// Text description about the clinical management provided.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-management
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRManagement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRManagement {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
