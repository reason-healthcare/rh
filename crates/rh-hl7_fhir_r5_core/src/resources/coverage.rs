use crate::bindings::coverage_kind::CoverageKind;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Coverage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coverage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier(s) for this coverage
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    pub status: FmStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// insurance | self-pay | other
    pub kind: CoverageKind,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// Self-pay parties and responsibility
    #[serde(rename = "paymentBy")]
    pub payment_by: Option<Vec<CoveragePaymentby>>,
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
    pub subscriber_id: Option<Vec<Identifier>>,
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
    pub insurer: Option<Reference>,
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
    /// Insurance plan details
    #[serde(rename = "insurancePlan")]
    pub insurance_plan: Option<Reference>,
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
    /// Benefit classification
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/ex-benefitcategory
    pub category: Option<CodeableConcept>,
    /// In or out of network
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-network
    pub network: Option<CodeableConcept>,
    /// Individual or family
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-unit
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/benefit-term
    pub term: Option<CodeableConcept>,
    /// The amount or percentage due from the beneficiary (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// The amount or percentage due from the beneficiary (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Option<Money>,
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
    /// Binding: extensible (The policy classifications, e.g. Group, Plan, Class, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/coverage-class
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Value associated with the type
    pub value: Identifier,
    /// Human readable description of the type and value
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
}
/// Coverage nested structure for the 'paymentBy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoveragePaymentby {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Parties performing self-payment
    pub party: Reference,
    /// Party's responsibility
    pub responsibility: Option<StringType>,
    /// Extension element for the 'responsibility' primitive field. Contains metadata and extensions.
    pub _responsibility: Option<Element>,
}

impl Default for Coverage {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: FmStatus::default(),
            _status: Default::default(),
            kind: CoverageKind::default(),
            _kind: Default::default(),
            payment_by: Default::default(),
            type_: Default::default(),
            policy_holder: Default::default(),
            subscriber: Default::default(),
            subscriber_id: Default::default(),
            beneficiary: Reference::default(),
            dependent: Default::default(),
            _dependent: Default::default(),
            relationship: Default::default(),
            period: Default::default(),
            insurer: Default::default(),
            class: Default::default(),
            order: Default::default(),
            _order: Default::default(),
            network: Default::default(),
            _network: Default::default(),
            cost_to_beneficiary: Default::default(),
            subrogation: Default::default(),
            _subrogation: Default::default(),
            contract: Default::default(),
            insurance_plan: Default::default(),
        }
    }
}

impl Default for CoverageCosttobeneficiary {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            exception: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            network: Default::default(),
            unit: Default::default(),
            term: Default::default(),
            value_quantity: Default::default(),
            value_money: Default::default(),
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
            value: Identifier::default(),
            name: Default::default(),
            _name: Default::default(),
        }
    }
}

impl Default for CoveragePaymentby {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            party: Default::default(),
            responsibility: Default::default(),
            _responsibility: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
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
                "Coverage.kind",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/coverage-kind|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Coverage.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Coverage.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/fm-status|5.0.0",
            )
            .with_description("A code specifying the state of the resource instance."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Coverage.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.contained", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.extension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy.extension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy.party", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.paymentBy.responsibility", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.policyHolder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.subscriber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.subscriberId", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.beneficiary", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.dependent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.relationship", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.insurer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.class", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.class.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.class.extension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.class.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.class.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.class.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.class.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.order", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.network", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.category",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.network",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.unit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.term", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Coverage.costToBeneficiary.exception", 0, None),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.exception.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.exception.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.exception.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.exception.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Coverage.costToBeneficiary.exception.period",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Coverage.subrogation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Coverage.contract", 0, None),
            rh_foundation::ElementCardinality::new("Coverage.insurancePlan", 0, Some(1)),
        ]
    });

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
    fn kind(&self) -> CoverageKind {
        self.kind.clone()
    }
    fn payment_by(&self) -> &[CoveragePaymentby] {
        self.payment_by.as_deref().unwrap_or(&[])
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
    fn subscriber_id(&self) -> &[Identifier] {
        self.subscriber_id.as_deref().unwrap_or(&[])
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
    fn insurer(&self) -> Option<Reference> {
        self.insurer.clone()
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
    fn insurance_plan(&self) -> Option<Reference> {
        self.insurance_plan.clone()
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
    fn set_kind(self, value: CoverageKind) -> Self {
        let mut resource = self.clone();
        resource.kind = value;
        resource
    }
    fn set_payment_by(self, value: Vec<CoveragePaymentby>) -> Self {
        let mut resource = self.clone();
        resource.payment_by = Some(value);
        resource
    }
    fn add_payment_by(self, item: CoveragePaymentby) -> Self {
        let mut resource = self.clone();
        resource.payment_by.get_or_insert_with(Vec::new).push(item);
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
    fn set_subscriber_id(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.subscriber_id = Some(value);
        resource
    }
    fn add_subscriber_id(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource
            .subscriber_id
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_insurer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurer = Some(value);
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
    fn set_insurance_plan(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance_plan = Some(value);
        resource
    }
}

impl crate::traits::coverage::CoverageExistence for Coverage {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_kind(&self) -> bool {
        true
    }
    fn has_payment_by(&self) -> bool {
        self.payment_by.as_ref().is_some_and(|v| !v.is_empty())
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
        self.subscriber_id.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_insurer(&self) -> bool {
        self.insurer.is_some()
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
    fn has_insurance_plan(&self) -> bool {
        self.insurance_plan.is_some()
    }
}

impl crate::validation::ValidatableResource for Coverage {
    fn resource_type(&self) -> &'static str {
        "Coverage"
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
        Some("http://hl7.org/fhir/StructureDefinition/Coverage")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::coverage::{CoverageAccessors, CoverageExistence, CoverageMutators};
