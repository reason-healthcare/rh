use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// maxValueSet
///
/// The maximum allowable value set, for use when the binding strength is 'extensible' or 'preferred'. This value set is the value set from which additional codes can be taken from. This defines a 'required' binding over the top of the extensible binding.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-maxValueSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionMaxValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionMaxValueSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
