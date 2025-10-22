use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/orientation-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrientationType {
    /// Sense orientation of referenceSeq
    #[serde(rename = "sense")]
    Sense,
    /// Antisense orientation of referenceSeq
    #[serde(rename = "antisense")]
    Antisense,
}
impl Default for OrientationType {
    fn default() -> Self {
        Self::Sense
    }
}
