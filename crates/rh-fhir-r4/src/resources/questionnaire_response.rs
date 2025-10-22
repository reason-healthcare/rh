use crate::bindings::questionnaire_answers_status::QuestionnaireAnswersStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// QuestionnaireResponse
///
/// A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: QuestionnaireResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique id for this set of answers
    pub identifier: Option<Identifier>,
    /// Request fulfilled by this QuestionnaireResponse
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of this action
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// Form being answered
    pub questionnaire: Option<StringType>,
    /// Extension element for the 'questionnaire' primitive field. Contains metadata and extensions.
    pub _questionnaire: Option<Element>,
    /// in-progress | completed | amended | entered-in-error | stopped
    pub status: QuestionnaireAnswersStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The subject of the questions
    pub subject: Option<Reference>,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// Date the answers were gathered
    pub authored: Option<DateTimeType>,
    /// Extension element for the 'authored' primitive field. Contains metadata and extensions.
    pub _authored: Option<Element>,
    /// Person who received and recorded the answers
    pub author: Option<Reference>,
    /// The person who answered the questions
    pub source: Option<Reference>,
    /// Groups and questions
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}
/// QuestionnaireResponseItem nested structure for the 'answer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireResponseItemAnswer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Single-valued answer to the question (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Single-valued answer to the question (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<DecimalType>,
    /// Single-valued answer to the question (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// Single-valued answer to the question (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<StringType>,
    /// Single-valued answer to the question (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// Single-valued answer to the question (time)
    #[serde(rename = "valueTime")]
    pub value_time: Option<TimeType>,
    /// Single-valued answer to the question (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Single-valued answer to the question (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: Option<StringType>,
    /// Single-valued answer to the question (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// Single-valued answer to the question (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
    /// Single-valued answer to the question (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Single-valued answer to the question (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// Nested groups and questions
    pub item: Option<Vec<StringType>>,
}
/// QuestionnaireResponse nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireResponseItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The response(s) to the question
    pub answer: Option<Vec<QuestionnaireResponseItemAnswer>>,
    /// Pointer to specific item from Questionnaire
    #[serde(rename = "linkId")]
    pub link_id: StringType,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// ElementDefinition - details for the item
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Name for group or question text
    pub text: Option<StringType>,
    /// Extension element for the 'text' primitive field. Contains metadata and extensions.
    pub _text: Option<Element>,
    /// Nested questionnaire response items
    pub item: Option<Vec<StringType>>,
}

impl Default for QuestionnaireResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            questionnaire: Default::default(),
            _questionnaire: Default::default(),
            status: QuestionnaireAnswersStatus::default(),
            _status: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            authored: Default::default(),
            _authored: Default::default(),
            author: Default::default(),
            source: Default::default(),
            item: Default::default(),
        }
    }
}

impl Default for QuestionnaireResponseItemAnswer {
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
            item: Default::default(),
        }
    }
}

impl Default for QuestionnaireResponseItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            answer: Default::default(),
            link_id: StringType::default(),
            _link_id: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            text: Default::default(),
            _text: Default::default(),
            item: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for QuestionnaireResponse {
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

impl crate::traits::resource::ResourceMutators for QuestionnaireResponse {
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

impl crate::traits::resource::ResourceExistence for QuestionnaireResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for QuestionnaireResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for QuestionnaireResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for QuestionnaireResponse {
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

impl crate::traits::questionnaire_response::QuestionnaireResponseAccessors
    for QuestionnaireResponse
{
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn questionnaire(&self) -> Option<StringType> {
        self.questionnaire.clone()
    }
    fn status(&self) -> QuestionnaireAnswersStatus {
        self.status.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn authored(&self) -> Option<DateTimeType> {
        self.authored.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn source(&self) -> Option<Reference> {
        self.source.clone()
    }
    fn item(&self) -> &[QuestionnaireResponseItem] {
        self.item.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::questionnaire_response::QuestionnaireResponseMutators
    for QuestionnaireResponse
{
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_questionnaire(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.questionnaire = Some(value);
        resource
    }
    fn set_status(self, value: QuestionnaireAnswersStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_authored(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored = Some(value);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn set_item(self, value: Vec<QuestionnaireResponseItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: QuestionnaireResponseItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::questionnaire_response::QuestionnaireResponseExistence
    for QuestionnaireResponse
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
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_questionnaire(&self) -> bool {
        self.questionnaire.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_authored(&self) -> bool {
        self.authored.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_source(&self) -> bool {
        self.source.is_some()
    }
    fn has_item(&self) -> bool {
        self.item.as_ref().is_some_and(|v| !v.is_empty())
    }
}
