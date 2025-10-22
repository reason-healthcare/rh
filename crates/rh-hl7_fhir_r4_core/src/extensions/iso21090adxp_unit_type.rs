use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-unitType
///
/// Indicates the type of specific unit contained within a building or complex. E.g. Appartment, Floor.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-unitType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPUnitType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPUnitType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
