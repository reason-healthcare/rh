use crate::bindings::fhir_types::FhirTypes;
use crate::bindings::sort_direction::SortDirection;
use crate::bindings::value_filter_comparator::ValueFilterComparator;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::data_type::DataType;
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
/// DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: DataRequirement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// The type of the required data
    #[serde(rename = "type")]
    pub type_: FhirTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The profile of the required data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _profile: Vec<Element>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Indicates specific structure elements that are referenced by the knowledge module
    #[serde(rename = "mustSupport")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_support: Vec<StringType>,
    /// Extension element for the 'mustSupport' primitive field. Contains metadata and extensions.
    #[serde(rename = "_mustSupport")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _must_support: Vec<Element>,
    /// What codes are expected
    #[serde(rename = "codeFilter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_filter: Vec<Element>,
    /// What dates/date ranges are expected
    #[serde(rename = "dateFilter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_filter: Vec<Element>,
    /// What values are expected
    #[serde(rename = "valueFilter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_filter: Vec<Element>,
    /// Number of results
    pub limit: Option<PositiveIntType>,
    /// Extension element for the 'limit' primitive field. Contains metadata and extensions.
    pub _limit: Option<Element>,
    /// Order of the results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<Element>,
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
    /// ValueSet for the filter
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// What code is expected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<Coding>,
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
/// DataRequirement nested structure for the 'valueFilter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirementValuefilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// An attribute to filter on
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// A parameter to search on
    #[serde(rename = "searchParam")]
    pub search_param: Option<StringType>,
    /// Extension element for the 'searchParam' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchParam")]
    pub _search_param: Option<Element>,
    /// eq | gt | lt | ge | le | sa | eb
    pub comparator: Option<ValueFilterComparator>,
    /// Extension element for the 'comparator' primitive field. Contains metadata and extensions.
    pub _comparator: Option<Element>,
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

impl Default for DataRequirement {
    fn default() -> Self {
        Self {
            base: DataType::default(),
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
            value_filter: Default::default(),
            limit: Default::default(),
            _limit: Default::default(),
            sort: Default::default(),
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

impl Default for DataRequirementValuefilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            search_param: Default::default(),
            _search_param: Default::default(),
            comparator: Default::default(),
            _comparator: Default::default(),
            value_date_time: Default::default(),
            value_period: Default::default(),
            value_duration: Default::default(),
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
                "drq-1",
                rh_foundation::Severity::Error,
                "Either a path or a searchParam must be provided, but not both",
                "path.exists() xor searchParam.exists()",
            ),
            rh_foundation::Invariant::new(
                "drq-2",
                rh_foundation::Severity::Error,
                "Either a path or a searchParam must be provided, but not both",
                "path.exists() xor searchParam.exists()",
            ),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "DataRequirement.sort.direction",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/sort-direction|5.0.0",
            )
            .with_description("The possible sort directions, ascending or descending."),
            rh_foundation::ElementBinding::new(
                "DataRequirement.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/fhir-types|5.0.0",
            )
            .with_description("List of FHIR types (resources, data types)."),
            rh_foundation::ElementBinding::new(
                "DataRequirement.valueFilter.comparator",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/value-filter-comparator|5.0.0",
            )
            .with_description("Possible comparators for the valueFilter element."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DataRequirement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.extension", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.profile", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.mustSupport", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.codeFilter", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.codeFilter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.codeFilter.extension", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.codeFilter.path", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.codeFilter.searchParam",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.codeFilter.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DataRequirement.codeFilter.code", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.dateFilter", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.dateFilter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.dateFilter.extension", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.dateFilter.path", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.dateFilter.searchParam",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.dateFilter.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DataRequirement.valueFilter", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.valueFilter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.valueFilter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DataRequirement.valueFilter.path", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.valueFilter.searchParam",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.valueFilter.comparator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DataRequirement.valueFilter.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DataRequirement.limit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.sort", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.sort.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.sort.extension", 0, None),
            rh_foundation::ElementCardinality::new("DataRequirement.sort.path", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DataRequirement.sort.direction", 1, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for DataRequirement {
    fn resource_type(&self) -> &'static str {
        "DataRequirement"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/DataRequirement")
    }
}
