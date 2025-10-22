use crate::datatypes::address::Address;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
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
use crate::datatypes::extension::Extension;
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
use crate::resources::resource::Resource;
use serde::{Deserialize, Serialize};
/// Parameters
///
/// This resource is a non-persisted resource used to pass information into and back from an [operation](operations.html). It has no other use, and there is no RESTful endpoint associated with it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Parameters
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Parameters
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Resource,
    /// Operation Parameter
    pub parameter: Option<Vec<ParametersParameter>>,
}
/// fullUrl for resource
///
/// This specifies the fullUrl for the resource in parameters.resource, if there is one. When fullUrl is provided, ithe [resource resolution method described for Bundle](bundle.html#references).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/parameters-fullUrl
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersFullURL {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Parameters nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name from the definition
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// If parameter is a data type (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Option<Base64BinaryType>,
    /// If parameter is a data type (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// If parameter is a data type (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: Option<StringType>,
    /// If parameter is a data type (code)
    #[serde(rename = "valueCode")]
    pub value_code: Option<StringType>,
    /// If parameter is a data type (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// If parameter is a data type (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
    /// If parameter is a data type (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<DecimalType>,
    /// If parameter is a data type (id)
    #[serde(rename = "valueId")]
    pub value_id: Option<StringType>,
    /// If parameter is a data type (instant)
    #[serde(rename = "valueInstant")]
    pub value_instant: Option<InstantType>,
    /// If parameter is a data type (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// If parameter is a data type (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<StringType>,
    /// If parameter is a data type (oid)
    #[serde(rename = "valueOid")]
    pub value_oid: Option<StringType>,
    /// If parameter is a data type (positiveInt)
    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: Option<PositiveIntType>,
    /// If parameter is a data type (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// If parameter is a data type (time)
    #[serde(rename = "valueTime")]
    pub value_time: Option<TimeType>,
    /// If parameter is a data type (unsignedInt)
    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: Option<UnsignedIntType>,
    /// If parameter is a data type (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: Option<StringType>,
    /// If parameter is a data type (url)
    #[serde(rename = "valueUrl")]
    pub value_url: Option<StringType>,
    /// If parameter is a data type (uuid)
    #[serde(rename = "valueUuid")]
    pub value_uuid: Option<StringType>,
    /// If parameter is a data type (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Option<Address>,
    /// If parameter is a data type (Age)
    #[serde(rename = "valueAge")]
    pub value_age: Option<Age>,
    /// If parameter is a data type (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Option<Annotation>,
    /// If parameter is a data type (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
    /// If parameter is a data type (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// If parameter is a data type (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
    /// If parameter is a data type (ContactPoint)
    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: Option<ContactPoint>,
    /// If parameter is a data type (Count)
    #[serde(rename = "valueCount")]
    pub value_count: Option<Count>,
    /// If parameter is a data type (Distance)
    #[serde(rename = "valueDistance")]
    pub value_distance: Option<Distance>,
    /// If parameter is a data type (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Option<Duration>,
    /// If parameter is a data type (HumanName)
    #[serde(rename = "valueHumanName")]
    pub value_human_name: Option<HumanName>,
    /// If parameter is a data type (Identifier)
    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Option<Identifier>,
    /// If parameter is a data type (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Option<Money>,
    /// If parameter is a data type (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,
    /// If parameter is a data type (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// If parameter is a data type (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
    /// If parameter is a data type (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,
    /// If parameter is a data type (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Reference>,
    /// If parameter is a data type (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,
    /// If parameter is a data type (Signature)
    #[serde(rename = "valueSignature")]
    pub value_signature: Option<Signature>,
    /// If parameter is a data type (Timing)
    #[serde(rename = "valueTiming")]
    pub value_timing: Option<Timing>,
    /// If parameter is a data type (ContactDetail)
    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: Option<ContactDetail>,
    /// If parameter is a data type (Contributor)
    #[serde(rename = "valueContributor")]
    pub value_contributor: Option<Contributor>,
    /// If parameter is a data type (DataRequirement)
    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: Option<DataRequirement>,
    /// If parameter is a data type (Expression)
    #[serde(rename = "valueExpression")]
    pub value_expression: Option<Expression>,
    /// If parameter is a data type (ParameterDefinition)
    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: Option<ParameterDefinition>,
    /// If parameter is a data type (RelatedArtifact)
    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: Option<RelatedArtifact>,
    /// If parameter is a data type (TriggerDefinition)
    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: Option<TriggerDefinition>,
    /// If parameter is a data type (UsageContext)
    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: Option<UsageContext>,
    /// If parameter is a data type (Dosage)
    #[serde(rename = "valueDosage")]
    pub value_dosage: Option<Dosage>,
    /// If parameter is a data type (Meta)
    #[serde(rename = "valueMeta")]
    pub value_meta: Option<Meta>,
    /// If parameter is a whole resource
    pub resource: Option<Resource>,
    /// Named part of a multi-part parameter
    pub part: Option<Vec<StringType>>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            base: Resource::default(),
            parameter: Default::default(),
        }
    }
}

impl Default for ParametersFullURL {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ParametersParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
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
            resource: Default::default(),
            part: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Parameters {
    fn id(&self) -> Option<String> {
        self.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for Parameters {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Parameters {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
}

impl crate::traits::parameters::ParametersAccessors for Parameters {
    fn parameter(&self) -> &[ParametersParameter] {
        self.parameter.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::parameters::ParametersMutators for Parameters {
    fn new() -> Self {
        Self::default()
    }
    fn set_parameter(self, value: Vec<ParametersParameter>) -> Self {
        let mut resource = self.clone();
        resource.parameter = Some(value);
        resource
    }
    fn add_parameter(self, item: ParametersParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::parameters::ParametersExistence for Parameters {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
    fn has_parameter(&self) -> bool {
        self.parameter.as_ref().is_some_and(|v| !v.is_empty())
    }
}
