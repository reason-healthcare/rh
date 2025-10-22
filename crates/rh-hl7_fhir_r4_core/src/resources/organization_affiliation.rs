use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// OrganizationAffiliation
///
/// Defines an affiliation/assotiation/relationship between 2 distinct oganizations, that is not a part-of relationship/sub-division relationship.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OrganizationAffiliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAffiliation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifiers that are specific to this role
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this organization affiliation record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// The period during which the participatingOrganization is affiliated with the primary organization
    pub period: Option<Period>,
    /// Organization where the role is available
    pub organization: Option<Reference>,
    /// Organization that provides/performs the role (e.g. providing services or is a member of)
    #[serde(rename = "participatingOrganization")]
    pub participating_organization: Option<Reference>,
    /// Health insurance provider network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    pub network: Option<Vec<Reference>>,
    /// Definition of the role the participatingOrganization plays
    ///
    /// Binding: example (The role the participating organization providing services to the primary organization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/organization-role
    pub code: Option<Vec<CodeableConcept>>,
    /// Specific specialty of the participatingOrganization in the context of the role
    ///
    /// Binding: preferred (Specific specialty associated with the participating organization.)
    ///
    /// Available values:
    /// - `408467006`: Adult mental illness
    /// - `394577000`: Anesthetics
    /// - `394578005`: Audiological medicine
    /// - `421661004`: Blood banking and transfusion medicine
    /// - `408462000`: Burns care
    /// - `394579002`: Cardiology
    /// - `394804000`: Clinical cytogenetics and molecular genetics
    /// - `394580004`: Clinical genetics
    /// - `394803006`: Clinical hematology
    /// - `408480009`: Clinical immunology
    /// - ... and 107 more values
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The location(s) at which the role occurs
    pub location: Option<Vec<Reference>>,
    /// Healthcare services provided through the role
    #[serde(rename = "healthcareService")]
    pub healthcare_service: Option<Vec<Reference>>,
    /// Contact details at the participatingOrganization relevant to this Affiliation
    pub telecom: Option<Vec<ContactPoint>>,
    /// Technical endpoints providing access to services operated for this role
    pub endpoint: Option<Vec<Reference>>,
}

impl Default for OrganizationAffiliation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            period: Default::default(),
            organization: Default::default(),
            participating_organization: Default::default(),
            network: Default::default(),
            code: Default::default(),
            specialty: Default::default(),
            location: Default::default(),
            healthcare_service: Default::default(),
            telecom: Default::default(),
            endpoint: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for OrganizationAffiliation {
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

impl crate::traits::resource::ResourceMutators for OrganizationAffiliation {
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

impl crate::traits::resource::ResourceExistence for OrganizationAffiliation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for OrganizationAffiliation {
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

impl crate::traits::domain_resource::DomainResourceMutators for OrganizationAffiliation {
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

impl crate::traits::domain_resource::DomainResourceExistence for OrganizationAffiliation {
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

impl crate::traits::organization_affiliation::OrganizationAffiliationAccessors
    for OrganizationAffiliation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn organization(&self) -> Option<Reference> {
        self.organization.clone()
    }
    fn participating_organization(&self) -> Option<Reference> {
        self.participating_organization.clone()
    }
    fn network(&self) -> &[Reference] {
        self.network.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> &[CodeableConcept] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> &[Reference] {
        self.location.as_deref().unwrap_or(&[])
    }
    fn healthcare_service(&self) -> &[Reference] {
        self.healthcare_service.as_deref().unwrap_or(&[])
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::organization_affiliation::OrganizationAffiliationMutators
    for OrganizationAffiliation
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
    fn set_active(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.active = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.organization = Some(value);
        resource
    }
    fn set_participating_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.participating_organization = Some(value);
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
    fn set_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.specialty = Some(value);
        resource
    }
    fn add_specialty(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.specialty.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn add_location(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.location.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_healthcare_service(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.healthcare_service = Some(value);
        resource
    }
    fn add_healthcare_service(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .healthcare_service
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.telecom = Some(value);
        resource
    }
    fn add_telecom(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.telecom.get_or_insert_with(Vec::new).push(item);
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
}

impl crate::traits::organization_affiliation::OrganizationAffiliationExistence
    for OrganizationAffiliation
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
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_organization(&self) -> bool {
        self.organization.is_some()
    }
    fn has_participating_organization(&self) -> bool {
        self.participating_organization.is_some()
    }
    fn has_network(&self) -> bool {
        self.network.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specialty(&self) -> bool {
        self.specialty.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_healthcare_service(&self) -> bool {
        self.healthcare_service
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
}
