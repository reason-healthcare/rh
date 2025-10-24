use crate::bindings::days_of_week::DaysOfWeek;
use crate::datatypes::attachment::Attachment;
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
/// HealthcareService
///
/// The details of a healthcare service available at a location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: HealthcareService
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareService {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifiers for this item
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this HealthcareService record is in active use
    pub active: Option<BooleanType>,
    /// Extension element for the 'active' primitive field. Contains metadata and extensions.
    pub _active: Option<Element>,
    /// Organization that provides this service
    #[serde(rename = "providedBy")]
    pub provided_by: Option<Reference>,
    /// Broad category of service being performed or delivered
    ///
    /// Binding: example (A category of the service(s) that could be provided.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of service that may be delivered or performed
    ///
    /// Binding: example (Additional details about where the content was created (e.g. clinical specialty).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Specialties handled by the HealthcareService
    ///
    /// Binding: preferred (A specialty that a healthcare service may provide.)
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
    /// Location(s) where service may be provided
    pub location: Option<Vec<Reference>>,
    /// Description of service as presented to a consumer while searching
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Extra details about the service that can't be placed in the other fields
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<StringType>,
    /// Extension element for the 'extraDetails' primitive field. Contains metadata and extensions.
    #[serde(rename = "_extraDetails")]
    pub _extra_details: Option<Element>,
    /// Facilitates quick identification of the service
    pub photo: Option<Attachment>,
    /// Contacts related to the healthcare service
    pub telecom: Option<Vec<ContactPoint>>,
    /// Location(s) service is intended for/available to
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Reference>>,
    /// Conditions under which service is available/offered
    ///
    /// Binding: example (The code(s) that detail the conditions under which the healthcare service is available/offered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-provision-conditions
    #[serde(rename = "serviceProvisionCode")]
    pub service_provision_code: Option<Vec<CodeableConcept>>,
    /// Specific eligibility requirements required to use the service
    pub eligibility: Option<Vec<HealthcareServiceEligibility>>,
    /// Programs that this service is applicable to
    ///
    /// Binding: example (Government or local programs that this service applies to.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/program
    pub program: Option<Vec<CodeableConcept>>,
    /// Collection of characteristics (attributes)
    ///
    /// Binding: example (A custom attribute that could be provided at a service (e.g. Wheelchair accessibiliy).)
    pub characteristic: Option<Vec<CodeableConcept>>,
    /// The language that this service is offered in
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
    /// Ways that the service accepts referrals
    ///
    /// Binding: example (The methods of referral can be used when referring to a specific HealthCareService resource.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-referral-method
    #[serde(rename = "referralMethod")]
    pub referral_method: Option<Vec<CodeableConcept>>,
    /// If an appointment is required for access to this service
    #[serde(rename = "appointmentRequired")]
    pub appointment_required: Option<BooleanType>,
    /// Extension element for the 'appointmentRequired' primitive field. Contains metadata and extensions.
    #[serde(rename = "_appointmentRequired")]
    pub _appointment_required: Option<Element>,
    /// Times the Service Site is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<HealthcareServiceAvailabletime>>,
    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailable")]
    pub not_available: Option<Vec<HealthcareServiceNotavailable>>,
    /// Description of availability exceptions
    #[serde(rename = "availabilityExceptions")]
    pub availability_exceptions: Option<StringType>,
    /// Extension element for the 'availabilityExceptions' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availabilityExceptions")]
    pub _availability_exceptions: Option<Element>,
    /// Technical endpoints providing access to electronic services operated for the healthcare service
    pub endpoint: Option<Vec<Reference>>,
}
/// HealthcareService nested structure for the 'eligibility' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareServiceEligibility {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Coded value for the eligibility
    ///
    /// Binding: example (Coded values underwhich a specific service is made available.)
    pub code: Option<CodeableConcept>,
    /// Describes the eligibility conditions for the service
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}
/// HealthcareService nested structure for the 'notAvailable' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareServiceNotavailable {
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
/// HealthcareService nested structure for the 'availableTime' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareServiceAvailabletime {
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

impl Default for HealthcareService {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            active: Default::default(),
            _active: Default::default(),
            provided_by: Default::default(),
            category: Default::default(),
            type_: Default::default(),
            specialty: Default::default(),
            location: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            extra_details: Default::default(),
            _extra_details: Default::default(),
            photo: Default::default(),
            telecom: Default::default(),
            coverage_area: Default::default(),
            service_provision_code: Default::default(),
            eligibility: Default::default(),
            program: Default::default(),
            characteristic: Default::default(),
            communication: Default::default(),
            referral_method: Default::default(),
            appointment_required: Default::default(),
            _appointment_required: Default::default(),
            available_time: Default::default(),
            not_available: Default::default(),
            availability_exceptions: Default::default(),
            _availability_exceptions: Default::default(),
            endpoint: Default::default(),
        }
    }
}

impl Default for HealthcareServiceEligibility {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
        }
    }
}

