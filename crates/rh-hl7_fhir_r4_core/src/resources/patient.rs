use crate::bindings::administrative_gender::AdministrativeGender;
use crate::bindings::link_type::LinkType;
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
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Patient
///
/// Demographics and other administrative information about an individual or animal receiving care or other health-related services.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Patient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// An identifier for this patient
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this patient's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// A name associated with the patient
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the individual
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// The date of birth for the individual
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
    /// An address for the individual
    pub address: Option<Vec<Address>>,
    /// Marital (civil) status of a patient
    ///
    /// Binding: extensible (The domestic partnership status of a person.)
    ///
    /// Available values:
    /// - `UNK`
    #[serde(rename = "maritalStatus")]
    pub marital_status: Option<CodeableConcept>,
    /// Whether patient is part of a multiple birth (boolean)
    #[serde(rename = "multipleBirthBoolean")]
    pub multiple_birth_boolean: Option<BooleanType>,
    /// Whether patient is part of a multiple birth (integer)
    #[serde(rename = "multipleBirthInteger")]
    pub multiple_birth_integer: Option<IntegerType>,
    /// Image of the patient
    pub photo: Option<Vec<Attachment>>,
    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<PatientContact>>,
    /// A language which may be used to communicate with the patient about his or her health
    pub communication: Option<Vec<PatientCommunication>>,
    /// Patient's nominated primary care provider
    #[serde(rename = "generalPractitioner")]
    pub general_practitioner: Option<Vec<Reference>>,
    /// Organization that is the custodian of the patient record
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Link to another patient resource that concerns the same actual person
    pub link: Option<Vec<PatientLink>>,
}
/// Patient nested structure for the 'contact' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientContact {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of relationship
    ///
    /// Binding: extensible (The nature of the relationship between a patient and a contact person for that patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/patient-contactrelationship
    pub relationship: Option<Vec<CodeableConcept>>,
    /// A name associated with the contact person
    pub name: Option<HumanName>,
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
    /// Address for the contact person
    pub address: Option<Address>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// Organization that is associated with the contact
    pub organization: Option<Reference>,
    /// The period during which this contact person or organization is valid to be contacted relating to this patient
    pub period: Option<Period>,
}
/// importance
///
/// The importance of the patient (e.g. VIP).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-importance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientImportance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// cadavericDonor
///
/// Flag indicating whether the patient authorized the donation of body parts after death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-cadavericDonor
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCadavericDonor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// disability
///
/// Value(s) identifying physical or mental condition(s) that limits a person's movements, senses, or activities.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-disability
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDisability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// genderIdentity
///
/// The gender the patient identifies with. The Patient's gender identity is used as guidance (e.g. for staff) about how to interact with the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-genderIdentity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientGenderIdentity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// animal
///
/// This patient is known to be an animal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-animal
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAnimal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// proficiency
///
/// Proficiency level of the communication.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-proficiency
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProficiency {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Patient nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The other patient or related person resource that the link refers to
    pub other: Reference,
    /// replaced-by | replaces | refer | seealso
    #[serde(rename = "type")]
    pub type_: LinkType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// Patient nested structure for the 'communication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCommunication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The language which can be used to communicate with the patient about his or her health
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
    pub language: CodeableConcept,
    /// Language preference indicator
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
}
/// congregation
///
/// A group or place of religious practice that may provide services to the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-congregation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCongregation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// preferenceType
///
/// Indicates what mode of communication the patient prefers to use for the indicated language.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-preferenceType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPreferenceType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// citizenship
///
/// The patient's legal status as citizen of a country.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-citizenship
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCitizenship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// relatedPerson
///
/// In some cases a Patient.contact will also be populated as a RelatedPerson resource. This linkage permits the linkage between the 2 resources to be able to accurately indicate a representation of the same individual, and updating details between could be appropriate.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-relatedPerson
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRelatedPerson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// adoptionInfo
///
/// Code indication the adoption status of the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/patient-adoptionInfo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdoptionInfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Patient {
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
            multiple_birth_boolean: Default::default(),
            multiple_birth_integer: Default::default(),
            photo: Default::default(),
            contact: Default::default(),
            communication: Default::default(),
            general_practitioner: Default::default(),
            managing_organization: Default::default(),
            link: Default::default(),
        }
    }
}

