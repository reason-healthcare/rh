use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::medicinal_product_definition::MedicinalProductDefinitionCharacteristic;
use crate::resources::medicinal_product_definition::MedicinalProductDefinitionContact;
use crate::resources::medicinal_product_definition::MedicinalProductDefinitionCrossreference;
use crate::resources::medicinal_product_definition::MedicinalProductDefinitionName;
use crate::resources::medicinal_product_definition::MedicinalProductDefinitionOperation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicinalProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the domain field.
    fn domain(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the combinedPharmaceuticalDoseForm field.
    fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the route field.
    fn route(&self) -> &[CodeableConcept];
    /// Returns a reference to the indication field.
    fn indication(&self) -> Option<StringType>;
    /// Returns a reference to the legalStatusOfSupply field.
    fn legal_status_of_supply(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the additionalMonitoringIndicator field.
    fn additional_monitoring_indicator(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specialMeasures field.
    fn special_measures(&self) -> &[CodeableConcept];
    /// Returns a reference to the pediatricUseIndicator field.
    fn pediatric_use_indicator(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the classification field.
    fn classification(&self) -> &[CodeableConcept];
    /// Returns a reference to the marketingStatus field.
    fn marketing_status(&self) -> &[MarketingStatus];
    /// Returns a reference to the packagedMedicinalProduct field.
    fn packaged_medicinal_product(&self) -> &[CodeableConcept];
    /// Returns a reference to the comprisedOf field.
    fn comprised_of(&self) -> &[Reference];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[CodeableConcept];
    /// Returns a reference to the impurity field.
    fn impurity(&self) -> &[CodeableReference];
    /// Returns a reference to the attachedDocument field.
    fn attached_document(&self) -> &[Reference];
    /// Returns a reference to the masterFile field.
    fn master_file(&self) -> &[Reference];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[MedicinalProductDefinitionContact];
    /// Returns a reference to the clinicalTrial field.
    fn clinical_trial(&self) -> &[Reference];
    /// Returns a reference to the code field.
    fn code(&self) -> &[Coding];
    /// Returns a reference to the name field.
    fn name(&self) -> &[MedicinalProductDefinitionName];
    /// Returns a reference to the crossReference field.
    fn cross_reference(&self) -> &[MedicinalProductDefinitionCrossreference];
    /// Returns a reference to the operation field.
    fn operation(&self) -> &[MedicinalProductDefinitionOperation];
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[MedicinalProductDefinitionCharacteristic];
}
/// MedicinalProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicinalProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductDefinitionMutators: DomainResourceMutators {
    /// Create a new MedicinalProductDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::medicinal_product_definition::MedicinalProductDefinition;
    /// use rh_hl7_fhir_r5_core::traits::medicinal_product_definition::MedicinalProductDefinitionMutators;
    ///
    /// let resource = MedicinalProductDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the domain field and returns self for chaining.
    fn set_domain(self, value: CodeableConcept) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the combinedPharmaceuticalDoseForm field and returns self for chaining.
    fn set_combined_pharmaceutical_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the route field and returns self for chaining.
    fn set_route(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the route field and returns self for chaining.
    fn add_route(self, item: CodeableConcept) -> Self;
    /// Sets the indication field and returns self for chaining.
    fn set_indication(self, value: String) -> Self;
    /// Sets the legalStatusOfSupply field and returns self for chaining.
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self;
    /// Sets the additionalMonitoringIndicator field and returns self for chaining.
    fn set_additional_monitoring_indicator(self, value: CodeableConcept) -> Self;
    /// Sets the specialMeasures field and returns self for chaining.
    fn set_special_measures(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialMeasures field and returns self for chaining.
    fn add_special_measures(self, item: CodeableConcept) -> Self;
    /// Sets the pediatricUseIndicator field and returns self for chaining.
    fn set_pediatric_use_indicator(self, value: CodeableConcept) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the classification field and returns self for chaining.
    fn add_classification(self, item: CodeableConcept) -> Self;
    /// Sets the marketingStatus field and returns self for chaining.
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self;
    /// Adds an item to the marketingStatus field and returns self for chaining.
    fn add_marketing_status(self, item: MarketingStatus) -> Self;
    /// Sets the packagedMedicinalProduct field and returns self for chaining.
    fn set_packaged_medicinal_product(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the packagedMedicinalProduct field and returns self for chaining.
    fn add_packaged_medicinal_product(self, item: CodeableConcept) -> Self;
    /// Sets the comprisedOf field and returns self for chaining.
    fn set_comprised_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the comprisedOf field and returns self for chaining.
    fn add_comprised_of(self, item: Reference) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: CodeableConcept) -> Self;
    /// Sets the impurity field and returns self for chaining.
    fn set_impurity(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the impurity field and returns self for chaining.
    fn add_impurity(self, item: CodeableReference) -> Self;
    /// Sets the attachedDocument field and returns self for chaining.
    fn set_attached_document(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the attachedDocument field and returns self for chaining.
    fn add_attached_document(self, item: Reference) -> Self;
    /// Sets the masterFile field and returns self for chaining.
    fn set_master_file(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the masterFile field and returns self for chaining.
    fn add_master_file(self, item: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<MedicinalProductDefinitionContact>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: MedicinalProductDefinitionContact) -> Self;
    /// Sets the clinicalTrial field and returns self for chaining.
    fn set_clinical_trial(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the clinicalTrial field and returns self for chaining.
    fn add_clinical_trial(self, item: Reference) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: Coding) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<MedicinalProductDefinitionName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: MedicinalProductDefinitionName) -> Self;
    /// Sets the crossReference field and returns self for chaining.
    fn set_cross_reference(self, value: Vec<MedicinalProductDefinitionCrossreference>) -> Self;
    /// Adds an item to the crossReference field and returns self for chaining.
    fn add_cross_reference(self, item: MedicinalProductDefinitionCrossreference) -> Self;
    /// Sets the operation field and returns self for chaining.
    fn set_operation(self, value: Vec<MedicinalProductDefinitionOperation>) -> Self;
    /// Adds an item to the operation field and returns self for chaining.
    fn add_operation(self, item: MedicinalProductDefinitionOperation) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<MedicinalProductDefinitionCharacteristic>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: MedicinalProductDefinitionCharacteristic) -> Self;
}
/// MedicinalProductDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicinalProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductDefinitionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the domain field is present (Some).
    fn has_domain(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the combined_pharmaceutical_dose_form field is present (Some).
    fn has_combined_pharmaceutical_dose_form(&self) -> bool;
    /// Returns true if the route field is not empty.
    fn has_route(&self) -> bool;
    /// Returns true if the indication field is present (Some).
    fn has_indication(&self) -> bool;
    /// Returns true if the legal_status_of_supply field is present (Some).
    fn has_legal_status_of_supply(&self) -> bool;
    /// Returns true if the additional_monitoring_indicator field is present (Some).
    fn has_additional_monitoring_indicator(&self) -> bool;
    /// Returns true if the special_measures field is not empty.
    fn has_special_measures(&self) -> bool;
    /// Returns true if the pediatric_use_indicator field is present (Some).
    fn has_pediatric_use_indicator(&self) -> bool;
    /// Returns true if the classification field is not empty.
    fn has_classification(&self) -> bool;
    /// Returns true if the marketing_status field is not empty.
    fn has_marketing_status(&self) -> bool;
    /// Returns true if the packaged_medicinal_product field is not empty.
    fn has_packaged_medicinal_product(&self) -> bool;
    /// Returns true if the comprised_of field is not empty.
    fn has_comprised_of(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the impurity field is not empty.
    fn has_impurity(&self) -> bool;
    /// Returns true if the attached_document field is not empty.
    fn has_attached_document(&self) -> bool;
    /// Returns true if the master_file field is not empty.
    fn has_master_file(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the clinical_trial field is not empty.
    fn has_clinical_trial(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the cross_reference field is not empty.
    fn has_cross_reference(&self) -> bool;
    /// Returns true if the operation field is not empty.
    fn has_operation(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
}
