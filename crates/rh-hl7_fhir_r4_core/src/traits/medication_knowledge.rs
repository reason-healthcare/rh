use crate::bindings::medicationknowledge_status::MedicationknowledgeStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medication_knowledge::MedicationKnowledgeAdministrationguidelines;
use crate::resources::medication_knowledge::MedicationKnowledgeCost;
use crate::resources::medication_knowledge::MedicationKnowledgeDrugcharacteristic;
use crate::resources::medication_knowledge::MedicationKnowledgeIngredient;
use crate::resources::medication_knowledge::MedicationKnowledgeKinetics;
use crate::resources::medication_knowledge::MedicationKnowledgeMedicineclassification;
use crate::resources::medication_knowledge::MedicationKnowledgeMonitoringprogram;
use crate::resources::medication_knowledge::MedicationKnowledgeMonograph;
use crate::resources::medication_knowledge::MedicationKnowledgePackaging;
use crate::resources::medication_knowledge::MedicationKnowledgeRegulatory;
use crate::resources::medication_knowledge::MedicationKnowledgeRelatedmedicationknowledge;
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
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationKnowledge
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationKnowledgeAccessors: DomainResourceAccessors {
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<MedicationknowledgeStatus>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> Option<Reference>;
    /// Returns a reference to the doseForm field.
    fn dose_form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the amount field.
    fn amount(&self) -> Option<Quantity>;
    /// Returns a reference to the synonym field.
    fn synonym(&self) -> &[StringType];
    /// Returns a reference to the relatedMedicationKnowledge field.
    fn related_medication_knowledge(&self) -> &[MedicationKnowledgeRelatedmedicationknowledge];
    /// Returns a reference to the associatedMedication field.
    fn associated_medication(&self) -> &[Reference];
    /// Returns a reference to the productType field.
    fn product_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the monograph field.
    fn monograph(&self) -> &[MedicationKnowledgeMonograph];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[MedicationKnowledgeIngredient];
    /// Returns a reference to the preparationInstruction field.
    fn preparation_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the intendedRoute field.
    fn intended_route(&self) -> &[CodeableConcept];
    /// Returns a reference to the cost field.
    fn cost(&self) -> &[MedicationKnowledgeCost];
    /// Returns a reference to the monitoringProgram field.
    fn monitoring_program(&self) -> &[MedicationKnowledgeMonitoringprogram];
    /// Returns a reference to the administrationGuidelines field.
    fn administration_guidelines(&self) -> &[MedicationKnowledgeAdministrationguidelines];
    /// Returns a reference to the medicineClassification field.
    fn medicine_classification(&self) -> &[MedicationKnowledgeMedicineclassification];
    /// Returns a reference to the packaging field.
    fn packaging(&self) -> Option<MedicationKnowledgePackaging>;
    /// Returns a reference to the drugCharacteristic field.
    fn drug_characteristic(&self) -> &[MedicationKnowledgeDrugcharacteristic];
    /// Returns a reference to the contraindication field.
    fn contraindication(&self) -> &[Reference];
    /// Returns a reference to the regulatory field.
    fn regulatory(&self) -> &[MedicationKnowledgeRegulatory];
    /// Returns a reference to the kinetics field.
    fn kinetics(&self) -> &[MedicationKnowledgeKinetics];
}
/// MedicationKnowledge Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::medication_knowledge::MedicationKnowledge;
    /// use hl7_fhir_r4_core::traits::medication_knowledge::MedicationKnowledgeMutators;
    ///
    /// let resource = MedicationKnowledge::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationknowledgeStatus) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Reference) -> Self;
    /// Sets the doseForm field and returns self for chaining.
    fn set_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the amount field and returns self for chaining.
    fn set_amount(self, value: Quantity) -> Self;
    /// Sets the synonym field and returns self for chaining.
    fn set_synonym(self, value: Vec<String>) -> Self;
    /// Adds an item to the synonym field and returns self for chaining.
    fn add_synonym(self, item: String) -> Self;
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
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<MedicationKnowledgeIngredient>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: MedicationKnowledgeIngredient) -> Self;
    /// Sets the preparationInstruction field and returns self for chaining.
    fn set_preparation_instruction(self, value: String) -> Self;
    /// Sets the intendedRoute field and returns self for chaining.
    fn set_intended_route(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the intendedRoute field and returns self for chaining.
    fn add_intended_route(self, item: CodeableConcept) -> Self;
    /// Sets the cost field and returns self for chaining.
    fn set_cost(self, value: Vec<MedicationKnowledgeCost>) -> Self;
    /// Adds an item to the cost field and returns self for chaining.
    fn add_cost(self, item: MedicationKnowledgeCost) -> Self;
    /// Sets the monitoringProgram field and returns self for chaining.
    fn set_monitoring_program(self, value: Vec<MedicationKnowledgeMonitoringprogram>) -> Self;
    /// Adds an item to the monitoringProgram field and returns self for chaining.
    fn add_monitoring_program(self, item: MedicationKnowledgeMonitoringprogram) -> Self;
    /// Sets the administrationGuidelines field and returns self for chaining.
    fn set_administration_guidelines(
        self,
        value: Vec<MedicationKnowledgeAdministrationguidelines>,
    ) -> Self;
    /// Adds an item to the administrationGuidelines field and returns self for chaining.
    fn add_administration_guidelines(
        self,
        item: MedicationKnowledgeAdministrationguidelines,
    ) -> Self;
    /// Sets the medicineClassification field and returns self for chaining.
    fn set_medicine_classification(
        self,
        value: Vec<MedicationKnowledgeMedicineclassification>,
    ) -> Self;
    /// Adds an item to the medicineClassification field and returns self for chaining.
    fn add_medicine_classification(self, item: MedicationKnowledgeMedicineclassification) -> Self;
    /// Sets the packaging field and returns self for chaining.
    fn set_packaging(self, value: MedicationKnowledgePackaging) -> Self;
    /// Sets the drugCharacteristic field and returns self for chaining.
    fn set_drug_characteristic(self, value: Vec<MedicationKnowledgeDrugcharacteristic>) -> Self;
    /// Adds an item to the drugCharacteristic field and returns self for chaining.
    fn add_drug_characteristic(self, item: MedicationKnowledgeDrugcharacteristic) -> Self;
    /// Sets the contraindication field and returns self for chaining.
    fn set_contraindication(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the contraindication field and returns self for chaining.
    fn add_contraindication(self, item: Reference) -> Self;
    /// Sets the regulatory field and returns self for chaining.
    fn set_regulatory(self, value: Vec<MedicationKnowledgeRegulatory>) -> Self;
    /// Adds an item to the regulatory field and returns self for chaining.
    fn add_regulatory(self, item: MedicationKnowledgeRegulatory) -> Self;
    /// Sets the kinetics field and returns self for chaining.
    fn set_kinetics(self, value: Vec<MedicationKnowledgeKinetics>) -> Self;
    /// Adds an item to the kinetics field and returns self for chaining.
    fn add_kinetics(self, item: MedicationKnowledgeKinetics) -> Self;
}
/// MedicationKnowledge Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Information about a medication that is used to support knowledge.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationKnowledge
/// - Version: 4.0.1
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the manufacturer field is present (Some).
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the dose_form field is present (Some).
    fn has_dose_form(&self) -> bool;
    /// Returns true if the amount field is present (Some).
    fn has_amount(&self) -> bool;
    /// Returns true if the synonym field is not empty.
    fn has_synonym(&self) -> bool;
    /// Returns true if the related_medication_knowledge field is not empty.
    fn has_related_medication_knowledge(&self) -> bool;
    /// Returns true if the associated_medication field is not empty.
    fn has_associated_medication(&self) -> bool;
    /// Returns true if the product_type field is not empty.
    fn has_product_type(&self) -> bool;
    /// Returns true if the monograph field is not empty.
    fn has_monograph(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the preparation_instruction field is present (Some).
    fn has_preparation_instruction(&self) -> bool;
    /// Returns true if the intended_route field is not empty.
    fn has_intended_route(&self) -> bool;
    /// Returns true if the cost field is not empty.
    fn has_cost(&self) -> bool;
    /// Returns true if the monitoring_program field is not empty.
    fn has_monitoring_program(&self) -> bool;
    /// Returns true if the administration_guidelines field is not empty.
    fn has_administration_guidelines(&self) -> bool;
    /// Returns true if the medicine_classification field is not empty.
    fn has_medicine_classification(&self) -> bool;
    /// Returns true if the packaging field is present (Some).
    fn has_packaging(&self) -> bool;
    /// Returns true if the drug_characteristic field is not empty.
    fn has_drug_characteristic(&self) -> bool;
    /// Returns true if the contraindication field is not empty.
    fn has_contraindication(&self) -> bool;
    /// Returns true if the regulatory field is not empty.
    fn has_regulatory(&self) -> bool;
    /// Returns true if the kinetics field is not empty.
    fn has_kinetics(&self) -> bool;
}
