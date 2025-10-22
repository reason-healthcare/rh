use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// table name
///
/// A name to use to show mappings of this type in the generated summary tables.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-table-name
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionTableName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionTableName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
