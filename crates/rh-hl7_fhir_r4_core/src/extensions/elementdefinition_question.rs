use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// question
///
/// The default/suggested phrasing to use when prompting a human to capture the data element in question form (e.g. In a survey).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-question
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionQuestion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionQuestion {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
