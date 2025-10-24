use crate::bindings::eligibilityresponse_purpose::EligibilityresponsePurpose;
use crate::bindings::fm_status::FmStatus;
use crate::bindings::remittance_outcome::RemittanceOutcome;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CoverageEligibilityResponse
///
/// This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CoverageEligibilityResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for coverage eligiblity request
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<EligibilityresponsePurpose>,
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
    /// Response creation date
    pub created: DateTimeType,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Party responsible for the request
    pub requestor: Option<Reference>,
    /// Eligibility request reference
    pub request: Reference,
    /// queued | complete | error | partial
    pub outcome: RemittanceOutcome,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Disposition Message
    pub disposition: Option<StringType>,
    /// Extension element for the 'disposition' primitive field. Contains metadata and extensions.
    pub _disposition: Option<Element>,
    /// Coverage issuer
    pub insurer: Reference,
    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityResponseInsurance>>,
    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<StringType>,
    /// Extension element for the 'preAuthRef' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preAuthRef")]
    pub _pre_auth_ref: Option<Element>,
    /// Printed form identifier
    ///
    /// Binding: example (The forms codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/forms
    pub form: Option<CodeableConcept>,
    /// Processing errors
    pub error: Option<Vec<CoverageEligibilityResponseError>>,
}
/// CoverageEligibilityResponse nested structure for the 'insurance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityResponseInsurance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Benefits and authorization details
    pub item: Option<Vec<CoverageEligibilityResponseInsuranceItem>>,
    /// Insurance information
    pub coverage: Reference,
    /// Coverage inforce indicator
    pub inforce: Option<BooleanType>,
    /// Extension element for the 'inforce' primitive field. Contains metadata and extensions.
    pub _inforce: Option<Element>,
    /// When the benefits are applicable
    #[serde(rename = "benefitPeriod")]
    pub benefit_period: Option<Period>,
}
/// CoverageEligibilityResponse nested structure for the 'error' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityResponseError {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Error code detailing processing issues
    ///
    /// Binding: example (The error codes for adjudication processing.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adjudication-error
    pub code: CodeableConcept,
}
/// CoverageEligibilityResponseInsurance nested structure for the 'item' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityResponseInsuranceItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
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
    /// Performing practitioner
    pub provider: Option<Reference>,
    /// Excluded from the plan
    pub excluded: Option<BooleanType>,
    /// Extension element for the 'excluded' primitive field. Contains metadata and extensions.
    pub _excluded: Option<Element>,
    /// Short name for the benefit
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Description of the benefit or services covered
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// In or out of network
    ///
    /// Binding: example (Code to classify in or out of network services.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-network
    pub network: Option<CodeableConcept>,
    /// Individual or family
    ///
    /// Binding: example (Unit covered/serviced - individual or family.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-unit
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    ///
    /// Binding: example (Coverage unit - annual, lifetime.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-term
    pub term: Option<CodeableConcept>,
    /// Authorization required flag
    #[serde(rename = "authorizationRequired")]
    pub authorization_required: Option<BooleanType>,
    /// Extension element for the 'authorizationRequired' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authorizationRequired")]
    pub _authorization_required: Option<Element>,
    /// Type of required supporting materials
    ///
    /// Binding: example (Type of supporting information to provide with a preauthorization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverageeligibilityresponse-ex-auth-support
    #[serde(rename = "authorizationSupporting")]
    pub authorization_supporting: Option<Vec<CodeableConcept>>,
    /// Preauthorization requirements endpoint
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: Option<StringType>,
    /// Extension element for the 'authorizationUrl' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authorizationUrl")]
    pub _authorization_url: Option<Element>,
}
/// CoverageEligibilityResponseInsuranceItem nested structure for the 'benefit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Benefit classification
    ///
    /// Binding: example (Deductable, visits, co-pay, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Benefits allowed (unsignedInt)
    #[serde(rename = "allowedUnsignedInt")]
    pub allowed_unsigned_int: Option<UnsignedIntType>,
    /// Benefits allowed (string)
    #[serde(rename = "allowedString")]
    pub allowed_string: Option<StringType>,
    /// Benefits allowed (Money)
    #[serde(rename = "allowedMoney")]
    pub allowed_money: Option<Money>,
    /// Benefits used (unsignedInt)
    #[serde(rename = "usedUnsignedInt")]
    pub used_unsigned_int: Option<UnsignedIntType>,
    /// Benefits used (string)
    #[serde(rename = "usedString")]
    pub used_string: Option<StringType>,
    /// Benefits used (Money)
    #[serde(rename = "usedMoney")]
    pub used_money: Option<Money>,
}

impl Default for CoverageEligibilityResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            purpose: Vec::new(),
            _purpose: Default::default(),
            patient: Reference::default(),
            serviced_date: Default::default(),
            serviced_period: Default::default(),
            created: DateTimeType::default(),
            _created: Default::default(),
            requestor: Default::default(),
            request: Reference::default(),
            outcome: RemittanceOutcome::default(),
            _outcome: Default::default(),
            disposition: Default::default(),
            _disposition: Default::default(),
            insurer: Reference::default(),
            insurance: Default::default(),
            pre_auth_ref: Default::default(),
            _pre_auth_ref: Default::default(),
            form: Default::default(),
            error: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityResponseInsurance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: Default::default(),
            coverage: Reference::default(),
            inforce: Default::default(),
            _inforce: Default::default(),
            benefit_period: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityResponseError {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
        }
    }
}

