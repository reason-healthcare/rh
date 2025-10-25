use crate::bindings::administrative_gender::AdministrativeGender;
use crate::datatypes::address::Address;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// RelatedPerson
///
/// Information about a person that is involved in the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RelatedPerson
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedPerson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A human identifier for this person
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this related person's record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// The patient this person is related to
    pub patient: Reference,
    /// The nature of the relationship
    ///
    /// Binding: preferred (The nature of the relationship between a patient and the related person.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/relatedperson-relationshiptype
    pub relationship: Option<Vec<CodeableConcept>>,
    /// A name associated with the person
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGender>,
    /// Extension element for the 'gender' primitive field. Contains metadata and extensions.
    pub _gender: Option<Element>,
    /// The date on which the related person was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<DateType>,
    /// Extension element for the 'birthDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_birthDate")]
    pub _birth_date: Option<Element>,
    /// Address where the related person can be contacted or visited
    pub address: Option<Vec<Address>>,
    /// Image of the person
    pub photo: Option<Vec<Attachment>>,
    /// Period of time that this relationship is considered valid
    pub period: Option<Period>,
    /// A language which may be used to communicate with about the patient's health
    pub communication: Option<Vec<RelatedPersonCommunication>>,
}
/// RelatedPerson nested structure for the 'communication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedPersonCommunication {
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

impl Default for RelatedPerson {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            patient: Reference::default(),
            relationship: Default::default(),
            name: Default::default(),
            telecom: Default::default(),
            gender: Default::default(),
            _gender: Default::default(),
            birth_date: Default::default(),
            _birth_date: Default::default(),
            address: Default::default(),
            photo: Default::default(),
            period: Default::default(),
            communication: Default::default(),
        }
    }
}

impl Default for RelatedPersonCommunication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            language: CodeableConcept::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "RelatedPerson.gender",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/administrative-gender|4.0.1",
        )
        .with_description("The gender of a person used for administrative purposes.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RelatedPerson.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.contained", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.extension", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.identifier", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.active", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.relationship", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.name", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.telecom", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.gender", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.birthDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.address", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.photo", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RelatedPerson.communication", 0, None),
            rh_foundation::ElementCardinality::new("RelatedPerson.communication.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RelatedPerson.communication.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RelatedPerson.communication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RelatedPerson.communication.language",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RelatedPerson.communication.preferred",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RelatedPerson {
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

impl crate::traits::resource::ResourceMutators for RelatedPerson {
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

impl crate::traits::resource::ResourceExistence for RelatedPerson {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RelatedPerson {
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

impl crate::traits::domain_resource::DomainResourceMutators for RelatedPerson {
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

impl crate::traits::domain_resource::DomainResourceExistence for RelatedPerson {
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

impl crate::traits::related_person::RelatedPersonAccessors for RelatedPerson {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn relationship(&self) -> &[CodeableConcept] {
        self.relationship.as_deref().unwrap_or(&[])
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
    fn photo(&self) -> &[Attachment] {
        self.photo.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn communication(&self) -> &[RelatedPersonCommunication] {
        self.communication.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::related_person::RelatedPersonMutators for RelatedPerson {
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
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_relationship(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.relationship = Some(value);
        resource
    }
    fn add_relationship(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .relationship
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_communication(self, value: Vec<RelatedPersonCommunication>) -> Self {
        let mut resource = self.clone();
        resource.communication = Some(value);
        resource
    }
    fn add_communication(self, item: RelatedPersonCommunication) -> Self {
        let mut resource = self.clone();
        resource
            .communication
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::related_person::RelatedPersonExistence for RelatedPerson {
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
    fn has_patient(&self) -> bool {
        true
    }
    fn has_relationship(&self) -> bool {
        self.relationship.as_ref().is_some_and(|v| !v.is_empty())
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
        self.photo.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_communication(&self) -> bool {
        self.communication.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for RelatedPerson {
    fn resource_type(&self) -> &'static str {
        "RelatedPerson"
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
        Some("http://hl7.org/fhir/StructureDefinition/RelatedPerson")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::related_person::{
    RelatedPersonAccessors, RelatedPersonExistence, RelatedPersonMutators,
};
