use crate::bindings::observation_status::ObservationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::resources::observation::ObservationComponent;
use crate::resources::observation::ObservationReferencerange;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Observation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Measurements and simple assertions made about a patient, device or other subject.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Observation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> ObservationStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[Reference];
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<InstantType>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[Reference];
    /// Returns a reference to the dataAbsentReason field.
    fn data_absent_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the interpretation field.
    fn interpretation(&self) -> &[CodeableConcept];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the method field.
    fn method(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> Option<Reference>;
    /// Returns a reference to the referenceRange field.
    fn reference_range(&self) -> &[ObservationReferencerange];
    /// Returns a reference to the hasMember field.
    fn has_member(&self) -> &[Reference];
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the component field.
    fn component(&self) -> &[ObservationComponent];
}
/// Observation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Measurements and simple assertions made about a patient, device or other subject.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Observation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationMutators: DomainResourceMutators {
    /// Create a new Observation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::observation::Observation;
    /// use hl7_fhir_r4_core::traits::observation::ObservationMutators;
    ///
    /// let resource = Observation::new();
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
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ObservationStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: Reference) -> Self;
    /// Sets the dataAbsentReason field and returns self for chaining.
    fn set_data_absent_reason(self, value: CodeableConcept) -> Self;
    /// Sets the interpretation field and returns self for chaining.
    fn set_interpretation(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the interpretation field and returns self for chaining.
    fn add_interpretation(self, item: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableConcept) -> Self;
    /// Sets the method field and returns self for chaining.
    fn set_method(self, value: CodeableConcept) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the referenceRange field and returns self for chaining.
    fn set_reference_range(self, value: Vec<ObservationReferencerange>) -> Self;
    /// Adds an item to the referenceRange field and returns self for chaining.
    fn add_reference_range(self, item: ObservationReferencerange) -> Self;
    /// Sets the hasMember field and returns self for chaining.
    fn set_has_member(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the hasMember field and returns self for chaining.
    fn add_has_member(self, item: Reference) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the component field and returns self for chaining.
    fn set_component(self, value: Vec<ObservationComponent>) -> Self;
    /// Adds an item to the component field and returns self for chaining.
    fn add_component(self, item: ObservationComponent) -> Self;
}
/// Observation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Measurements and simple assertions made about a patient, device or other subject.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Observation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Observation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationExistence: DomainResourceExistence {
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
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the effective field is present (Some).
    fn has_effective(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the value field is present (Some).
    fn has_value(&self) -> bool;
    /// Returns true if the data_absent_reason field is present (Some).
    fn has_data_absent_reason(&self) -> bool;
    /// Returns true if the interpretation field is not empty.
    fn has_interpretation(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the method field is present (Some).
    fn has_method(&self) -> bool;
    /// Returns true if the specimen field is present (Some).
    fn has_specimen(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the reference_range field is not empty.
    fn has_reference_range(&self) -> bool;
    /// Returns true if the has_member field is not empty.
    fn has_has_member(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the component field is not empty.
    fn has_component(&self) -> bool;
}
