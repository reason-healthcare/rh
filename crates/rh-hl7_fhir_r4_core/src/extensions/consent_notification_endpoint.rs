use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Disclosure Notification Endpoint
///
/// Endpoint for sending Disclosure notifications in the form of FHIR AuditEvent records.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/consent-NotificationEndpoint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentNotificationEndpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConsentNotificationEndpoint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
