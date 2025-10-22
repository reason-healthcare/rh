use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::healthcare_service::HealthcareServiceAvailabletime;
use crate::resources::healthcare_service::HealthcareServiceEligibility;
use crate::resources::healthcare_service::HealthcareServiceNotavailable;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// HealthcareService Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The details of a healthcare service available at a location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: HealthcareService
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait HealthcareServiceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the providedBy field.
    fn provided_by(&self) -> Option<Reference>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the type field.
    fn type_(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialty field.
    fn specialty(&self) -> &[CodeableConcept];
    /// Returns a reference to the location field.
    fn location(&self) -> &[Reference];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the extraDetails field.
    fn extra_details(&self) -> Option<StringType>;
    /// Returns a reference to the photo field.
    fn photo(&self) -> Option<Attachment>;
    /// Returns a reference to the telecom field.
    fn telecom(&self) -> &[ContactPoint];
    /// Returns a reference to the coverageArea field.
    fn coverage_area(&self) -> &[Reference];
    /// Returns a reference to the serviceProvisionCode field.
    fn service_provision_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the eligibility field.
    fn eligibility(&self) -> &[HealthcareServiceEligibility];
    /// Returns a reference to the program field.
    fn program(&self) -> &[CodeableConcept];
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[CodeableConcept];
    /// Returns a reference to the communication field.
    fn communication(&self) -> &[CodeableConcept];
    /// Returns a reference to the referralMethod field.
    fn referral_method(&self) -> &[CodeableConcept];
    /// Returns a reference to the appointmentRequired field.
    fn appointment_required(&self) -> Option<BooleanType>;
    /// Returns a reference to the availableTime field.
    fn available_time(&self) -> &[HealthcareServiceAvailabletime];
    /// Returns a reference to the notAvailable field.
    fn not_available(&self) -> &[HealthcareServiceNotavailable];
    /// Returns a reference to the availabilityExceptions field.
    fn availability_exceptions(&self) -> Option<StringType>;
    /// Returns a reference to the endpoint field.
    fn endpoint(&self) -> &[Reference];
}
/// HealthcareService Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The details of a healthcare service available at a location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: HealthcareService
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait HealthcareServiceMutators: DomainResourceMutators {
    /// Create a new HealthcareService with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::healthcare_service::HealthcareService;
    /// use hl7_fhir_r4_core::traits::healthcare_service::HealthcareServiceMutators;
    ///
    /// let resource = HealthcareService::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the active field and returns self for chaining.
    fn set_active(self, value: bool) -> Self;
    /// Sets the providedBy field and returns self for chaining.
    fn set_provided_by(self, value: Reference) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the type field and returns self for chaining.
    fn add_type_(self, item: CodeableConcept) -> Self;
    /// Sets the specialty field and returns self for chaining.
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialty field and returns self for chaining.
    fn add_specialty(self, item: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the location field and returns self for chaining.
    fn add_location(self, item: Reference) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the extraDetails field and returns self for chaining.
    fn set_extra_details(self, value: String) -> Self;
    /// Sets the photo field and returns self for chaining.
    fn set_photo(self, value: Attachment) -> Self;
    /// Sets the telecom field and returns self for chaining.
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the telecom field and returns self for chaining.
    fn add_telecom(self, item: ContactPoint) -> Self;
    /// Sets the coverageArea field and returns self for chaining.
    fn set_coverage_area(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the coverageArea field and returns self for chaining.
    fn add_coverage_area(self, item: Reference) -> Self;
    /// Sets the serviceProvisionCode field and returns self for chaining.
    fn set_service_provision_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the serviceProvisionCode field and returns self for chaining.
    fn add_service_provision_code(self, item: CodeableConcept) -> Self;
    /// Sets the eligibility field and returns self for chaining.
    fn set_eligibility(self, value: Vec<HealthcareServiceEligibility>) -> Self;
    /// Adds an item to the eligibility field and returns self for chaining.
    fn add_eligibility(self, item: HealthcareServiceEligibility) -> Self;
    /// Sets the program field and returns self for chaining.
    fn set_program(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the program field and returns self for chaining.
    fn add_program(self, item: CodeableConcept) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: CodeableConcept) -> Self;
    /// Sets the communication field and returns self for chaining.
    fn set_communication(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the communication field and returns self for chaining.
    fn add_communication(self, item: CodeableConcept) -> Self;
    /// Sets the referralMethod field and returns self for chaining.
    fn set_referral_method(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the referralMethod field and returns self for chaining.
    fn add_referral_method(self, item: CodeableConcept) -> Self;
    /// Sets the appointmentRequired field and returns self for chaining.
    fn set_appointment_required(self, value: bool) -> Self;
    /// Sets the availableTime field and returns self for chaining.
    fn set_available_time(self, value: Vec<HealthcareServiceAvailabletime>) -> Self;
    /// Adds an item to the availableTime field and returns self for chaining.
    fn add_available_time(self, item: HealthcareServiceAvailabletime) -> Self;
    /// Sets the notAvailable field and returns self for chaining.
    fn set_not_available(self, value: Vec<HealthcareServiceNotavailable>) -> Self;
    /// Adds an item to the notAvailable field and returns self for chaining.
    fn add_not_available(self, item: HealthcareServiceNotavailable) -> Self;
    /// Sets the availabilityExceptions field and returns self for chaining.
    fn set_availability_exceptions(self, value: String) -> Self;
    /// Sets the endpoint field and returns self for chaining.
    fn set_endpoint(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the endpoint field and returns self for chaining.
    fn add_endpoint(self, item: Reference) -> Self;
}
/// HealthcareService Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The details of a healthcare service available at a location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: HealthcareService
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait HealthcareServiceExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the active field is present (Some).
    fn has_active(&self) -> bool;
    /// Returns true if the provided_by field is present (Some).
    fn has_provided_by(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the type_ field is not empty.
    fn has_type_(&self) -> bool;
    /// Returns true if the specialty field is not empty.
    fn has_specialty(&self) -> bool;
    /// Returns true if the location field is not empty.
    fn has_location(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the extra_details field is present (Some).
    fn has_extra_details(&self) -> bool;
    /// Returns true if the photo field is present (Some).
    fn has_photo(&self) -> bool;
    /// Returns true if the telecom field is not empty.
    fn has_telecom(&self) -> bool;
    /// Returns true if the coverage_area field is not empty.
    fn has_coverage_area(&self) -> bool;
    /// Returns true if the service_provision_code field is not empty.
    fn has_service_provision_code(&self) -> bool;
    /// Returns true if the eligibility field is not empty.
    fn has_eligibility(&self) -> bool;
    /// Returns true if the program field is not empty.
    fn has_program(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the communication field is not empty.
    fn has_communication(&self) -> bool;
    /// Returns true if the referral_method field is not empty.
    fn has_referral_method(&self) -> bool;
    /// Returns true if the appointment_required field is present (Some).
    fn has_appointment_required(&self) -> bool;
    /// Returns true if the available_time field is not empty.
    fn has_available_time(&self) -> bool;
    /// Returns true if the not_available field is not empty.
    fn has_not_available(&self) -> bool;
    /// Returns true if the availability_exceptions field is present (Some).
    fn has_availability_exceptions(&self) -> bool;
    /// Returns true if the endpoint field is not empty.
    fn has_endpoint(&self) -> bool;
}
