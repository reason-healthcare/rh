use crate::bindings::administrative_gender::AdministrativeGender;
use crate::bindings::identity_assurance_level::IdentityAssuranceLevel;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Person
///
/// Demographics and administrative information about a person independent of a specific health-related context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Person
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Person
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A human identifier for this person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// This person's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// A name associated with the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<HumanName>,
    /// A contact detail for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<ContactPoint>,
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
    /// Indicates if the individual is deceased or not (boolean)
    #[serde(rename = "deceasedBoolean")]
    pub deceased_boolean: Option<BooleanType>,
    /// Indicates if the individual is deceased or not (dateTime)
    #[serde(rename = "deceasedDateTime")]
    pub deceased_date_time: Option<DateTimeType>,
    /// One or more addresses for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<Address>,
    /// Marital (civil) status of a person
    ///
    /// Binding: extensible (The domestic partnership status of a person.)
    ///
    /// Available values:
    /// - `UNK`
    #[serde(rename = "maritalStatus")]
    pub marital_status: Option<CodeableConcept>,
    /// Image of the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<Attachment>,
    /// A language which may be used to communicate with the person about his or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PersonCommunication>,
    /// The organization that is the custodian of the person record
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Link to a resource that concerns the same actual person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PersonLink>,
}
/// Person nested structure for the 'communication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonCommunication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The language which can be used to communicate with the person about his or her health
    pub language: StringType,
    /// Language preference indicator
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
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
            active: Default::default(),
            _active: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            birth_date: Default::default(),
            _birth_date: Default::default(),
            deceased_boolean: Default::default(),
            deceased_date_time: Default::default(),
            address: Default::default(),
            marital_status: Default::default(),
            photo: Default::default(),
            communication: Default::default(),
            managing_organization: Default::default(),
            link: Default::default(),
        }
    }
}

impl Default for PersonCommunication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            language: StringType::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
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
    rh_foundation::ElementBinding::new("Person.communication.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Person.gender", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/administrative-gender|5.0.0").with_description("The gender of a person used for administrative purposes."),
    rh_foundation::ElementBinding::new("Person.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Person.link.assurance", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/identity-assuranceLevel|5.0.0").with_description("The level of confidence that this link represents the same actual person, based on NIST Authentication Levels."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Person.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.contained", 0, None),
            rh_foundation::ElementCardinality::new("Person.extension", 0, None),
            rh_foundation::ElementCardinality::new("Person.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Person.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Person.active", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.name", 0, None),
            rh_foundation::ElementCardinality::new("Person.telecom", 0, None),
            rh_foundation::ElementCardinality::new("Person.gender", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.birthDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.deceased[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.address", 0, None),
            rh_foundation::ElementCardinality::new("Person.maritalStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.photo", 0, None),
            rh_foundation::ElementCardinality::new("Person.communication", 0, None),
            rh_foundation::ElementCardinality::new("Person.communication.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.communication.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Person.communication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Person.communication.language", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Person.communication.preferred", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.managingOrganization", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.link", 0, None),
            rh_foundation::ElementCardinality::new("Person.link.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Person.link.extension", 0, None),
            rh_foundation::ElementCardinality::new("Person.link.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Person.link.target", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Person.link.assurance", 0, Some(1)),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for Person {
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

impl crate::traits::person::PersonAccessors for Person {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn name(&self) -> &[HumanName] {
        self.name.as_slice()
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_slice()
    }
    fn gender(&self) -> Option<AdministrativeGender> {
        self.gender.clone()
    }
    fn birth_date(&self) -> Option<DateType> {
        self.birth_date.clone()
    }
    fn address(&self) -> &[Address] {
        self.address.as_slice()
    }
    fn marital_status(&self) -> Option<CodeableConcept> {
        self.marital_status.clone()
    }
    fn photo(&self) -> &[Attachment] {
        self.photo.as_slice()
    }
    fn communication(&self) -> &[PersonCommunication] {
        self.communication.as_slice()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn link(&self) -> &[PersonLink] {
        self.link.as_slice()
    }
}

impl crate::traits::person::PersonMutators for Person {
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
    fn set_name(self, value: Vec<HumanName>) -> Self {
        let mut resource = self.clone();
        resource.name = value;
        resource
    }
    fn add_name(self, item: HumanName) -> Self {
        let mut resource = self.clone();
        resource.name.push(item);
        resource
    }
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.telecom = value;
        resource
    }
    fn add_telecom(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.telecom.push(item);
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
        resource.address = value;
        resource
    }
    fn add_address(self, item: Address) -> Self {
        let mut resource = self.clone();
        resource.address.push(item);
        resource
    }
    fn set_marital_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.marital_status = Some(value);
        resource
    }
    fn set_photo(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.photo = value;
        resource
    }
    fn add_photo(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.photo.push(item);
        resource
    }
    fn set_communication(self, value: Vec<PersonCommunication>) -> Self {
        let mut resource = self.clone();
        resource.communication = value;
        resource
    }
    fn add_communication(self, item: PersonCommunication) -> Self {
        let mut resource = self.clone();
        resource.communication.push(item);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_link(self, value: Vec<PersonLink>) -> Self {
        let mut resource = self.clone();
        resource.link = value;
        resource
    }
    fn add_link(self, item: PersonLink) -> Self {
        let mut resource = self.clone();
        resource.link.push(item);
        resource
    }
}

impl crate::traits::person::PersonExistence for Person {
    fn has_deceased(&self) -> bool {
        self.deceased_boolean.is_some() || self.deceased_date_time.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_active(&self) -> bool {
        self.active.is_some()
    }
    fn has_name(&self) -> bool {
        !self.name.is_empty()
    }
    fn has_telecom(&self) -> bool {
        !self.telecom.is_empty()
    }
    fn has_gender(&self) -> bool {
        self.gender.is_some()
    }
    fn has_birth_date(&self) -> bool {
        self.birth_date.is_some()
    }
    fn has_address(&self) -> bool {
        !self.address.is_empty()
    }
    fn has_marital_status(&self) -> bool {
        self.marital_status.is_some()
    }
    fn has_photo(&self) -> bool {
        !self.photo.is_empty()
    }
    fn has_communication(&self) -> bool {
        !self.communication.is_empty()
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_link(&self) -> bool {
        !self.link.is_empty()
    }
}

impl crate::validation::ValidatableResource for Person {
    fn resource_type(&self) -> &'static str {
        "Person"
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
        Some("http://hl7.org/fhir/StructureDefinition/Person")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::person::{PersonAccessors, PersonExistence, PersonMutators};
