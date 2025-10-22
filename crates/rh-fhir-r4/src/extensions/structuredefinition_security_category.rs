use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Security Category
///
/// Provides general guidance around the kind of access Control to Read, Search, Create, Update, or Delete the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-security-category
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionSecurityCategory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionSecurityCategory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
