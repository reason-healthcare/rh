use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// partner-name
///
/// The portion of the family name that is derived from the person's partner's surname, as distinguished from any portion that is derived from the surname of the person's own name.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/humanname-partner-name
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumannamePartnerName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HumannamePartnerName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
