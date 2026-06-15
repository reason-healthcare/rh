use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Observation Nature of Abnormal Test
///
/// This field contains the nature of the abnormal test. This is represented in v2 as OBX-10 Nature of abnormal test (ID) 00578.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-nature-of-abnormal-test
/// - Version: 5.1.0-snapshot1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationNatureOfAbnormalTest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationNatureOfAbnormalTest {
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
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
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
            rh_foundation::ElementCardinality::new("Extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.value[x]", 1, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ObservationNatureOfAbnormalTest {
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
        Some("http://hl7.org/fhir/StructureDefinition/observation-nature-of-abnormal-test")
    }
}
