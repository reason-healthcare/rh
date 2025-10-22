use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::catalog_entry::CatalogEntryRelatedentry;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// CatalogEntry Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Catalog entries are wrappers that contextualize items included in a catalog.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CatalogEntry
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CatalogEntry
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CatalogEntryAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the orderable field.
    fn orderable(&self) -> BooleanType;
    /// Returns a reference to the referencedItem field.
    fn referenced_item(&self) -> Reference;
    /// Returns a reference to the additionalIdentifier field.
    fn additional_identifier(&self) -> &[Identifier];
    /// Returns a reference to the classification field.
    fn classification(&self) -> &[CodeableConcept];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<PublicationStatus>;
    /// Returns a reference to the validityPeriod field.
    fn validity_period(&self) -> Option<Period>;
    /// Returns a reference to the validTo field.
    fn valid_to(&self) -> Option<DateTimeType>;
    /// Returns a reference to the lastUpdated field.
    fn last_updated(&self) -> Option<DateTimeType>;
    /// Returns a reference to the additionalCharacteristic field.
    fn additional_characteristic(&self) -> &[CodeableConcept];
    /// Returns a reference to the additionalClassification field.
    fn additional_classification(&self) -> &[CodeableConcept];
    /// Returns a reference to the relatedEntry field.
    fn related_entry(&self) -> &[CatalogEntryRelatedentry];
}
/// CatalogEntry Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Catalog entries are wrappers that contextualize items included in a catalog.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CatalogEntry
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CatalogEntry
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CatalogEntryMutators: DomainResourceMutators {
    /// Create a new CatalogEntry with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::catalog_entry::CatalogEntry;
    /// use hl7_fhir_r4_core::traits::catalog_entry::CatalogEntryMutators;
    ///
    /// let resource = CatalogEntry::new();
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
    /// Sets the orderable field and returns self for chaining.
    fn set_orderable(self, value: bool) -> Self;
    /// Sets the referencedItem field and returns self for chaining.
    fn set_referenced_item(self, value: Reference) -> Self;
    /// Sets the additionalIdentifier field and returns self for chaining.
    fn set_additional_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the additionalIdentifier field and returns self for chaining.
    fn add_additional_identifier(self, item: Identifier) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the classification field and returns self for chaining.
    fn add_classification(self, item: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the validityPeriod field and returns self for chaining.
    fn set_validity_period(self, value: Period) -> Self;
    /// Sets the validTo field and returns self for chaining.
    fn set_valid_to(self, value: String) -> Self;
    /// Sets the lastUpdated field and returns self for chaining.
    fn set_last_updated(self, value: String) -> Self;
    /// Sets the additionalCharacteristic field and returns self for chaining.
    fn set_additional_characteristic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the additionalCharacteristic field and returns self for chaining.
    fn add_additional_characteristic(self, item: CodeableConcept) -> Self;
    /// Sets the additionalClassification field and returns self for chaining.
    fn set_additional_classification(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the additionalClassification field and returns self for chaining.
    fn add_additional_classification(self, item: CodeableConcept) -> Self;
    /// Sets the relatedEntry field and returns self for chaining.
    fn set_related_entry(self, value: Vec<CatalogEntryRelatedentry>) -> Self;
    /// Adds an item to the relatedEntry field and returns self for chaining.
    fn add_related_entry(self, item: CatalogEntryRelatedentry) -> Self;
}
/// CatalogEntry Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Catalog entries are wrappers that contextualize items included in a catalog.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CatalogEntry
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CatalogEntry
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait CatalogEntryExistence: DomainResourceExistence {
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
    /// Returns true if the orderable field is present (Some).
    fn has_orderable(&self) -> bool;
    /// Returns true if the referenced_item field is present (Some).
    fn has_referenced_item(&self) -> bool;
    /// Returns true if the additional_identifier field is not empty.
    fn has_additional_identifier(&self) -> bool;
    /// Returns true if the classification field is not empty.
    fn has_classification(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the validity_period field is present (Some).
    fn has_validity_period(&self) -> bool;
    /// Returns true if the valid_to field is present (Some).
    fn has_valid_to(&self) -> bool;
    /// Returns true if the last_updated field is present (Some).
    fn has_last_updated(&self) -> bool;
    /// Returns true if the additional_characteristic field is not empty.
    fn has_additional_characteristic(&self) -> bool;
    /// Returns true if the additional_classification field is not empty.
    fn has_additional_classification(&self) -> bool;
    /// Returns true if the related_entry field is not empty.
    fn has_related_entry(&self) -> bool;
}
