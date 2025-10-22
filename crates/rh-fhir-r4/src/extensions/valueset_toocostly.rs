use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Expansion Truncated
///
/// Marks that the expansion is  incomplete, because the full value set is too large to represent, and the client asked for an incomplete fragment.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-toocostly
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetToocostly {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetToocostly {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
