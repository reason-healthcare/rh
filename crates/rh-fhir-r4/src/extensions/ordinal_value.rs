use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Ordinal Value
///
/// A numeric value that allows the comparison (less than, greater than) or other numerical  manipulation of a concept (e.g. Adding up components of a score). Scores are usually a whole number, but occasionally decimals are encountered in scores.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ordinalValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdinalValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OrdinalValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
