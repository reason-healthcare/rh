use crate::bindings::days_of_week::DaysOfWeek;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PractitionerRole
///
/// A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PractitionerRole
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerRole {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business Identifiers that are specific to a role/location
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this practitioner role record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// The period during which the practitioner is authorized to perform in these role(s)
    pub period: Option<Period>,
    /// Practitioner that is able to provide the defined services for the organization
    pub practitioner: Option<Reference>,
    /// Organization where the roles are available
    pub organization: Option<Reference>,
    /// Roles which this practitioner may perform
    ///
    /// Binding: example (The role a person plays representing an organization.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/practitioner-role
    pub code: Option<Vec<CodeableConcept>>,
    /// Specific specialty of the practitioner
    ///
    /// Binding: preferred (Specific specialty associated with the agency.)
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
    /// The location(s) at which this practitioner provides care
    pub location: Option<Vec<Reference>>,
    /// The list of healthcare services that this worker provides for this role's Organization/Location(s)
    #[serde(rename = "healthcareService")]
    pub healthcare_service: Option<Vec<Reference>>,
    /// Contact details that are specific to the role/location/service
    pub telecom: Option<Vec<ContactPoint>>,
    /// Times the Service Site is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<PractitionerRoleAvailabletime>>,
    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailable")]
    pub not_available: Option<Vec<PractitionerRoleNotavailable>>,
    /// Description of availability exceptions
    #[serde(rename = "availabilityExceptions")]
    pub availability_exceptions: Option<StringType>,
    /// Extension element for the 'availabilityExceptions' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availabilityExceptions")]
    pub _availability_exceptions: Option<Element>,
    /// Technical endpoints providing access to services operated for the practitioner with this role
    pub endpoint: Option<Vec<Reference>>,
}
/// PractitionerRole nested structure for the 'notAvailable' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerRoleNotavailable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reason presented to the user explaining why time not available
    pub description: StringType,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Service not available from this date
    pub during: Option<Period>,
}
/// PractitionerRole nested structure for the 'availableTime' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerRoleAvailabletime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Option<Vec<DaysOfWeek>>,
    /// Extension element for the 'daysOfWeek' primitive field. Contains metadata and extensions.
    #[serde(rename = "_daysOfWeek")]
    pub _days_of_week: Option<Element>,
    /// Always available? e.g. 24 hour service
    #[serde(rename = "allDay")]
    pub all_day: Option<BooleanType>,
    /// Extension element for the 'allDay' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allDay")]
    pub _all_day: Option<Element>,
    /// Opening time of day (ignored if allDay = true)
    #[serde(rename = "availableStartTime")]
    pub available_start_time: Option<TimeType>,
    /// Extension element for the 'availableStartTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availableStartTime")]
    pub _available_start_time: Option<Element>,
    /// Closing time of day (ignored if allDay = true)
    #[serde(rename = "availableEndTime")]
    pub available_end_time: Option<TimeType>,
    /// Extension element for the 'availableEndTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availableEndTime")]
    pub _available_end_time: Option<Element>,
}

impl Default for PractitionerRole {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            period: Default::default(),
            practitioner: Default::default(),
            organization: Default::default(),
            code: Default::default(),
            specialty: Default::default(),
            location: Default::default(),
            healthcare_service: Default::default(),
            telecom: Default::default(),
            available_time: Default::default(),
            not_available: Default::default(),
            availability_exceptions: Default::default(),
            _availability_exceptions: Default::default(),
            endpoint: Default::default(),
        }
    }
}

impl Default for PractitionerRoleNotavailable {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            during: Default::default(),
        }
    }
}

impl Default for PractitionerRoleAvailabletime {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            days_of_week: Default::default(),
            _days_of_week: Default::default(),
            all_day: Default::default(),
            _all_day: Default::default(),
            available_start_time: Default::default(),
            _available_start_time: Default::default(),
            available_end_time: Default::default(),
            _available_end_time: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PractitionerRole {
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

impl crate::traits::resource::ResourceMutators for PractitionerRole {
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

impl crate::traits::resource::ResourceExistence for PractitionerRole {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PractitionerRole {
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

impl crate::traits::domain_resource::DomainResourceMutators for PractitionerRole {
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

impl crate::traits::domain_resource::DomainResourceExistence for PractitionerRole {
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

impl crate::traits::practitioner_role::PractitionerRoleAccessors for PractitionerRole {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn practitioner(&self) -> Option<Reference> {
        self.practitioner.clone()
    }
    fn organization(&self) -> Option<Reference> {
        self.organization.clone()
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
    fn available_time(&self) -> &[PractitionerRoleAvailabletime] {
        self.available_time.as_deref().unwrap_or(&[])
    }
    fn not_available(&self) -> &[PractitionerRoleNotavailable] {
        self.not_available.as_deref().unwrap_or(&[])
    }
    fn availability_exceptions(&self) -> Option<StringType> {
        self.availability_exceptions.clone()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::practitioner_role::PractitionerRoleMutators for PractitionerRole {
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
    fn set_practitioner(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.practitioner = Some(value);
        resource
    }
    fn set_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.organization = Some(value);
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
    fn set_available_time(self, value: Vec<PractitionerRoleAvailabletime>) -> Self {
        let mut resource = self.clone();
        resource.available_time = Some(value);
        resource
    }
    fn add_available_time(self, item: PractitionerRoleAvailabletime) -> Self {
        let mut resource = self.clone();
        resource
            .available_time
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_not_available(self, value: Vec<PractitionerRoleNotavailable>) -> Self {
        let mut resource = self.clone();
        resource.not_available = Some(value);
        resource
    }
    fn add_not_available(self, item: PractitionerRoleNotavailable) -> Self {
        let mut resource = self.clone();
        resource
            .not_available
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_availability_exceptions(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.availability_exceptions = Some(value);
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

impl crate::traits::practitioner_role::PractitionerRoleExistence for PractitionerRole {
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
    fn has_practitioner(&self) -> bool {
        self.practitioner.is_some()
    }
    fn has_organization(&self) -> bool {
        self.organization.is_some()
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
    fn has_available_time(&self) -> bool {
        self.available_time.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_not_available(&self) -> bool {
        self.not_available.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_availability_exceptions(&self) -> bool {
        self.availability_exceptions.is_some()
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
}
