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
    pub identifier: Option<Vec<Identifier>>,
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
    pub derived_from: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    pub _derived_from: Option<Element>,
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
    pub subject_type: Option<Vec<ResourceTypes>>,
    /// Extension element for the 'subjectType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subjectType")]
    pub _subject_type: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the questionnaire
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for questionnaire (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
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
    pub approval_date: Option<StringType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the questionnaire was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<StringType>,
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
    pub code: Option<Vec<Coding>>,
    /// Questions and sections within the Questionnaire
    pub item: Option<Vec<QuestionnaireItem>>,
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
    pub value_date: StringType,
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
    pub value_date: StringType,
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
/// Questionnaire nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Only allow data when
    #[serde(rename = "enableWhen")]
    pub enable_when: Option<Vec<QuestionnaireItemEnablewhen>>,
    /// Permitted answer
    #[serde(rename = "answerOption")]
    pub answer_option: Option<Vec<QuestionnaireItemAnsweroption>>,
    /// Initial value(s) when item is first rendered
    pub initial: Option<Vec<QuestionnaireItemInitial>>,
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
    pub code: Option<Vec<Coding>>,
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
    pub item: Option<Vec<StringType>>,
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
    pub answer_date: StringType,
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

impl Default for QuestionnaireSignatureRequired {
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

impl Default for QuestionnaireHidden {
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

impl Default for QuestionnaireMinOccurs {
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

impl Default for QuestionnaireItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            enable_when: Default::default(),
            answer_option: Default::default(),
            initial: Default::default(),
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

impl Default for QuestionnaireSupportLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for Questionnaire {
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

impl crate::traits::questionnaire::QuestionnaireAccessors for Questionnaire {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
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
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn subject_type(&self) -> &[ResourceTypes] {
        self.subject_type.as_deref().unwrap_or(&[])
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
    fn approval_date(&self) -> Option<StringType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<StringType> {
        self.last_review_date.clone()
    }
    fn effective_period(&self) -> Option<Period> {
        self.effective_period.clone()
    }
    fn code(&self) -> &[Coding] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn item(&self) -> &[QuestionnaireItem] {
        self.item.as_deref().unwrap_or(&[])
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
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
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
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
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
        resource.subject_type = Some(value);
        resource
    }
    fn add_subject_type(self, item: ResourceTypes) -> Self {
        let mut resource = self.clone();
        resource
            .subject_type
            .get_or_insert_with(Vec::new)
            .push(item);
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
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_item(self, value: Vec<QuestionnaireItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: QuestionnaireItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::questionnaire::QuestionnaireExistence for Questionnaire {
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
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_subject_type(&self) -> bool {
        self.subject_type.as_ref().is_some_and(|v| !v.is_empty())
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
        self.code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_item(&self) -> bool {
        self.item.as_ref().is_some_and(|v| !v.is_empty())
    }
}
