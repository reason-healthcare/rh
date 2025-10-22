use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/contact-point-use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactPointUse {
    /// Home
    #[serde(rename = "home")]
    Home,
    /// Work
    #[serde(rename = "work")]
    Work,
    /// Temp
    #[serde(rename = "temp")]
    Temp,
    /// Old
    #[serde(rename = "old")]
    Old,
    /// Mobile
    #[serde(rename = "mobile")]
    Mobile,
}
impl Default for ContactPointUse {
    fn default() -> Self {
        Self::Home
    }
}
