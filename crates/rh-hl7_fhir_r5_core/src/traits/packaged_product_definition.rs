use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::packaged_product_definition::PackagedProductDefinitionLegalstatusofsupply;
use crate::resources::packaged_product_definition::PackagedProductDefinitionPackaging;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// PackagedProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medically related item or items, in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PackagedProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PackagedProductDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the packageFor field.
    fn package_for(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the containedItemQuantity field.
    fn contained_item_quantity(&self) -> &[Quantity];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the legalStatusOfSupply field.
    fn legal_status_of_supply(&self) -> &[PackagedProductDefinitionLegalstatusofsupply];
    /// Returns a reference to the marketingStatus field.
    fn marketing_status(&self) -> &[MarketingStatus];
    /// Returns a reference to the copackagedIndicator field.
    fn copackaged_indicator(&self) -> Option<BooleanType>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the attachedDocument field.
    fn attached_document(&self) -> &[Reference];
    /// Returns a reference to the packaging field.
    fn packaging(&self) -> Option<PackagedProductDefinitionPackaging>;
}
/// PackagedProductDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A medically related item or items, in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PackagedProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PackagedProductDefinitionMutators: DomainResourceMutators {
    /// Create a new PackagedProductDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::packaged_product_definition::PackagedProductDefinition;
    /// use rh_hl7_fhir_r5_core::traits::packaged_product_definition::PackagedProductDefinitionMutators;
    ///
    /// let resource = PackagedProductDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the packageFor field and returns self for chaining.
    fn set_package_for(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the packageFor field and returns self for chaining.
    fn add_package_for(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the containedItemQuantity field and returns self for chaining.
    fn set_contained_item_quantity(self, value: Vec<Quantity>) -> Self;
    /// Adds an item to the containedItemQuantity field and returns self for chaining.
    fn add_contained_item_quantity(self, item: Quantity) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the legalStatusOfSupply field and returns self for chaining.
    fn set_legal_status_of_supply(
        self,
        value: Vec<PackagedProductDefinitionLegalstatusofsupply>,
    ) -> Self;
    /// Adds an item to the legalStatusOfSupply field and returns self for chaining.
    fn add_legal_status_of_supply(self, item: PackagedProductDefinitionLegalstatusofsupply)
        -> Self;
    /// Sets the marketingStatus field and returns self for chaining.
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self;
    /// Adds an item to the marketingStatus field and returns self for chaining.
    fn add_marketing_status(self, item: MarketingStatus) -> Self;
    /// Sets the copackagedIndicator field and returns self for chaining.
    fn set_copackaged_indicator(self, value: bool) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the attachedDocument field and returns self for chaining.
    fn set_attached_document(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the attachedDocument field and returns self for chaining.
    fn add_attached_document(self, item: Reference) -> Self;
    /// Sets the packaging field and returns self for chaining.
    fn set_packaging(self, value: PackagedProductDefinitionPackaging) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<String>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: String) -> Self;
}
/// PackagedProductDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A medically related item or items, in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PackagedProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait PackagedProductDefinitionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the package_for field is not empty.
    fn has_package_for(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the contained_item_quantity field is not empty.
    fn has_contained_item_quantity(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the legal_status_of_supply field is not empty.
    fn has_legal_status_of_supply(&self) -> bool;
    /// Returns true if the marketing_status field is not empty.
    fn has_marketing_status(&self) -> bool;
    /// Returns true if the copackaged_indicator field is present (Some).
    fn has_copackaged_indicator(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the attached_document field is not empty.
    fn has_attached_document(&self) -> bool;
    /// Returns true if the packaging field is present (Some).
    fn has_packaging(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
}
