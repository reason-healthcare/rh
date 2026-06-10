use crate::bindings::medication_statement_status::MedicationStatementStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::medication_statement::MedicationStatementAdherence;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicationStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains.   The primary difference between a medicationstatement and a medicationadministration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medicationstatement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the Medication Statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationStatementAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> MedicationStatementStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the medication field.
    fn medication(&self) -> CodeableReference;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the dateAsserted field.
    fn date_asserted(&self) -> Option<DateTimeType>;
    /// Returns a reference to the informationSource field.
    fn information_source(&self) -> &[Reference];
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[Reference];
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the relatedClinicalInformation field.
    fn related_clinical_information(&self) -> &[Reference];
    /// Returns a reference to the renderedDosageInstruction field.
    fn rendered_dosage_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the dosage field.
    fn dosage(&self) -> &[Dosage];
    /// Returns a reference to the adherence field.
    fn adherence(&self) -> Option<MedicationStatementAdherence>;
}
/// MedicationStatement Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains.   The primary difference between a medicationstatement and a medicationadministration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medicationstatement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the Medication Statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationStatementMutators: DomainResourceMutators {
    /// Create a new MedicationStatement with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::medication_statement::MedicationStatement;
    /// use rh_hl7_fhir_r5_core::traits::medication_statement::MedicationStatementMutators;
    ///
    /// let resource = MedicationStatement::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationStatementStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the medication field and returns self for chaining.
    fn set_medication(self, value: CodeableReference) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the dateAsserted field and returns self for chaining.
    fn set_date_asserted(self, value: String) -> Self;
    /// Sets the informationSource field and returns self for chaining.
    fn set_information_source(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the informationSource field and returns self for chaining.
    fn add_information_source(self, item: Reference) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the relatedClinicalInformation field and returns self for chaining.
    fn set_related_clinical_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the relatedClinicalInformation field and returns self for chaining.
    fn add_related_clinical_information(self, item: Reference) -> Self;
    /// Sets the renderedDosageInstruction field and returns self for chaining.
    fn set_rendered_dosage_instruction(self, value: String) -> Self;
    /// Sets the dosage field and returns self for chaining.
    fn set_dosage(self, value: Vec<Dosage>) -> Self;
    /// Adds an item to the dosage field and returns self for chaining.
    fn add_dosage(self, item: Dosage) -> Self;
    /// Sets the adherence field and returns self for chaining.
    fn set_adherence(self, value: MedicationStatementAdherence) -> Self;
}
/// MedicationStatement Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains.
///
/// The primary difference between a medicationstatement and a medicationadministration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medicationstatement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the Medication Statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationStatementExistence: DomainResourceExistence {
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
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the medication field is present (Some).
    fn has_medication(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the effective field is present (Some).
    fn has_effective(&self) -> bool;
    /// Returns true if the date_asserted field is present (Some).
    fn has_date_asserted(&self) -> bool;
    /// Returns true if the information_source field is not empty.
    fn has_information_source(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the related_clinical_information field is not empty.
    fn has_related_clinical_information(&self) -> bool;
    /// Returns true if the rendered_dosage_instruction field is present (Some).
    fn has_rendered_dosage_instruction(&self) -> bool;
    /// Returns true if the dosage field is not empty.
    fn has_dosage(&self) -> bool;
    /// Returns true if the adherence field is present (Some).
    fn has_adherence(&self) -> bool;
}
