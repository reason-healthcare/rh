use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/quantity-comparator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantityComparator {
    /// Less than
    #[serde(rename = "<")]
    LessThan,
    /// Less or Equal to
    #[serde(rename = "<=")]
    LessThanOrEqual,
    /// Greater or Equal to
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    /// Greater than
    #[serde(rename = ">")]
    GreaterThan,
    /// Sufficient to achieve this total quantity
    #[serde(rename = "ad")]
    Ad,
}
impl Default for QuantityComparator {
    fn default() -> Self {
        Self::LessThan
    }
}
