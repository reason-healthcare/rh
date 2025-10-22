use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Related Artifact
///
/// Documentation  or 'knowledge artifacts' relevant to the base resource such as citations, supporting evidence, documentation of processes, caveats around testing methodology.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-relatedArtifact
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRelatedArtifact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowRelatedArtifact {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
