use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// calculatedValue
///
/// The name of an expression in a referenced library that determines a calculated value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-calculatedValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFCalculatedValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFCalculatedValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
