use crate::bindings::administrative_gender::AdministrativeGender;
use crate::bindings::observation_range_category::ObservationRangeCategory;
use crate::bindings::permitted_data_type::PermittedDataType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ObservationDefinition
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Category of observation
    ///
    /// Binding: example (Codes for high level observation categories.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of observation (code / type)
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Business identifier for this ObservationDefinition instance
    pub identifier: Option<Vec<Identifier>>,
    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<PermittedDataType>>,
    /// Extension element for the 'permittedDataType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_permittedDataType")]
    pub _permitted_data_type: Option<Element>,
    /// Multiple results allowed
    #[serde(rename = "multipleResultsAllowed")]
    pub multiple_results_allowed: Option<BooleanType>,
    /// Extension element for the 'multipleResultsAllowed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_multipleResultsAllowed")]
    pub _multiple_results_allowed: Option<Element>,
    /// Method used to produce the observation
    ///
    /// Binding: example (Methods for simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-methods
    pub method: Option<CodeableConcept>,
    /// Preferred report name
    #[serde(rename = "preferredReportName")]
    pub preferred_report_name: Option<StringType>,
    /// Extension element for the 'preferredReportName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preferredReportName")]
    pub _preferred_report_name: Option<Element>,
    /// Characteristics of quantitative results
    #[serde(rename = "quantitativeDetails")]
    pub quantitative_details: Option<ObservationDefinitionQuantitativedetails>,
    /// Qualified range for continuous and ordinal observation results
    #[serde(rename = "qualifiedInterval")]
    pub qualified_interval: Option<Vec<ObservationDefinitionQualifiedinterval>>,
    /// Value set of valid coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "validCodedValueSet")]
    pub valid_coded_value_set: Option<Reference>,
    /// Value set of normal coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "normalCodedValueSet")]
    pub normal_coded_value_set: Option<Reference>,
    /// Value set of abnormal coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "abnormalCodedValueSet")]
    pub abnormal_coded_value_set: Option<Reference>,
    /// Value set of critical coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "criticalCodedValueSet")]
    pub critical_coded_value_set: Option<Reference>,
}
/// ObservationDefinition nested structure for the 'qualifiedInterval' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinitionQualifiedinterval {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// reference | critical | absolute
    pub category: Option<ObservationRangeCategory>,
    /// Extension element for the 'category' primitive field. Contains metadata and extensions.
    pub _category: Option<Element>,
    /// The interval itself, for continuous or ordinal observations
    pub range: Option<Range>,
    /// Range context qualifier
    ///
    /// Binding: extensible (Code identifying the health context of a range.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/referencerange-meaning
    pub context: Option<CodeableConcept>,
    /// Targetted population of the range
    ///
    /// Binding: example (Codes identifying the population the reference range applies to.)
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
    /// Applicable age range, if relevant
    pub age: Option<Range>,
    /// Applicable gestational age range, if relevant
    #[serde(rename = "gestationalAge")]
    pub gestational_age: Option<Range>,
    /// Condition associated with the reference range
    pub condition: Option<StringType>,
    /// Extension element for the 'condition' primitive field. Contains metadata and extensions.
    pub _condition: Option<Element>,
}
/// ObservationDefinition nested structure for the 'quantitativeDetails' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDefinitionQuantitativedetails {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Customary unit for quantitative results
    ///
    /// Binding: extensible (Codes identifying units of measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ucum-units
    #[serde(rename = "customaryUnit")]
    pub customary_unit: Option<CodeableConcept>,
    /// SI unit for quantitative results
    ///
    /// Binding: extensible (Codes identifying units of measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ucum-units
    pub unit: Option<CodeableConcept>,
    /// SI to Customary unit conversion factor
    #[serde(rename = "conversionFactor")]
    pub conversion_factor: Option<DecimalType>,
    /// Extension element for the 'conversionFactor' primitive field. Contains metadata and extensions.
    #[serde(rename = "_conversionFactor")]
    pub _conversion_factor: Option<Element>,
    /// Decimal precision of observation quantitative results
    #[serde(rename = "decimalPrecision")]
    pub decimal_precision: Option<IntegerType>,
    /// Extension element for the 'decimalPrecision' primitive field. Contains metadata and extensions.
    #[serde(rename = "_decimalPrecision")]
    pub _decimal_precision: Option<Element>,
}

