use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// isCommonBinding
///
/// Whether the binding is used on multiple resources, or only on this resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-isCommonBinding
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionIsCommonBinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionIsCommonBinding {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
