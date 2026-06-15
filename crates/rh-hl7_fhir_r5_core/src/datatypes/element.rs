use crate::datatypes::base::Base;
use crate::datatypes::extension::Extension;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Element
///
/// Element Type: Base definition for all elements in a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Element
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Element
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Base,
    /// Unique id for inter-element referencing
    pub id: Option<StringType>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<Extension>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            base: Base::default(),
            id: Default::default(),
            extension: Default::default(),
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
            rh_foundation::ElementCardinality::new("Element.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Element.extension", 0, None),
        ]
    });

impl crate::validation::ValidatableResource for Element {
    fn resource_type(&self) -> &'static str {
        "Element"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }
}
