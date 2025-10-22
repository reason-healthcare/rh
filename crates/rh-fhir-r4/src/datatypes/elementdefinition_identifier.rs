use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// identifier
///
/// External Identifiers associated with this element - these are identifiers that are associated with the concept this element represents.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-identifier
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionIdentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionIdentifier {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
