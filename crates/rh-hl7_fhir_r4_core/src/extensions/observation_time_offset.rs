use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Time-offset
///
/// A specific offset time in milliseconds from the stated time in the Observation.appliesDateTime to allow for representation of sequential recording  of sampled data from the same lead or data stream.  For example, an ECG recorder may record sequentially 3 leads four time to receive 12-lead ECG, see [ISO 22077](https://www.iso.org/obp/ui/#iso:std:61871:en).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-timeOffset
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationTimeOffset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationTimeOffset {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), 'value')])"),
        ]
    });

impl crate::validation::ValidatableResource for ObservationTimeOffset {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/observation-timeOffset")
    }
}
