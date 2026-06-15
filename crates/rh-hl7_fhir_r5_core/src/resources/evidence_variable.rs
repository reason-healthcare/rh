use crate::bindings::characteristic_combination::CharacteristicCombination;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::variable_handling::VariableHandling;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EvidenceVariable
///
/// The EvidenceVariable resource describes an element that knowledge (Evidence) is about.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EvidenceVariable
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this evidence variable, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the evidence variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the evidence variable
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this evidence variable (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this evidence variable (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<StringType>,
    /// Extension element for the 'shortTitle' primitive field. Contains metadata and extensions.
    #[serde(rename = "_shortTitle")]
    pub _short_title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the evidence variable
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Why this EvidenceVariable is defined
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
    /// When the resource was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the resource was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the resource is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<ContactDetail>,
    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<ContactDetail>,
    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<ContactDetail>,
    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<ContactDetail>,
    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// Actual or conceptual
    pub actual: Option<BooleanType>,
    /// Extension element for the 'actual' primitive field. Contains metadata and extensions.
    pub _actual: Option<Element>,
    /// A defining factor of the EvidenceVariable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<EvidenceVariableCharacteristic>,
    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<VariableHandling>,
    /// Extension element for the 'handling' primitive field. Contains metadata and extensions.
    pub _handling: Option<Element>,
    /// A grouping for ordinal or polychotomous variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<EvidenceVariableCategory>,
}
/// EvidenceVariable nested structure for the 'category' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariableCategory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of the grouping
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Definition of the grouping (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Definition of the grouping (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Definition of the grouping (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
}
/// EvidenceVariableCharacteristic nested structure for the 'definitionByCombination' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariableCharacteristicDefinitionbycombination {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// all-of | any-of | at-least | at-most | statistical | net-effect | dataset
    pub code: CharacteristicCombination,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    pub threshold: Option<PositiveIntType>,
    /// Extension element for the 'threshold' primitive field. Contains metadata and extensions.
    pub _threshold: Option<Element>,
    /// A defining factor of the characteristic
    pub characteristic: Vec<StringType>,
}
/// EvidenceVariableCharacteristic nested structure for the 'timeFromEvent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariableCharacteristicTimefromevent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Human readable description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// The event used as a base point (reference point) in time (CodeableConcept)
    #[serde(rename = "eventCodeableConcept")]
    pub event_codeable_concept: Option<CodeableConcept>,
    /// The event used as a base point (reference point) in time (Reference)
    #[serde(rename = "eventReference")]
    pub event_reference: Option<Reference>,
    /// The event used as a base point (reference point) in time (dateTime)
    #[serde(rename = "eventDateTime")]
    pub event_date_time: Option<DateTimeType>,
    /// The event used as a base point (reference point) in time (id)
    #[serde(rename = "eventId")]
    pub event_id: Option<StringType>,
    /// Used to express the observation at a defined amount of time before or after the event
    pub quantity: Option<Quantity>,
    /// Used to express the observation within a period before and/or after the event
    pub range: Option<Range>,
}
/// EvidenceVariable nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariableCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Timing in which the characteristic is determined
    #[serde(rename = "timeFromEvent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_from_event: Vec<EvidenceVariableCharacteristicTimefromevent>,
    /// Defines the characteristic using type and value
    #[serde(rename = "definitionByTypeAndValue")]
    pub definition_by_type_and_value:
        Option<EvidenceVariableCharacteristicDefinitionbytypeandvalue>,
    /// Used to specify how two or more characteristics are combined
    #[serde(rename = "definitionByCombination")]
    pub definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionbycombination>,
    /// Label for internal linking
    #[serde(rename = "linkId")]
    pub link_id: Option<StringType>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// Natural language description of the characteristic
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Whether the characteristic is an inclusion criterion or exclusion criterion
    pub exclude: Option<BooleanType>,
    /// Extension element for the 'exclude' primitive field. Contains metadata and extensions.
    pub _exclude: Option<Element>,
    /// Defines the characteristic (without using type and value) by a Reference
    #[serde(rename = "definitionReference")]
    pub definition_reference: Option<Reference>,
    /// Defines the characteristic (without using type and value) by a Canonical
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<StringType>,
    /// Extension element for the 'definitionCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_definitionCanonical")]
    pub _definition_canonical: Option<Element>,
    /// Defines the characteristic (without using type and value) by a CodeableConcept
    #[serde(rename = "definitionCodeableConcept")]
    pub definition_codeable_concept: Option<CodeableConcept>,
    /// Defines the characteristic (without using type and value) by an expression
    #[serde(rename = "definitionExpression")]
    pub definition_expression: Option<Expression>,
    /// Defines the characteristic (without using type and value) by an id
    #[serde(rename = "definitionId")]
    pub definition_id: Option<StringType>,
    /// Extension element for the 'definitionId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_definitionId")]
    pub _definition_id: Option<Element>,
    /// Number of occurrences meeting the characteristic (Quantity)
    #[serde(rename = "instancesQuantity")]
    pub instances_quantity: Option<Quantity>,
    /// Number of occurrences meeting the characteristic (Range)
    #[serde(rename = "instancesRange")]
    pub instances_range: Option<Range>,
    /// Length of time in which the characteristic is met (Quantity)
    #[serde(rename = "durationQuantity")]
    pub duration_quantity: Option<Quantity>,
    /// Length of time in which the characteristic is met (Range)
    #[serde(rename = "durationRange")]
    pub duration_range: Option<Range>,
}
/// EvidenceVariableCharacteristic nested structure for the 'definitionByTypeAndValue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariableCharacteristicDefinitionbytypeandvalue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Expresses the type of characteristic
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/usage-context-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Method for how the characteristic value was determined
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-method
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method: Vec<CodeableConcept>,
    /// Device used for determining characteristic
    pub device: Option<Reference>,
    /// Defines the characteristic when coupled with characteristic.type (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Defines the characteristic when coupled with characteristic.type (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Defines the characteristic when coupled with characteristic.type (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Defines the characteristic when coupled with characteristic.type (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Defines the characteristic when coupled with characteristic.type (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Defines the characteristic when coupled with characteristic.type (id)
    #[serde(rename = "valueId")]
    pub value_id: StringType,
    /// Reference point for valueQuantity or valueRange
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/characteristic-offset
    pub offset: Option<CodeableConcept>,
}

