use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::procedure::ProcedureFocaldevice;
use crate::resources::procedure::ProcedurePerformer;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Procedure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Procedure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProcedureAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> EventStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the focus field.
    fn focus(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[ProcedurePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableConcept];
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the report field.
    fn report(&self) -> &[Reference];
    /// Returns a reference to the complication field.
    fn complication(&self) -> &[CodeableReference];
    /// Returns a reference to the followUp field.
    fn follow_up(&self) -> &[CodeableConcept];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the focalDevice field.
    fn focal_device(&self) -> &[ProcedureFocaldevice];
    /// Returns a reference to the used field.
    fn used(&self) -> &[CodeableReference];
    /// Returns a reference to the supportingInfo field.
    fn supporting_info(&self) -> &[Reference];
}
/// Procedure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Procedure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProcedureMutators: DomainResourceMutators {
    /// Create a new Procedure with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::procedure::Procedure;
    /// use rh_hl7_fhir_r5_core::traits::procedure::ProcedureMutators;
    ///
    /// let resource = Procedure::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesCanonical field and returns self for chaining.
    fn add_instantiates_canonical(self, item: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesUri field and returns self for chaining.
    fn add_instantiates_uri(self, item: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: EventStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<ProcedurePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: ProcedurePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the bodySite field and returns self for chaining.
    fn add_body_site(self, item: CodeableConcept) -> Self;
    /// Sets the outcome field and returns self for chaining.
    fn set_outcome(self, value: CodeableConcept) -> Self;
    /// Sets the report field and returns self for chaining.
    fn set_report(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the report field and returns self for chaining.
    fn add_report(self, item: Reference) -> Self;
    /// Sets the complication field and returns self for chaining.
    fn set_complication(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the complication field and returns self for chaining.
    fn add_complication(self, item: CodeableReference) -> Self;
    /// Sets the followUp field and returns self for chaining.
    fn set_follow_up(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the followUp field and returns self for chaining.
    fn add_follow_up(self, item: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the focalDevice field and returns self for chaining.
    fn set_focal_device(self, value: Vec<ProcedureFocaldevice>) -> Self;
    /// Adds an item to the focalDevice field and returns self for chaining.
    fn add_focal_device(self, item: ProcedureFocaldevice) -> Self;
    /// Sets the used field and returns self for chaining.
    fn set_used(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the used field and returns self for chaining.
    fn add_used(self, item: CodeableReference) -> Self;
    /// Sets the supportingInfo field and returns self for chaining.
    fn set_supporting_info(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInfo field and returns self for chaining.
    fn add_supporting_info(self, item: Reference) -> Self;
}
/// Procedure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Procedure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProcedureExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the instantiates_canonical field is not empty.
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is not empty.
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the focus field is present (Some).
    fn has_focus(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the reported field is present (Some).
    fn has_reported(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the report field is not empty.
    fn has_report(&self) -> bool;
    /// Returns true if the complication field is not empty.
    fn has_complication(&self) -> bool;
    /// Returns true if the follow_up field is not empty.
    fn has_follow_up(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the focal_device field is not empty.
    fn has_focal_device(&self) -> bool;
    /// Returns true if the used field is not empty.
    fn has_used(&self) -> bool;
    /// Returns true if the supporting_info field is not empty.
    fn has_supporting_info(&self) -> bool;
}