impl Default for HealthcareServiceNotavailable {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            during: Default::default(),
        }
    }
}

impl Default for HealthcareServiceAvailabletime {
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
impl crate::traits::resource::ResourceAccessors for HealthcareService {
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

impl crate::traits::resource::ResourceMutators for HealthcareService {
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

impl crate::traits::resource::ResourceExistence for HealthcareService {
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

impl crate::traits::domain_resource::DomainResourceAccessors for HealthcareService {
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

impl crate::traits::domain_resource::DomainResourceMutators for HealthcareService {
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

impl crate::traits::domain_resource::DomainResourceExistence for HealthcareService {
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

impl crate::traits::healthcare_service::HealthcareServiceAccessors for HealthcareService {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn active(&self) -> Option<BooleanType> {
        self.active
    }
    fn provided_by(&self) -> Option<Reference> {
        self.provided_by.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> &[Reference] {
        self.location.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn extra_details(&self) -> Option<StringType> {
        self.extra_details.clone()
    }
    fn photo(&self) -> Option<Attachment> {
        self.photo.clone()
    }
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn coverage_area(&self) -> &[Reference] {
        self.coverage_area.as_deref().unwrap_or(&[])
    }
    fn service_provision_code(&self) -> &[CodeableConcept] {
        self.service_provision_code.as_deref().unwrap_or(&[])
    }
    fn eligibility(&self) -> &[HealthcareServiceEligibility] {
        self.eligibility.as_deref().unwrap_or(&[])
    }
    fn program(&self) -> &[CodeableConcept] {
        self.program.as_deref().unwrap_or(&[])
    }
    fn characteristic(&self) -> &[CodeableConcept] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
    fn communication(&self) -> &[CodeableConcept] {
        self.communication.as_deref().unwrap_or(&[])
    }
    fn referral_method(&self) -> &[CodeableConcept] {
        self.referral_method.as_deref().unwrap_or(&[])
    }
    fn appointment_required(&self) -> Option<BooleanType> {
        self.appointment_required
    }
    fn available_time(&self) -> &[HealthcareServiceAvailabletime] {
        self.available_time.as_deref().unwrap_or(&[])
    }
    fn not_available(&self) -> &[HealthcareServiceNotavailable] {
        self.not_available.as_deref().unwrap_or(&[])
    }
    fn availability_exceptions(&self) -> Option<StringType> {
        self.availability_exceptions.clone()
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::healthcare_service::HealthcareServiceMutators for HealthcareService {
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
    fn set_provided_by(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.provided_by = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
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
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_extra_details(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.extra_details = Some(value);
        resource
    }
    fn set_photo(self, value: Attachment) -> Self {
        let mut resource = self.clone();
        resource.photo = Some(value);
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
    fn set_service_provision_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_provision_code = Some(value);
        resource
    }
    fn add_service_provision_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_provision_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_eligibility(self, value: Vec<HealthcareServiceEligibility>) -> Self {
        let mut resource = self.clone();
        resource.eligibility = Some(value);
        resource
    }
    fn add_eligibility(self, item: HealthcareServiceEligibility) -> Self {
        let mut resource = self.clone();
        resource.eligibility.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_program(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.program = Some(value);
        resource
    }
    fn add_program(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.program.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_characteristic(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
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
    fn set_referral_method(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.referral_method = Some(value);
        resource
    }
    fn add_referral_method(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .referral_method
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_appointment_required(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.appointment_required = Some(value);
        resource
    }
    fn set_available_time(self, value: Vec<HealthcareServiceAvailabletime>) -> Self {
        let mut resource = self.clone();
        resource.available_time = Some(value);
        resource
    }
    fn add_available_time(self, item: HealthcareServiceAvailabletime) -> Self {
        let mut resource = self.clone();
        resource
            .available_time
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_not_available(self, value: Vec<HealthcareServiceNotavailable>) -> Self {
        let mut resource = self.clone();
        resource.not_available = Some(value);
        resource
    }
    fn add_not_available(self, item: HealthcareServiceNotavailable) -> Self {
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

impl crate::traits::healthcare_service::HealthcareServiceExistence for HealthcareService {
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
    fn has_provided_by(&self) -> bool {
        self.provided_by.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specialty(&self) -> bool {
        self.specialty.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_extra_details(&self) -> bool {
        self.extra_details.is_some()
    }
    fn has_photo(&self) -> bool {
        self.photo.is_some()
    }
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_coverage_area(&self) -> bool {
        self.coverage_area.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_provision_code(&self) -> bool {
        self.service_provision_code
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_eligibility(&self) -> bool {
        self.eligibility.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_program(&self) -> bool {
        self.program.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_communication(&self) -> bool {
        self.communication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_referral_method(&self) -> bool {
        self.referral_method.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_appointment_required(&self) -> bool {
        self.appointment_required.is_some()
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

impl crate::validation::ValidatableResource for HealthcareService {
    fn resource_type(&self) -> &'static str {
        "HealthcareService"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/HealthcareService")
    }
}
