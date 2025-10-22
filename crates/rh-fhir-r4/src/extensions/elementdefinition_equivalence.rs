use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// equivalence
///
/// The level of equivalence between the element containing the mapping and the element mapped to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-equivalence
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionEquivalence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionEquivalence {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
