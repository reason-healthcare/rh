use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Age
///
/// Base StructureDefinition for Age Type: A duration of time during which an organism (or a process) has existed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Age
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Age
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Age {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for Age {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
