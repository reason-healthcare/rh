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
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Coverage
///
/// Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Coverage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Coverage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coverage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for the coverage
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Coverage category such as medical or accident
    ///
    /// Binding: preferred (The type of insurance: public health, worker compensation; private accident, auto, private health, etc.) or a direct payment by an individual or organization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverage-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Owner of the policy
    #[serde(rename = "policyHolder")]
    pub policy_holder: Option<Reference>,
    /// Subscriber to the policy
    pub subscriber: Option<Reference>,
    /// ID assigned to the subscriber
    #[serde(rename = "subscriberId")]
    pub subscriber_id: Option<StringType>,
    /// Extension element for the 'subscriberId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subscriberId")]
    pub _subscriber_id: Option<Element>,
    /// Plan beneficiary
    pub beneficiary: Reference,
    /// Dependent number
    pub dependent: Option<StringType>,
    /// Extension element for the 'dependent' primitive field. Contains metadata and extensions.
    pub _dependent: Option<Element>,
    /// Beneficiary relationship to the subscriber
    ///
    /// Binding: extensible (The relationship between the Subscriber and the Beneficiary (insured/covered party/patient).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/subscriber-relationship
    pub relationship: Option<CodeableConcept>,
    /// Coverage start and end dates
    pub period: Option<Period>,
    /// Issuer of the policy
    pub payor: Vec<Reference>,
    /// Additional coverage classifications
    pub class: Option<Vec<CoverageClass>>,
    /// Relative order of the coverage
    pub order: Option<PositiveIntType>,
    /// Extension element for the 'order' primitive field. Contains metadata and extensions.
    pub _order: Option<Element>,
    /// Insurer network
    pub network: Option<StringType>,
    /// Extension element for the 'network' primitive field. Contains metadata and extensions.
    pub _network: Option<Element>,
    /// Patient payments for services/products
    #[serde(rename = "costToBeneficiary")]
    pub cost_to_beneficiary: Option<Vec<CoverageCosttobeneficiary>>,
    /// Reimbursement to insurer
    pub subrogation: Option<BooleanType>,
    /// Extension element for the 'subrogation' primitive field. Contains metadata and extensions.
    pub _subrogation: Option<Element>,
    /// Contract details
    pub contract: Option<Vec<Reference>>,
}
/// CoverageCosttobeneficiary nested structure for the 'exception' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageCosttobeneficiaryException {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Exception category
    ///
    /// Binding: example (The types of exceptions from the part or full value of financial obligations such as copays.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverage-financial-exception
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The effective period of the exception
    pub period: Option<Period>,
}
/// Coverage nested structure for the 'class' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageClass {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of class such as 'group' or 'plan'
    ///
    /// Binding: extensible (The policy classifications, eg. Group, Plan, Class, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverage-class
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Value associated with the type
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Human readable description of the type and value
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
}
/// Coverage nested structure for the 'costToBeneficiary' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageCosttobeneficiary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Exceptions for patient payments
    pub exception: Option<Vec<CoverageCosttobeneficiaryException>>,
    /// Cost category
    ///
    /// Binding: extensible (The types of services to which patient copayments are specified.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverage-copay-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The amount or percentage due from the beneficiary (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// The amount or percentage due from the beneficiary (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Money,
}

impl Default for Coverage {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            policy_holder: Default::default(),
            subscriber: Default::default(),
            subscriber_id: Default::default(),
            _subscriber_id: Default::default(),
            beneficiary: Reference::default(),
            dependent: Default::default(),
            _dependent: Default::default(),
            relationship: Default::default(),
            period: Default::default(),
            payor: Vec::new(),
            class: Default::default(),
            order: Default::default(),
            _order: Default::default(),
            network: Default::default(),
            _network: Default::default(),
            cost_to_beneficiary: Default::default(),
            subrogation: Default::default(),
            _subrogation: Default::default(),
            contract: Default::default(),
        }
    }
}

impl Default for CoverageCosttobeneficiaryException {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for CoverageClass {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value: StringType::default(),
            _value: Default::default(),
            name: Default::default(),
            _name: Default::default(),
        }
    }
}

