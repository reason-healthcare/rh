use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Relevant History
///
/// Links to Provenance records for past versions of this resource or fulfilling request or event resources that identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/request-relevantHistory
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRelevantHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RequestRelevantHistory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
