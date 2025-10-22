use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// preferred
///
/// Flag denoting whether parent item is preferred - e.g., a preferred address or telephone number.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-preferred
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090Preferred {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090Preferred {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
