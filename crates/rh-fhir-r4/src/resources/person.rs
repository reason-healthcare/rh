use crate::bindings::administrative_gender::AdministrativeGender;
use crate::bindings::identity_assurance_level::IdentityAssuranceLevel;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Person
///
/// Demographics and administrative information about a person independent of a specific health-related context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Person
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Person
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A human identifier for this person
    pub identifier: Option<Vec<Identifier>>,
    /// A name associated with the person
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// The date on which the person was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<DateType>,
    /// Extension element for the 'birthDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_birthDate")]
    pub _birth_date: Option<Element>,
    /// One or more addresses for the person
    pub address: Option<Vec<Address>>,
    /// Image of the person
    pub photo: Option<Attachment>,
    /// The organization that is the custodian of the person record
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// This person's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// Link to a resource that concerns the same actual person
    pub link: Option<Vec<PersonLink>>,
}
/// Person nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The resource to which this actual person is associated
    pub target: Reference,
    /// level1 | level2 | level3 | level4
    pub assurance: Option<IdentityAssuranceLevel>,
    /// Extension element for the 'assurance' primitive field. Contains metadata and extensions.
    pub _assurance: Option<Element>,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            birth_date: Default::default(),
            _birth_date: Default::default(),
            address: Default::default(),
            photo: Default::default(),
            managing_organization: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            link: Default::default(),
        }
    }
}

impl Default for PersonLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target: Reference::default(),
            assurance: Default::default(),
            _assurance: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Person {
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

impl crate::traits::resource::ResourceMutators for Person {
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

impl crate::traits::resource::ResourceExistence for Person {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Person {
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

impl crate::traits::domain_resource::DomainResourceMutators for Person {
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

impl crate::traits::domain_resource::DomainResourceExistence for Person {
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

impl crate::traits::person::PersonAccessors for Person {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[HumanName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn gender(&self) -> Option<AdministrativeGender> {
        self.gender.clone()
    }
    fn birth_date(&self) -> Option<DateType> {
        self.birth_date.clone()
    }
    fn address(&self) -> &[Address] {
        self.address.as_deref().unwrap_or(&[])
    }
    fn photo(&self) -> Option<Attachment> {
        self.photo.clone()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn link(&self) -> &[PersonLink] {
        self.link.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::person::PersonMutators for Person {
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
    fn set_name(self, value: Vec<HumanName>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: HumanName) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
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
    fn set_gender(self, value: AdministrativeGender) -> Self {
        let mut resource = self.clone();
        resource.gender = Some(value);
        resource
    }
    fn set_birth_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.birth_date = Some(value);
        resource
    }
    fn set_address(self, value: Vec<Address>) -> Self {
        let mut resource = self.clone();
        resource.address = Some(value);
        resource
    }
    fn add_address(self, item: Address) -> Self {
        let mut resource = self.clone();
        resource.address.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_photo(self, value: Attachment) -> Self {
        let mut resource = self.clone();
        resource.photo = Some(value);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_active(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.active = Some(value);
        resource
    }
    fn set_link(self, value: Vec<PersonLink>) -> Self {
        let mut resource = self.clone();
        resource.link = Some(value);
        resource
    }
    fn add_link(self, item: PersonLink) -> Self {
        let mut resource = self.clone();
        resource.link.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::person::PersonExistence for Person {
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
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_gender(&self) -> bool {
        self.gender.is_some()
    }
    fn has_birth_date(&self) -> bool {
        self.birth_date.is_some()
    }
    fn has_address(&self) -> bool {
        self.address.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_photo(&self) -> bool {
        self.photo.is_some()
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_link(&self) -> bool {
        self.link.as_ref().is_some_and(|v| !v.is_empty())
    }
}
