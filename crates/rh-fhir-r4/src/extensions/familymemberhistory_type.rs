use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// type
///
/// Purpose of the family member history or why it was created, such as when family member history is targeted for cardiovascular health, mental health, or genetic counseling.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-type
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilymemberhistoryType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilymemberhistoryType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
