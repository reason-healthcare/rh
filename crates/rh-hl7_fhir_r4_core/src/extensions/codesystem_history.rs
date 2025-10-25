use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// history
///
/// Information on changes made to the Value Set Definition over time, and also has a contained audit trail of all such changes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-history
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemHistory {
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

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.extension",
                0,
                Some(0),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Extension.extension.extension.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.value[x]", 0, Some(0)),
        ]
    });

impl crate::validation::ValidatableResource for CodesystemHistory {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/codesystem-history")
    }
}
