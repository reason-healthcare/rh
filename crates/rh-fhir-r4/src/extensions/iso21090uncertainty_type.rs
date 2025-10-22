use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// uncertaintyType
///
/// A code specifying the type of probability distribution for the uncertainty.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-uncertaintyType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090UncertaintyType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090UncertaintyType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
