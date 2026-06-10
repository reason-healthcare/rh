use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/ucum-units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UcumUnits {
    Unknown,
}
impl Default for UcumUnits {
    fn default() -> Self {
        Self::Unknown
    }
}
