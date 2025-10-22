use crate::datatypes::element_definition::ElementDefinition;
use serde::{Deserialize, Serialize};
/// DataElement constraint on ElementDefinition data type
///
/// Identifies how the ElementDefinition data type is used when it appears within a data element
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-de
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ElementDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ElementDefinition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionDe {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ElementDefinition,
}

impl Default for ElementdefinitionDe {
    fn default() -> Self {
        Self {
            base: ElementDefinition::default(),
        }
    }
}
