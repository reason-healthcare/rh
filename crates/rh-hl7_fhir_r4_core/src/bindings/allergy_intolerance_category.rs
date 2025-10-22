use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/allergy-intolerance-category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllergyIntoleranceCategory {
    /// Food
    #[serde(rename = "food")]
    Food,
    /// Medication
    #[serde(rename = "medication")]
    Medication,
    /// Environment
    #[serde(rename = "environment")]
    Environment,
    /// Biologic
    #[serde(rename = "biologic")]
    Biologic,
}
impl Default for AllergyIntoleranceCategory {
    fn default() -> Self {
        Self::Food
    }
}
