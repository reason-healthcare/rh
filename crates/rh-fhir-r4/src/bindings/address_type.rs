use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/address-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressType {
    /// Postal
    #[serde(rename = "postal")]
    Postal,
    /// Physical
    #[serde(rename = "physical")]
    Physical,
    /// Postal & Physical
    #[serde(rename = "both")]
    Both,
}
impl Default for AddressType {
    fn default() -> Self {
        Self::Postal
    }
}
