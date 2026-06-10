use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/ingredient-manufacturer-role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IngredientManufacturerRole {
    /// Manufacturer is specifically allowed for this ingredient
    #[serde(rename = "allowed")]
    Allowed,
    /// Manufacturer is known to make this ingredient in general
    #[serde(rename = "possible")]
    Possible,
    /// Manufacturer actually makes this particular ingredient
    #[serde(rename = "actual")]
    Actual,
}
impl Default for IngredientManufacturerRole {
    fn default() -> Self {
        Self::Allowed
    }
}
