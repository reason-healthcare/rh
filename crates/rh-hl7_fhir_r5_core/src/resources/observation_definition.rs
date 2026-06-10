use crate::bindings::administrative_gender::AdministrativeGender;
use crate::bindings::observation_range_category::ObservationRangeCategory;
use crate::bindings::permitted_data_type::PermittedDataType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ObservationDefinition
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Logical canonical URL to reference this ObservationDefinition (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business identifier of the ObservationDefinition
    pub identifier: Option<Identifier>,
    /// Business version of the ObservationDefinition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this ObservationDefinition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this ObservationDefinition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// If for testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The name of the individual or organization that published the ObservationDefinition
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the ObservationDefinition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Content intends to support these contexts
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for this ObservationDefinition (if applicable)
    ///
    /// Binding: extensible (Codes for country, country subdivision and region for indicating where a resource is intended to be used.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this ObservationDefinition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// When ObservationDefinition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// Date on which the asset content was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// The effective date range for the ObservationDefinition
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Based on FHIR definition of another observation
    #[serde(rename = "derivedFromCanonical")]
    pub derived_from_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFromCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFromCanonical")]
    pub _derived_from_canonical: Option<Element>,
    /// Based on external definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFromUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFromUri")]
    pub _derived_from_uri: Option<Element>,
    /// Type of subject for the defined observation
    pub subject: Option<Vec<CodeableConcept>>,
    /// Desired kind of performer for such kind of observation
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// General type of observation
    ///
    /// Binding: example (Codes for high level observation categories.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of observation
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<PermittedDataType>>,
    /// Extension element for the 'permittedDataType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_permittedDataType")]
    pub _permitted_data_type: Option<Element>,
    /// Multiple results allowed for conforming observations
    #[serde(rename = "multipleResultsAllowed")]
    pub multiple_results_allowed: Option<BooleanType>,
    /// Extension element for the 'multipleResultsAllowed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_multipleResultsAllowed")]
    pub _multiple_results_allowed: Option<Element>,
    /// Body part to be observed
    ///
    /// Binding: example (SNOMED CT body structures.)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Method used to produce the observation
    ///
    /// Binding: example (Methods for simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-methods
    pub method: Option<CodeableConcept>,
    /// Kind of specimen used by this type of observation
    pub specimen: Option<Vec<Reference>>,
    /// Measurement device or model of device
    pub device: Option<Vec<Reference>>,
    /// The preferred name to be used when reporting the observation results
    #[serde(rename = "preferredReportName")]
    pub preferred_report_name: Option<StringType>,
    /// Extension element for the 'preferredReportName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preferredReportName")]
    pub _preferred_report_name: Option<Element>,
    /// Unit for quantitative results
    ///
    /// Binding: preferred (Codes identifying units of measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ucum-units
    #[serde(rename = "permittedUnit")]
    pub permitted_unit: Option<Vec<Coding>>,
    /// Set of qualified values for observation results
    #[serde(rename = "qualifiedValue")]
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedvalue>>,
    /// Definitions of related resources belonging to this kind of observation group
    #[serde(rename = "hasMember")]
    pub has_member: Option<Vec<Reference>>,
    /// Component results
    pub component: Option<Vec<ObservationDefinitionComponent>>,
}
/// ObservationDefinition nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinitionComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of observation
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<PermittedDataType>>,
    /// Extension element for the 'permittedDataType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_permittedDataType")]
    pub _permitted_data_type: Option<Element>,
    /// Unit for quantitative results
    ///
    /// Binding: preferred (Codes identifying units of measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ucum-units
    #[serde(rename = "permittedUnit")]
    pub permitted_unit: Option<Vec<Coding>>,
    /// Set of qualified values for observation results
    #[serde(rename = "qualifiedValue")]
    pub qualified_value: Option<Vec<StringType>>,
}
/// ObservationDefinition nested structure for the 'qualifiedValue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinitionQualifiedvalue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Context qualifier for the set of qualified values
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/referencerange-meaning
    pub context: Option<CodeableConcept>,
    /// Targetted population for the set of qualified values
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `248153007`
    /// - `248152002`
    /// - `77386006`
    #[serde(rename = "appliesTo")]
    pub applies_to: Option<Vec<CodeableConcept>>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// Applicable age range for the set of qualified values
    pub age: Option<Range>,
    /// Applicable gestational age range for the set of qualified values
    #[serde(rename = "gestationalAge")]
    pub gestational_age: Option<Range>,
    /// Condition associated with the set of qualified values
    pub condition: Option<StringType>,
    /// Extension element for the 'condition' primitive field. Contains metadata and extensions.
    pub _condition: Option<Element>,
    /// reference | critical | absolute
    #[serde(rename = "rangeCategory")]
    pub range_category: Option<ObservationRangeCategory>,
    /// Extension element for the 'rangeCategory' primitive field. Contains metadata and extensions.
    #[serde(rename = "_rangeCategory")]
    pub _range_category: Option<Element>,
    /// The range for continuous or ordinal observations
    pub range: Option<Range>,
    /// Value set of valid coded values as part of this set of qualified values
    #[serde(rename = "validCodedValueSet")]
    pub valid_coded_value_set: Option<StringType>,
    /// Extension element for the 'validCodedValueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_validCodedValueSet")]
    pub _valid_coded_value_set: Option<Element>,
    /// Value set of normal coded values as part of this set of qualified values
    #[serde(rename = "normalCodedValueSet")]
    pub normal_coded_value_set: Option<StringType>,
    /// Extension element for the 'normalCodedValueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_normalCodedValueSet")]
    pub _normal_coded_value_set: Option<Element>,
    /// Value set of abnormal coded values as part of this set of qualified values
    #[serde(rename = "abnormalCodedValueSet")]
    pub abnormal_coded_value_set: Option<StringType>,
    /// Extension element for the 'abnormalCodedValueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_abnormalCodedValueSet")]
    pub _abnormal_coded_value_set: Option<Element>,
    /// Value set of critical coded values as part of this set of qualified values
    #[serde(rename = "criticalCodedValueSet")]
    pub critical_coded_value_set: Option<StringType>,
    /// Extension element for the 'criticalCodedValueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_criticalCodedValueSet")]
    pub _critical_coded_value_set: Option<Element>,
}

