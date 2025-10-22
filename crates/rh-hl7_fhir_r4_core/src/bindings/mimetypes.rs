use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/mimetypes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mimetypes {
    Unknown,
}
impl Default for Mimetypes {
    fn default() -> Self {
        Self::Unknown
    }
}
