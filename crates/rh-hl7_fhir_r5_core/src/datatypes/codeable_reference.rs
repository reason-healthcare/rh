use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_type::DataType;
use crate::datatypes::reference::Reference;
use serde::{Deserialize, Serialize};
/// CodeableReference
///
/// CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CodeableReference
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: CodeableReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Reference to a concept (by class)
    pub concept: Option<CodeableConcept>,
    /// Reference to a resource (by instance)
    pub reference: Option<Reference>,
}

impl Default for CodeableReference {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            concept: Default::default(),
            reference: Default::default(),
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
            rh_foundation::ElementCardinality::new("CodeableReference.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeableReference.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeableReference.concept", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeableReference.reference", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for CodeableReference {
    fn resource_type(&self) -> &'static str {
        "CodeableReference"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/CodeableReference")
    }
}
