use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// profile-element
///
/// The specific element to use in the referenced profile. This is used when a backbone element is being profiled, rather than an established type.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-profile-element
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionProfileElement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionProfileElement {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
