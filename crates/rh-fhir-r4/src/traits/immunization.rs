use crate::bindings::immunization_status::ImmunizationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::immunization::ImmunizationEducation;
use crate::resources::immunization::ImmunizationPerformer;
use crate::resources::immunization::ImmunizationProtocolapplied;
use crate::resources::immunization::ImmunizationReaction;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Immunization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Immunization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Immunization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ImmunizationStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the vaccineCode field.
    fn vaccine_code(&self) -> CodeableConcept;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recorded field.
    fn recorded(&self) -> Option<DateTimeType>;
    /// Returns a reference to the primarySource field.
    fn primary_source(&self) -> Option<BooleanType>;
    /// Returns a reference to the reportOrigin field.
    fn report_origin(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> Option<Reference>;
    /// Returns a reference to the lotNumber field.
    fn lot_number(&self) -> Option<StringType>;
    /// Returns a reference to the expirationDate field.
    fn expiration_date(&self) -> Option<StringType>;
    /// Returns a reference to the site field.
    fn site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the route field.
    fn route(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the doseQuantity field.
    fn dose_quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[ImmunizationPerformer];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the isSubpotent field.
    fn is_subpotent(&self) -> Option<BooleanType>;
    /// Returns a reference to the subpotentReason field.
    fn subpotent_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the education field.
    fn education(&self) -> &[ImmunizationEducation];
    /// Returns a reference to the programEligibility field.
    fn program_eligibility(&self) -> &[CodeableConcept];
    /// Returns a reference to the fundingSource field.
    fn funding_source(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the reaction field.
    fn reaction(&self) -> &[ImmunizationReaction];
    /// Returns a reference to the protocolApplied field.
    fn protocol_applied(&self) -> &[ImmunizationProtocolapplied];
}
/// Immunization Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Immunization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Immunization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationMutators: DomainResourceMutators {
    /// Create a new Immunization with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::immunization::Immunization;
    /// use hl7_fhir_r4_core::traits::immunization::ImmunizationMutators;
    ///
    /// let resource = Immunization::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ImmunizationStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the vaccineCode field and returns self for chaining.
    fn set_vaccine_code(self, value: CodeableConcept) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recorded field and returns self for chaining.
    fn set_recorded(self, value: String) -> Self;
    /// Sets the primarySource field and returns self for chaining.
    fn set_primary_source(self, value: bool) -> Self;
    /// Sets the reportOrigin field and returns self for chaining.
    fn set_report_origin(self, value: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Reference) -> Self;
    /// Sets the lotNumber field and returns self for chaining.
    fn set_lot_number(self, value: String) -> Self;
    /// Sets the expirationDate field and returns self for chaining.
    fn set_expiration_date(self, value: String) -> Self;
    /// Sets the site field and returns self for chaining.
    fn set_site(self, value: CodeableConcept) -> Self;
    /// Sets the route field and returns self for chaining.
    fn set_route(self, value: CodeableConcept) -> Self;
    /// Sets the doseQuantity field and returns self for chaining.
    fn set_dose_quantity(self, value: Quantity) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<ImmunizationPerformer>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: ImmunizationPerformer) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the isSubpotent field and returns self for chaining.
    fn set_is_subpotent(self, value: bool) -> Self;
    /// Sets the subpotentReason field and returns self for chaining.
    fn set_subpotent_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the subpotentReason field and returns self for chaining.
    fn add_subpotent_reason(self, item: CodeableConcept) -> Self;
    /// Sets the education field and returns self for chaining.
    fn set_education(self, value: Vec<ImmunizationEducation>) -> Self;
    /// Adds an item to the education field and returns self for chaining.
    fn add_education(self, item: ImmunizationEducation) -> Self;
    /// Sets the programEligibility field and returns self for chaining.
    fn set_program_eligibility(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the programEligibility field and returns self for chaining.
    fn add_program_eligibility(self, item: CodeableConcept) -> Self;
    /// Sets the fundingSource field and returns self for chaining.
    fn set_funding_source(self, value: CodeableConcept) -> Self;
    /// Sets the reaction field and returns self for chaining.
    fn set_reaction(self, value: Vec<ImmunizationReaction>) -> Self;
    /// Adds an item to the reaction field and returns self for chaining.
    fn add_reaction(self, item: ImmunizationReaction) -> Self;
    /// Sets the protocolApplied field and returns self for chaining.
    fn set_protocol_applied(self, value: Vec<ImmunizationProtocolapplied>) -> Self;
    /// Adds an item to the protocolApplied field and returns self for chaining.
    fn add_protocol_applied(self, item: ImmunizationProtocolapplied) -> Self;
}
/// Immunization Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Immunization
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Immunization
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the vaccine_code field is present (Some).
    fn has_vaccine_code(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence field is present (Some).
    fn has_occurrence(&self) -> bool;
    /// Returns true if the recorded field is present (Some).
    fn has_recorded(&self) -> bool;
    /// Returns true if the primary_source field is present (Some).
    fn has_primary_source(&self) -> bool;
    /// Returns true if the report_origin field is present (Some).
    fn has_report_origin(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the manufacturer field is present (Some).
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the lot_number field is present (Some).
    fn has_lot_number(&self) -> bool;
    /// Returns true if the expiration_date field is present (Some).
    fn has_expiration_date(&self) -> bool;
    /// Returns true if the site field is present (Some).
    fn has_site(&self) -> bool;
    /// Returns true if the route field is present (Some).
    fn has_route(&self) -> bool;
    /// Returns true if the dose_quantity field is present (Some).
    fn has_dose_quantity(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the is_subpotent field is present (Some).
    fn has_is_subpotent(&self) -> bool;
    /// Returns true if the subpotent_reason field is not empty.
    fn has_subpotent_reason(&self) -> bool;
    /// Returns true if the education field is not empty.
    fn has_education(&self) -> bool;
    /// Returns true if the program_eligibility field is not empty.
    fn has_program_eligibility(&self) -> bool;
    /// Returns true if the funding_source field is present (Some).
    fn has_funding_source(&self) -> bool;
    /// Returns true if the reaction field is not empty.
    fn has_reaction(&self) -> bool;
    /// Returns true if the protocol_applied field is not empty.
    fn has_protocol_applied(&self) -> bool;
}
