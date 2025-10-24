use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::primitives::date_time::DateTimeType;
use serde::{Deserialize, Serialize};
/// MarketingStatus
///
/// Base StructureDefinition for MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MarketingStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: MarketingStatus
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The country in which the marketing authorisation has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements
    pub country: CodeableConcept,
    /// Where a Medicines Regulatory Agency has granted a marketing authorisation for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified
    pub jurisdiction: Option<CodeableConcept>,
    /// This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples
    pub status: CodeableConcept,
    /// The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    #[serde(rename = "dateRange")]
    pub date_range: Period,
    /// The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<DateTimeType>,
    /// Extension element for the 'restoreDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_restoreDate")]
    pub _restore_date: Option<Element>,
}

impl Default for MarketingStatus {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            country: CodeableConcept::default(),
            jurisdiction: Default::default(),
            status: CodeableConcept::default(),
            date_range: Period::default(),
            restore_date: Default::default(),
            _restore_date: Default::default(),
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

impl crate::validation::ValidatableResource for MarketingStatus {
    fn resource_type(&self) -> &'static str {
        "MarketingStatus"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/MarketingStatus")
    }
}
