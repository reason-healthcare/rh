use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// severity
///
/// A qualification of the seriousness or impact on health of the family member condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-severity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilymemberhistorySeverity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilymemberhistorySeverity {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