impl Default for CoverageCosttobeneficiary {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            exception: Default::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_money: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Coverage {
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

impl crate::traits::resource::ResourceMutators for Coverage {
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

impl crate::traits::resource::ResourceExistence for Coverage {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Coverage {
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

impl crate::traits::domain_resource::DomainResourceMutators for Coverage {
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

impl crate::traits::domain_resource::DomainResourceExistence for Coverage {
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

impl crate::traits::coverage::CoverageAccessors for Coverage {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> FmStatus {
        self.status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn policy_holder(&self) -> Option<Reference> {
        self.policy_holder.clone()
    }
    fn subscriber(&self) -> Option<Reference> {
        self.subscriber.clone()
    }
    fn subscriber_id(&self) -> Option<StringType> {
        self.subscriber_id.clone()
    }
    fn beneficiary(&self) -> Reference {
        self.beneficiary.clone()
    }
    fn dependent(&self) -> Option<StringType> {
        self.dependent.clone()
    }
    fn relationship(&self) -> Option<CodeableConcept> {
        self.relationship.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn payor(&self) -> &[Reference] {
        &self.payor
    }
    fn class(&self) -> &[CoverageClass] {
        self.class.as_deref().unwrap_or(&[])
    }
    fn order(&self) -> Option<PositiveIntType> {
        self.order
    }
    fn network(&self) -> Option<StringType> {
        self.network.clone()
    }
    fn cost_to_beneficiary(&self) -> &[CoverageCosttobeneficiary] {
        self.cost_to_beneficiary.as_deref().unwrap_or(&[])
    }
    fn subrogation(&self) -> Option<BooleanType> {
        self.subrogation
    }
    fn contract(&self) -> &[Reference] {
        self.contract.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::coverage::CoverageMutators for Coverage {
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
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_policy_holder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.policy_holder = Some(value);
        resource
    }
    fn set_subscriber(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subscriber = Some(value);
        resource
    }
    fn set_subscriber_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.subscriber_id = Some(value);
        resource
    }
    fn set_beneficiary(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.beneficiary = value;
        resource
    }
    fn set_dependent(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.dependent = Some(value);
        resource
    }
    fn set_relationship(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.relationship = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_payor(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.payor = value;
        resource
    }
    fn add_payor(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.payor.push(item);
        resource
    }
    fn set_class(self, value: Vec<CoverageClass>) -> Self {
        let mut resource = self.clone();
        resource.class = Some(value);
        resource
    }
    fn add_class(self, item: CoverageClass) -> Self {
        let mut resource = self.clone();
        resource.class.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_order(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.order = Some(value);
        resource
    }
    fn set_network(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.network = Some(value);
        resource
    }
    fn set_cost_to_beneficiary(self, value: Vec<CoverageCosttobeneficiary>) -> Self {
        let mut resource = self.clone();
        resource.cost_to_beneficiary = Some(value);
        resource
    }
    fn add_cost_to_beneficiary(self, item: CoverageCosttobeneficiary) -> Self {
        let mut resource = self.clone();
        resource
            .cost_to_beneficiary
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subrogation(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.subrogation = Some(value);
        resource
    }
    fn set_contract(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.contract = Some(value);
        resource
    }
    fn add_contract(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.contract.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::coverage::CoverageExistence for Coverage {
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
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_policy_holder(&self) -> bool {
        self.policy_holder.is_some()
    }
    fn has_subscriber(&self) -> bool {
        self.subscriber.is_some()
    }
    fn has_subscriber_id(&self) -> bool {
        self.subscriber_id.is_some()
    }
    fn has_beneficiary(&self) -> bool {
        true
    }
    fn has_dependent(&self) -> bool {
        self.dependent.is_some()
    }
    fn has_relationship(&self) -> bool {
        self.relationship.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_payor(&self) -> bool {
        !self.payor.is_empty()
    }
    fn has_class(&self) -> bool {
        self.class.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_order(&self) -> bool {
        self.order.is_some()
    }
    fn has_network(&self) -> bool {
        self.network.is_some()
    }
    fn has_cost_to_beneficiary(&self) -> bool {
        self.cost_to_beneficiary
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_subrogation(&self) -> bool {
        self.subrogation.is_some()
    }
    fn has_contract(&self) -> bool {
        self.contract.as_ref().is_some_and(|v| !v.is_empty())
    }
}