impl Default for PatientContact {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relationship: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            address: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            organization: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for PatientImportance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientCadavericDonor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientDisability {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientGenderIdentity {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientAnimal {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientProficiency {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            other: Reference::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for PatientCommunication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            language: CodeableConcept::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
        }
    }
}

impl Default for PatientCongregation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientPreferenceType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientCitizenship {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientRelatedPerson {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for PatientAdoptionInfo {
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
    rh_foundation::Invariant::new("pat-1", rh_foundation::Severity::Error, "SHALL at least contain a contact's details or a reference to an organization", "name.exists() or telecom.exists() or address.exists() or organization.exists()").with_xpath("exists(f:name) or exists(f:telecom) or exists(f:address) or exists(f:organization)"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Patient {
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

impl crate::traits::resource::ResourceMutators for Patient {
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

impl crate::traits::resource::ResourceExistence for Patient {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Patient {
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

impl crate::traits::domain_resource::DomainResourceMutators for Patient {
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

impl crate::traits::domain_resource::DomainResourceExistence for Patient {
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

impl crate::traits::patient::PatientAccessors for Patient {
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
    fn gender(&self) -> Option<AdministrativeGender> {
        self.gender.clone()
    }
    fn birth_date(&self) -> Option<DateType> {
        self.birth_date.clone()
    }
    fn address(&self) -> &[Address] {
        self.address.as_deref().unwrap_or(&[])
    }
    fn marital_status(&self) -> Option<CodeableConcept> {
        self.marital_status.clone()
    }
    fn photo(&self) -> &[Attachment] {
        self.photo.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[PatientContact] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn communication(&self) -> &[PatientCommunication] {
        self.communication.as_deref().unwrap_or(&[])
    }
    fn general_practitioner(&self) -> &[Reference] {
        self.general_practitioner.as_deref().unwrap_or(&[])
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn link(&self) -> &[PatientLink] {
        self.link.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::patient::PatientMutators for Patient {
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
    fn set_marital_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.marital_status = Some(value);
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
    fn set_contact(self, value: Vec<PatientContact>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: PatientContact) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_communication(self, value: Vec<PatientCommunication>) -> Self {
        let mut resource = self.clone();
        resource.communication = Some(value);
        resource
    }
    fn add_communication(self, item: PatientCommunication) -> Self {
        let mut resource = self.clone();
        resource
            .communication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_general_practitioner(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.general_practitioner = Some(value);
        resource
    }
    fn add_general_practitioner(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .general_practitioner
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_link(self, value: Vec<PatientLink>) -> Self {
        let mut resource = self.clone();
        resource.link = Some(value);
        resource
    }
    fn add_link(self, item: PatientLink) -> Self {
        let mut resource = self.clone();
        resource.link.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::patient::PatientExistence for Patient {
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
    fn has_multiple_birth(&self) -> bool {
        self.multiple_birth_boolean.is_some() || self.multiple_birth_integer.is_some()
    }
    fn has_deceased(&self) -> bool {
        self.deceased_boolean.is_some() || self.deceased_date_time.is_some()
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
    fn has_gender(&self) -> bool {
        self.gender.is_some()
    }
    fn has_birth_date(&self) -> bool {
        self.birth_date.is_some()
    }
    fn has_address(&self) -> bool {
        self.address.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_marital_status(&self) -> bool {
        self.marital_status.is_some()
    }
    fn has_photo(&self) -> bool {
        self.photo.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_communication(&self) -> bool {
        self.communication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_general_practitioner(&self) -> bool {
        self.general_practitioner
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_link(&self) -> bool {
        self.link.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Patient {
    fn resource_type(&self) -> &'static str {
        "Patient"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Patient")
    }
}
