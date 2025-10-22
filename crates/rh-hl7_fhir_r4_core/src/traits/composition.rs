use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::v3confidentiality_classification::V3ConfidentialityClassification;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::composition::CompositionAttester;
use crate::resources::composition::CompositionEvent;
use crate::resources::composition::CompositionRelatesto;
use crate::resources::composition::CompositionSection;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Composition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Composition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompositionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> CompositionStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> CodeableConcept;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the author field.
    fn author(&self) -> &[Reference];
    /// Returns a reference to the title field.
    fn title(&self) -> StringType;
    /// Returns a reference to the confidentiality field.
    fn confidentiality(&self) -> Option<V3ConfidentialityClassification>;
    /// Returns a reference to the attester field.
    fn attester(&self) -> &[CompositionAttester];
    /// Returns a reference to the custodian field.
    fn custodian(&self) -> Option<Reference>;
    /// Returns a reference to the relatesTo field.
    fn relates_to(&self) -> &[CompositionRelatesto];
    /// Returns a reference to the event field.
    fn event(&self) -> &[CompositionEvent];
    /// Returns a reference to the section field.
    fn section(&self) -> &[CompositionSection];
}
/// Composition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Composition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompositionMutators: DomainResourceMutators {
    /// Create a new Composition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::composition::Composition;
    /// use hl7_fhir_r4_core::traits::composition::CompositionMutators;
    ///
    /// let resource = Composition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CompositionStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: Reference) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the confidentiality field and returns self for chaining.
    fn set_confidentiality(self, value: V3ConfidentialityClassification) -> Self;
    /// Sets the attester field and returns self for chaining.
    fn set_attester(self, value: Vec<CompositionAttester>) -> Self;
    /// Adds an item to the attester field and returns self for chaining.
    fn add_attester(self, item: CompositionAttester) -> Self;
    /// Sets the custodian field and returns self for chaining.
    fn set_custodian(self, value: Reference) -> Self;
    /// Sets the relatesTo field and returns self for chaining.
    fn set_relates_to(self, value: Vec<CompositionRelatesto>) -> Self;
    /// Adds an item to the relatesTo field and returns self for chaining.
    fn add_relates_to(self, item: CompositionRelatesto) -> Self;
    /// Sets the event field and returns self for chaining.
    fn set_event(self, value: Vec<CompositionEvent>) -> Self;
    /// Adds an item to the event field and returns self for chaining.
    fn add_event(self, item: CompositionEvent) -> Self;
    /// Sets the section field and returns self for chaining.
    fn set_section(self, value: Vec<CompositionSection>) -> Self;
    /// Adds an item to the section field and returns self for chaining.
    fn add_section(self, item: CompositionSection) -> Self;
}
/// Composition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Composition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CompositionExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the confidentiality field is present (Some).
    fn has_confidentiality(&self) -> bool;
    /// Returns true if the attester field is not empty.
    fn has_attester(&self) -> bool;
    /// Returns true if the custodian field is present (Some).
    fn has_custodian(&self) -> bool;
    /// Returns true if the relates_to field is not empty.
    fn has_relates_to(&self) -> bool;
    /// Returns true if the event field is not empty.
    fn has_event(&self) -> bool;
    /// Returns true if the section field is not empty.
    fn has_section(&self) -> bool;
}
