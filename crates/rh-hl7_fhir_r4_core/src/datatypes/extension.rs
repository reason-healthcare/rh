use crate::datatypes::address::Address;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::contributor::Contributor;
use crate::datatypes::count::Count;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::distance::Distance;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::meta::Meta;
use crate::datatypes::money::Money;
use crate::datatypes::parameter_definition::ParameterDefinition;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::sampled_data::SampledData;
use crate::datatypes::signature::Signature;
use crate::datatypes::timing::Timing;
use crate::datatypes::trigger_definition::TriggerDefinition;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::integer::IntegerType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::primitives::unsigned_int::UnsignedIntType;
use serde::{Deserialize, Serialize};
/// Extension
///
/// Base StructureDefinition for Extension Type: Optional Extension Element - found in all resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Extension
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// identifies the meaning of the extension
    pub url: StringType,
    /// Value of extension (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Option<Base64BinaryType>,
    /// Value of extension (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Value of extension (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: Option<StringType>,
    /// Value of extension (code)
    #[serde(rename = "valueCode")]
    pub value_code: Option<StringType>,
    /// Value of extension (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// Value of extension (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// Value of extension (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<DecimalType>,
    /// Value of extension (id)
    #[serde(rename = "valueId")]
    pub value_id: Option<StringType>,
    /// Value of extension (instant)
    #[serde(rename = "valueInstant")]
    pub value_instant: Option<InstantType>,
    /// Value of extension (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// Value of extension (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<StringType>,
    /// Value of extension (oid)
    #[serde(rename = "valueOid")]
    pub value_oid: Option<StringType>,
    /// Value of extension (positiveInt)
    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: Option<PositiveIntType>,
    /// Value of extension (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Value of extension (time)
    #[serde(rename = "valueTime")]
    pub value_time: Option<TimeType>,
    /// Value of extension (unsignedInt)
    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: Option<UnsignedIntType>,
    /// Value of extension (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: Option<StringType>,
    /// Value of extension (url)
    #[serde(rename = "valueUrl")]
    pub value_url: Option<StringType>,
    /// Value of extension (uuid)
    #[serde(rename = "valueUuid")]
    pub value_uuid: Option<StringType>,
    /// Value of extension (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Option<Address>,
    /// Value of extension (Age)
    #[serde(rename = "valueAge")]
    pub value_age: Option<Age>,
    /// Value of extension (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Option<Annotation>,
    /// Value of extension (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// Value of extension (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Value of extension (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
    /// Value of extension (ContactPoint)
    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: Option<ContactPoint>,
    /// Value of extension (Count)
    #[serde(rename = "valueCount")]
    pub value_count: Option<Count>,
    /// Value of extension (Distance)
    #[serde(rename = "valueDistance")]
    pub value_distance: Option<Distance>,
    /// Value of extension (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Option<Duration>,
    /// Value of extension (HumanName)
    #[serde(rename = "valueHumanName")]
    pub value_human_name: Option<HumanName>,
    /// Value of extension (Identifier)
    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Option<Identifier>,
    /// Value of extension (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Option<Money>,
    /// Value of extension (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,
    /// Value of extension (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// Value of extension (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
    /// Value of extension (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,
    /// Value of extension (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// Value of extension (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,
    /// Value of extension (Signature)
    #[serde(rename = "valueSignature")]
    pub value_signature: Option<Signature>,
    /// Value of extension (Timing)
    #[serde(rename = "valueTiming")]
    pub value_timing: Option<Timing>,
    /// Value of extension (ContactDetail)
    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: Option<ContactDetail>,
    /// Value of extension (Contributor)
    #[serde(rename = "valueContributor")]
    pub value_contributor: Option<Contributor>,
    /// Value of extension (DataRequirement)
    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: Option<DataRequirement>,
    /// Value of extension (Expression)
    #[serde(rename = "valueExpression")]
    pub value_expression: Option<Expression>,
    /// Value of extension (ParameterDefinition)
    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: Option<ParameterDefinition>,
    /// Value of extension (RelatedArtifact)
    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: Option<RelatedArtifact>,
    /// Value of extension (TriggerDefinition)
    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: Option<TriggerDefinition>,
    /// Value of extension (UsageContext)
    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: Option<UsageContext>,
    /// Value of extension (Dosage)
    #[serde(rename = "valueDosage")]
    pub value_dosage: Option<Dosage>,
    /// Value of extension (Meta)
    #[serde(rename = "valueMeta")]
    pub value_meta: Option<Meta>,
}

impl Default for Extension {
    fn default() -> Self {
        Self {
            base: Element::default(),
            url: StringType::default(),
            value_base64_binary: Default::default(),
            value_boolean: Default::default(),
            value_canonical: Default::default(),
            value_code: Default::default(),
            value_date: Default::default(),
            value_date_time: Default::default(),
            value_decimal: Default::default(),
            value_id: Default::default(),
            value_instant: Default::default(),
            value_integer: Default::default(),
            value_markdown: Default::default(),
            value_oid: Default::default(),
            value_positive_int: Default::default(),
            value_string: Default::default(),
            value_time: Default::default(),
            value_unsigned_int: Default::default(),
            value_uri: Default::default(),
            value_url: Default::default(),
            value_uuid: Default::default(),
            value_address: Default::default(),
            value_age: Default::default(),
            value_annotation: Default::default(),
            value_attachment: Default::default(),
            value_codeable_concept: Default::default(),
            value_coding: Default::default(),
            value_contact_point: Default::default(),
            value_count: Default::default(),
            value_distance: Default::default(),
            value_duration: Default::default(),
            value_human_name: Default::default(),
            value_identifier: Default::default(),
            value_money: Default::default(),
            value_period: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_reference: Default::default(),
            value_sampled_data: Default::default(),
            value_signature: Default::default(),
            value_timing: Default::default(),
            value_contact_detail: Default::default(),
            value_contributor: Default::default(),
            value_data_requirement: Default::default(),
            value_expression: Default::default(),
            value_parameter_definition: Default::default(),
            value_related_artifact: Default::default(),
            value_trigger_definition: Default::default(),
            value_usage_context: Default::default(),
            value_dosage: Default::default(),
            value_meta: Default::default(),
        }
    }
}
