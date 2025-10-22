use crate::bindings::event_status::EventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
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
/// An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 4.0.1
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
    fn category(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the asserter field.
    fn asserter(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[ProcedurePerformer];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableConcept];
    /// Returns a reference to the outcome field.
    fn outcome(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the report field.
    fn report(&self) -> &[Reference];
    /// Returns a reference to the complication field.
    fn complication(&self) -> &[CodeableConcept];
    /// Returns a reference to the complicationDetail field.
    fn complication_detail(&self) -> &[Reference];
    /// Returns a reference to the followUp field.
    fn follow_up(&self) -> &[CodeableConcept];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the focalDevice field.
    fn focal_device(&self) -> &[ProcedureFocaldevice];
    /// Returns a reference to the usedReference field.
    fn used_reference(&self) -> &[Reference];
    /// Returns a reference to the usedCode field.
    fn used_code(&self) -> &[CodeableConcept];
}
/// Procedure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::procedure::Procedure;
    /// use hl7_fhir_r4_core::traits::procedure::ProcedureMutators;
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
    fn set_category(self, value: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the asserter field and returns self for chaining.
    fn set_asserter(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<ProcedurePerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: ProcedurePerformer) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
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
    fn set_complication(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the complication field and returns self for chaining.
    fn add_complication(self, item: CodeableConcept) -> Self;
    /// Sets the complicationDetail field and returns self for chaining.
    fn set_complication_detail(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the complicationDetail field and returns self for chaining.
    fn add_complication_detail(self, item: Reference) -> Self;
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
    /// Sets the usedReference field and returns self for chaining.
    fn set_used_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the usedReference field and returns self for chaining.
    fn add_used_reference(self, item: Reference) -> Self;
    /// Sets the usedCode field and returns self for chaining.
    fn set_used_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the usedCode field and returns self for chaining.
    fn add_used_code(self, item: CodeableConcept) -> Self;
}
/// Procedure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Procedure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Procedure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ProcedureExistence: DomainResourceExistence {
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
    /// Returns true if the category field is present (Some).
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the performed field is present (Some).
    fn has_performed(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the asserter field is present (Some).
    fn has_asserter(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the outcome field is present (Some).
    fn has_outcome(&self) -> bool;
    /// Returns true if the report field is not empty.
    fn has_report(&self) -> bool;
    /// Returns true if the complication field is not empty.
    fn has_complication(&self) -> bool;
    /// Returns true if the complication_detail field is not empty.
    fn has_complication_detail(&self) -> bool;
    /// Returns true if the follow_up field is not empty.
    fn has_follow_up(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the focal_device field is not empty.
    fn has_focal_device(&self) -> bool;
    /// Returns true if the used_reference field is not empty.
    fn has_used_reference(&self) -> bool;
    /// Returns true if the used_code field is not empty.
    fn has_used_code(&self) -> bool;
}
