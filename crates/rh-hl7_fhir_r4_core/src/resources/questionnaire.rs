use crate::bindings::item_type::ItemType;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::questionnaire_enable_behavior::QuestionnaireEnableBehavior;
use crate::bindings::questionnaire_enable_operator::QuestionnaireEnableOperator;
use crate::bindings::resource_types::ResourceTypes;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Questionnaire
///
/// A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questionnaire {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this questionnaire, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the questionnaire
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this questionnaire (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this questionnaire (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Instantiates protocol or definition
    #[serde(rename = "derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<StringType>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _derived_from: Vec<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Resource that can be subject of QuestionnaireResponse
    #[serde(rename = "subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type: Vec<ResourceTypes>,
    /// Extension element for the 'subjectType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _subject_type: Vec<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the questionnaire
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for questionnaire (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this questionnaire is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the questionnaire was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the questionnaire was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the questionnaire is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Concept that represents the overall questionnaire
    ///
    /// Binding: example (Codes for questionnaires, groups and individual questions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<Coding>,
    /// Questions and sections within the Questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireItem>,
}
/// unit
///
/// Provides a computable unit of measure associated with numeric questions to support subsequent computation on responses. This is for use on items of type integer and decimal, and it's purpose is to support converting the integer or decimal answer into a Quantity when extracting the data into a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unit
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// hidden
///
/// If true, indicates that the extended item should not be displayed to the user.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-hidden
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireHidden {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// minOccurs
///
/// The minimum number of times the group must appear, or the minimum number of answers for a question - when greater than 1.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-minOccurs
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireMinOccurs {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// optionExclusive
///
/// If true, indicates that if this answerOption is selected, no other possible answers may be selected, even if the item is a repeating question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-optionExclusive
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireOptionExclusive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// maxOccurs
///
/// The maximum number of times the group must appear, or the maximum number of answers for a question - when greater than 1 and not unlimited.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-maxOccurs
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireMaxOccurs {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// QuestionnaireItem nested structure for the 'answerOption' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItemAnsweroption {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Answer value (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Answer value (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Answer value (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Answer value (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Answer value (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Answer value (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Whether option is selected by default
    #[serde(rename = "initialSelected")]
    pub initial_selected: Option<BooleanType>,
    /// Extension element for the 'initialSelected' primitive field. Contains metadata and extensions.
    #[serde(rename = "_initialSelected")]
    pub _initial_selected: Option<Element>,
}
/// signatureRequired
///
/// Indicates that a signature (of the specified type) is needed when completing the QuestionnaireResponse.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-signatureRequired
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSignatureRequired {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Questionnaire nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Initial value(s) when item is first rendered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial: Vec<QuestionnaireItemInitial>,
    /// Permitted answer
    #[serde(rename = "answerOption")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer_option: Vec<QuestionnaireItemAnsweroption>,
    /// Only allow data when
    #[serde(rename = "enableWhen")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_when: Vec<QuestionnaireItemEnablewhen>,
    /// Unique id for item in questionnaire
    #[serde(rename = "linkId")]
    pub link_id: StringType,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// ElementDefinition - details for the item
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Corresponding concept for this item in a terminology
    ///
    /// Binding: example (Codes for questionnaires, groups and individual questions.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/questionnaire-questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<Coding>,
    /// E.g. "1(a)", "2.5.3"
    pub prefix: Option<StringType>,
    /// Extension element for the 'prefix' primitive field. Contains metadata and extensions.
    pub _prefix: Option<Element>,
    /// Primary text for the item
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// group | display | boolean | decimal | integer | date | dateTime +
    #[serde(rename = "type")]
    pub type_: ItemType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// all | any
    #[serde(rename = "enableBehavior")]
    pub enable_behavior: Option<QuestionnaireEnableBehavior>,
    /// Extension element for the 'enableBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_enableBehavior")]
    pub _enable_behavior: Option<Element>,
    /// Whether the item must be included in data results
    pub required: Option<BooleanType>,
    /// Extension element for the 'required' primitive field. Contains metadata and extensions.
    pub _required: Option<Element>,
    /// Whether the item may repeat
    pub repeats: Option<BooleanType>,
    /// Extension element for the 'repeats' primitive field. Contains metadata and extensions.
    pub _repeats: Option<Element>,
    /// Don't allow human editing
    #[serde(rename = "readOnly")]
    pub read_only: Option<BooleanType>,
    /// Extension element for the 'readOnly' primitive field. Contains metadata and extensions.
    #[serde(rename = "_readOnly")]
    pub _read_only: Option<Element>,
    /// No more than this many characters
    #[serde(rename = "maxLength")]
    pub max_length: Option<IntegerType>,
    /// Extension element for the 'maxLength' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maxLength")]
    pub _max_length: Option<Element>,
    /// Valueset containing permitted answers
    #[serde(rename = "answerValueSet")]
    pub answer_value_set: Option<StringType>,
    /// Extension element for the 'answerValueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_answerValueSet")]
    pub _answer_value_set: Option<Element>,
    /// Nested questionnaire items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<StringType>,
}
/// optionPrefix
///
/// The label to list in front of a code when presenting a list of possible values in a questionnaire-like fashion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-optionPrefix
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireOptionPrefix {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// supportLink
///
/// A URL that resolves to additional supporting information or guidance related to the question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-supportLink
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSupportLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// referenceProfile
///
/// Where the type for a question is "Reference", indicates a profile that the resource instances pointed to in answers to this question must be valid against.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceProfile
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceProfile {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// unitValueSet
///
/// A set of units that the user may choose when providing a quantity value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unitValueSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnitValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// choiceOrientation
///
/// Identifies the desired orientation when rendering a list of choices (typically radio-box or check-box lists).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-choiceOrientation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireChoiceOrientation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// referenceResource
///
/// Where the type for a question is "Reference", indicates a type of resource that is permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceResource
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// sliderStepValue
///
/// For slider-based controls, indicates the step size to use when toggling the control up or down.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-sliderStepValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSliderStepValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// fhirType
///
/// For questionnaires generated from FHIR profiles, indicates the FHIR data type or resource type that corresponds to this node.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-fhirType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireFHIRType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// unitOption
///
/// A unit that the user may choose when providing a quantity value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unitOption
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnitOption {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// QuestionnaireItem nested structure for the 'initial' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItemInitial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Actual value for initializing the question (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Actual value for initializing the question (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Actual value for initializing the question (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Actual value for initializing the question (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Actual value for initializing the question (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Actual value for initializing the question (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Actual value for initializing the question (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Actual value for initializing the question (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// Actual value for initializing the question (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// Actual value for initializing the question (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Actual value for initializing the question (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Actual value for initializing the question (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
}
/// referenceFilter
///
/// Identifies a filter to apply when looking up candidate answers for the question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceFilter
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceFilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// QuestionnaireItem nested structure for the 'enableWhen' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItemEnablewhen {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Question that determines whether item is enabled
    pub question: StringType,
    /// Extension element for the 'question' primitive field. Contains metadata and extensions.
    pub _question: Option<Element>,
    /// exists | = | != | > | < | >= | <=
    pub operator: QuestionnaireEnableOperator,
    /// Extension element for the 'operator' primitive field. Contains metadata and extensions.
    pub _operator: Option<Element>,
    /// Value for question comparison based on operator (boolean)
    #[serde(rename = "answerBoolean")]
    pub answer_boolean: BooleanType,
    /// Value for question comparison based on operator (decimal)
    #[serde(rename = "answerDecimal")]
    pub answer_decimal: DecimalType,
    /// Value for question comparison based on operator (integer)
    #[serde(rename = "answerInteger")]
    pub answer_integer: IntegerType,
    /// Value for question comparison based on operator (date)
    #[serde(rename = "answerDate")]
    pub answer_date: DateType,
    /// Value for question comparison based on operator (dateTime)
    #[serde(rename = "answerDateTime")]
    pub answer_date_time: DateTimeType,
    /// Value for question comparison based on operator (time)
    #[serde(rename = "answerTime")]
    pub answer_time: TimeType,
    /// Value for question comparison based on operator (string)
    #[serde(rename = "answerString")]
    pub answer_string: StringType,
    /// Value for question comparison based on operator (Coding)
    #[serde(rename = "answerCoding")]
    pub answer_coding: Coding,
    /// Value for question comparison based on operator (Quantity)
    #[serde(rename = "answerQuantity")]
    pub answer_quantity: Quantity,
    /// Value for question comparison based on operator (Reference)
    #[serde(rename = "answerReference")]
    pub answer_reference: Reference,
}
/// constraint
///
/// An invariant that must be satisfied before responses to the questionnaire can be considered "complete".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-constraint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireConstraint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// baseType
///
/// This identifies the underlying type in a profile, when a questionnaire is generated from a profile.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-baseType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireBaseType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Questionnaire {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            derived_from: Default::default(),
            _derived_from: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            subject_type: Default::default(),
            _subject_type: Default::default(),
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
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            effective_period: Default::default(),
            code: Default::default(),
            item: Default::default(),
        }
    }
}

impl Default for QuestionnaireUnit {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireHidden {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireMinOccurs {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireOptionExclusive {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireMaxOccurs {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireItemAnsweroption {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_integer: Default::default(),
            value_date: Default::default(),
            value_time: Default::default(),
            value_string: Default::default(),
            value_coding: Default::default(),
            value_reference: Default::default(),
            initial_selected: Default::default(),
            _initial_selected: Default::default(),
        }
    }
}

impl Default for QuestionnaireSignatureRequired {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            initial: Default::default(),
            answer_option: Default::default(),
            enable_when: Default::default(),
            link_id: StringType::default(),
            _link_id: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            code: Default::default(),
            prefix: Default::default(),
            _prefix: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            enable_behavior: Default::default(),
            _enable_behavior: Default::default(),
            required: Default::default(),
            _required: Default::default(),
            repeats: Default::default(),
            _repeats: Default::default(),
            read_only: Default::default(),
            _read_only: Default::default(),
            max_length: Default::default(),
            _max_length: Default::default(),
            answer_value_set: Default::default(),
            _answer_value_set: Default::default(),
            item: Default::default(),
        }
    }
}

impl Default for QuestionnaireOptionPrefix {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireSupportLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireReferenceProfile {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireUnitValueSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireChoiceOrientation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireReferenceResource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireSliderStepValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireFHIRType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireUnitOption {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireItemInitial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_boolean: Default::default(),
            value_decimal: Default::default(),
            value_integer: Default::default(),
            value_date: Default::default(),
            value_date_time: Default::default(),
            value_time: Default::default(),
            value_string: Default::default(),
            value_uri: Default::default(),
            value_attachment: Default::default(),
            value_coding: Default::default(),
            value_quantity: Default::default(),
            value_reference: Default::default(),
        }
    }
}

impl Default for QuestionnaireReferenceFilter {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireItemEnablewhen {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            question: Default::default(),
            _question: Default::default(),
            operator: Default::default(),
            _operator: Default::default(),
            answer_boolean: Default::default(),
            answer_decimal: Default::default(),
            answer_integer: Default::default(),
            answer_date: Default::default(),
            answer_date_time: Default::default(),
            answer_time: Default::default(),
            answer_string: Default::default(),
            answer_coding: Default::default(),
            answer_quantity: Default::default(),
            answer_reference: Default::default(),
        }
    }
}

impl Default for QuestionnaireConstraint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for QuestionnaireBaseType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("que-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("que-1", rh_foundation::Severity::Error, "Group items must have nested items, display items cannot have nested items", "(type='group' implies item.empty().not()) and (type.trace('type')='display' implies item.trace('item').empty())").with_xpath("not((f:type/@value='group' and not(f:item)) or (f:type/@value='display' and f:item))"),
    rh_foundation::Invariant::new("que-10", rh_foundation::Severity::Error, "Maximum length can only be declared for simple question types", "(type in ('boolean' | 'decimal' | 'integer' | 'string' | 'text' | 'url' | 'open-choice')) or maxLength.empty()").with_xpath("f:type/@value=('boolean', 'decimal', 'integer', 'open-choice', 'string', 'text', 'url') or not(f:maxLength)"),
    rh_foundation::Invariant::new("que-11", rh_foundation::Severity::Error, "If one or more answerOption is present, initial[x] must be missing", "answerOption.empty() or initial.empty()").with_xpath("not(f:answerOption) or not(count(f:*[starts-with(local-name(.), 'initial')]))"),
    rh_foundation::Invariant::new("que-12", rh_foundation::Severity::Error, "If there are more than one enableWhen, enableBehavior must be specified", "enableWhen.count() > 2 implies enableBehavior.exists()").with_xpath("not(f:answerOption) or not(count(f:*[starts-with(local-name(.), 'initial')]))"),
    rh_foundation::Invariant::new("que-13", rh_foundation::Severity::Error, "Can only have multiple initial values for repeating items", "repeats=true or initial.count() <= 1").with_xpath("f:repeats/@value='true' or count(f:initial)<=1"),
    rh_foundation::Invariant::new("que-2", rh_foundation::Severity::Error, "The link ids for groups and questions must be unique within the questionnaire", "descendants().linkId.isDistinct()").with_xpath("count(descendant::f:linkId/@value)=count(distinct-values(descendant::f:linkId/@value))"),
    rh_foundation::Invariant::new("que-3", rh_foundation::Severity::Error, "Display items cannot have a \"code\" asserted", "type!='display' or code.empty()").with_xpath("not(f:type/@value='display' and f:code)"),
    rh_foundation::Invariant::new("que-4", rh_foundation::Severity::Error, "A question cannot have both answerOption and answerValueSet", "answerOption.empty() or answerValueSet.empty()").with_xpath("not(f:answerValueSet and f:answerOption)"),
    rh_foundation::Invariant::new("que-5", rh_foundation::Severity::Error, "Only 'choice' and 'open-choice' items can have answerValueSet", "(type ='choice' or type = 'open-choice' or type = 'decimal' or type = 'integer' or type = 'date' or type = 'dateTime' or type = 'time' or type = 'string' or type = 'quantity') or (answerValueSet.empty() and answerOption.empty())").with_xpath("f:type/@value=('choice','open-choice','decimal','integer','date','dateTime','time','string','quantity',') or not(f:answerOption or f:answerValueSet)"),
    rh_foundation::Invariant::new("que-6", rh_foundation::Severity::Error, "Required and repeat aren't permitted for display items", "type!='display' or (required.empty() and repeats.empty())").with_xpath("not(f:type/@value='display' and (f:required or f:repeats))"),
    rh_foundation::Invariant::new("que-7", rh_foundation::Severity::Error, "If the operator is 'exists', the value must be a boolean", "operator = 'exists' implies (answer is Boolean)").with_xpath("f:operator/@value != 'exists' or exists(f:answerBoolean)"),
    rh_foundation::Invariant::new("que-8", rh_foundation::Severity::Error, "Initial values can't be specified for groups or display items", "(type!='group' and type!='display') or initial.empty()").with_xpath("not(f:type/@value=('group', 'display') and f:*[starts-with(local-name(.), 'initial')])"),
    rh_foundation::Invariant::new("que-9", rh_foundation::Severity::Error, "Read-only can't be specified for \"display\" items", "type!='display' or readOnly.empty()").with_xpath("not(f:type/@value=('group', 'display') and f:*[starts-with(local-name(.), 'initial')])"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("Questionnaire.item.enableBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/questionnaire-enable-behavior|4.0.1").with_description("Controls how multiple enableWhen values are interpreted -  whether all or any must be true."),
    rh_foundation::ElementBinding::new("Questionnaire.item.enableWhen.operator", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/questionnaire-enable-operator|4.0.1").with_description("The criteria by which a question is enabled."),
    rh_foundation::ElementBinding::new("Questionnaire.item.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/item-type|4.0.1").with_description("Distinguishes groups from questions and display text and indicates data type for questions."),
    rh_foundation::ElementBinding::new("Questionnaire.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
    rh_foundation::ElementBinding::new("Questionnaire.subjectType", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-types|4.0.1").with_description("One of the resource types defined as part of this version of FHIR."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Questionnaire.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.contained", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.extension", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.subjectType", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.contact", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.code", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.extension", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.linkId", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.code", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.prefix", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.enableWhen", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.enableWhen.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.enableWhen.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.enableWhen.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.enableWhen.question",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.enableWhen.operator",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.enableWhen.answer[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Questionnaire.item.enableBehavior", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.required", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.repeats", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.readOnly", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.maxLength", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.answerValueSet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.answerOption", 0, None),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.answerOption.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.answerOption.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.answerOption.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.answerOption.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.answerOption.initialSelected",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Questionnaire.item.initial", 0, None),
            rh_foundation::ElementCardinality::new("Questionnaire.item.initial.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Questionnaire.item.initial.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.initial.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Questionnaire.item.initial.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Questionnaire.item.item", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Questionnaire {
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

impl crate::traits::resource::ResourceMutators for Questionnaire {
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

impl crate::traits::resource::ResourceExistence for Questionnaire {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Questionnaire {
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

impl crate::traits::domain_resource::DomainResourceMutators for Questionnaire {
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

impl crate::traits::domain_resource::DomainResourceExistence for Questionnaire {
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

impl crate::traits::questionnaire::QuestionnaireAccessors for Questionnaire {
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
    fn derived_from(&self) -> &[StringType] {
        self.derived_from.as_slice()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn subject_type(&self) -> &[ResourceTypes] {
        self.subject_type.as_slice()
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
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
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
    fn code(&self) -> &[Coding] {
        self.code.as_slice()
    }
    fn item(&self) -> &[QuestionnaireItem] {
        self.item.as_slice()
    }
}

impl crate::traits::questionnaire::QuestionnaireMutators for Questionnaire {
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
    fn set_derived_from(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = value;
        resource
    }
    fn add_derived_from(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.derived_from.push(item);
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
    fn set_subject_type(self, value: Vec<ResourceTypes>) -> Self {
        let mut resource = self.clone();
        resource.subject_type = value;
        resource
    }
    fn add_subject_type(self, item: ResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.subject_type.push(item);
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
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
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
    fn set_code(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn add_code(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource.code.push(item);
        resource
    }
    fn set_item(self, value: Vec<QuestionnaireItem>) -> Self {
        let mut resource = self.clone();
        resource.item = value;
        resource
    }
    fn add_item(self, item: QuestionnaireItem) -> Self {
        let mut resource = self.clone();
        resource.item.push(item);
        resource
    }
}

impl crate::traits::questionnaire::QuestionnaireExistence for Questionnaire {
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
    fn has_derived_from(&self) -> bool {
        !self.derived_from.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_subject_type(&self) -> bool {
        !self.subject_type.is_empty()
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
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
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
    fn has_code(&self) -> bool {
        !self.code.is_empty()
    }
    fn has_item(&self) -> bool {
        !self.item.is_empty()
    }
}

impl crate::validation::ValidatableResource for Questionnaire {
    fn resource_type(&self) -> &'static str {
        "Questionnaire"
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
        Some("http://hl7.org/fhir/StructureDefinition/Questionnaire")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::questionnaire::{
    QuestionnaireAccessors, QuestionnaireExistence, QuestionnaireMutators,
};
