use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/all-languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllLanguages {
    Unknown,
}
impl Default for AllLanguages {
    fn default() -> Self {
        Self::Unknown
    }
}
