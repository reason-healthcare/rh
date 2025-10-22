use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Standards Status
///
/// The Current HL7 ballot/Standards status of this artifact.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-standards-status
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionStandardsStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionStandardsStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
