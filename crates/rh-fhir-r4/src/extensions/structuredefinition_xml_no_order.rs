use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// No Order in XML
///
/// Whether elements can come in any order in XML.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-xml-no-order
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionXMLNoOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionXMLNoOrder {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
