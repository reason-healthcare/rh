use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Dependent Profiles
///
/// Instances can only be valid against this StructureDefinition, if they also sucessfully validate against the dependent profile identified in this extension.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-dependencies
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionDependencies {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionDependencies {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
