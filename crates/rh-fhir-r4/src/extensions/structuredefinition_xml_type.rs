use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// xml-type
///
/// The XML (schema) type of a property - used for the value attribute of a primitive type (for which there is no type in the FHIR typing system).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-xml-type
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionXMLType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionXMLType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
