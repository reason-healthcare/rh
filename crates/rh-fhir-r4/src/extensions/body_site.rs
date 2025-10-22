use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Body Site
///
/// Record details about the anatomical location of a specimen or body part. This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/bodySite
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySite {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for BodySite {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