impl Default for EvidenceVariable {
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
            short_title: Default::default(),
            _short_title: Default::default(),
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
            note: Default::default(),
            use_context: Default::default(),
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
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            related_artifact: Default::default(),
            actual: Default::default(),
            _actual: Default::default(),
            characteristic: Default::default(),
            handling: Default::default(),
            _handling: Default::default(),
            category: Default::default(),
        }
    }
}

impl Default for EvidenceVariableCategory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
        }
    }
}

impl Default for EvidenceVariableCharacteristicDefinitionbycombination {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            threshold: Default::default(),
            _threshold: Default::default(),
            characteristic: Default::default(),
        }
    }
}

impl Default for EvidenceVariableCharacteristicTimefromevent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            event_codeable_concept: Default::default(),
            event_reference: Default::default(),
            event_date_time: Default::default(),
            event_id: Default::default(),
            quantity: Default::default(),
            range: Default::default(),
        }
    }
}

impl Default for EvidenceVariableCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            time_from_event: Default::default(),
            definition_by_type_and_value: Default::default(),
            definition_by_combination: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            exclude: Default::default(),
            _exclude: Default::default(),
            definition_reference: Default::default(),
            definition_canonical: Default::default(),
            _definition_canonical: Default::default(),
            definition_codeable_concept: Default::default(),
            definition_expression: Default::default(),
            definition_id: Default::default(),
            _definition_id: Default::default(),
            instances_quantity: Default::default(),
            instances_range: Default::default(),
            duration_quantity: Default::default(),
            duration_range: Default::default(),
        }
    }
}

