use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Variable
///
/// Variable specifying a logic to generate a variable for use in subsequent logic.  The name of the variable will be added to FHIRPath's context when processing descendants of the element that contains this extension.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/variable
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
