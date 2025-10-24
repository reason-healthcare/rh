use crate::bindings::all_types::AllTypes;
use crate::bindings::sort_direction::SortDirection;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// DataRequirement
///
/// Base StructureDefinition for DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: DataRequirement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// The type of the required data
    #[serde(rename = "type")]
    pub type_: AllTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The profile of the required data
    pub profile: Option<Vec<StringType>>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Indicates specific structure elements that are referenced by the knowledge module
    #[serde(rename = "mustSupport")]
    pub must_support: Option<Vec<StringType>>,
    /// Extension element for the 'mustSupport' primitive field. Contains metadata and extensions.
    #[serde(rename = "_mustSupport")]
    pub _must_support: Option<Element>,
    /// What codes are expected
    #[serde(rename = "codeFilter")]
    pub code_filter: Option<Vec<Element>>,
    /// What dates/date ranges are expected
    #[serde(rename = "dateFilter")]
    pub date_filter: Option<Vec<Element>>,
    /// Number of results
    pub limit: Option<PositiveIntType>,
    /// Extension element for the 'limit' primitive field. Contains metadata and extensions.
    pub _limit: Option<Element>,
    /// Order of the results
    pub sort: Option<Vec<Element>>,
}
/// DataRequirement nested structure for the 'dateFilter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirementDatefilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A date-valued attribute to filter on
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// A date valued parameter to search on
    #[serde(rename = "searchParam")]
    pub search_param: Option<StringType>,
    /// Extension element for the 'searchParam' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchParam")]
    pub _search_param: Option<Element>,
    /// The value of the filter, as a Period, DateTime, or Duration value (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// The value of the filter, as a Period, DateTime, or Duration value (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,
    /// The value of the filter, as a Period, DateTime, or Duration value (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Option<Duration>,
}
/// DataRequirement nested structure for the 'sort' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirementSort {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The name of the attribute to perform the sort
    pub path: StringType,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// ascending | descending
    pub direction: SortDirection,
    /// Extension element for the 'direction' primitive field. Contains metadata and extensions.
    pub _direction: Option<Element>,
}
/// DataRequirement nested structure for the 'codeFilter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirementCodefilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code-valued attribute to filter on
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// A coded (token) parameter to search on
    #[serde(rename = "searchParam")]
    pub search_param: Option<StringType>,
    /// Extension element for the 'searchParam' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchParam")]
    pub _search_param: Option<Element>,
    /// Valueset for the filter
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// What code is expected
    pub code: Option<Vec<Coding>>,
}

impl Default for DataRequirement {
    fn default() -> Self {
        Self {
            base: Element::default(),
            type_: Default::default(),
            _type: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
            must_support: Default::default(),
            _must_support: Default::default(),
            code_filter: Default::default(),
            date_filter: Default::default(),
            limit: Default::default(),
            _limit: Default::default(),
            sort: Default::default(),
        }
    }
}

impl Default for DataRequirementDatefilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            search_param: Default::default(),
            _search_param: Default::default(),
            value_date_time: Default::default(),
            value_period: Default::default(),
            value_duration: Default::default(),
        }
    }
}

impl Default for DataRequirementSort {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: StringType::default(),
            _path: Default::default(),
            direction: SortDirection::default(),
            _direction: Default::default(),
        }
    }
}

impl Default for DataRequirementCodefilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            search_param: Default::default(),
            _search_param: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
            code: Default::default(),
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
    rh_foundation::Invariant::new("drq-1", rh_foundation::Severity::Error, "Either a path or a searchParam must be provided, but not both", "path.exists() xor searchParam.exists()").with_xpath("(exists(f:path) and not(exists(f:searchParam))) or (not(exists(f:path)) and exists(f:searchParam))"),
    rh_foundation::Invariant::new("drq-2", rh_foundation::Severity::Error, "Either a path or a searchParam must be provided, but not both", "path.exists() xor searchParam.exists()").with_xpath("(exists(f:path) and not(exists(f:searchParam))) or (not(exists(f:path)) and exists(f:searchParam))"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

impl crate::validation::ValidatableResource for DataRequirement {
    fn resource_type(&self) -> &'static str {
        "DataRequirement"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/DataRequirement")
    }
}
