use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// validDate
///
/// Indicates a date on which this identifier value was deemed to apply to this instance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/identifier-validDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierValidDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for IdentifierValidDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
