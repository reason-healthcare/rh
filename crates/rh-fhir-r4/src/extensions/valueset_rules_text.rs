use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// rules-text
///
/// An expression that provides an alternative definition of the content of the value set in some form that is not computable - e.g instructions that could only be followed by a human.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-rules-text
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetRulesText {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetRulesText {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
