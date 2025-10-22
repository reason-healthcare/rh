use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medicinal_product_packaged::MedicinalProductPackagedBatchidentifier;
use crate::resources::medicinal_product_packaged::MedicinalProductPackagedPackageitem;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductPackaged Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medicinal product in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPackaged
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPackaged
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPackagedAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the legalStatusOfSupply field.
    fn legal_status_of_supply(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the marketingStatus field.
    fn marketing_status(&self) -> &[MarketingStatus];
    /// Returns a reference to the marketingAuthorization field.
    fn marketing_authorization(&self) -> Option<Reference>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the batchIdentifier field.
    fn batch_identifier(&self) -> &[MedicinalProductPackagedBatchidentifier];
    /// Returns a reference to the packageItem field.
    fn package_item(&self) -> &[MedicinalProductPackagedPackageitem];
}
/// MedicinalProductPackaged Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medicinal product in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPackaged
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPackaged
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPackagedMutators: DomainResourceMutators {
    /// Create a new MedicinalProductPackaged with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_packaged::MedicinalProductPackaged;
    /// use hl7_fhir_r4_core::traits::medicinal_product_packaged::MedicinalProductPackagedMutators;
    ///
    /// let resource = MedicinalProductPackaged::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the legalStatusOfSupply field and returns self for chaining.
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self;
    /// Sets the marketingStatus field and returns self for chaining.
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self;
    /// Adds an item to the marketingStatus field and returns self for chaining.
    fn add_marketing_status(self, item: MarketingStatus) -> Self;
    /// Sets the marketingAuthorization field and returns self for chaining.
    fn set_marketing_authorization(self, value: Reference) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the batchIdentifier field and returns self for chaining.
    fn set_batch_identifier(self, value: Vec<MedicinalProductPackagedBatchidentifier>) -> Self;
    /// Adds an item to the batchIdentifier field and returns self for chaining.
    fn add_batch_identifier(self, item: MedicinalProductPackagedBatchidentifier) -> Self;
    /// Sets the packageItem field and returns self for chaining.
    fn set_package_item(self, value: Vec<MedicinalProductPackagedPackageitem>) -> Self;
    /// Adds an item to the packageItem field and returns self for chaining.
    fn add_package_item(self, item: MedicinalProductPackagedPackageitem) -> Self;
}
/// MedicinalProductPackaged Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A medicinal product in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPackaged
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPackaged
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductPackagedExistence: DomainResourceExistence {
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
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the legal_status_of_supply field is present (Some).
    fn has_legal_status_of_supply(&self) -> bool;
    /// Returns true if the marketing_status field is not empty.
    fn has_marketing_status(&self) -> bool;
    /// Returns true if the marketing_authorization field is present (Some).
    fn has_marketing_authorization(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the batch_identifier field is not empty.
    fn has_batch_identifier(&self) -> bool;
    /// Returns true if the package_item field is not empty.
    fn has_package_item(&self) -> bool;
}
