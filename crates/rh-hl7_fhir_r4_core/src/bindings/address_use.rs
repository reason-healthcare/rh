use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/address-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressUse {
    /// Home
    #[serde(rename = "home")]
    Home,
    /// Work
    #[serde(rename = "work")]
    Work,
    /// Temporary
    #[serde(rename = "temp")]
    Temp,
    /// Old / Incorrect
    #[serde(rename = "old")]
    Old,
    /// Billing
    #[serde(rename = "billing")]
    Billing,
}
impl Default for AddressUse {
    fn default() -> Self {
        Self::Home
    }
}
