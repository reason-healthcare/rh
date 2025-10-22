use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/allergy-intolerance-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllergyIntoleranceType {
    /// Allergy
    #[serde(rename = "allergy")]
    Allergy,
    /// Intolerance
    #[serde(rename = "intolerance")]
    Intolerance,
}
impl Default for AllergyIntoleranceType {
    fn default() -> Self {
        Self::Allergy
    }
}
