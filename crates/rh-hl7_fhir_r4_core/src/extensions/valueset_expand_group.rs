use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expand-group
///
/// This extension declares a group of concepts that is generated into the ValueSet.expansion.contains hierarchy when the expansion is generated for a UI. THere is no inherent assigned meaning to the hierarchy; it is used to help the user navigate the concepts. Each group has a display and/or a code, and a list of members, which are either concepts in the value set, or other groups (by code).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expand-group
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpandGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpandGroup {
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

impl crate::validation::ValidatableResource for ValuesetExpandGroup {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/valueset-expand-group")
    }
}
