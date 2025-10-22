use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// bindingName
///
/// A name that can be used for code generation when generating named enumerations for the binding.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionBindingName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionBindingName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
