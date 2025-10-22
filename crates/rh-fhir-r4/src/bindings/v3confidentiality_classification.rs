use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://terminology.hl7.org/ValueSet/v3-ConfidentialityClassification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum V3ConfidentialityClassification {
    #[serde(rename = "U")]
    U,
    #[serde(rename = "L")]
    L,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "N")]
    N,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "V")]
    V,
}
impl Default for V3ConfidentialityClassification {
    fn default() -> Self {
        Self::U
    }
}
