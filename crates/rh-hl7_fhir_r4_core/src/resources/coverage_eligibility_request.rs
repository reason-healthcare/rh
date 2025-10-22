use crate::bindings::eligibilityrequest_purpose::EligibilityrequestPurpose;
use crate::bindings::fm_status::FmStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CoverageEligibilityRequest
///
/// The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for coverage eligiblity request
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Desired processing priority
    ///
    /// Binding: example (The timeliness with which processing is required: STAT, normal, Deferred.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/process-priority
    pub priority: Option<CodeableConcept>,
    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<EligibilityrequestPurpose>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Intended recipient of products and services
    pub patient: Reference,
    /// Estimated date or dates of service (date)
    #[serde(rename = "servicedDate")]
    pub serviced_date: Option<DateType>,
    /// Estimated date or dates of service (Period)
    #[serde(rename = "servicedPeriod")]
    pub serviced_period: Option<Period>,
    /// Creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Author
    pub enterer: Option<Reference>,
    /// Party responsible for the request
    pub provider: Option<Reference>,
    /// Coverage issuer
    pub insurer: Reference,
    /// Servicing facility
    pub facility: Option<Reference>,
    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<CoverageEligibilityRequestSupportinginfo>>,
    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityRequestInsurance>>,
    /// Item to be evaluated for eligibiity
    pub item: Option<Vec<CoverageEligibilityRequestItem>>,
}
/// CoverageEligibilityRequest nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityRequestItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Applicable diagnosis
    pub diagnosis: Option<Vec<CoverageEligibilityRequestItemDiagnosis>>,
    /// Applicable exception or supporting information
    #[serde(rename = "supportingInfoSequence")]
    pub supporting_info_sequence: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'supportingInfoSequence' primitive field. Contains metadata and extensions.
    #[serde(rename = "_supportingInfoSequence")]
    pub _supporting_info_sequence: Option<Element>,
    /// Benefit classification
    ///
    /// Binding: example (Benefit categories such as: oral, medical, vision etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    ///
    /// Binding: example (Allowable service and product codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-uscls
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,
    /// Product or service billing modifiers
    ///
    /// Binding: example (Item type or modifiers codes, eg for Oral whether the treatment is cosmetic or associated with TMJ, or an appliance was lost or stolen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/claim-modifiers
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Perfoming practitioner
    pub provider: Option<Reference>,
    /// Count of products or services
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,
    /// Servicing facility
    pub facility: Option<Reference>,
    /// Product or service details
    pub detail: Option<Vec<Reference>>,
}
/// CoverageEligibilityRequestItem nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityRequestItemDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Nature of illness or problem (CodeableConcept)
    #[serde(rename = "diagnosisCodeableConcept")]
    pub diagnosis_codeable_concept: Option<CodeableConcept>,
    /// Nature of illness or problem (Reference)
    #[serde(rename = "diagnosisReference")]
    pub diagnosis_reference: Option<Reference>,
}
/// CoverageEligibilityRequest nested structure for the 'insurance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityRequestInsurance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Applicable coverage
    pub focal: Option<BooleanType>,
    /// Extension element for the 'focal' primitive field. Contains metadata and extensions.
    pub _focal: Option<Element>,
    /// Insurance information
    pub coverage: Reference,
    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<StringType>,
    /// Extension element for the 'businessArrangement' primitive field. Contains metadata and extensions.
    #[serde(rename = "_businessArrangement")]
    pub _business_arrangement: Option<Element>,
}
/// CoverageEligibilityRequest nested structure for the 'supportingInfo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityRequestSupportinginfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information instance identifier
    pub sequence: PositiveIntType,
    /// Extension element for the 'sequence' primitive field. Contains metadata and extensions.
    pub _sequence: Option<Element>,
    /// Data to be provided
    pub information: Reference,
    /// Applies to all items
    #[serde(rename = "appliesToAll")]
    pub applies_to_all: Option<BooleanType>,
    /// Extension element for the 'appliesToAll' primitive field. Contains metadata and extensions.
    #[serde(rename = "_appliesToAll")]
    pub _applies_to_all: Option<Element>,
}

