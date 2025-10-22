use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/identity-assuranceLevel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityAssuranceLevel {
    /// Level 1
    #[serde(rename = "level1")]
    Level1,
    /// Level 2
    #[serde(rename = "level2")]
    Level2,
    /// Level 3
    #[serde(rename = "level3")]
    Level3,
    /// Level 4
    #[serde(rename = "level4")]
    Level4,
}
impl Default for IdentityAssuranceLevel {
    fn default() -> Self {
        Self::Level1
    }
}
