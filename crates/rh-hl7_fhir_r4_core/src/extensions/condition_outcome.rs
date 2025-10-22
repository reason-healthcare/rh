use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Outcome
///
/// A result of the condition. The "Cause of death" for a patient is typically captured as an Observation.  The "outcome" doesn't imply causality.  Some outcomes might not be assessable until the condition.clinicalStatus is no longer active.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/condition-outcome
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionOutcome {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConditionOutcome {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
