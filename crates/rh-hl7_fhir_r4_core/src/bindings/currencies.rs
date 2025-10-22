use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/currencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currencies {
    Unknown,
}
impl Default for Currencies {
    fn default() -> Self {
        Self::Unknown
    }
}
