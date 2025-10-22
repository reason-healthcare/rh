use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/product-storage-scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductStorageScale {
    /// Fahrenheit
    #[serde(rename = "farenheit")]
    Farenheit,
    /// Celsius
    #[serde(rename = "celsius")]
    Celsius,
    /// Kelvin
    #[serde(rename = "kelvin")]
    Kelvin,
}
impl Default for ProductStorageScale {
    fn default() -> Self {
        Self::Farenheit
    }
}
