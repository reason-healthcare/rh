use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// measureInfo
///
/// The measure criteria that resulted in the resource being included in a particular evaluatedResources bundle.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-measureInfo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFMeasureInfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFMeasureInfo {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
