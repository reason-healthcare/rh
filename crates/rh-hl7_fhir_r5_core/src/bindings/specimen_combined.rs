use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/specimen-combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecimenCombined {
    /// Grouped
    #[serde(rename = "grouped")]
    Grouped,
    /// Pooled
    #[serde(rename = "pooled")]
    Pooled,
}
impl Default for SpecimenCombined {
    fn default() -> Self {
        Self::Grouped
    }
}
