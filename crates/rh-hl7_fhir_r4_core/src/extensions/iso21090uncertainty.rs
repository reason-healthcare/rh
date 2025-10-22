use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// uncertainty
///
/// The primary measure of variance/uncertainty of the value (the square root of the sum of the squares of the differences between all data points and the mean).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-uncertainty
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090Uncertainty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
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

impl Default for Iso21090Uncertainty {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
