use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::address::Address;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::money::Money;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// InsurancePlan
///
/// Details of a Health Insurance product/plan provided by an organization.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: InsurancePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifier for Product
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    pub status: Option<PublicationStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Kind of product
    ///
    /// Binding: example (Used to categorize the product/plan.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/insuranceplan-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Official name
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Alternate names
    pub alias: Option<Vec<StringType>>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    pub _alias: Option<Element>,
    /// When the product is available
    pub period: Option<Period>,
    /// Plan issuer
    #[serde(rename = "ownedBy")]
    pub owned_by: Option<Reference>,
    /// Product administrator
    #[serde(rename = "administeredBy")]
    pub administered_by: Option<Reference>,
    /// Where product applies
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Reference>>,
    /// Contact for the product
    pub contact: Option<Vec<InsurancePlanContact>>,
    /// Technical endpoint
    pub endpoint: Option<Vec<Reference>>,
    /// What networks are Included
    pub network: Option<Vec<Reference>>,
    /// Coverage details
    pub coverage: Option<Vec<InsurancePlanCoverage>>,
    /// Plan details
    pub plan: Option<Vec<InsurancePlanPlan>>,
}
/// InsurancePlan nested structure for the 'coverage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanCoverage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// List of benefits
    pub benefit: Vec<InsurancePlanCoverageBenefit>,
    /// Type of coverage
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// What networks provide coverage
    pub network: Option<Vec<Reference>>,
}
/// InsurancePlanPlan nested structure for the 'generalCost' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanPlanGeneralcost {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of cost
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Number of enrollees
    #[serde(rename = "groupSize")]
    pub group_size: Option<PositiveIntType>,
    /// Extension element for the 'groupSize' primitive field. Contains metadata and extensions.
    #[serde(rename = "_groupSize")]
    pub _group_size: Option<Element>,
    /// Cost value
    pub cost: Option<Money>,
    /// Additional cost information
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}
/// InsurancePlanCoverageBenefit nested structure for the 'limit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanCoverageBenefitLimit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Maximum value allowed
    pub value: Option<Quantity>,
    /// Benefit limit details
    pub code: Option<CodeableConcept>,
}
/// InsurancePlanPlan nested structure for the 'specificCost' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanPlanSpecificcost {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// General category of benefit
    pub category: CodeableConcept,
}
/// InsurancePlan nested structure for the 'contact' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanContact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of contact
    ///
    /// Binding: extensible (The purpose for which you would contact a contact party.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/contactentity-type
    pub purpose: Option<CodeableConcept>,
    /// A name associated with the contact
    pub name: Option<HumanName>,
    /// Contact details (telephone, email, etc.)  for a contact
    pub telecom: Option<Vec<ContactPoint>>,
    /// Visiting or postal addresses for the contact
    pub address: Option<Address>,
}
/// InsurancePlan nested structure for the 'plan' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanPlan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Specific costs
    #[serde(rename = "specificCost")]
    pub specific_cost: Option<Vec<InsurancePlanPlanSpecificcost>>,
    /// Overall costs
    #[serde(rename = "generalCost")]
    pub general_cost: Option<Vec<InsurancePlanPlanGeneralcost>>,
    /// Business Identifier for Product
    pub identifier: Option<Vec<Identifier>>,
    /// Type of plan
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Where product applies
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Reference>>,
    /// What networks provide coverage
    pub network: Option<Vec<Reference>>,
}
/// InsurancePlanCoverage nested structure for the 'benefit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanCoverageBenefit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of benefit
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Referral requirements
    pub requirement: Option<StringType>,
    /// Extension element for the 'requirement' primitive field. Contains metadata and extensions.
    pub _requirement: Option<Element>,
}
/// InsurancePlanPlanSpecificcostBenefit nested structure for the 'cost' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanPlanSpecificcostBenefitCost {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of cost
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// in-network | out-of-network | other
    pub applicability: Option<CodeableConcept>,
    /// Additional information about the cost
    pub qualifiers: Option<Vec<CodeableConcept>>,
    /// The actual cost value
    pub value: Option<Quantity>,
}
/// InsurancePlanPlanSpecificcost nested structure for the 'benefit' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePlanPlanSpecificcostBenefit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of specific benefit
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
}

impl Default for InsurancePlan {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            type_: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            period: Default::default(),
            owned_by: Default::default(),
            administered_by: Default::default(),
            coverage_area: Default::default(),
            contact: Default::default(),
            endpoint: Default::default(),
            network: Default::default(),
            coverage: Default::default(),
            plan: Default::default(),
        }
    }
}

