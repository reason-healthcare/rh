use crate::bindings::medicationknowledge_status::MedicationknowledgeStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medication_knowledge::MedicationKnowledgeCost;
use crate::resources::medication_knowledge::MedicationKnowledgeDefinitional;
use crate::resources::medication_knowledge::MedicationKnowledgeIndicationguideline;
use crate::resources::medication_knowledge::MedicationKnowledgeMedicineclassification;
use crate::resources::medication_knowledge::MedicationKnowledgeMonitoringprogram;
use crate::resources::medication_knowledge::MedicationKnowledgeMonograph;
use crate::resources::medication_knowledge::MedicationKnowledgePackaging;
use crate::resources::medication_knowledge::MedicationKnowledgeRegulatory;
use crate::resources::medication_knowledge::MedicationKnowledgeRelatedmedicationknowledge;
use crate::resources::medication_knowledge::MedicationKnowledgeStorageguideline;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicationKnowledge Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationKnowledgeAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<MedicationknowledgeStatus>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the intendedJurisdiction field.
    fn intended_jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> &[StringType];
    /// Returns a reference to the relatedMedicationKnowledge field.
    fn related_medication_knowledge(&self) -> &[MedicationKnowledgeRelatedmedicationknowledge];
    /// Returns a reference to the associatedMedication field.
    fn associated_medication(&self) -> &[Reference];
    /// Returns a reference to the productType field.
    fn product_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the monograph field.
    fn monograph(&self) -> &[MedicationKnowledgeMonograph];
    /// Returns a reference to the preparationInstruction field.
    fn preparation_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the cost field.
    fn cost(&self) -> &[MedicationKnowledgeCost];
    /// Returns a reference to the monitoringProgram field.
    fn monitoring_program(&self) -> &[MedicationKnowledgeMonitoringprogram];
    /// Returns a reference to the indicationGuideline field.
    fn indication_guideline(&self) -> &[MedicationKnowledgeIndicationguideline];
    /// Returns a reference to the medicineClassification field.
    fn medicine_classification(&self) -> &[MedicationKnowledgeMedicineclassification];
    /// Returns a reference to the packaging field.
    fn packaging(&self) -> &[MedicationKnowledgePackaging];
    /// Returns a reference to the clinicalUseIssue field.
    fn clinical_use_issue(&self) -> &[Reference];
    /// Returns a reference to the storageGuideline field.
    fn storage_guideline(&self) -> &[MedicationKnowledgeStorageguideline];
    /// Returns a reference to the regulatory field.
    fn regulatory(&self) -> &[MedicationKnowledgeRegulatory];
    /// Returns a reference to the definitional field.
    fn definitional(&self) -> Option<MedicationKnowledgeDefinitional>;
}
/// MedicationKnowledge Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationKnowledgeMutators: DomainResourceMutators {
    /// Create a new MedicationKnowledge with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::medication_knowledge::MedicationKnowledge;
    /// use rh_hl7_fhir_r5_core::traits::medication_knowledge::MedicationKnowledgeMutators;
    ///
    /// let resource = MedicationKnowledge::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationknowledgeStatus) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the intendedJurisdiction field and returns self for chaining.
    fn set_intended_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the intendedJurisdiction field and returns self for chaining.
    fn add_intended_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<String>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: String) -> Self;
    /// Sets the relatedMedicationKnowledge field and returns self for chaining.
    fn set_related_medication_knowledge(
        self,
        value: Vec<MedicationKnowledgeRelatedmedicationknowledge>,
    ) -> Self;
    /// Adds an item to the relatedMedicationKnowledge field and returns self for chaining.
    fn add_related_medication_knowledge(
        self,
        item: MedicationKnowledgeRelatedmedicationknowledge,
    ) -> Self;
    /// Sets the associatedMedication field and returns self for chaining.
    fn set_associated_medication(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the associatedMedication field and returns self for chaining.
    fn add_associated_medication(self, item: Reference) -> Self;
    /// Sets the productType field and returns self for chaining.
    fn set_product_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the productType field and returns self for chaining.
    fn add_product_type(self, item: CodeableConcept) -> Self;
    /// Sets the monograph field and returns self for chaining.
    fn set_monograph(self, value: Vec<MedicationKnowledgeMonograph>) -> Self;
    /// Adds an item to the monograph field and returns self for chaining.
    fn add_monograph(self, item: MedicationKnowledgeMonograph) -> Self;
    /// Sets the preparationInstruction field and returns self for chaining.
    fn set_preparation_instruction(self, value: String) -> Self;
    /// Sets the cost field and returns self for chaining.
    fn set_cost(self, value: Vec<MedicationKnowledgeCost>) -> Self;
    /// Adds an item to the cost field and returns self for chaining.
    fn add_cost(self, item: MedicationKnowledgeCost) -> Self;
    /// Sets the monitoringProgram field and returns self for chaining.
    fn set_monitoring_program(self, value: Vec<MedicationKnowledgeMonitoringprogram>) -> Self;
    /// Adds an item to the monitoringProgram field and returns self for chaining.
    fn add_monitoring_program(self, item: MedicationKnowledgeMonitoringprogram) -> Self;
    /// Sets the indicationGuideline field and returns self for chaining.
    fn set_indication_guideline(self, value: Vec<MedicationKnowledgeIndicationguideline>) -> Self;
    /// Adds an item to the indicationGuideline field and returns self for chaining.
    fn add_indication_guideline(self, item: MedicationKnowledgeIndicationguideline) -> Self;
    /// Sets the medicineClassification field and returns self for chaining.
    fn set_medicine_classification(
        self,
        value: Vec<MedicationKnowledgeMedicineclassification>,
    ) -> Self;
    /// Adds an item to the medicineClassification field and returns self for chaining.
    fn add_medicine_classification(self, item: MedicationKnowledgeMedicineclassification) -> Self;
    /// Sets the packaging field and returns self for chaining.
    fn set_packaging(self, value: Vec<MedicationKnowledgePackaging>) -> Self;
    /// Adds an item to the packaging field and returns self for chaining.
    fn add_packaging(self, item: MedicationKnowledgePackaging) -> Self;
    /// Sets the clinicalUseIssue field and returns self for chaining.
    fn set_clinical_use_issue(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the clinicalUseIssue field and returns self for chaining.
    fn add_clinical_use_issue(self, item: Reference) -> Self;
    /// Sets the storageGuideline field and returns self for chaining.
    fn set_storage_guideline(self, value: Vec<MedicationKnowledgeStorageguideline>) -> Self;
    /// Adds an item to the storageGuideline field and returns self for chaining.
    fn add_storage_guideline(self, item: MedicationKnowledgeStorageguideline) -> Self;
    /// Sets the regulatory field and returns self for chaining.
    fn set_regulatory(self, value: Vec<MedicationKnowledgeRegulatory>) -> Self;
    /// Adds an item to the regulatory field and returns self for chaining.
    fn add_regulatory(self, item: MedicationKnowledgeRegulatory) -> Self;
    /// Sets the definitional field and returns self for chaining.
    fn set_definitional(self, value: MedicationKnowledgeDefinitional) -> Self;
}
/// MedicationKnowledge Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationKnowledgeExistence: DomainResourceExistence {
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the intended_jurisdiction field is not empty.
    fn has_intended_jurisdiction(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the related_medication_knowledge field is not empty.
    fn has_related_medication_knowledge(&self) -> bool;
    /// Returns true if the associated_medication field is not empty.
    fn has_associated_medication(&self) -> bool;
    /// Returns true if the product_type field is not empty.
    fn has_product_type(&self) -> bool;
    /// Returns true if the monograph field is not empty.
    fn has_monograph(&self) -> bool;
    /// Returns true if the preparation_instruction field is present (Some).
    fn has_preparation_instruction(&self) -> bool;
    /// Returns true if the cost field is not empty.
    fn has_cost(&self) -> bool;
    /// Returns true if the monitoring_program field is not empty.
    fn has_monitoring_program(&self) -> bool;
    /// Returns true if the indication_guideline field is not empty.
    fn has_indication_guideline(&self) -> bool;
    /// Returns true if the medicine_classification field is not empty.
    fn has_medicine_classification(&self) -> bool;
    /// Returns true if the packaging field is not empty.
    fn has_packaging(&self) -> bool;
    /// Returns true if the clinical_use_issue field is not empty.
    fn has_clinical_use_issue(&self) -> bool;
    /// Returns true if the storage_guideline field is not empty.
    fn has_storage_guideline(&self) -> bool;
    /// Returns true if the regulatory field is not empty.
    fn has_regulatory(&self) -> bool;
    /// Returns true if the definitional field is present (Some).
    fn has_definitional(&self) -> bool;
}
