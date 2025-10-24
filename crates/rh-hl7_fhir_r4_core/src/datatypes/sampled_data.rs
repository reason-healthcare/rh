use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// SampledData
///
/// Base StructureDefinition for SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SampledData
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: SampledData
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampledData {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Zero value and units
    pub origin: Quantity,
    /// Number of milliseconds between samples
    pub period: DecimalType,
    /// Extension element for the 'period' primitive field. Contains metadata and extensions.
    pub _period: Option<Element>,
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
    /// Decimal values with spaces, or "E" | "U" | "L"
    pub data: Option<StringType>,
    /// Extension element for the 'data' primitive field. Contains metadata and extensions.
    pub _data: Option<Element>,
}

impl Default for SampledData {
    fn default() -> Self {
        Self {
            base: Element::default(),
            origin: Quantity::default(),
            period: DecimalType::default(),
            _period: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            lower_limit: Default::default(),
            _lower_limit: Default::default(),
            upper_limit: Default::default(),
            _upper_limit: Default::default(),
            dimensions: PositiveIntType::default(),
            _dimensions: Default::default(),
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

impl crate::validation::ValidatableResource for SampledData {
    fn resource_type(&self) -> &'static str {
        "SampledData"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SampledData")
    }
}
