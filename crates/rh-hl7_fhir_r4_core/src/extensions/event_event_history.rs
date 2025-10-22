use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Event History
///
/// Links to *Provenance* records for past versions of this resource that document  key state transitions or updates that are deemed “relevant” or important to a user looking at the current version of the resource. E.g, when an observation was verified or corrected.  This extension does not point to the Provenance associated with the current version of the resource - as it would be created after this version existed. The *Provenance* for the current version can be retrieved with a [` _revinclude`](search.html#revinclude).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-eventHistory
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEventHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventEventHistory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
