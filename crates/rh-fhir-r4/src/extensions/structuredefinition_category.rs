use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// category
///
/// The category under which the resource type is presented on the official resource list.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-category
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionCategory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionCategory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
