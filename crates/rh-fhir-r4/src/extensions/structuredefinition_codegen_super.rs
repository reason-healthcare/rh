use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// codegen-super
///
/// A specific instruction to use an intermediate parent class when generating code for the classes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-codegen-super
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionCodegenSuper {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionCodegenSuper {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
