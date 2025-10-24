use crate::bindings::administrative_gender::AdministrativeGender;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Practitioner
///
/// A person who is directly or indirectly involved in the provisioning of healthcare.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Practitioner
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Practitioner
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practitioner {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// An identifier for the person as this agent
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this practitioner's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// The name(s) associated with the practitioner
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the practitioner (that apply to all roles)
    pub telecom: Option<Vec<ContactPoint>>,
    /// Address(es) of the practitioner that are not role specific (typically home address)
    pub address: Option<Vec<Address>>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// The date  on which the practitioner was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<DateType>,
    /// Extension element for the 'birthDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_birthDate")]
    pub _birth_date: Option<Element>,
    /// Image of the person
    pub photo: Option<Vec<Attachment>>,
    /// Certification, licenses, or training pertaining to the provision of care
    pub qualification: Option<Vec<PractitionerQualification>>,
    /// A language the practitioner can use in patient communication
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `da`: Danish
    /// - `de`: German
    /// - `de-AT`: German (Austria)
    /// - `de-CH`: German (Switzerland)
    /// - `de-DE`: German (Germany)
    /// - `el`: Greek
    /// - `en`: English
    /// - ... and 46 more values
    pub communication: Option<Vec<CodeableConcept>>,
}
/// Practitioner nested structure for the 'qualification' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerQualification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// An identifier for this qualification for the practitioner
    pub identifier: Option<Vec<Identifier>>,
    /// Coded representation of the qualification
    ///
    /// Binding: example (Specific qualification the practitioner has to provide a service.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-2.7-0360
    pub code: CodeableConcept,
    /// Period during which the qualification is valid
    pub period: Option<Period>,
    /// Organization that regulates and issues the qualification
    pub issuer: Option<Reference>,
}
/// animalSpecies
///
/// This extension should be used to specifiy that a practioner or RelatedPerson resource is a service animal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/practitioner-animalSpecies
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerAnimalSpecies {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Practitioner {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            address: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            birth_date: Default::default(),
            _birth_date: Default::default(),
            photo: Default::default(),
            qualification: Default::default(),
            communication: Default::default(),
        }
    }
}

impl Default for PractitionerQualification {
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

impl Default for PractitionerAnimalSpecies {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
impl crate::traits::resource::ResourceAccessors for Practitioner {
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

impl crate::traits::resource::ResourceMutators for Practitioner {
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

impl crate::traits::resource::ResourceExistence for Practitioner {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Practitioner {
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

impl crate::traits::domain_resource::DomainResourceMutators for Practitioner {
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

impl crate::traits::domain_resource::DomainResourceExistence for Practitioner {
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

impl crate::traits::practitioner::PractitionerAccessors for Practitioner {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn name(&self) -> &[HumanName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn address(&self) -> &[Address] {
        self.address.as_deref().unwrap_or(&[])
    }
    fn gender(&self) -> Option<AdministrativeGender> {
        self.gender.clone()
    }
    fn birth_date(&self) -> Option<DateType> {
        self.birth_date.clone()
    }
    fn photo(&self) -> &[Attachment] {
        self.photo.as_deref().unwrap_or(&[])
    }
    fn qualification(&self) -> &[PractitionerQualification] {
        self.qualification.as_deref().unwrap_or(&[])
    }
    fn communication(&self) -> &[CodeableConcept] {
        self.communication.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::practitioner::PractitionerMutators for Practitioner {
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
    fn set_photo(self, value: Vec<Attachment>) -> Self {
        let mut resource = self.clone();
        resource.photo = Some(value);
        resource
    }
    fn add_photo(self, item: Attachment) -> Self {
        let mut resource = self.clone();
        resource.photo.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_qualification(self, value: Vec<PractitionerQualification>) -> Self {
        let mut resource = self.clone();
        resource.qualification = Some(value);
        resource
    }
    fn add_qualification(self, item: PractitionerQualification) -> Self {
        let mut resource = self.clone();
        resource
            .qualification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_communication(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.communication = Some(value);
        resource
    }
    fn add_communication(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .communication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::practitioner::PractitionerExistence for Practitioner {
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
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_address(&self) -> bool {
        self.address.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_gender(&self) -> bool {
        self.gender.is_some()
    }
    fn has_birth_date(&self) -> bool {
        self.birth_date.is_some()
    }
    fn has_photo(&self) -> bool {
        self.photo.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_qualification(&self) -> bool {
        self.qualification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_communication(&self) -> bool {
        self.communication.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Practitioner {
    fn resource_type(&self) -> &'static str {
        "Practitioner"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Practitioner")
    }
}
