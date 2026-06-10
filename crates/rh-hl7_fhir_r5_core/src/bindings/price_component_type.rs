use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/price-component-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceComponentType {
    /// base price
    #[serde(rename = "base")]
    Base,
    /// surcharge
    #[serde(rename = "surcharge")]
    Surcharge,
    /// deduction
    #[serde(rename = "deduction")]
    Deduction,
    /// discount
    #[serde(rename = "discount")]
    Discount,
    /// tax
    #[serde(rename = "tax")]
    Tax,
    /// informational
    #[serde(rename = "informational")]
    Informational,
}
impl Default for PriceComponentType {
    fn default() -> Self {
        Self::Base
    }
}
