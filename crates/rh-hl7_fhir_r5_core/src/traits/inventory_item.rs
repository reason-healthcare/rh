use crate::bindings::inventoryitem_status::InventoryitemStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::resources::inventory_item::InventoryItemAssociation;
use crate::resources::inventory_item::InventoryItemCharacteristic;
use crate::resources::inventory_item::InventoryItemDescription;
use crate::resources::inventory_item::InventoryItemInstance;
use crate::resources::inventory_item::InventoryItemName;
use crate::resources::inventory_item::InventoryItemResponsibleorganization;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// InventoryItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// functional description of an inventory item used in inventory and supply-related workflows.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryItemAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> InventoryitemStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> &[CodeableConcept];
    /// Returns a reference to the name field.
    fn name(&self) -> &[InventoryItemName];
    /// Returns a reference to the responsibleOrganization field.
    fn responsible_organization(&self) -> &[InventoryItemResponsibleorganization];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<InventoryItemDescription>;
    /// Returns a reference to the inventoryStatus field.
    fn inventory_status(&self) -> &[CodeableConcept];
    /// Returns a reference to the baseUnit field.
    fn base_unit(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the netContent field.
    fn net_content(&self) -> Option<Quantity>;
    /// Returns a reference to the association field.
    fn association(&self) -> &[InventoryItemAssociation];
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[InventoryItemCharacteristic];
    /// Returns a reference to the instance field.
    fn instance(&self) -> Option<InventoryItemInstance>;
    /// Returns a reference to the productReference field.
    fn product_reference(&self) -> Option<Reference>;
}
/// InventoryItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// functional description of an inventory item used in inventory and supply-related workflows.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryItemMutators: DomainResourceMutators {
    /// Create a new InventoryItem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::inventory_item::InventoryItem;
    /// use rh_hl7_fhir_r5_core::traits::inventory_item::InventoryItemMutators;
    ///
    /// let resource = InventoryItem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: InventoryitemStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: CodeableConcept) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<InventoryItemName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: InventoryItemName) -> Self;
    /// Sets the responsibleOrganization field and returns self for chaining.
    fn set_responsible_organization(self, value: Vec<InventoryItemResponsibleorganization>)
        -> Self;
    /// Adds an item to the responsibleOrganization field and returns self for chaining.
    fn add_responsible_organization(self, item: InventoryItemResponsibleorganization) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: InventoryItemDescription) -> Self;
    /// Sets the inventoryStatus field and returns self for chaining.
    fn set_inventory_status(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the inventoryStatus field and returns self for chaining.
    fn add_inventory_status(self, item: CodeableConcept) -> Self;
    /// Sets the baseUnit field and returns self for chaining.
    fn set_base_unit(self, value: CodeableConcept) -> Self;
    /// Sets the netContent field and returns self for chaining.
    fn set_net_content(self, value: Quantity) -> Self;
    /// Sets the association field and returns self for chaining.
    fn set_association(self, value: Vec<InventoryItemAssociation>) -> Self;
    /// Adds an item to the association field and returns self for chaining.
    fn add_association(self, item: InventoryItemAssociation) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<InventoryItemCharacteristic>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: InventoryItemCharacteristic) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: InventoryItemInstance) -> Self;
    /// Sets the productReference field and returns self for chaining.
    fn set_product_reference(self, value: Reference) -> Self;
}
/// InventoryItem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// functional description of an inventory item used in inventory and supply-related workflows.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait InventoryItemExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the responsible_organization field is not empty.
    fn has_responsible_organization(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the inventory_status field is not empty.
    fn has_inventory_status(&self) -> bool;
    /// Returns true if the base_unit field is present (Some).
    fn has_base_unit(&self) -> bool;
    /// Returns true if the net_content field is present (Some).
    fn has_net_content(&self) -> bool;
    /// Returns true if the association field is not empty.
    fn has_association(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the instance field is present (Some).
    fn has_instance(&self) -> bool;
    /// Returns true if the product_reference field is present (Some).
    fn has_product_reference(&self) -> bool;
}
