use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-actuality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdverseEventActuality {
    /// Adverse Event
    #[serde(rename = "actual")]
    Actual,
    /// Potential Adverse Event
    #[serde(rename = "potential")]
    Potential,
}
impl Default for AdverseEventActuality {
    fn default() -> Self {
        Self::Actual
    }
}