impl Default for CoverageEligibilityRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            priority: Default::default(),
            purpose: Vec::new(),
            _purpose: Default::default(),
            patient: Reference::default(),
            serviced_date: Default::default(),
            serviced_period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            enterer: Default::default(),
            provider: Default::default(),
            insurer: Reference::default(),
            facility: Default::default(),
            supporting_info: Default::default(),
            insurance: Default::default(),
            item: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityRequestItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            diagnosis: Default::default(),
            supporting_info_sequence: Default::default(),
            _supporting_info_sequence: Default::default(),
            category: Default::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            provider: Default::default(),
            quantity: Default::default(),
            unit_price: Default::default(),
            facility: Default::default(),
            detail: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityRequestItemDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            diagnosis_codeable_concept: Default::default(),
            diagnosis_reference: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityRequestInsurance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            focal: Default::default(),
            _focal: Default::default(),
            coverage: Reference::default(),
            business_arrangement: Default::default(),
            _business_arrangement: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityRequestSupportinginfo {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            sequence: Default::default(),
            _sequence: Default::default(),
            information: Default::default(),
            applies_to_all: Default::default(),
            _applies_to_all: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CoverageEligibilityRequest {
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

impl crate::traits::resource::ResourceMutators for CoverageEligibilityRequest {
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

impl crate::traits::resource::ResourceExistence for CoverageEligibilityRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CoverageEligibilityRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for CoverageEligibilityRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for CoverageEligibilityRequest {
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

impl crate::traits::coverage_eligibility_request::CoverageEligibilityRequestAccessors
    for CoverageEligibilityRequest
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn purpose(&self) -> &[EligibilityrequestPurpose] {
        &self.purpose
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn enterer(&self) -> Option<Reference> {
        self.enterer.clone()
    }
    fn provider(&self) -> Option<Reference> {
        self.provider.clone()
    }
    fn insurer(&self) -> Reference {
        self.insurer.clone()
    }
    fn facility(&self) -> Option<Reference> {
        self.facility.clone()
    }
    fn supporting_info(&self) -> &[CoverageEligibilityRequestSupportinginfo] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn insurance(&self) -> &[CoverageEligibilityRequestInsurance] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn item(&self) -> &[CoverageEligibilityRequestItem] {
        self.item.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::coverage_eligibility_request::CoverageEligibilityRequestMutators
    for CoverageEligibilityRequest
{
    fn new() -> Self {
        Self::default()
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
    fn set_status(self, value: FmStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_purpose(self, value: Vec<EligibilityrequestPurpose>) -> Self {
        let mut resource = self.clone();
        resource.purpose = value;
        resource
    }
    fn add_purpose(self, item: EligibilityrequestPurpose) -> Self {
        let mut resource = self.clone();
        resource.purpose.push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = value;
        resource
    }
    fn set_enterer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.enterer = Some(value);
        resource
    }
    fn set_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.provider = Some(value);
        resource
    }
    fn set_insurer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurer = value;
        resource
    }
    fn set_facility(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.facility = Some(value);
        resource
    }
    fn set_supporting_info(self, value: Vec<CoverageEligibilityRequestSupportinginfo>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: CoverageEligibilityRequestSupportinginfo) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_insurance(self, value: Vec<CoverageEligibilityRequestInsurance>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: CoverageEligibilityRequestInsurance) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_item(self, value: Vec<CoverageEligibilityRequestItem>) -> Self {
        let mut resource = self.clone();
        resource.item = Some(value);
        resource
    }
    fn add_item(self, item: CoverageEligibilityRequestItem) -> Self {
        let mut resource = self.clone();
        resource.item.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::coverage_eligibility_request::CoverageEligibilityRequestExistence
    for CoverageEligibilityRequest
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
    fn has_serviced(&self) -> bool {
        self.serviced_date.is_some() || self.serviced_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_purpose(&self) -> bool {
        !self.purpose.is_empty()
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_enterer(&self) -> bool {
        self.enterer.is_some()
    }
    fn has_provider(&self) -> bool {
        self.provider.is_some()
    }
    fn has_insurer(&self) -> bool {
        true
    }
    fn has_facility(&self) -> bool {
        self.facility.is_some()
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_item(&self) -> bool {
        self.item.as_ref().is_some_and(|v| !v.is_empty())
    }
}
