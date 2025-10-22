use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medicinal_product::MedicinalProductManufacturingbusinessoperation;
use crate::resources::medicinal_product::MedicinalProductName;
use crate::resources::medicinal_product::MedicinalProductSpecialdesignation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the domain field.
    fn domain(&self) -> Option<Coding>;
    /// Returns a reference to the combinedPharmaceuticalDoseForm field.
    fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the legalStatusOfSupply field.
    fn legal_status_of_supply(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the additionalMonitoringIndicator field.
    fn additional_monitoring_indicator(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specialMeasures field.
    fn special_measures(&self) -> &[StringType];
    /// Returns a reference to the paediatricUseIndicator field.
    fn paediatric_use_indicator(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the productClassification field.
    fn product_classification(&self) -> &[CodeableConcept];
    /// Returns a reference to the marketingStatus field.
    fn marketing_status(&self) -> &[MarketingStatus];
    /// Returns a reference to the pharmaceuticalProduct field.
    fn pharmaceutical_product(&self) -> &[Reference];
    /// Returns a reference to the packagedMedicinalProduct field.
    fn packaged_medicinal_product(&self) -> &[Reference];
    /// Returns a reference to the attachedDocument field.
    fn attached_document(&self) -> &[Reference];
    /// Returns a reference to the masterFile field.
    fn master_file(&self) -> &[Reference];
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[Reference];
    /// Returns a reference to the clinicalTrial field.
    fn clinical_trial(&self) -> &[Reference];
    /// Returns a reference to the name field.
    fn name(&self) -> &[MedicinalProductName];
    /// Returns a reference to the crossReference field.
    fn cross_reference(&self) -> &[Identifier];
    /// Returns a reference to the manufacturingBusinessOperation field.
    fn manufacturing_business_operation(&self)
        -> &[MedicinalProductManufacturingbusinessoperation];
    /// Returns a reference to the specialDesignation field.
    fn special_designation(&self) -> &[MedicinalProductSpecialdesignation];
}
/// MedicinalProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductMutators: DomainResourceMutators {
    /// Create a new MedicinalProduct with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product::MedicinalProduct;
    /// use hl7_fhir_r4_core::traits::medicinal_product::MedicinalProductMutators;
    ///
    /// let resource = MedicinalProduct::new();
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
    fn set_domain(self, value: Coding) -> Self;
    /// Sets the combinedPharmaceuticalDoseForm field and returns self for chaining.
    fn set_combined_pharmaceutical_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the legalStatusOfSupply field and returns self for chaining.
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self;
    /// Sets the additionalMonitoringIndicator field and returns self for chaining.
    fn set_additional_monitoring_indicator(self, value: CodeableConcept) -> Self;
    /// Sets the specialMeasures field and returns self for chaining.
    fn set_special_measures(self, value: Vec<String>) -> Self;
    /// Adds an item to the specialMeasures field and returns self for chaining.
    fn add_special_measures(self, item: String) -> Self;
    /// Sets the paediatricUseIndicator field and returns self for chaining.
    fn set_paediatric_use_indicator(self, value: CodeableConcept) -> Self;
    /// Sets the productClassification field and returns self for chaining.
    fn set_product_classification(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the productClassification field and returns self for chaining.
    fn add_product_classification(self, item: CodeableConcept) -> Self;
    /// Sets the marketingStatus field and returns self for chaining.
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self;
    /// Adds an item to the marketingStatus field and returns self for chaining.
    fn add_marketing_status(self, item: MarketingStatus) -> Self;
    /// Sets the pharmaceuticalProduct field and returns self for chaining.
    fn set_pharmaceutical_product(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the pharmaceuticalProduct field and returns self for chaining.
    fn add_pharmaceutical_product(self, item: Reference) -> Self;
    /// Sets the packagedMedicinalProduct field and returns self for chaining.
    fn set_packaged_medicinal_product(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the packagedMedicinalProduct field and returns self for chaining.
    fn add_packaged_medicinal_product(self, item: Reference) -> Self;
    /// Sets the attachedDocument field and returns self for chaining.
    fn set_attached_document(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the attachedDocument field and returns self for chaining.
    fn add_attached_document(self, item: Reference) -> Self;
    /// Sets the masterFile field and returns self for chaining.
    fn set_master_file(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the masterFile field and returns self for chaining.
    fn add_master_file(self, item: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: Reference) -> Self;
    /// Sets the clinicalTrial field and returns self for chaining.
    fn set_clinical_trial(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the clinicalTrial field and returns self for chaining.
    fn add_clinical_trial(self, item: Reference) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<MedicinalProductName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: MedicinalProductName) -> Self;
    /// Sets the crossReference field and returns self for chaining.
    fn set_cross_reference(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the crossReference field and returns self for chaining.
    fn add_cross_reference(self, item: Identifier) -> Self;
    /// Sets the manufacturingBusinessOperation field and returns self for chaining.
    fn set_manufacturing_business_operation(
        self,
        value: Vec<MedicinalProductManufacturingbusinessoperation>,
    ) -> Self;
    /// Adds an item to the manufacturingBusinessOperation field and returns self for chaining.
    fn add_manufacturing_business_operation(
        self,
        item: MedicinalProductManufacturingbusinessoperation,
    ) -> Self;
    /// Sets the specialDesignation field and returns self for chaining.
    fn set_special_designation(self, value: Vec<MedicinalProductSpecialdesignation>) -> Self;
    /// Adds an item to the specialDesignation field and returns self for chaining.
    fn add_special_designation(self, item: MedicinalProductSpecialdesignation) -> Self;
}
/// MedicinalProduct Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the domain field is present (Some).
    fn has_domain(&self) -> bool;
    /// Returns true if the combined_pharmaceutical_dose_form field is present (Some).
    fn has_combined_pharmaceutical_dose_form(&self) -> bool;
    /// Returns true if the legal_status_of_supply field is present (Some).
    fn has_legal_status_of_supply(&self) -> bool;
    /// Returns true if the additional_monitoring_indicator field is present (Some).
    fn has_additional_monitoring_indicator(&self) -> bool;
    /// Returns true if the special_measures field is not empty.
    fn has_special_measures(&self) -> bool;
    /// Returns true if the paediatric_use_indicator field is present (Some).
    fn has_paediatric_use_indicator(&self) -> bool;
    /// Returns true if the product_classification field is not empty.
    fn has_product_classification(&self) -> bool;
    /// Returns true if the marketing_status field is not empty.
    fn has_marketing_status(&self) -> bool;
    /// Returns true if the pharmaceutical_product field is not empty.
    fn has_pharmaceutical_product(&self) -> bool;
    /// Returns true if the packaged_medicinal_product field is not empty.
    fn has_packaged_medicinal_product(&self) -> bool;
    /// Returns true if the attached_document field is not empty.
    fn has_attached_document(&self) -> bool;
    /// Returns true if the master_file field is not empty.
    fn has_master_file(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the clinical_trial field is not empty.
    fn has_clinical_trial(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the cross_reference field is not empty.
    fn has_cross_reference(&self) -> bool;
    /// Returns true if the manufacturing_business_operation field is not empty.
    fn has_manufacturing_business_operation(&self) -> bool;
    /// Returns true if the special_designation field is not empty.
    fn has_special_designation(&self) -> bool;
}
