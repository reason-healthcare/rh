use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/fhir-types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FhirTypes {
    /// Base
    #[serde(rename = "Base")]
    Base,
}
impl Default for FhirTypes {
    fn default() -> Self {
        Self::Base
    }
}
