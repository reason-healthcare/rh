use crate::datatypes::element::Element;
use crate::primitives::date_time::DateTimeType;
use serde::{Deserialize, Serialize};
/// Period
///
/// Base StructureDefinition for Period Type: A time period defined by a start and end date and optionally time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Period
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Period
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Starting time with inclusive boundary
    pub start: Option<DateTimeType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// End time with inclusive boundary, if not ongoing
    pub end: Option<DateTimeType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
}

impl Default for Period {
    fn default() -> Self {
        Self {
            base: Element::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
        }
    }
}
