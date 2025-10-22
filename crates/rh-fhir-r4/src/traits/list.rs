use crate::bindings::list_mode::ListMode;
use crate::bindings::list_status::ListStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::list::ListEntry;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// List Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A list is a curated collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/List
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: List
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ListAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ListStatus;
    /// Returns a reference to the mode field.
    fn mode(&self) -> ListMode;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the source field.
    fn source(&self) -> Option<Reference>;
    /// Returns a reference to the orderedBy field.
    fn ordered_by(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the entry field.
    fn entry(&self) -> &[ListEntry];
    /// Returns a reference to the emptyReason field.
    fn empty_reason(&self) -> Option<CodeableConcept>;
}
/// List Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A list is a curated collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/List
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: List
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ListMutators: DomainResourceMutators {
    /// Create a new List with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::list::List;
    /// use hl7_fhir_r4_core::traits::list::ListMutators;
    ///
    /// let resource = List::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ListStatus) -> Self;
    /// Sets the mode field and returns self for chaining.
    fn set_mode(self, value: ListMode) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: Reference) -> Self;
    /// Sets the orderedBy field and returns self for chaining.
    fn set_ordered_by(self, value: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the entry field and returns self for chaining.
    fn set_entry(self, value: Vec<ListEntry>) -> Self;
    /// Adds an item to the entry field and returns self for chaining.
    fn add_entry(self, item: ListEntry) -> Self;
    /// Sets the emptyReason field and returns self for chaining.
    fn set_empty_reason(self, value: CodeableConcept) -> Self;
}
/// List Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A list is a curated collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/List
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: List
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ListExistence: DomainResourceExistence {
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
    /// Returns true if the mode field is present (Some).
    fn has_mode(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the ordered_by field is present (Some).
    fn has_ordered_by(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the entry field is not empty.
    fn has_entry(&self) -> bool;
    /// Returns true if the empty_reason field is present (Some).
    fn has_empty_reason(&self) -> bool;
}
