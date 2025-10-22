use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ancestor
///
/// A canonical reference to a StructureDefinition that this is derived from. This is a de-normalization of a chain of StructureDefinition.baseDefinition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-ancestor
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionAncestor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionAncestor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
