use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-censusTract
///
/// A geographic sub-unit delineated for demographic purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-censusTract
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPCensusTract {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPCensusTract {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
