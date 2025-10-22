use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// activityStatusDate
///
/// The date when the associated Value Set Definition Version activity status is in effect.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-activityStatusDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetActivityStatusDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetActivityStatusDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
