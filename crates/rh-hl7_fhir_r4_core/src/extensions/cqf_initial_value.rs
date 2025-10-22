use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// initialValue
///
/// The name of an expression in a referenced library that determines an initial value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-initialValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFInitialValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFInitialValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
