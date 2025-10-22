use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Hierarchy
///
/// For circular references (references back to the same type of resource): whether they are allowed to transitively point back to the same instance (false), or whether the relationship must be a strict one-way hierarchy (true).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-hierarchy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionHierarchy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionHierarchy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