impl Default for ObservationDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            identifier: Default::default(),
            permitted_data_type: Default::default(),
            _permitted_data_type: Default::default(),
            multiple_results_allowed: Default::default(),
            _multiple_results_allowed: Default::default(),
            method: Default::default(),
            preferred_report_name: Default::default(),
            _preferred_report_name: Default::default(),
            quantitative_details: Default::default(),
            qualified_interval: Default::default(),
            valid_coded_value_set: Default::default(),
            normal_coded_value_set: Default::default(),
            abnormal_coded_value_set: Default::default(),
            critical_coded_value_set: Default::default(),
        }
    }
}

impl Default for ObservationDefinitionQualifiedinterval {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            _category: Default::default(),
            range: Default::default(),
            context: Default::default(),
            applies_to: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            age: Default::default(),
            gestational_age: Default::default(),
            condition: Default::default(),
            _condition: Default::default(),
        }
    }
}

impl Default for ObservationDefinitionQuantitativedetails {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            customary_unit: Default::default(),
            unit: Default::default(),
            conversion_factor: Default::default(),
            _conversion_factor: Default::default(),
            decimal_precision: Default::default(),
            _decimal_precision: Default::default(),
        }
    }
}

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
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn permitted_data_type(&self) -> &[PermittedDataType] {
        self.permitted_data_type.as_deref().unwrap_or(&[])
    }
    fn multiple_results_allowed(&self) -> Option<BooleanType> {
        self.multiple_results_allowed
    }
    fn method(&self) -> Option<CodeableConcept> {
        self.method.clone()
    }
    fn preferred_report_name(&self) -> Option<StringType> {
        self.preferred_report_name.clone()
    }
    fn quantitative_details(&self) -> Option<ObservationDefinitionQuantitativedetails> {
        self.quantitative_details.clone()
    }
    fn qualified_interval(&self) -> &[ObservationDefinitionQualifiedinterval] {
        self.qualified_interval.as_deref().unwrap_or(&[])
    }
    fn valid_coded_value_set(&self) -> Option<Reference> {
        self.valid_coded_value_set.clone()
    }
    fn normal_coded_value_set(&self) -> Option<Reference> {
        self.normal_coded_value_set.clone()
    }
    fn abnormal_coded_value_set(&self) -> Option<Reference> {
        self.abnormal_coded_value_set.clone()
    }
    fn critical_coded_value_set(&self) -> Option<Reference> {
        self.critical_coded_value_set.clone()
    }
}

impl crate::traits::observation_definition::ObservationDefinitionMutators
    for ObservationDefinition
{
    fn new() -> Self {
        Self::default()
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
    fn set_method(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.method = Some(value);
        resource
    }
    fn set_preferred_report_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.preferred_report_name = Some(value);
        resource
    }
    fn set_quantitative_details(self, value: ObservationDefinitionQuantitativedetails) -> Self {
        let mut resource = self.clone();
        resource.quantitative_details = Some(value);
        resource
    }
    fn set_qualified_interval(self, value: Vec<ObservationDefinitionQualifiedinterval>) -> Self {
        let mut resource = self.clone();
        resource.qualified_interval = Some(value);
        resource
    }
    fn add_qualified_interval(self, item: ObservationDefinitionQualifiedinterval) -> Self {
        let mut resource = self.clone();
        resource
            .qualified_interval
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_valid_coded_value_set(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.valid_coded_value_set = Some(value);
        resource
    }
    fn set_normal_coded_value_set(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.normal_coded_value_set = Some(value);
        resource
    }
    fn set_abnormal_coded_value_set(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.abnormal_coded_value_set = Some(value);
        resource
    }
    fn set_critical_coded_value_set(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.critical_coded_value_set = Some(value);
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
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_permitted_data_type(&self) -> bool {
        self.permitted_data_type
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_multiple_results_allowed(&self) -> bool {
        self.multiple_results_allowed.is_some()
    }
    fn has_method(&self) -> bool {
        self.method.is_some()
    }
    fn has_preferred_report_name(&self) -> bool {
        self.preferred_report_name.is_some()
    }
    fn has_quantitative_details(&self) -> bool {
        self.quantitative_details.is_some()
    }
    fn has_qualified_interval(&self) -> bool {
        self.qualified_interval
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_valid_coded_value_set(&self) -> bool {
        self.valid_coded_value_set.is_some()
    }
    fn has_normal_coded_value_set(&self) -> bool {
        self.normal_coded_value_set.is_some()
    }
    fn has_abnormal_coded_value_set(&self) -> bool {
        self.abnormal_coded_value_set.is_some()
    }
    fn has_critical_coded_value_set(&self) -> bool {
        self.critical_coded_value_set.is_some()
    }
}
