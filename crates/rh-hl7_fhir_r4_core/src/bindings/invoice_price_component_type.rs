use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/invoice-priceComponentType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoicePriceComponentType {
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
impl Default for InvoicePriceComponentType {
    fn default() -> Self {
        Self::Base
    }
}
