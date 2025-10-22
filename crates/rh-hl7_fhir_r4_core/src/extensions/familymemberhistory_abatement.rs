use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// abatement
///
/// The approximate date, age, or flag indicating that the condition of the family member resolved. The abatement should only be specified if the condition is stated in the positive sense, i.e., the didNotHave flag is false.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/familymemberhistory-abatement
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilymemberhistoryAbatement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FamilymemberhistoryAbatement {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