impl Default for EvidenceVariableCharacteristicDefinitionbytypeandvalue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            method: Default::default(),
            device: Default::default(),
            value_codeable_concept: Default::default(),
            value_boolean: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_reference: Default::default(),
            value_id: Default::default(),
            offset: Default::default(),
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
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("evv-1", rh_foundation::Severity::Error, "In a characteristic, at most one of these six elements shall be used: definitionReference or definitionCanonical or definitionCodeableConcept or definitionId or definitionByTypeAndValue or definitionByCombination", "(definitionReference.count() + definitionCanonical.count() + definitionCodeableConcept.count() + definitionId.count() + definitionByTypeAndValue.count() + definitionByCombination.count())  < 2"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
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
                "EvidenceVariable.characteristic.definitionByCombination.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/characteristic-combination|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "EvidenceVariable.handling",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/variable-handling|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "EvidenceVariable.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "EvidenceVariable.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EvidenceVariable.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.contained", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.extension", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceVariable.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.shortTitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.contact", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.note", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.useContext", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.author", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.editor", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.endorser", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.actual", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.characteristic", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.linkId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceVariable.characteristic.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.exclude",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionCodeableConcept",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionExpression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.method",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.device",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByTypeAndValue.offset",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.threshold",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.definitionByCombination.characteristic",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.instances[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.duration[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.note",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.event[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.characteristic.timeFromEvent.range",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EvidenceVariable.handling", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.category", 0, None),
            rh_foundation::ElementCardinality::new("EvidenceVariable.category.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EvidenceVariable.category.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.category.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EvidenceVariable.category.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EvidenceVariable.category.value[x]",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EvidenceVariable {
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

impl crate::traits::resource::ResourceMutators for EvidenceVariable {
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

impl crate::traits::resource::ResourceExistence for EvidenceVariable {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EvidenceVariable {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for EvidenceVariable {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for EvidenceVariable {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::evidence_variable::EvidenceVariableAccessors for EvidenceVariable {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
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
    fn short_title(&self) -> Option<StringType> {
        self.short_title.clone()
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
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
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
    fn author(&self) -> &[ContactDetail] {
        self.author.as_slice()
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_slice()
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_slice()
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_slice()
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn actual(&self) -> Option<BooleanType> {
        self.actual
    }
    fn characteristic(&self) -> &[EvidenceVariableCharacteristic] {
        self.characteristic.as_slice()
    }
    fn handling(&self) -> Option<VariableHandling> {
        self.handling.clone()
    }
    fn category(&self) -> &[EvidenceVariableCategory] {
        self.category.as_slice()
    }
}

impl crate::traits::evidence_variable::EvidenceVariableMutators for EvidenceVariable {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
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
    fn set_short_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.short_title = Some(value);
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
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
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
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = value;
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = value;
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = value;
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = value;
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = value;
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource.related_artifact.push(item);
        resource
    }
    fn set_actual(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.actual = Some(value);
        resource
    }
    fn set_characteristic(self, value: Vec<EvidenceVariableCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = value;
        resource
    }
    fn add_characteristic(self, item: EvidenceVariableCharacteristic) -> Self {
        let mut resource = self.clone();
        resource.characteristic.push(item);
        resource
    }
    fn set_handling(self, value: VariableHandling) -> Self {
        let mut resource = self.clone();
        resource.handling = Some(value);
        resource
    }
    fn set_category(self, value: Vec<EvidenceVariableCategory>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: EvidenceVariableCategory) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
        resource
    }
}

impl crate::traits::evidence_variable::EvidenceVariableExistence for EvidenceVariable {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
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
    fn has_short_title(&self) -> bool {
        self.short_title.is_some()
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
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
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
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_editor(&self) -> bool {
        !self.editor.is_empty()
    }
    fn has_reviewer(&self) -> bool {
        !self.reviewer.is_empty()
    }
    fn has_endorser(&self) -> bool {
        !self.endorser.is_empty()
    }
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_actual(&self) -> bool {
        self.actual.is_some()
    }
    fn has_characteristic(&self) -> bool {
        !self.characteristic.is_empty()
    }
    fn has_handling(&self) -> bool {
        self.handling.is_some()
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
}

impl crate::validation::ValidatableResource for EvidenceVariable {
    fn resource_type(&self) -> &'static str {
        "EvidenceVariable"
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
        Some("http://hl7.org/fhir/StructureDefinition/EvidenceVariable")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::evidence_variable::{
    EvidenceVariableAccessors, EvidenceVariableExistence, EvidenceVariableMutators,
};
