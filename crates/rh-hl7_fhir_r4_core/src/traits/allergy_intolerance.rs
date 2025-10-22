use crate::bindings::allergy_intolerance_category::AllergyIntoleranceCategory;
use crate::bindings::allergy_intolerance_criticality::AllergyIntoleranceCriticality;
use crate::bindings::allergy_intolerance_type::AllergyIntoleranceType;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::allergy_intolerance::AllergyIntoleranceReaction;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AllergyIntolerance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AllergyIntolerance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AllergyIntoleranceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the clinicalStatus field.
    fn clinical_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the verificationStatus field.
    fn verification_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<AllergyIntoleranceType>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[AllergyIntoleranceCategory];
    /// Returns a reference to the criticality field.
    fn criticality(&self) -> Option<AllergyIntoleranceCriticality>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the recordedDate field.
    fn recorded_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the asserter field.
    fn asserter(&self) -> Option<Reference>;
    /// Returns a reference to the lastOccurrence field.
    fn last_occurrence(&self) -> Option<DateTimeType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the reaction field.
    fn reaction(&self) -> &[AllergyIntoleranceReaction];
}
/// AllergyIntolerance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AllergyIntolerance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AllergyIntoleranceMutators: DomainResourceMutators {
    /// Create a new AllergyIntolerance with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::allergy_intolerance::AllergyIntolerance;
    /// use hl7_fhir_r4_core::traits::allergy_intolerance::AllergyIntoleranceMutators;
    ///
    /// let resource = AllergyIntolerance::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the clinicalStatus field and returns self for chaining.
    fn set_clinical_status(self, value: CodeableConcept) -> Self;
    /// Sets the verificationStatus field and returns self for chaining.
    fn set_verification_status(self, value: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: AllergyIntoleranceType) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<AllergyIntoleranceCategory>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: AllergyIntoleranceCategory) -> Self;
    /// Sets the criticality field and returns self for chaining.
    fn set_criticality(self, value: AllergyIntoleranceCriticality) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the recordedDate field and returns self for chaining.
    fn set_recorded_date(self, value: String) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the asserter field and returns self for chaining.
    fn set_asserter(self, value: Reference) -> Self;
    /// Sets the lastOccurrence field and returns self for chaining.
    fn set_last_occurrence(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the reaction field and returns self for chaining.
    fn set_reaction(self, value: Vec<AllergyIntoleranceReaction>) -> Self;
    /// Adds an item to the reaction field and returns self for chaining.
    fn add_reaction(self, item: AllergyIntoleranceReaction) -> Self;
}
/// AllergyIntolerance Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AllergyIntolerance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AllergyIntoleranceExistence: DomainResourceExistence {
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
    /// Returns true if the clinical_status field is present (Some).
    fn has_clinical_status(&self) -> bool;
    /// Returns true if the verification_status field is present (Some).
    fn has_verification_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the criticality field is present (Some).
    fn has_criticality(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the onset field is present (Some).
    fn has_onset(&self) -> bool;
    /// Returns true if the recorded_date field is present (Some).
    fn has_recorded_date(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the asserter field is present (Some).
    fn has_asserter(&self) -> bool;
    /// Returns true if the last_occurrence field is present (Some).
    fn has_last_occurrence(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the reaction field is not empty.
    fn has_reaction(&self) -> bool;
}
