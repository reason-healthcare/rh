use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/specimen-contained-preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecimenContainedPreference {
    /// Preferred
    #[serde(rename = "preferred")]
    Preferred,
    /// Alternate
    #[serde(rename = "alternate")]
    Alternate,
}
impl Default for SpecimenContainedPreference {
    fn default() -> Self {
        Self::Preferred
    }
}