impl Default for ObservationDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            effective_period: Default::default(),
            derived_from_canonical: Default::default(),
            _derived_from_canonical: Default::default(),
            derived_from_uri: Default::default(),
            _derived_from_uri: Default::default(),
            subject: Default::default(),
            performer_type: Default::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            permitted_data_type: Default::default(),
            _permitted_data_type: Default::default(),
            multiple_results_allowed: Default::default(),
            _multiple_results_allowed: Default::default(),
            body_site: Default::default(),
            method: Default::default(),
            specimen: Default::default(),
            device: Default::default(),
            preferred_report_name: Default::default(),
            _preferred_report_name: Default::default(),
            permitted_unit: Default::default(),
            qualified_value: Default::default(),
            has_member: Default::default(),
            component: Default::default(),
        }
    }
}

impl Default for ObservationDefinitionComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            permitted_data_type: Default::default(),
            _permitted_data_type: Default::default(),
            permitted_unit: Default::default(),
            qualified_value: Default::default(),
        }
    }
}

impl Default for ObservationDefinitionQualifiedvalue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            context: Default::default(),
            applies_to: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            age: Default::default(),
            gestational_age: Default::default(),
            condition: Default::default(),
            _condition: Default::default(),
            range_category: Default::default(),
            _range_category: Default::default(),
            range: Default::default(),
            valid_coded_value_set: Default::default(),
            _valid_coded_value_set: Default::default(),
            normal_coded_value_set: Default::default(),
            _normal_coded_value_set: Default::default(),
            abnormal_coded_value_set: Default::default(),
            _abnormal_coded_value_set: Default::default(),
            critical_coded_value_set: Default::default(),
            _critical_coded_value_set: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("obd-0", rh_foundation::Severity::Error, "If permittedUnit exists, then permittedDataType=Quantity must exist.", "permittedUnit.exists() implies (permittedDataType = 'Quantity').exists()"),
    rh_foundation::Invariant::new("obd-1", rh_foundation::Severity::Error, "If permittedUnit exists, then permittedDataType=Quantity must exist.", "permittedUnit.exists() implies (permittedDataType = 'Quantity').exists()"),
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
                "ObservationDefinition.component.permittedDataType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/permitted-data-type|5.0.0",
            )
            .with_description("Permitted data type for observation value."),
            rh_foundation::ElementBinding::new(
                "ObservationDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ObservationDefinition.permittedDataType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/permitted-data-type|5.0.0",
            )
            .with_description("Permitted data type for observation value."),
            rh_foundation::ElementBinding::new(
                "ObservationDefinition.qualifiedValue.gender",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/administrative-gender|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ObservationDefinition.qualifiedValue.rangeCategory",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/observation-range-category|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "ObservationDefinition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("Codes identifying the state of an ObservationDefinition."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ObservationDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.experimental",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.copyrightLabel",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.approvalDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.lastReviewDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.derivedFromCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.derivedFromUri", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.subject", 0, None),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.performerType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.category", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.permittedDataType",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.multipleResultsAllowed",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.method", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ObservationDefinition.specimen", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.device", 0, None),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.preferredReportName",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.permittedUnit", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.qualifiedValue", 0, None),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.context",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.appliesTo",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.gender",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.age",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.gestationalAge",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.condition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.rangeCategory",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.range",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.validCodedValueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.normalCodedValueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.abnormalCodedValueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.qualifiedValue.criticalCodedValueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ObservationDefinition.hasMember", 0, None),
            rh_foundation::ElementCardinality::new("ObservationDefinition.component", 0, None),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.permittedDataType",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.permittedUnit",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ObservationDefinition.component.qualifiedValue",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ObservationDefinition {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for ObservationDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ObservationDefinition {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for ObservationDefinition {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for ObservationDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ObservationDefinition {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::observation_definition::ObservationDefinitionAccessors
    for ObservationDefinition
{
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn effective_period(&self) -> Option<Period> {
        self.effective_period.clone()
    }
    fn derived_from_canonical(&self) -> &[StringType] {
        self.derived_from_canonical.as_deref().unwrap_or(&[])
    }
    fn derived_from_uri(&self) -> &[StringType] {
        self.derived_from_uri.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> &[CodeableConcept] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn performer_type(&self) -> Option<CodeableConcept> {
        self.performer_type.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn permitted_data_type(&self) -> &[PermittedDataType] {
        self.permitted_data_type.as_deref().unwrap_or(&[])
    }
    fn multiple_results_allowed(&self) -> Option<BooleanType> {
        self.multiple_results_allowed
    }
    fn body_site(&self) -> Option<CodeableConcept> {
        self.body_site.clone()
    }
    fn method(&self) -> Option<CodeableConcept> {
        self.method.clone()
    }
    fn specimen(&self) -> &[Reference] {
        self.specimen.as_deref().unwrap_or(&[])
    }
    fn device(&self) -> &[Reference] {
        self.device.as_deref().unwrap_or(&[])
    }
    fn preferred_report_name(&self) -> Option<StringType> {
        self.preferred_report_name.clone()
    }
    fn permitted_unit(&self) -> &[Coding] {
        self.permitted_unit.as_deref().unwrap_or(&[])
    }
    fn qualified_value(&self) -> &[ObservationDefinitionQualifiedvalue] {
        self.qualified_value.as_deref().unwrap_or(&[])
    }
    fn has_member(&self) -> &[Reference] {
        self.has_member.as_deref().unwrap_or(&[])
    }
    fn component(&self) -> &[ObservationDefinitionComponent] {
        self.component.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::observation_definition::ObservationDefinitionMutators
    for ObservationDefinition
{
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_approval_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.approval_date = Some(value);
        resource
    }
    fn set_last_review_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_review_date = Some(value);
        resource
    }
    fn set_effective_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.effective_period = Some(value);
        resource
    }
    fn set_derived_from_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from_canonical = Some(value);
        resource
    }
    fn add_derived_from_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_derived_from_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from_uri = Some(value);
        resource
    }
    fn add_derived_from_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from_uri
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn add_subject(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.subject.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_performer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_permitted_data_type(self, value: Vec<PermittedDataType>) -> Self {
        let mut resource = self.clone();
        resource.permitted_data_type = Some(value);
        resource
    }
    fn add_permitted_data_type(self, item: PermittedDataType) -> Self {
        let mut resource = self.clone();
        resource
            .permitted_data_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_multiple_results_allowed(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.multiple_results_allowed = Some(value);
        resource
    }
    fn set_body_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_method(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.method = Some(value);
        resource
    }
    fn set_specimen(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn add_specimen(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_device(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn add_device(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.device.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_preferred_report_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.preferred_report_name = Some(value);
        resource
    }
    fn set_permitted_unit(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.permitted_unit = Some(value);
        resource
    }
    fn add_permitted_unit(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource
            .permitted_unit
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_qualified_value(self, value: Vec<ObservationDefinitionQualifiedvalue>) -> Self {
        let mut resource = self.clone();
        resource.qualified_value = Some(value);
        resource
    }
    fn add_qualified_value(self, item: ObservationDefinitionQualifiedvalue) -> Self {
        let mut resource = self.clone();
        resource
            .qualified_value
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_has_member(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.has_member = Some(value);
        resource
    }
    fn add_has_member(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.has_member.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_component(self, value: Vec<ObservationDefinitionComponent>) -> Self {
        let mut resource = self.clone();
        resource.component = Some(value);
        resource
    }
    fn add_component(self, item: ObservationDefinitionComponent) -> Self {
        let mut resource = self.clone();
        resource.component.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::observation_definition::ObservationDefinitionExistence
    for ObservationDefinition
{
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_effective_period(&self) -> bool {
        self.effective_period.is_some()
    }
    fn has_derived_from_canonical(&self) -> bool {
        self.derived_from_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_derived_from_uri(&self) -> bool {
        self.derived_from_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_permitted_data_type(&self) -> bool {
        self.permitted_data_type
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_multiple_results_allowed(&self) -> bool {
        self.multiple_results_allowed.is_some()
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_method(&self) -> bool {
        self.method.is_some()
    }
    fn has_specimen(&self) -> bool {
        self.specimen.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_device(&self) -> bool {
        self.device.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_preferred_report_name(&self) -> bool {
        self.preferred_report_name.is_some()
    }
    fn has_permitted_unit(&self) -> bool {
        self.permitted_unit.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_qualified_value(&self) -> bool {
        self.qualified_value.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_has_member(&self) -> bool {
        self.has_member.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_component(&self) -> bool {
        self.component.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ObservationDefinition {
    fn resource_type(&self) -> &'static str {
        "ObservationDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ObservationDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::observation_definition::{
    ObservationDefinitionAccessors, ObservationDefinitionExistence, ObservationDefinitionMutators,
};
