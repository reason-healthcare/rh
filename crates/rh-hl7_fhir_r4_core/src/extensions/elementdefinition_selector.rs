use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// selector
///
/// A FHIRPath statement that defines whether an element is in the slice.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-selector
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionSelector {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionSelector {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
