use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// namespace
///
/// Use this extension to indicate tha the element has an XML namespace different to http://hl7.org/fhir.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-namespace
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionNamespace {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionNamespace {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
