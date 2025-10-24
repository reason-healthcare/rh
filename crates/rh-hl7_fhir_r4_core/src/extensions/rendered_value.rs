use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Rendered Value
///
/// Provides a rendered version of the value intended for human display.  For example, a sensitive identifier (e.g. social security number) partially obscured by asterisks; a drivers licence number with dashes inserted; a date formatted as MMM dd, yyyy; etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendered-value
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderedValue {
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

impl crate::validation::ValidatableResource for RenderedValue {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/rendered-value")
    }
}
