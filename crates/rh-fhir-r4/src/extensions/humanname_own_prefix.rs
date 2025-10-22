use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// own-prefix
///
/// The prefix portion (e.g. voorvoegsel) of the family name that is derived from the person's own surname, as distinguished from any portion that is derived from the surname of the person's partner or spouse.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/humanname-own-prefix
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumannameOwnPrefix {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HumannameOwnPrefix {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
