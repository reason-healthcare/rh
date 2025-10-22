use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// minValueSet
///
/// The minimum allowable value set, for use when the binding strength is 'required' or 'extensible'. This value set is the minimum value set that any conformant system SHALL support.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-minValueSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionMinValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionMinValueSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