impl Default for InsurancePlanCoverage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            benefit: Vec::new(),
            type_: Default::default(),
            network: Default::default(),
        }
    }
}

impl Default for InsurancePlanPlanGeneralcost {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            group_size: Default::default(),
            _group_size: Default::default(),
            cost: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
        }
    }
}

impl Default for InsurancePlanCoverageBenefitLimit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value: Default::default(),
            code: Default::default(),
        }
    }
}

impl Default for InsurancePlanPlanSpecificcost {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
        }
    }
}

impl Default for InsurancePlanContact {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            purpose: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            address: Default::default(),
        }
    }
}

impl Default for InsurancePlanPlan {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            specific_cost: Default::default(),
            general_cost: Default::default(),
            identifier: Default::default(),
            type_: Default::default(),
            coverage_area: Default::default(),
            network: Default::default(),
        }
    }
}

impl Default for InsurancePlanCoverageBenefit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            requirement: Default::default(),
            _requirement: Default::default(),
        }
    }
}

impl Default for InsurancePlanPlanSpecificcostBenefitCost {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            applicability: Default::default(),
            qualifiers: Default::default(),
            value: Default::default(),
        }
    }
}

impl Default for InsurancePlanPlanSpecificcostBenefit {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for InsurancePlan {
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

impl crate::traits::resource::ResourceMutators for InsurancePlan {
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

impl crate::traits::resource::ResourceExistence for InsurancePlan {
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

impl crate::traits::domain_resource::DomainResourceAccessors for InsurancePlan {
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

impl crate::traits::domain_resource::DomainResourceMutators for InsurancePlan {
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

impl crate::traits::domain_resource::DomainResourceExistence for InsurancePlan {
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

impl crate::traits::insurance_plan::InsurancePlanAccessors for InsurancePlan {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<PublicationStatus> {
        self.status.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn alias(&self) -> &[StringType] {
        self.alias.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn owned_by(&self) -> Option<Reference> {
        self.owned_by.clone()
    }
    fn administered_by(&self) -> Option<Reference> {
        self.administered_by.clone()
    }
    fn coverage_area(&self) -> &[Reference] {
        self.coverage_area.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[InsurancePlanContact] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
    fn network(&self) -> &[Reference] {
        self.network.as_deref().unwrap_or(&[])
    }
    fn coverage(&self) -> &[InsurancePlanCoverage] {
        self.coverage.as_deref().unwrap_or(&[])
    }
    fn plan(&self) -> &[InsurancePlanPlan] {
        self.plan.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::insurance_plan::InsurancePlanMutators for InsurancePlan {
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
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_alias(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.alias = Some(value);
        resource
    }
    fn add_alias(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.alias.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_owned_by(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.owned_by = Some(value);
        resource
    }
    fn set_administered_by(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.administered_by = Some(value);
        resource
    }
    fn set_coverage_area(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.coverage_area = Some(value);
        resource
    }
    fn add_coverage_area(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .coverage_area
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_contact(self, value: Vec<InsurancePlanContact>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: InsurancePlanContact) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_endpoint(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.endpoint = Some(value);
        resource
    }
    fn add_endpoint(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.endpoint.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_network(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.network = Some(value);
        resource
    }
    fn add_network(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.network.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_coverage(self, value: Vec<InsurancePlanCoverage>) -> Self {
        let mut resource = self.clone();
        resource.coverage = Some(value);
        resource
    }
    fn add_coverage(self, item: InsurancePlanCoverage) -> Self {
        let mut resource = self.clone();
        resource.coverage.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_plan(self, value: Vec<InsurancePlanPlan>) -> Self {
        let mut resource = self.clone();
        resource.plan = Some(value);
        resource
    }
    fn add_plan(self, item: InsurancePlanPlan) -> Self {
        let mut resource = self.clone();
        resource.plan.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::insurance_plan::InsurancePlanExistence for InsurancePlan {
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
        self.status.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_alias(&self) -> bool {
        self.alias.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_owned_by(&self) -> bool {
        self.owned_by.is_some()
    }
    fn has_administered_by(&self) -> bool {
        self.administered_by.is_some()
    }
    fn has_coverage_area(&self) -> bool {
        self.coverage_area.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_network(&self) -> bool {
        self.network.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_coverage(&self) -> bool {
        self.coverage.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_plan(&self) -> bool {
        self.plan.as_ref().is_some_and(|v| !v.is_empty())
    }
}
