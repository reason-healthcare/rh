use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// PQ-translation
///
/// An alternative representation of the same physical quantity expressed in a different unit from a different unit code system and possibly with a different value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-PQ-translation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090PQTranslation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090PQTranslation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
