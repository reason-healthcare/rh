use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-additionalLocator
///
/// This can be a unit designator, such as apartment number, suite number, or floor. There may be several unit designators in an address (e.g., "3rd floor, Appt. 342"). This can also be a designator pointing away from the location, rather than specifying a smaller location within some larger one (e.g., Dutch "t.o." means "opposite to" for house boats located across the street facing houses).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-additionalLocator
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPAdditionalLocator {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPAdditionalLocator {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
