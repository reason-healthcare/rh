use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Organization
///
/// A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Organization
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Organization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifies this organization  across multiple systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Whether the organization's record is still in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// Kind of organization
    ///
    /// Binding: example (Used to categorize the organization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/organization-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
    /// Name used for the organization
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// A list of alternate names that the organization is known as, or was known as in the past
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<StringType>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _alias: Vec<Element>,
    /// Additional details about the Organization that could be displayed as further information to identify the Organization beyond its name
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Official contact details for the Organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ExtendedContactDetail>,
    /// The organization of which this organization forms a part
    #[serde(rename = "partOf")]
    pub part_of: Option<Reference>,
    /// Technical endpoints providing access to services operated for the organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<Reference>,
    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualification: Vec<OrganizationQualification>,
}
/// Organization nested structure for the 'qualification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationQualification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// An identifier for this qualification for the organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Coded representation of the qualification
    ///
    /// Binding: example (Specific qualification the organization has to provide a service.)
    pub code: CodeableConcept,
    /// Period during which the qualification is valid
    pub period: Option<Period>,
    /// Organization that regulates and issues the qualification
    pub issuer: Option<Reference>,
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            type_: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            contact: Default::default(),
            part_of: Default::default(),
            endpoint: Default::default(),
            qualification: Default::default(),
        }
    }
}

impl Default for OrganizationQualification {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            code: CodeableConcept::default(),
            period: Default::default(),
            issuer: Default::default(),
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
    rh_foundation::Invariant::new("org-1", rh_foundation::Severity::Error, "The organization SHALL at least have a name or an identifier, and possibly more than one", "(identifier.count() + name.count()) > 0"),
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
            "Organization.language",
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
            rh_foundation::ElementCardinality::new("Organization.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.contained", 0, None),
            rh_foundation::ElementCardinality::new("Organization.extension", 0, None),
            rh_foundation::ElementCardinality::new("Organization.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Organization.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Organization.active", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.type", 0, None),
            rh_foundation::ElementCardinality::new("Organization.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.alias", 0, None),
            rh_foundation::ElementCardinality::new("Organization.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.contact", 0, None),
            rh_foundation::ElementCardinality::new("Organization.partOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.endpoint", 0, None),
            rh_foundation::ElementCardinality::new("Organization.qualification", 0, None),
            rh_foundation::ElementCardinality::new("Organization.qualification.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.qualification.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Organization.qualification.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Organization.qualification.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Organization.qualification.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.qualification.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Organization.qualification.issuer", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Organization {
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

impl crate::traits::resource::ResourceMutators for Organization {
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

impl crate::traits::resource::ResourceExistence for Organization {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Organization {
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

impl crate::traits::domain_resource::DomainResourceMutators for Organization {
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

impl crate::traits::domain_resource::DomainResourceExistence for Organization {
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

impl crate::traits::organization::OrganizationAccessors for Organization {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_slice()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn alias(&self) -> &[StringType] {
        self.alias.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn contact(&self) -> &[ExtendedContactDetail] {
        self.contact.as_slice()
    }
    fn part_of(&self) -> Option<Reference> {
        self.part_of.clone()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_slice()
    }
    fn qualification(&self) -> &[OrganizationQualification] {
        self.qualification.as_slice()
    }
}

impl crate::traits::organization::OrganizationMutators for Organization {
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
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_alias(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.alias = value;
        resource
    }
    fn add_alias(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.alias.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
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
    fn set_part_of(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
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
    fn set_qualification(self, value: Vec<OrganizationQualification>) -> Self {
        let mut resource = self.clone();
        resource.qualification = value;
        resource
    }
    fn add_qualification(self, item: OrganizationQualification) -> Self {
        let mut resource = self.clone();
        resource.qualification.push(item);
        resource
    }
}

impl crate::traits::organization::OrganizationExistence for Organization {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_type_(&self) -> bool {
        !self.type_.is_empty()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_alias(&self) -> bool {
        !self.alias.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_part_of(&self) -> bool {
        self.part_of.is_some()
    }
    fn has_endpoint(&self) -> bool {
        !self.endpoint.is_empty()
    }
    fn has_qualification(&self) -> bool {
        !self.qualification.is_empty()
    }
}

impl crate::validation::ValidatableResource for Organization {
    fn resource_type(&self) -> &'static str {
        "Organization"
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
        Some("http://hl7.org/fhir/StructureDefinition/Organization")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::organization::{
    OrganizationAccessors, OrganizationExistence, OrganizationMutators,
};
