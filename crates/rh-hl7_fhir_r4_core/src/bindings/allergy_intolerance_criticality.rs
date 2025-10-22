use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/allergy-intolerance-criticality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllergyIntoleranceCriticality {
    /// Low Risk
    #[serde(rename = "low")]
    Low,
    /// High Risk
    #[serde(rename = "high")]
    High,
    /// Unable to Assess Risk
    #[serde(rename = "unable-to-assess")]
    UnableToAssess,
}
impl Default for AllergyIntoleranceCriticality {
    fn default() -> Self {
        Self::Low
    }
}
