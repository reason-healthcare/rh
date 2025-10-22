use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// template-status
///
/// Status code taken from [HL7 template specification](http://www.hl7.org/implement/standards/product_brief.cfm?product_id=377) - allows for alignment with the template DSTU, and has more authoring status codes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-template-status
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionTemplateStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionTemplateStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
