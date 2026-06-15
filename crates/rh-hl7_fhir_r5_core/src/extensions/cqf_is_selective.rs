use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Is Selective
///
/// Allows a given data requirement to be identified as "selective", meaning that it can be used as an additive criteria to filter a population. A selective data requirement is guaranteed to define a subset (not necessarily proper) of the initial population of an artifact. If multiple data requirements are marked selective, they all apply (i.e. AND semantics).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-isSelective
/// - Version: 5.1.0-snapshot1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFIsSelective {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFIsSelective {
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
            rh_foundation::ElementCardinality::new("Extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.value[x]", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for CQFIsSelective {
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
        Some("http://hl7.org/fhir/StructureDefinition/cqf-isSelective")
    }
}
