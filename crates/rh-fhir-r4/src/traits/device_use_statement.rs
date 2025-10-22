use crate::bindings::device_statement_status::DeviceStatementStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceUseStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or another clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUseStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceUseStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUseStatementAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> DeviceStatementStatus;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the recordedOn field.
    fn recorded_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the source field.
    fn source(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> Reference;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// DeviceUseStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or another clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUseStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceUseStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUseStatementMutators: DomainResourceMutators {
    /// Create a new DeviceUseStatement with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::device_use_statement::DeviceUseStatement;
    /// use hl7_fhir_r4_core::traits::device_use_statement::DeviceUseStatementMutators;
    ///
    /// let resource = DeviceUseStatement::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DeviceStatementStatus) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the recordedOn field and returns self for chaining.
    fn set_recorded_on(self, value: String) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// DeviceUseStatement Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or another clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUseStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceUseStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceUseStatementExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the timing field is present (Some).
    fn has_timing(&self) -> bool;
    /// Returns true if the recorded_on field is present (Some).
    fn has_recorded_on(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
