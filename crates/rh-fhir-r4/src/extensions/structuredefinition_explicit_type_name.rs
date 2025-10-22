use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// explicit-type-name
///
/// A name to use for the type, in implementations. This is a suggestion; it's not a normative part of the FHIR specification, but it does appear in the UML diagrams, and is used in generated code, schemas, etc.to identify the type.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionExplicitTypeName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionExplicitTypeName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
