use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// reference
///
/// A logical reference (e.g. a reference to ValueSet.url) that identifies the value set/version that identifies the set of possible coded values this coding was chosen from or constrained by.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-reference
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetReference {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
