use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// allowedUnits
///
/// Identifies the units of measure in which the element should be captured or expressed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-allowedUnits
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionAllowedUnits {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionAllowedUnits {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
