use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// json-type
///
/// The JSON type of a property - used for the value property of a primitive type (for which there is no type in the FHIR typing system).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-json-type
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionJSONType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionJSONType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
