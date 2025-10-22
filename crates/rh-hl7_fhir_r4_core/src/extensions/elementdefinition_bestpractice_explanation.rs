use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// bestpractice-explanation
///
/// Explains why an invariant is labelled as a best practice invariant.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-bestpractice-explanation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionBestpracticeExplanation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionBestpracticeExplanation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
