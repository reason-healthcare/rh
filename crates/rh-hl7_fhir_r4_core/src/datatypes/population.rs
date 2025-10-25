use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::range::Range;
use serde::{Deserialize, Serialize};
/// Population
///
/// Base StructureDefinition for Population Type: A populatioof people with some set of grouping criteria.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Population
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Population
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The age of the specific population (Range)
    #[serde(rename = "ageRange")]
    pub age_range: Option<Range>,
    /// The age of the specific population (CodeableConcept)
    #[serde(rename = "ageCodeableConcept")]
    pub age_codeable_concept: Option<CodeableConcept>,
    /// The gender of the specific population
    pub gender: Option<CodeableConcept>,
    /// Race of the specific population
    pub race: Option<CodeableConcept>,
    /// The existing physiological conditions of the specific population to which this applies
    #[serde(rename = "physiologicalCondition")]
    pub physiological_condition: Option<CodeableConcept>,
}

impl Default for Population {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            age_range: Default::default(),
            age_codeable_concept: Default::default(),
            gender: Default::default(),
            race: Default::default(),
            physiological_condition: Default::default(),
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
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Population.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Population.extension", 0, None),
            rh_foundation::ElementCardinality::new("Population.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Population.age[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Population.gender", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Population.race", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Population.physiologicalCondition", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Population {
    fn resource_type(&self) -> &'static str {
        "Population"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Population")
    }
}
