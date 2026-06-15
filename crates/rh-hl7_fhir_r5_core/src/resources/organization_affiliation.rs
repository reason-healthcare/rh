use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// OrganizationAffiliation
///
/// Defines an affiliation/assotiation/relationship between 2 distinct organizations, that is not a part-of relationship/sub-division relationship.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: OrganizationAffiliation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAffiliation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifiers that are specific to this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
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
    /// The network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<Reference>,
    /// Definition of the role the participatingOrganization plays
    ///
    /// Binding: example (The role the participating organization providing services to the primary organization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/organization-role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<CodeableConcept>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<CodeableConcept>,
    /// The location(s) at which the role occurs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<Reference>,
    /// Healthcare services provided through the role
    #[serde(rename = "healthcareService")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<Reference>,
    /// Official contact details at the participatingOrganization relevant to this Affiliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ExtendedContactDetail>,
    /// Technical endpoints providing access to services operated for this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<Reference>,
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
            contact: Default::default(),
            endpoint: Default::default(),
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
    rh_foundation::Invariant::new("org-3", rh_foundation::Severity::Error, "The telecom of an organization can never be of use 'home'", "telecom.where(use = 'home').empty()"),
    rh_foundation::Invariant::new("org-4", rh_foundation::Severity::Error, "The address of an organization can never be of use 'home'", "address.where(use = 'home').empty()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "OrganizationAffiliation.language",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
        )
        .with_description("IETF language tag for a human language")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OrganizationAffiliation.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.contained", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "OrganizationAffiliation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.identifier", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.active", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OrganizationAffiliation.organization",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OrganizationAffiliation.participatingOrganization",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.network", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.code", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.specialty", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.location", 0, None),
            rh_foundation::ElementCardinality::new(
                "OrganizationAffiliation.healthcareService",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.contact", 0, None),
            rh_foundation::ElementCardinality::new("OrganizationAffiliation.endpoint", 0, None),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for OrganizationAffiliation {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::organization_affiliation::OrganizationAffiliationAccessors
    for OrganizationAffiliation
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
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
        self.network.as_slice()
    }
    fn code(&self) -> &[CodeableConcept] {
        self.code.as_slice()
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_slice()
    }
    fn location(&self) -> &[Reference] {
        self.location.as_slice()
    }
    fn healthcare_service(&self) -> &[Reference] {
        self.healthcare_service.as_slice()
    }
    fn contact(&self) -> &[ExtendedContactDetail] {
        self.contact.as_slice()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_slice()
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
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
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
        resource.network = value;
        resource
    }
    fn add_network(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.network.push(item);
        resource
    }
    fn set_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn add_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code.push(item);
        resource
    }
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.specialty = value;
        resource
    }
    fn add_specialty(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.specialty.push(item);
        resource
    }
    fn set_location(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.location = value;
        resource
    }
    fn add_location(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.location.push(item);
        resource
    }
    fn set_healthcare_service(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.healthcare_service = value;
        resource
    }
    fn add_healthcare_service(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.healthcare_service.push(item);
        resource
    }
    fn set_contact(self, value: Vec<ExtendedContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ExtendedContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_endpoint(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.endpoint = value;
        resource
    }
    fn add_endpoint(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.endpoint.push(item);
        resource
    }
}

impl crate::traits::organization_affiliation::OrganizationAffiliationExistence
    for OrganizationAffiliation
{
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
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
        !self.network.is_empty()
    }
    fn has_code(&self) -> bool {
        !self.code.is_empty()
    }
    fn has_specialty(&self) -> bool {
        !self.specialty.is_empty()
    }
    fn has_location(&self) -> bool {
        !self.location.is_empty()
    }
    fn has_healthcare_service(&self) -> bool {
        !self.healthcare_service.is_empty()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_endpoint(&self) -> bool {
        !self.endpoint.is_empty()
    }
}

impl crate::validation::ValidatableResource for OrganizationAffiliation {
    fn resource_type(&self) -> &'static str {
        "OrganizationAffiliation"
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
        Some("http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::organization_affiliation::{
    OrganizationAffiliationAccessors, OrganizationAffiliationExistence,
    OrganizationAffiliationMutators,
};
