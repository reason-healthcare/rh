use crate::bindings::ucum_units::UcumUnits;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// SampledData
///
/// SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SampledData
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: SampledData
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampledData {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Zero value and units
    pub origin: Quantity,
    /// Number of intervalUnits between samples
    pub interval: Option<DecimalType>,
    /// Extension element for the 'interval' primitive field. Contains metadata and extensions.
    pub _interval: Option<Element>,
    /// The measurement unit of the interval between samples
    #[serde(rename = "intervalUnit")]
    pub interval_unit: UcumUnits,
    /// Extension element for the 'intervalUnit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_intervalUnit")]
    pub _interval_unit: Option<Element>,
    /// Multiply data by this before adding to origin
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Lower limit of detection
    #[serde(rename = "lowerLimit")]
    pub lower_limit: Option<DecimalType>,
    /// Extension element for the 'lowerLimit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lowerLimit")]
    pub _lower_limit: Option<Element>,
    /// Upper limit of detection
    #[serde(rename = "upperLimit")]
    pub upper_limit: Option<DecimalType>,
    /// Extension element for the 'upperLimit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_upperLimit")]
    pub _upper_limit: Option<Element>,
    /// Number of sample points at each time point
    pub dimensions: PositiveIntType,
    /// Extension element for the 'dimensions' primitive field. Contains metadata and extensions.
    pub _dimensions: Option<Element>,
    /// Defines the codes used in the data
    #[serde(rename = "codeMap")]
    pub code_map: Option<StringType>,
    /// Extension element for the 'codeMap' primitive field. Contains metadata and extensions.
    #[serde(rename = "_codeMap")]
    pub _code_map: Option<Element>,
    /// Offsets, typically in time, at which data values were taken
    pub offsets: Option<StringType>,
    /// Extension element for the 'offsets' primitive field. Contains metadata and extensions.
    pub _offsets: Option<Element>,
    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    pub data: Option<StringType>,
    /// Extension element for the 'data' primitive field. Contains metadata and extensions.
    pub _data: Option<Element>,
}

impl Default for SampledData {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            origin: Quantity::default(),
            interval: Default::default(),
            _interval: Default::default(),
            interval_unit: UcumUnits::default(),
            _interval_unit: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            lower_limit: Default::default(),
            _lower_limit: Default::default(),
            upper_limit: Default::default(),
            _upper_limit: Default::default(),
            dimensions: PositiveIntType::default(),
            _dimensions: Default::default(),
            code_map: Default::default(),
            _code_map: Default::default(),
            offsets: Default::default(),
            _offsets: Default::default(),
            data: Default::default(),
            _data: Default::default(),
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
            rh_foundation::Invariant::new(
                "sdd-1",
                rh_foundation::Severity::Error,
                "A SampledData SAHLL have either an interval and offsets but not both",
                "interval.exists().not() xor offsets.exists().not()",
            ),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "SampledData.intervalUnit",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/ucum-units|5.0.0",
        )
        .with_description("Units of measure allowed for an element.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SampledData.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.extension", 0, None),
            rh_foundation::ElementCardinality::new("SampledData.origin", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.interval", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.intervalUnit", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.factor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.lowerLimit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.upperLimit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.dimensions", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.codeMap", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.offsets", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SampledData.data", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for SampledData {
    fn resource_type(&self) -> &'static str {
        "SampledData"
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
        Some("http://hl7.org/fhir/StructureDefinition/SampledData")
    }
}
