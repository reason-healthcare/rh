use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/product-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductStatus {
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Unavailable
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl Default for ProductStatus {
    fn default() -> Self {
        Self::Available
    }
}
