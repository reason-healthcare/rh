use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// translatable
///
/// Whether translations might be expected for this element in resource instances.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-translatable
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionTranslatable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionTranslatable {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
