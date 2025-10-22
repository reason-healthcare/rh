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