impl Default for CoverageEligibilityResponseInsuranceItem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            product_or_service: Default::default(),
            modifier: Default::default(),
            provider: Default::default(),
            excluded: Default::default(),
            _excluded: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            network: Default::default(),
            unit: Default::default(),
            term: Default::default(),
            authorization_required: Default::default(),
            _authorization_required: Default::default(),
            authorization_supporting: Default::default(),
            authorization_url: Default::default(),
            _authorization_url: Default::default(),
        }
    }
}

impl Default for CoverageEligibilityResponseInsuranceItemBenefit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            allowed_unsigned_int: Default::default(),
            allowed_string: Default::default(),
            allowed_money: Default::default(),
            used_unsigned_int: Default::default(),
            used_string: Default::default(),
            used_money: Default::default(),
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
    rh_foundation::Invariant::new("ces-1", rh_foundation::Severity::Error, "SHALL contain a category or a billcode but not both.", "category.exists() xor productOrService.exists()").with_xpath("exists(f:category) or exists(f:productOrService)"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CoverageEligibilityResponse {
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

impl crate::traits::resource::ResourceMutators for CoverageEligibilityResponse {
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

impl crate::traits::resource::ResourceExistence for CoverageEligibilityResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CoverageEligibilityResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for CoverageEligibilityResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for CoverageEligibilityResponse {
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

impl crate::traits::coverage_eligibility_response::CoverageEligibilityResponseAccessors
    for CoverageEligibilityResponse
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn purpose(&self) -> &[EligibilityresponsePurpose] {
        &self.purpose
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn created(&self) -> DateTimeType {
        self.created.clone()
    }
    fn requestor(&self) -> Option<Reference> {
        self.requestor.clone()
    }
    fn request(&self) -> Reference {
        self.request.clone()
    }
    fn outcome(&self) -> RemittanceOutcome {
        self.outcome.clone()
    }
    fn disposition(&self) -> Option<StringType> {
        self.disposition.clone()
    }
    fn insurer(&self) -> Reference {
        self.insurer.clone()
    }
    fn insurance(&self) -> &[CoverageEligibilityResponseInsurance] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn pre_auth_ref(&self) -> Option<StringType> {
        self.pre_auth_ref.clone()
    }
    fn form(&self) -> Option<CodeableConcept> {
        self.form.clone()
    }
    fn error(&self) -> &[CoverageEligibilityResponseError] {
        self.error.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::coverage_eligibility_response::CoverageEligibilityResponseMutators
    for CoverageEligibilityResponse
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
    fn set_purpose(self, value: Vec<EligibilityresponsePurpose>) -> Self {
        let mut resource = self.clone();
        resource.purpose = value;
        resource
    }
    fn add_purpose(self, item: EligibilityresponsePurpose) -> Self {
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
    fn set_requestor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requestor = Some(value);
        resource
    }
    fn set_request(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.request = value;
        resource
    }
    fn set_outcome(self, value: RemittanceOutcome) -> Self {
        let mut resource = self.clone();
        resource.outcome = value;
        resource
    }
    fn set_disposition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.disposition = Some(value);
        resource
    }
    fn set_insurer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurer = value;
        resource
    }
    fn set_insurance(self, value: Vec<CoverageEligibilityResponseInsurance>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: CoverageEligibilityResponseInsurance) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_pre_auth_ref(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.pre_auth_ref = Some(value);
        resource
    }
    fn set_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form = Some(value);
        resource
    }
    fn set_error(self, value: Vec<CoverageEligibilityResponseError>) -> Self {
        let mut resource = self.clone();
        resource.error = Some(value);
        resource
    }
    fn add_error(self, item: CoverageEligibilityResponseError) -> Self {
        let mut resource = self.clone();
        resource.error.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::coverage_eligibility_response::CoverageEligibilityResponseExistence
    for CoverageEligibilityResponse
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
    fn has_purpose(&self) -> bool {
        !self.purpose.is_empty()
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_created(&self) -> bool {
        true
    }
    fn has_requestor(&self) -> bool {
        self.requestor.is_some()
    }
    fn has_request(&self) -> bool {
        true
    }
    fn has_outcome(&self) -> bool {
        true
    }
    fn has_disposition(&self) -> bool {
        self.disposition.is_some()
    }
    fn has_insurer(&self) -> bool {
        true
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_pre_auth_ref(&self) -> bool {
        self.pre_auth_ref.is_some()
    }
    fn has_form(&self) -> bool {
        self.form.is_some()
    }
    fn has_error(&self) -> bool {
        self.error.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CoverageEligibilityResponse {
    fn resource_type(&self) -> &'static str {
        "CoverageEligibilityResponse"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse")
    }
}
